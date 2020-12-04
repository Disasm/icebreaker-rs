#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SPIFLASH

use crate::{RWRegister};
use core::marker::PhantomData;

/// Bitbang controls for SPI output.  Only standard 1x SPI is supported, and as a
/// result all four wires are ganged together.  This means that it is only possible
/// to perform half-duplex operations, using this SPI core.
pub mod BITBANG {
    /// Output value for MOSI pin, valid whenever ``dir`` is ``0``.
    pub mod mosi {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (1 bit: 0x1 << 0)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Output value for SPI CLK pin.
    pub mod clk {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
    
        /// Mask (1 bit: 0x1 << 1)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Output value for SPI CSn pin.
    pub mod cs_n {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
    
        /// Mask (1 bit: 0x1 << 2)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Sets the direction for *ALL* SPI data pins except CLK and CSn.
    pub mod dir {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
    
        /// Mask (1 bit: 0x1 << 3)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// Incoming value of MISO signal.
pub mod MISO {
    pub mod miso {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (1 bit: 0x1 << 0)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// Write a ``1`` here to disable memory-mapped mode and enable bitbang mode.
pub mod BITBANG_EN {
    pub mod bitbang_en {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (1 bit: 0x1 << 0)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub struct RegisterBlock {
    /// Bitbang controls for SPI output.  Only standard 1x SPI is supported, and as a
    /// result all four wires are ganged together.  This means that it is only possible
    /// to perform half-duplex operations, using this SPI core.
    pub BITBANG: RWRegister<u32>,

    /// Incoming value of MISO signal.
    pub MISO: RWRegister<u32>,

    /// Write a ``1`` here to disable memory-mapped mode and enable bitbang mode.
    pub BITBANG_EN: RWRegister<u32>,
}

pub struct ResetValues {
    pub BITBANG: u32,
    pub MISO: u32,
    pub BITBANG_EN: u32,
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
