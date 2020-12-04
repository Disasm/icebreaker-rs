//! Prelude

pub use embedded_hal::prelude::*;
pub use embedded_hal::digital::v2::OutputPin as _embedded_hal_digital_v2_OutputPin;
pub use embedded_hal::digital::v2::StatefulOutputPin as _embedded_hal_digital_v2_StatefulOutputPin;
pub use embedded_hal::digital::v2::ToggleableOutputPin as _embedded_hal_digital_v2_ToggleableOutputPin;
pub use embedded_hal::digital::v2::InputPin as _embedded_hal_digital_v2_InputPin;

#[cfg(with_gpio)]
pub use crate::gpio::GpioExt as _litex_hal_gpio_GpioExt;

pub use crate::time::U32Ext as _litex_hal_time_U32Ext;
