//! Components for the Screen.
//!
//! Buffer Size
//! -----------
//!
//! Displays can receive a large amount of data and having larger transfer buffers
//! optimizes the number of bus writes.
//!
//! As memory is limited on some MCUs, the `components::screen_buffer_size``
//! macro allows users to define the size of the screen buffer.
//!
//! Usage
//! -----
//!
//! // Screen
//! ```rust
//! let screen =
//!     components::screen::ScreenComponent::new(board_kernel, tft, None)
//!         .finalize(components::screen_buffer_size!(40960));
//! ```
//!
//! // Screen with Setup
//! ```rust
//! let screen =
//!     components::screen::ScreenComponent::new(board_kernel, tft, Some(tft))
//!         .finalize(components::screen_buffer_size!(40960));
//! ```
use kernel::capabilities;
use kernel::component::Component;
use kernel::create_capability;
use kernel::static_init;

#[macro_export]
macro_rules! screen_buffer_size {
    ($s:literal $(,)?) => {{
        static mut BUFFER: [u8; $s] = [0; $s];
        (&mut BUFFER)
    }};
}

pub struct ScreenComponent {
    board_kernel: &'static kernel::Kernel,
    driver_num: usize,
    screen: &'static dyn kernel::hil::screen::Screen,
    screen_setup: Option<&'static dyn kernel::hil::screen::ScreenSetup>,
}

impl ScreenComponent {
    pub fn new(
        board_kernel: &'static kernel::Kernel,
        driver_num: usize,
        screen: &'static dyn kernel::hil::screen::Screen,
        screen_setup: Option<&'static dyn kernel::hil::screen::ScreenSetup>,
    ) -> ScreenComponent {
        ScreenComponent {
            board_kernel: board_kernel,
            driver_num: driver_num,
            screen: screen,
            screen_setup: screen_setup,
        }
    }
}

impl Component for ScreenComponent {
    type StaticInput = &'static mut [u8];
    type Output = &'static capsules::screen::Screen<'static>;

    unsafe fn finalize(self, static_input: Self::StaticInput) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
        let grant_screen = self.board_kernel.create_grant(self.driver_num, &grant_cap);

        let screen = static_init!(
            capsules::screen::Screen,
            capsules::screen::Screen::new(
                self.screen,
                self.screen_setup,
                static_input,
                grant_screen
            )
        );

        kernel::hil::screen::Screen::set_client(self.screen, Some(screen));
        if let Some(screen_setup) = self.screen_setup {
            kernel::hil::screen::ScreenSetup::set_client(screen_setup, Some(screen));
        }

        screen
    }
}
