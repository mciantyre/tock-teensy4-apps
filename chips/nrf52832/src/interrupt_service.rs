use crate::deferred_call_tasks::DeferredCallTask;
use nrf52::chip::Nrf52DefaultPeripherals;

/// This struct, when initialized, instantiates all peripheral drivers for the nrf52840.
/// If a board wishes to use only a subset of these peripherals, this
/// should not be used or imported, and a modified version should be
/// constructed manually in main.rs.
pub struct Nrf52832DefaultPeripherals<'a> {
    pub nrf52: Nrf52DefaultPeripherals<'a>,
    // put additional 52832 specific peripherals here
}
impl<'a> Nrf52832DefaultPeripherals<'a> {
    pub unsafe fn new(ppi: &'a crate::ppi::Ppi) -> Self {
        Self {
            // Note: The use of the global static mut crate::gpio::PORT
            // does not fit with the updated model of not using globals
            // to instantiate peripherals, however it is unergonomic
            // to transition it to the new model until `min_const_generics`
            // is made stable, such that there can be a shared Port type
            // across chips with different numbers of gpio pins.
            nrf52: Nrf52DefaultPeripherals::new(&crate::gpio::PORT, ppi),
        }
    }
    // Necessary for setting up circular dependencies
    pub fn init(&'a self) {
        self.nrf52.init();
    }
}
impl<'a> kernel::InterruptService<DeferredCallTask> for Nrf52832DefaultPeripherals<'a> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        self.nrf52.service_interrupt(interrupt)
    }
    unsafe fn service_deferred_call(&self, task: DeferredCallTask) -> bool {
        self.nrf52.service_deferred_call(task)
    }
}
