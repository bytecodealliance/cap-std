//! Capability-oriented random number generators
//!
//! This corresponds to [`rand`].
//!
//! Capability-oriented APIs represent access to external resources as
//! objects which can be passed around between different parts of a
//! program.
//!
//! Two notable features are the [`OsRng`] and [`CapRng`] types, which
//! wrap up access to the operating system entropy source in capability
//! objects.
//!
//! This crate uses the existing `rand::SeedableRng` trait rather than having
//! its own version, however while `rand::SeedableRng` is mostly just a pure
//! interface, it provides a `from_entropy` function which directly reads
//! from the operating system entropy source. To preserve the
//! capability-oriented interface, avoid using `rand::SeedableRng`'s
//! `from_entropy` function on any of the types that implement that trait.
//!
//! [`OsRng`]: crate::rngs::OsRng
//! [`CapRng`]: crate::rngs::CapRng

#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

pub use rand::{distributions, seq, CryptoRng, Error, Fill, Rng, RngCore, SeedableRng};

/// Convenience re-export of common members.
///
/// This corresponds to [`rand::prelude`].
pub mod prelude {
    #[cfg(feature = "small_rng")]
    pub use crate::rngs::SmallRng;
    pub use crate::{
        distributions::Distribution,
        random,
        rngs::{CapRng, StdRng},
        seq::{IteratorRandom, SliceRandom},
        thread_rng, CryptoRng, Rng, RngCore, SeedableRng,
    };
}

/// Random number generators and adapters.
///
/// This corresponds to [`rand::rngs`].
pub mod rngs {
    pub use rand::rngs::{adapter, mock, StdRng};

    #[cfg(feature = "small_rng")]
    pub use rand::rngs::SmallRng;

    /// A random number generator that retrieves randomness from from the
    /// operating system.
    ///
    /// This corresponds to [`rand::rngs::OsRng`], except instead of
    /// implementing `Default` it has an unsafe `default` function since
    /// accessing the operating system requires ambient authority.
    #[derive(Clone, Copy, Debug)]
    pub struct OsRng(());

    impl OsRng {
        /// Returns an `OsRng` instance.
        ///
        /// # Safety
        ///
        /// This function is unsafe because it makes use of ambient authority
        /// to access the platform entropy source, which doesn't uphold the
        /// invariant of the rest of the API. It is otherwise safe to use.
        #[inline]
        pub const unsafe fn default() -> Self {
            Self(())
        }
    }

    impl crate::RngCore for OsRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            rand::rngs::OsRng.next_u32()
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            rand::rngs::OsRng.next_u64()
        }

        #[inline]
        fn fill_bytes(&mut self, bytes: &mut [u8]) {
            rand::rngs::OsRng.fill_bytes(bytes)
        }

        #[inline]
        fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), crate::Error> {
            rand::rngs::OsRng.try_fill_bytes(bytes)
        }
    }

    impl crate::CryptoRng for OsRng {}

    /// The type returned by `thread_rng`, essentially just a reference to a
    /// PRNG in memory.
    ///
    /// This corresponds to [`rand::rngs::ThreadRng`], except that it isn't
    /// tied to thread-local memory.
    #[derive(Clone, Debug)]
    pub struct CapRng {
        pub(super) inner: rand::rngs::ThreadRng,
    }

    impl CapRng {
        /// A convenience alias for calling `thread_rng`.
        ///
        /// # Safety
        ///
        /// This function is unsafe because it makes use of ambient authority
        /// to access the platform entropy source, which doesn't uphold the
        /// invariant of the rest of the API. It is otherwise safe to use.
        #[inline]
        pub unsafe fn default() -> Self {
            crate::thread_rng()
        }
    }

    impl crate::RngCore for CapRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.inner.next_u32()
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            self.inner.next_u64()
        }

        #[inline]
        fn fill_bytes(&mut self, bytes: &mut [u8]) {
            self.inner.fill_bytes(bytes)
        }

        #[inline]
        fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), crate::Error> {
            self.inner.try_fill_bytes(bytes)
        }
    }

    impl crate::CryptoRng for CapRng {}
}

/// Retrieve the lazily-initialized thread-local random number generator,
/// seeded by the system.
///
/// This corresponds to [`rand::thread_rng`].
///
/// # Safety
///
/// This function is unsafe because it makes use of ambient authority to
/// access the platform entropy source, which doesn't uphold the invariant
/// of the rest of the API. It is otherwise safe to use.
#[inline]
pub unsafe fn thread_rng() -> rngs::CapRng {
    rngs::CapRng {
        inner: rand::thread_rng(),
    }
}

/// Generates a random value using the thread-local random number generator.
///
/// This corresponds to [`rand::random`].
///
/// # Safety
///
/// This function is unsafe because it makes use of ambient authority to
/// access the platform entropy source, which doesn't uphold the invariant
/// of the rest of the API. It is otherwise safe to use.
#[inline]
pub unsafe fn random<T>() -> T
where
    crate::distributions::Standard: crate::distributions::Distribution<T>,
{
    rand::random()
}
