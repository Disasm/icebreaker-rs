
/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

/// Metadata
pub mod metadata;

pub use self::instances::ctrl;
pub use self::instances::uart_phy;
pub use self::instances::uart;
pub use self::instances::timer0;
pub use self::instances::spiflash;
pub use self::instances::clock;
pub use self::instances::gpio;
