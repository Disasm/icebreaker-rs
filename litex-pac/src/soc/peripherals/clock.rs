#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CLOCK

use crate::{RWRegister};
use core::marker::PhantomData;

/// Core clock frequency
pub mod CORECLK {
    pub mod coreclk {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (32 bit: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub struct RegisterBlock {
    /// Core clock frequency
    pub CORECLK: RWRegister<u32>,
}

pub struct ResetValues {
    pub CORECLK: u32,
}

pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}

impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
