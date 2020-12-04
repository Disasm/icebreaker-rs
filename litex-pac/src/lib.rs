#![no_std]

use riscv as arch;

mod register;
pub use crate::register::{RORegister, UnsafeRORegister};
pub use crate::register::{WORegister, UnsafeWORegister};
pub use crate::register::{RWRegister, UnsafeRWRegister};

mod soc;
pub use soc::*;
