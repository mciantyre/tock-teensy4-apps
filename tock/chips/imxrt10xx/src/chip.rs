//! Chip trait setup.

use core::fmt::Write;
use cortexm7;
use kernel::debug;
use kernel::Chip;

use crate::gpio;
use crate::gpt;
use crate::lpi2c;
use crate::lpuart;
use crate::nvic;

pub struct Imxrt10xx {
    mpu: cortexm7::mpu::MPU,
    userspace_kernel_boundary: cortexm7::syscall::SysCall,
    scheduler_timer: cortexm7::systick::SysTick,
}

impl Imxrt10xx {
    pub unsafe fn new() -> Imxrt10xx {
        Imxrt10xx {
            mpu: cortexm7::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm7::syscall::SysCall::new(),
            scheduler_timer: cortexm7::systick::SysTick::new_with_calibration(792_000_000),
        }
    }
}

impl Chip for Imxrt10xx {
    type MPU = cortexm7::mpu::MPU;
    type UserspaceKernelBoundary = cortexm7::syscall::SysCall;
    type SchedulerTimer = cortexm7::systick::SysTick;
    type WatchDog = ();

    fn service_pending_interrupts(&self) {
        unsafe {
            loop {
                if let Some(interrupt) = cortexm7::nvic::next_pending() {
                    match interrupt {
                        nvic::LPUART1 => lpuart::LPUART1.handle_interrupt(),
                        nvic::LPUART2 => lpuart::LPUART2.handle_interrupt(),
                        nvic::LPI2C1 => lpi2c::LPI2C1.handle_event(),
                        nvic::GPT1 => gpt::GPT1.handle_interrupt(),
                        nvic::GPT2 => gpt::GPT2.handle_interrupt(),
                        nvic::GPIO1_1 => gpio::PORTS.port(gpio::GpioPort::GPIO1).handle_interrupt(),
                        nvic::GPIO1_2 => gpio::PORTS.port(gpio::GpioPort::GPIO1).handle_interrupt(),
                        nvic::GPIO2_1 => gpio::PORTS.port(gpio::GpioPort::GPIO2).handle_interrupt(),
                        nvic::GPIO2_2 => gpio::PORTS.port(gpio::GpioPort::GPIO2).handle_interrupt(),
                        nvic::GPIO3_1 => gpio::PORTS.port(gpio::GpioPort::GPIO3).handle_interrupt(),
                        nvic::GPIO3_2 => gpio::PORTS.port(gpio::GpioPort::GPIO3).handle_interrupt(),
                        nvic::GPIO4_1 => gpio::PORTS.port(gpio::GpioPort::GPIO4).handle_interrupt(),
                        nvic::GPIO4_2 => gpio::PORTS.port(gpio::GpioPort::GPIO4).handle_interrupt(),
                        nvic::GPIO5_1 => gpio::PORTS.port(gpio::GpioPort::GPIO5).handle_interrupt(),
                        nvic::GPIO5_2 => gpio::PORTS.port(gpio::GpioPort::GPIO5).handle_interrupt(),
                        nvic::SNVS_LP_WRAPPER => {
                            debug!("A venit intreruperea de SNVS_LP_WRAPPER");
                        }
                        _ => {
                            panic!("unhandled interrupt {}", interrupt);
                        }
                    }

                    let n = cortexm7::nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm7::nvic::has_pending() }
    }

    fn mpu(&self) -> &cortexm7::mpu::MPU {
        &self.mpu
    }

    fn scheduler_timer(&self) -> &cortexm7::systick::SysTick {
        &self.scheduler_timer
    }

    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }

    fn userspace_kernel_boundary(&self) -> &cortexm7::syscall::SysCall {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        unsafe {
            cortexm7::scb::unset_sleepdeep();
            cortexm7::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm7::support::atomic(f)
    }

    unsafe fn print_state(&self, write: &mut dyn Write) {
        cortexm7::print_cortexm7_state(write);
    }
}
