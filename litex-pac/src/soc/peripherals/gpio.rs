#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO

use crate::{RWRegister};
use core::marker::PhantomData;

/// GPIO pin direction
pub mod DIR {
    /// Pin direction
    pub mod dir0 {
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
    /// Pin direction
    pub mod dir1 {
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
    /// Pin direction
    pub mod dir2 {
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
    /// Pin direction
    pub mod dir3 {
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
    
    }
    /// Pin direction
    pub mod dir4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin direction
    pub mod dir31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// GPIO output type register
pub mod OTYPER {
    /// Pin output type
    pub mod otype0 {
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
    /// Pin output type
    pub mod otype1 {
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
    /// Pin output type
    pub mod otype2 {
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
    /// Pin output type
    pub mod otype3 {
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
    
    }
    /// Pin output type
    pub mod otype4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output type
    pub mod otype31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// GPIO input data register
pub mod IDR {
    /// Pin input data
    pub mod id0 {
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
    /// Pin input data
    pub mod id1 {
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
    /// Pin input data
    pub mod id2 {
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
    /// Pin input data
    pub mod id3 {
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
    
    }
    /// Pin input data
    pub mod id4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin input data
    pub mod id31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// GPIO output data register
pub mod ODR {
    /// Pin output data
    pub mod od0 {
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
    /// Pin output data
    pub mod od1 {
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
    /// Pin output data
    pub mod od2 {
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
    /// Pin output data
    pub mod od3 {
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
    
    }
    /// Pin output data
    pub mod od4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin output data
    pub mod od31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// GPIO bit set register
pub mod BSR {
    /// Pin set bit
    pub mod bs0 {
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
    /// Pin set bit
    pub mod bs1 {
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
    /// Pin set bit
    pub mod bs2 {
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
    /// Pin set bit
    pub mod bs3 {
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
    
    }
    /// Pin set bit
    pub mod bs4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin set bit
    pub mod bs31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// GPIO bit reset register
pub mod BRR {
    /// Pin reset bit
    pub mod bs0 {
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
    /// Pin reset bit
    pub mod bs1 {
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
    /// Pin reset bit
    pub mod bs2 {
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
    /// Pin reset bit
    pub mod bs3 {
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
    
    }
    /// Pin reset bit
    pub mod bs4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin reset bit
    pub mod bs31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub struct RegisterBlock {
    /// GPIO pin direction
    pub DIR: RWRegister<u32>,

    /// GPIO output type register
    pub OTYPER: RWRegister<u32>,

    /// GPIO input data register
    pub IDR: RWRegister<u32>,

    /// GPIO output data register
    pub ODR: RWRegister<u32>,

    /// GPIO bit set register
    pub BSR: RWRegister<u32>,

    /// GPIO bit reset register
    pub BRR: RWRegister<u32>,
}

pub struct ResetValues {
    pub DIR: u32,
    pub OTYPER: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSR: u32,
    pub BRR: u32,
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
