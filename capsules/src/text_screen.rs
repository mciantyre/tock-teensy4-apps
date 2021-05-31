//! Provides userspace with access to the text screen.
//!
//! Usage:
//! -----
//!
//! You need a screen that provides the `hil::text_screen::TextScreen`
//! trait.
//!
//! ```rust
//! let text_screen = components::text_screen::TextScreenComponent::new(board_kernel, lcd)
//!         .finalize(components::screen_buffer_size!(64));
//! ```

use core::convert::From;
use core::{cmp, mem};
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::hil;
use kernel::{CommandReturn, Driver, ErrorCode, Grant, ProcessId, Read, ReadOnlyAppSlice, Upcall};

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::TextScreen as usize;

#[derive(Clone, Copy, PartialEq)]
enum TextScreenCommand {
    Idle,
    GetResolution,
    Display,
    NoDisplay,
    Blink,
    NoBlink,
    SetCursor,
    NoCursor,
    ShowCursor,
    Write,
    Clear,
    Home,
}

pub struct App {
    callback: Upcall,
    pending_command: bool,
    shared: ReadOnlyAppSlice,
    write_len: usize,
    command: TextScreenCommand,
    data1: usize,
    data2: usize,
}

impl Default for App {
    fn default() -> App {
        App {
            callback: Upcall::default(),
            pending_command: false,
            shared: ReadOnlyAppSlice::default(),
            write_len: 0,
            command: TextScreenCommand::Idle,
            data1: 1,
            data2: 0,
        }
    }
}

pub struct TextScreen<'a> {
    text_screen: &'a dyn hil::text_screen::TextScreen<'static>,
    apps: Grant<App>,
    current_app: OptionalCell<ProcessId>,
    buffer: TakeCell<'static, [u8]>,
}

impl<'a> TextScreen<'a> {
    pub fn new(
        text_screen: &'static dyn hil::text_screen::TextScreen,
        buffer: &'static mut [u8],
        grant: Grant<App>,
    ) -> TextScreen<'a> {
        TextScreen {
            text_screen: text_screen,
            apps: grant,
            current_app: OptionalCell::empty(),
            buffer: TakeCell::new(buffer),
        }
    }

    fn enqueue_command(
        &self,
        command: TextScreenCommand,
        data1: usize,
        data2: usize,
        appid: ProcessId,
    ) -> CommandReturn {
        let res = self
            .apps
            .enter(appid, |app| {
                if self.current_app.is_none() {
                    self.current_app.set(appid);
                    app.command = command;
                    let r = self.do_command(command, data1, data2, appid);
                    if r != Ok(()) {
                        self.current_app.clear();
                    }
                    r
                } else {
                    if app.pending_command == true {
                        Err(ErrorCode::BUSY)
                    } else {
                        app.pending_command = true;
                        app.command = command;
                        app.data1 = data1;
                        app.data2 = data2;
                        Ok(())
                    }
                }
            })
            .map_err(ErrorCode::from);
        if let Err(err) = res {
            CommandReturn::failure(err)
        } else {
            CommandReturn::success()
        }
    }

    fn do_command(
        &self,
        command: TextScreenCommand,
        data1: usize,
        data2: usize,
        appid: ProcessId,
    ) -> Result<(), ErrorCode> {
        match command {
            TextScreenCommand::GetResolution => {
                let (x, y) = self.text_screen.get_size();
                self.schedule_callback(kernel::into_statuscode(Ok(())), x, y);
                self.run_next_command();
                Ok(())
            }
            TextScreenCommand::Display => self.text_screen.display_on(),
            TextScreenCommand::NoDisplay => self.text_screen.display_off(),
            TextScreenCommand::Blink => self.text_screen.blink_cursor_on(),
            TextScreenCommand::NoBlink => self.text_screen.blink_cursor_off(),
            TextScreenCommand::SetCursor => self.text_screen.set_cursor(data1, data2),
            TextScreenCommand::NoCursor => self.text_screen.hide_cursor(),
            TextScreenCommand::Write => self
                .apps
                .enter(appid, |app| {
                    if data1 > 0 {
                        app.write_len = data1;
                        app.shared.map_or(Err(ErrorCode::NOMEM), |to_write_buffer| {
                            self.buffer.take().map_or(Err(ErrorCode::BUSY), |buffer| {
                                let len = cmp::min(app.write_len, buffer.len());
                                for n in 0..len {
                                    buffer[n] = to_write_buffer[n];
                                }
                                match self.text_screen.print(buffer, len) {
                                    Ok(()) => Ok(()),
                                    Err((ecode, buffer)) => {
                                        self.buffer.replace(buffer);
                                        Err(ecode)
                                    }
                                }
                            })
                        })
                    } else {
                        Err(ErrorCode::NOMEM)
                    }
                })
                .unwrap_or_else(|err| err.into()),
            TextScreenCommand::Clear => self.text_screen.clear(),
            TextScreenCommand::Home => self.text_screen.clear(),
            TextScreenCommand::ShowCursor => self.text_screen.show_cursor(),
            _ => Err(ErrorCode::NOSUPPORT),
        }
    }

    fn run_next_command(&self) {
        // Check for pending events.
        for app in self.apps.iter() {
            let appid = app.processid();
            let current_command = app.enter(|app| {
                if app.pending_command {
                    app.pending_command = false;
                    self.current_app.set(appid);
                    let r = self.do_command(app.command, app.data1, app.data2, appid);
                    if r != Ok(()) {
                        self.current_app.clear();
                    }
                    r == Ok(())
                } else {
                    false
                }
            });
            if current_command {
                break;
            }
        }
    }

    fn schedule_callback(&self, data1: usize, data2: usize, data3: usize) {
        self.current_app.take().map(|appid| {
            let _ = self.apps.enter(appid, |app| {
                app.pending_command = false;
                app.callback.schedule(data1, data2, data3);
            });
        });
    }
}

impl<'a> Driver for TextScreen<'a> {
    fn subscribe(
        &self,
        subscribe_num: usize,
        mut callback: Upcall,
        app_id: ProcessId,
    ) -> Result<Upcall, (Upcall, ErrorCode)> {
        match subscribe_num {
            0 => {
                let res = self
                    .apps
                    .enter(app_id, |app| {
                        mem::swap(&mut app.callback, &mut callback);
                    })
                    .map_err(ErrorCode::from);
                if let Err(e) = res {
                    Err((callback, e))
                } else {
                    Ok(callback)
                }
            }
            _ => Err((callback, ErrorCode::NOSUPPORT)),
        }
    }

    fn command(
        &self,
        command_num: usize,
        data1: usize,
        data2: usize,
        appid: ProcessId,
    ) -> CommandReturn {
        match command_num {
            // This driver exists.
            0 => CommandReturn::success(),
            // Get Resolution
            1 => self.enqueue_command(TextScreenCommand::GetResolution, data1, data2, appid),
            // Display
            2 => self.enqueue_command(TextScreenCommand::Display, data1, data2, appid),
            // No Display
            3 => self.enqueue_command(TextScreenCommand::NoDisplay, data1, data2, appid),
            // Blink
            4 => self.enqueue_command(TextScreenCommand::Blink, data1, data2, appid),
            // No Blink
            5 => self.enqueue_command(TextScreenCommand::NoBlink, data1, data2, appid),
            // Show Cursor
            6 => self.enqueue_command(TextScreenCommand::ShowCursor, data1, data2, appid),
            // No Cursor
            7 => self.enqueue_command(TextScreenCommand::NoCursor, data1, data2, appid),
            // Write
            8 => self.enqueue_command(TextScreenCommand::Write, data1, data2, appid),
            // Clear
            9 => self.enqueue_command(TextScreenCommand::Clear, data1, data2, appid),
            // Home
            10 => self.enqueue_command(TextScreenCommand::Home, data1, data2, appid),
            //Set Curosr
            11 => self.enqueue_command(TextScreenCommand::SetCursor, data1, data2, appid),
            // NOSUPPORT
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allow_readonly(
        &self,
        appid: ProcessId,
        allow_num: usize,
        mut slice: ReadOnlyAppSlice,
    ) -> Result<ReadOnlyAppSlice, (ReadOnlyAppSlice, ErrorCode)> {
        match allow_num {
            0 => {
                let res = self
                    .apps
                    .enter(appid, |app| {
                        mem::swap(&mut app.shared, &mut slice);
                    })
                    .map_err(ErrorCode::from);
                if let Err(e) = res {
                    Err((slice, e))
                } else {
                    Ok(slice)
                }
            }
            _ => Err((slice, ErrorCode::NOSUPPORT)),
        }
    }
}

impl<'a> hil::text_screen::TextScreenClient for TextScreen<'a> {
    fn command_complete(&self, r: Result<(), ErrorCode>) {
        self.schedule_callback(kernel::into_statuscode(r), 0, 0);
        self.run_next_command();
    }

    fn write_complete(&self, buffer: &'static mut [u8], len: usize, r: Result<(), ErrorCode>) {
        self.buffer.replace(buffer);
        self.schedule_callback(kernel::into_statuscode(r), len, 0);
        self.run_next_command();
    }
}
