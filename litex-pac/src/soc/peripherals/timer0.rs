#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TIMER0

use crate::{RWRegister};
use core::marker::PhantomData;

/// Load value when Timer is (re-)enabled. In One-Shot mode, the value written to
/// this register specifies the Timer's duration in clock cycles.
pub mod LOAD {
    pub mod load {
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

/// Reload value when Timer reaches ``0``. In Periodic mode, the value written to
/// this register specify the Timer's period in clock cycles.
pub mod RELOAD {
    pub mod reload {
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

/// Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer.  Set
/// to ``0`` to disable the Timer.
pub mod EN {
    pub mod en {
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

/// Update trigger for the current countdown value. A write to this register latches
/// the current countdown value to ``value`` register.
pub mod UPDATE_VALUE {
    pub mod update_value {
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

/// Latched countdown value. This value is updated by writing to ``update_value``.
pub mod VALUE {
    pub mod value {
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

/// This register contains the current raw level of the zero event trigger.  Writes
/// to this register have no effect.
pub mod EV_STATUS {
    /// Level of the ``zero`` event
    pub mod zero {
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

/// When a  zero event occurs, the corresponding bit will be set in this register.
/// To clear the Event, set the corresponding bit in this register.
pub mod EV_PENDING {
    /// `1` if a `zero` event occurred. This Event is triggered on a **falling** edge.
    pub mod zero {
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

/// This register enables the corresponding zero events.  Write a ``0`` to this
/// register to disable individual events.
pub mod EV_ENABLE {
    /// Write a ``1`` to enable the ``zero`` Event
    pub mod zero {
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
    /// Load value when Timer is (re-)enabled. In One-Shot mode, the value written to
    /// this register specifies the Timer's duration in clock cycles.
    pub LOAD: RWRegister<u32>,

    /// Reload value when Timer reaches ``0``. In Periodic mode, the value written to
    /// this register specify the Timer's period in clock cycles.
    pub RELOAD: RWRegister<u32>,

    /// Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer.  Set
    /// to ``0`` to disable the Timer.
    pub EN: RWRegister<u32>,

    /// Update trigger for the current countdown value. A write to this register latches
    /// the current countdown value to ``value`` register.
    pub UPDATE_VALUE: RWRegister<u32>,

    /// Latched countdown value. This value is updated by writing to ``update_value``.
    pub VALUE: RWRegister<u32>,

    /// This register contains the current raw level of the zero event trigger.  Writes
    /// to this register have no effect.
    pub EV_STATUS: RWRegister<u32>,

    /// When a  zero event occurs, the corresponding bit will be set in this register.
    /// To clear the Event, set the corresponding bit in this register.
    pub EV_PENDING: RWRegister<u32>,

    /// This register enables the corresponding zero events.  Write a ``0`` to this
    /// register to disable individual events.
    pub EV_ENABLE: RWRegister<u32>,
}

pub struct ResetValues {
    pub LOAD: u32,
    pub RELOAD: u32,
    pub EN: u32,
    pub UPDATE_VALUE: u32,
    pub VALUE: u32,
    pub EV_STATUS: u32,
    pub EV_PENDING: u32,
    pub EV_ENABLE: u32,
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
