#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! UART

pub use super::super::peripherals::uart::Instance;
pub use super::super::peripherals::uart::{RegisterBlock, ResetValues};
pub use super::super::peripherals::uart::{RXTX, TXFULL, RXEMPTY, EV_STATUS, EV_PENDING, EV_ENABLE, TXEMPTY, RXFULL};


/// Access functions for the UART peripheral instance
pub mod UART {
    use super::ResetValues;
    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0xe0001800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in UART
    pub const reset: ResetValues = ResetValues {
        RXTX: 0x0,
        TXFULL: 0x0,
        RXEMPTY: 0x0,
        EV_STATUS: 0x0,
        EV_PENDING: 0x0,
        EV_ENABLE: 0x0,
        TXEMPTY: 0x0,
        RXFULL: 0x0,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut UART_TAKEN: bool = false;

    /// Safe access to UART
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        crate::arch::interrupt::free(|_| unsafe {
            if UART_TAKEN {
                None
            } else {
                UART_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to UART
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        crate::arch::interrupt::free(|_| unsafe {
            if UART_TAKEN && inst.addr == INSTANCE.addr {
                UART_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal UART
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        UART_TAKEN = true;
        INSTANCE
    }

    /// Unsafely obtains an instance of UART
    ///
    /// This will not check if `take()` or `steal()` have already been called
    /// before. It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    #[inline]
    pub unsafe fn conjure() -> Instance {
        INSTANCE
    }
}

/// Raw pointer to UART
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const UART: *const RegisterBlock = 0xe0001800 as *const _;
