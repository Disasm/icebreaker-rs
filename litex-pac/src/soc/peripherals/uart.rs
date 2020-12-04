#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! UART

use crate::{RWRegister};
use core::marker::PhantomData;

pub mod RXTX {
    pub mod rxtx {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (8 bit: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub mod TXFULL {
    pub mod txfull {
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

pub mod RXEMPTY {
    pub mod rxempty {
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

/// This register contains the current raw level of the rx event trigger.  Writes to
/// this register have no effect.
pub mod EV_STATUS {
    /// Level of the ``tx`` event
    pub mod tx {
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
    /// Level of the ``rx`` event
    pub mod rx {
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
    
    }}

/// When a  rx event occurs, the corresponding bit will be set in this register.  To
/// clear the Event, set the corresponding bit in this register.
pub mod EV_PENDING {
    /// `1` if a `tx` event occurred. This Event is triggered on a **falling** edge.
    pub mod tx {
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
    /// `1` if a `rx` event occurred. This Event is triggered on a **falling** edge.
    pub mod rx {
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
    
    }}

/// This register enables the corresponding rx events.  Write a ``0`` to this
/// register to disable individual events.
pub mod EV_ENABLE {
    /// Write a ``1`` to enable the ``tx`` Event
    pub mod tx {
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
    /// Write a ``1`` to enable the ``rx`` Event
    pub mod rx {
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
    
    }}

pub mod TXEMPTY {
    pub mod txempty {
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

pub mod RXFULL {
    pub mod rxfull {
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
    pub RXTX: RWRegister<u32>,

    pub TXFULL: RWRegister<u32>,

    pub RXEMPTY: RWRegister<u32>,

    /// This register contains the current raw level of the rx event trigger.  Writes to
    /// this register have no effect.
    pub EV_STATUS: RWRegister<u32>,

    /// When a  rx event occurs, the corresponding bit will be set in this register.  To
    /// clear the Event, set the corresponding bit in this register.
    pub EV_PENDING: RWRegister<u32>,

    /// This register enables the corresponding rx events.  Write a ``0`` to this
    /// register to disable individual events.
    pub EV_ENABLE: RWRegister<u32>,

    pub TXEMPTY: RWRegister<u32>,

    pub RXFULL: RWRegister<u32>,
}

pub struct ResetValues {
    pub RXTX: u32,
    pub TXFULL: u32,
    pub RXEMPTY: u32,
    pub EV_STATUS: u32,
    pub EV_PENDING: u32,
    pub EV_ENABLE: u32,
    pub TXEMPTY: u32,
    pub RXFULL: u32,
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
