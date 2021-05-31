//! HMAC (Hash-based Message Authentication Code).
//!
//! Usage
//! -----
//!
//! ```rust
//! let hmac = &earlgrey::hmac::HMAC;
//!
//! let mux_hmac = static_init!(MuxHmac<'static, lowrisc::hmac::Hmac>, MuxHmac::new(hmac));
//! digest::Digest::set_client(&earlgrey::hmac::HMAC, mux_hmac);
//!
//! let virtual_hmac_user = static_init!(
//!     VirtualMuxHmac<'static, lowrisc::hmac::Hmac>,
//!     VirtualMuxHmac::new(mux_hmac)
//! );
//! let hmac = static_init!(
//!     capsules::hmac::HmacDriver<'static, VirtualMuxHmac<'static, lowrisc::hmac::Hmac>>,
//!     capsules::hmac::HmacDriver::new(
//!         virtual_hmac_user,
//!         board_kernel.create_grant(&memory_allocation_cap),
//!     )
//! );
//! digest::Digest::set_client(virtual_hmac_user, hmac);
//! ```

use crate::driver;
/// Syscall driver number.
pub const DRIVER_NUM: usize = driver::NUM::Hmac as usize;

use core::cell::Cell;
use core::convert::TryInto;
use core::marker::PhantomData;
use core::mem;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::common::leasable_buffer::LeasableBuffer;
use kernel::hil::digest;
use kernel::hil::digest::DigestType;
use kernel::{
    CommandReturn, Driver, ErrorCode, Grant, ProcessId, Read, ReadWrite, ReadWriteAppSlice, Upcall,
};

pub struct HmacDriver<'a, H: digest::Digest<'a, T>, T: 'static + DigestType> {
    hmac: &'a H,

    active: Cell<bool>,

    apps: Grant<App>,
    appid: OptionalCell<ProcessId>,
    phantom: PhantomData<&'a T>,

    data_buffer: TakeCell<'static, [u8]>,
    data_copied: Cell<usize>,
    dest_buffer: TakeCell<'static, T>,
}

impl<'a, H: digest::Digest<'a, T> + digest::HMACSha256, T: DigestType> HmacDriver<'a, H, T>
where
    T: AsMut<[u8]>,
{
    pub fn new(
        hmac: &'a H,
        data_buffer: &'static mut [u8],
        dest_buffer: &'static mut T,
        grant: Grant<App>,
    ) -> HmacDriver<'a, H, T> {
        HmacDriver {
            hmac: hmac,
            active: Cell::new(false),
            apps: grant,
            appid: OptionalCell::empty(),
            phantom: PhantomData,
            data_buffer: TakeCell::new(data_buffer),
            data_copied: Cell::new(0),
            dest_buffer: TakeCell::new(dest_buffer),
        }
    }

    fn run(&self) -> Result<(), ErrorCode> {
        self.appid.map_or(Err(ErrorCode::RESERVE), |appid| {
            self.apps
                .enter(*appid, |app| {
                    app.key.map_or((), |k| {
                        self.hmac
                            .set_mode_hmacsha256(k.as_ref().try_into().unwrap())
                            .unwrap();
                    });

                    app.data.map_or(Err(ErrorCode::RESERVE), |d| {
                        self.data_buffer.map(|buf| {
                            let data = d.as_ref();

                            // Determine the size of the static buffer we have
                            let static_buffer_len = buf.len();

                            // If we have more data then the static buffer we set how much data we are going to copy
                            if data.len() > static_buffer_len {
                                self.data_copied.set(static_buffer_len);
                            }

                            // Copy the data into the static buffer
                            buf.copy_from_slice(&data[..static_buffer_len]);
                        });

                        // Add the data from the static buffer to the HMAC
                        if let Err(e) = self
                            .hmac
                            .add_data(LeasableBuffer::new(self.data_buffer.take().unwrap()))
                        {
                            self.data_buffer.replace(e.1);
                            return Err(e.0);
                        }
                        Ok(())
                    })
                })
                .unwrap_or_else(|err| Err(err.into()))
        })
    }

    fn check_queue(&self) {
        for appiter in self.apps.iter() {
            let started_command = appiter.enter(|app| {
                // If an app is already running let it complete
                if self.appid.is_some() {
                    return true;
                }

                // If this app has a pending command let's use it.
                app.pending_run_app.take().map_or(false, |appid| {
                    // Mark this driver as being in use.
                    self.appid.set(appid);
                    // Actually make the buzz happen.
                    self.run() == Ok(())
                })
            });
            if started_command {
                break;
            }
        }
    }
}

impl<'a, H: digest::Digest<'a, T> + digest::HMACSha256, T: DigestType> digest::Client<'a, T>
    for HmacDriver<'a, H, T>
{
    fn add_data_done(&'a self, _result: Result<(), ErrorCode>, data: &'static mut [u8]) {
        self.appid.map(move |id| {
            self.apps
                .enter(*id, move |app| {
                    let mut data_len = 0;
                    let mut static_buffer_len = 0;

                    self.data_buffer.replace(data);

                    self.data_buffer.map(|buf| {
                        let ret = app.data.map_or(Err(ErrorCode::RESERVE), |d| {
                            let data = d.as_ref();

                            // Determine the size of the static buffer we have
                            static_buffer_len = buf.len();
                            // Determine how much data we have already copied
                            let copied_data = self.data_copied.get();

                            data_len = data.len();

                            if data_len > copied_data {
                                let remaining_data = &d.as_ref()[copied_data..];
                                let remaining_len = data_len - copied_data;

                                if remaining_len < static_buffer_len {
                                    buf[..remaining_len].copy_from_slice(remaining_data);
                                } else {
                                    buf.copy_from_slice(&remaining_data[..static_buffer_len]);
                                }
                            }
                            Ok(())
                        });

                        if ret == Err(ErrorCode::RESERVE) {
                            // No data buffer, clear the appid and data
                            self.hmac.clear_data();
                            self.appid.clear();
                            self.check_queue();
                        }
                    });

                    if static_buffer_len > 0 {
                        let copied_data = self.data_copied.get();

                        if data_len > copied_data {
                            // Update the amount of data copied
                            self.data_copied.set(copied_data + static_buffer_len);

                            let mut lease_buf =
                                LeasableBuffer::new(self.data_buffer.take().unwrap());

                            // Add the data from the static buffer to the HMAC
                            if data_len < (copied_data + static_buffer_len) {
                                lease_buf.slice(..(data_len - copied_data))
                            }

                            if self.hmac.add_data(lease_buf).is_err() {
                                // Error, clear the appid and data
                                self.hmac.clear_data();
                                self.appid.clear();
                                self.check_queue();
                                return;
                            }

                            // Return as we don't want to run the digest yet
                            return;
                        }
                    }

                    // If we get here we are ready to run the digest, reset the copied data
                    self.data_copied.set(0);

                    if let Err(e) = self.hmac.run(self.dest_buffer.take().unwrap()) {
                        // Error, clear the appid and data
                        self.hmac.clear_data();
                        self.appid.clear();

                        app.callback
                            .schedule(kernel::into_statuscode(e.0.into()), 0, 0);

                        self.check_queue();
                        return;
                    }
                })
                .map_err(|err| {
                    if err == kernel::procs::Error::NoSuchApp
                        || err == kernel::procs::Error::InactiveApp
                    {
                        self.appid.clear();
                        self.check_queue();
                    }
                })
        });
    }

    fn hash_done(&'a self, result: Result<(), ErrorCode>, digest: &'static mut T) {
        self.appid.map(|id| {
            self.apps
                .enter(*id, |app| {
                    self.hmac.clear_data();

                    let pointer = digest.as_ref()[0] as *mut u8;

                    app.data.mut_map_or((), |dest| {
                        dest.as_mut().copy_from_slice(digest.as_ref());
                    });

                    match result {
                        Ok(_) => app.callback.schedule(0, pointer as usize, 0),
                        Err(e) => app.callback.schedule(
                            kernel::into_statuscode(e.into()),
                            pointer as usize,
                            0,
                        ),
                    };

                    // Clear the current appid as it has finished running
                    self.appid.clear();
                    self.check_queue();
                })
                .map_err(|err| {
                    if err == kernel::procs::Error::NoSuchApp
                        || err == kernel::procs::Error::InactiveApp
                    {
                        self.appid.clear();
                        self.check_queue();
                    }
                })
        });

        self.dest_buffer.replace(digest);
    }
}

/// Specify memory regions to be used.
///
/// ### `allow_num`
///
/// - `0`: Allow a buffer for storing the key.
///        The kernel will read from this when running
///        This should not be changed after running `run` until the HMAC
///        has completed
/// - `1`: Allow a buffer for storing the buffer.
///        The kernel will read from this when running
///        This should not be changed after running `run` until the HMAC
///        has completed
/// - `2`: Allow a buffer for storing the digest.
///        The kernel will fill this with the HMAC digest before calling
///        the `hash_done` callback.
impl<'a, H: digest::Digest<'a, T> + digest::HMACSha256, T: DigestType> Driver
    for HmacDriver<'a, H, T>
{
    fn allow_readwrite(
        &self,
        appid: ProcessId,
        allow_num: usize,
        mut slice: ReadWriteAppSlice,
    ) -> Result<ReadWriteAppSlice, (ReadWriteAppSlice, ErrorCode)> {
        let res = match allow_num {
            // Pass buffer for the key to be in
            0 => self
                .apps
                .enter(appid, |app| {
                    mem::swap(&mut slice, &mut app.key);
                    Ok(())
                })
                .unwrap_or(Err(ErrorCode::FAIL)),

            // Pass buffer for the data to be in
            1 => self
                .apps
                .enter(appid, |app| {
                    mem::swap(&mut slice, &mut app.data);
                    Ok(())
                })
                .unwrap_or(Err(ErrorCode::FAIL)),

            // Pass buffer for the digest to be in.
            2 => self
                .apps
                .enter(appid, |app| {
                    mem::swap(&mut slice, &mut app.dest);
                    Ok(())
                })
                .unwrap_or(Err(ErrorCode::FAIL)),

            // default
            _ => Err(ErrorCode::NOSUPPORT),
        };

        match res {
            Ok(()) => Ok(slice),
            Err(e) => Err((slice, e)),
        }
    }

    /// Subscribe to HmacDriver events.
    ///
    /// ### `subscribe_num`
    ///
    /// - `0`: Subscribe to interrupts from HMAC events.
    ///        The callback signature is `fn(result: u32)`
    fn subscribe(
        &self,
        subscribe_num: usize,
        mut callback: Upcall,
        appid: ProcessId,
    ) -> Result<Upcall, (Upcall, ErrorCode)> {
        let res = match subscribe_num {
            0 => {
                // set callback
                self.apps
                    .enter(appid, |app| {
                        mem::swap(&mut app.callback, &mut callback);
                        Ok(())
                    })
                    .unwrap_or(Err(ErrorCode::FAIL))
            }

            // default
            _ => Err(ErrorCode::NOSUPPORT),
        };

        match res {
            Ok(()) => Ok(callback),
            Err(e) => Err((callback, e)),
        }
    }

    /// Setup and run the HMAC hardware
    ///
    /// We expect userspace to setup buffers for the key, data and digest.
    /// These buffers must be allocated and specified to the kernel from the
    /// above allow calls.
    ///
    /// We expect userspace not to change the value while running. If userspace
    /// changes the value we have no guarentee of what is passed to the
    /// hardware. This isn't a security issue, it will just prove the requesting
    /// app with invalid data.
    ///
    /// The driver will take care of clearing data from the underlying impelemenation
    /// by calling the `clear_data()` function when the `hash_complete()` callback
    /// is called or if an error is encounted.
    ///
    /// ### `command_num`
    ///
    /// - `0`: set_algorithm
    /// - `1`: run
    fn command(
        &self,
        command_num: usize,
        data1: usize,
        _data2: usize,
        appid: ProcessId,
    ) -> CommandReturn {
        let match_or_empty_or_nonexistant = self.appid.map_or(true, |owning_app| {
            // We have recorded that an app has ownership of the HMAC.

            // If the HMAC is still active, then we need to wait for the operation
            // to finish and the app, whether it exists or not (it may have crashed),
            // still owns this capsule. If the HMAC is not active, then
            // we need to verify that that application still exists, and remove
            // it as owner if not.
            if self.active.get() {
                owning_app == &appid
            } else {
                // Check the app still exists.
                //
                // If the `.enter()` succeeds, then the app is still valid, and
                // we can check if the owning app matches the one that called
                // the command. If the `.enter()` fails, then the owning app no
                // longer exists and we return `true` to signify the
                // "or_nonexistant" case.
                self.apps
                    .enter(*owning_app, |_| owning_app == &appid)
                    .unwrap_or(true)
            }
        });

        match command_num {
            // set_algorithm
            0 => {
                match data1 {
                    // SHA256
                    0 => {
                        // Only Sha256 is supported, we don't need to do anything
                        CommandReturn::success()
                    }
                    _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
                }
            }

            // run
            1 => {
                if match_or_empty_or_nonexistant {
                    self.appid.set(appid);
                    let ret = self.run();

                    if let Err(e) = ret {
                        self.hmac.clear_data();
                        self.appid.clear();
                        self.check_queue();
                        CommandReturn::failure(e)
                    } else {
                        CommandReturn::success()
                    }
                } else {
                    // There is an active app, so queue this request (if possible).
                    self.apps
                        .enter(appid, |app| {
                            // Some app is using the storage, we must wait.
                            if app.pending_run_app.is_some() {
                                // No more room in the queue, nowhere to store this
                                // request.
                                CommandReturn::failure(ErrorCode::NOMEM)
                            } else {
                                // We can store this, so lets do it.
                                app.pending_run_app = Some(appid);
                                CommandReturn::success()
                            }
                        })
                        .unwrap_or_else(|err| err.into())
                }
            }

            // default
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }
}

#[derive(Default)]
pub struct App {
    callback: Upcall,
    pending_run_app: Option<ProcessId>,
    key: ReadWriteAppSlice,
    data: ReadWriteAppSlice,
    dest: ReadWriteAppSlice,
}
