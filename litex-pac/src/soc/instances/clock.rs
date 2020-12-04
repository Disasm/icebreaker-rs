#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CLOCK

pub use super::super::peripherals::clock::Instance;
pub use super::super::peripherals::clock::{RegisterBlock, ResetValues};
pub use super::super::peripherals::clock::{CORECLK};


/// Access functions for the CLOCK peripheral instance
pub mod CLOCK {
    use super::ResetValues;
    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0xe0003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CLOCK
    pub const reset: ResetValues = ResetValues {
        CORECLK: 0x0,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CLOCK_TAKEN: bool = false;

    /// Safe access to CLOCK
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
            if CLOCK_TAKEN {
                None
            } else {
                CLOCK_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CLOCK
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        crate::arch::interrupt::free(|_| unsafe {
            if CLOCK_TAKEN && inst.addr == INSTANCE.addr {
                CLOCK_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CLOCK
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Instance {
        CLOCK_TAKEN = true;
        INSTANCE
    }

    /// Unsafely obtains an instance of CLOCK
    ///
    /// This will not check if `take()` or `steal()` have already been called
    /// before. It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    #[inline]
    pub unsafe fn conjure() -> Instance {
        INSTANCE
    }
}

/// Raw pointer to CLOCK
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CLOCK: *const RegisterBlock = 0xe0003000 as *const _;
