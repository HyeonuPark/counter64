//! 64bit increase-only atomic counter for everywhere.
//!
//! `Counter64` uses single `AtomicUsize` when it can serve `u64`.
//! Otherwise, it fallbacks to use multiple `AtomicUsize` and combine them.
//!

pub use counter::*;

#[cfg(not(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32")))]
mod counter {
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering::Relaxed};

    #[derive(Debug)]
    pub struct Counter(AtomicUsize);

    pub const COUNTER_INIT: Counter = Counter(ATOMIC_USIZE_INIT);

    impl Counter {
        /// Create new counter from 0
        pub fn new() -> Self {
            COUNTER_INIT
        }

        pub unsafe fn with_init(num: u64) -> Self {
            Counter(AtomicUsize::new(num as usize))
        }

        /// Get counter's current value
        pub fn get(&self) -> u64 {
            self.0.load(Relaxed) as u64
        }

        /// Increase counter by 1, and return previous value
        pub fn incr(&self) -> u64 {
            self.0.fetch_add(1, Relaxed) as u64
        }
    }
}

#[cfg(target_pointer_width = "16")]
mod counter {
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering as O};
    use std::usize;

    #[derive(Debug)]
    pub struct Counter {
        n1: AtomicUsize,
        n2: AtomicUsize,
    }

    pub const COUNTER_INIT: Counter = Counter {
        n1: ATOMIC_USIZE_INIT,
        n2: ATOMIC_USIZE_INIT,
    };

    impl Counter {
        /// Create new counter from 0
        pub fn new() -> Self {
            COUNTER_INIT
        }

        pub unsafe fn with_init(num: u64) -> Self {
            Counter {
                n1: AtomicUsize::new((num & 0xFFFFFFFF) as usize),
                n2: AtomicUsize::new((num >> 32) as usize),
            }
        }

        /// Get counter's current value
        pub fn get(&self) -> u64 {
            loop {
                let n1 = self.n1.load(O::SeqCst);
                let n2 = self.n2.load(O::SeqCst);

                if n1 == self.n1.load(O::SeqCst) {
                    let mut count = 0u64;
                    count += n2;
                    count <<= 32;
                    count += n1;

                    return count;
                }
            }
        }

        /// Increase counter by 1, and return previous value
        pub fn incr(&self) -> u64 {
            let prev = self.get();

            let _ =
                self.n1.fetch_add(1, O::Release) == usize::MAX &&
                self.n2.fetch_add(1, O::Release) == usize::MAX;

            prev
        }
    }
}

#[cfg(target_pointer_width = "16")]
mod counter {
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering as O};
    use std::usize;

    #[derive(Debug)]
    pub struct Counter {
        n1: AtomicUsize,
        n2: AtomicUsize,
        n3: AtomicUsize,
        n4: AtomicUsize,
    }

    pub const COUNTER_INIT: Counter = Counter {
        n1: ATOMIC_USIZE_INIT,
        n2: ATOMIC_USIZE_INIT,
        n3: ATOMIC_USIZE_INIT,
        n4: ATOMIC_USIZE_INIT,
    };

    impl Counter {
        /// Create new counter from 0
        pub fn new() -> Self {
            COUNTER_INIT
        }

        pub unsafe fn with_init(num: u64) -> Self {
            Counter {
                n1: AtomicUsize::new((num & 0xFFFF) as usize),
                n2: AtomicUsize::new(((num >> 16) & 0xFFFF) as usize),
                n3: AtomicUsize::new(((num >> 32) & 0xFFFF) as usize),
                n4: AtomicUsize::new(((num >> 48) & 0xFFFF) as usize),
            }
        }

        /// Get counter's current value
        pub fn get(&self) -> u64 {
            loop {
                let n1 = self.n1.load(O::SeqCst);
                let n2 = self.n2.load(O::SeqCst);
                let n3 = self.n3.load(O::SeqCst);
                let n4 = self.n4.load(O::SeqCst);

                if n1 == self.n1.load(O::SeqCst) {
                    let mut count = 0u64;
                    count += n4;
                    count <<= 16;
                    count += n3;
                    count <<= 16;
                    count += n2;
                    count <<= 16;
                    count += n1;

                    return count;
                }
            }
        }

        /// Increase counter by 1, and return previous value
        pub fn incr(&self) -> u64 {
            let prev = self.get();

            let _ =
                self.n1.fetch_add(1, O::Release) == usize::MAX &&
                self.n2.fetch_add(1, O::Release) == usize::MAX &&
                self.n3.fetch_add(1, O::Release) == usize::MAX &&
                self.n4.fetch_add(1, O::Release) == usize::MAX;

            prev
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_multithread_incr() {
        use std::u32;

        let counter = unsafe { Counter::with_init(u32::MAX as u64 - 80000) };
        let counter = Arc::new(counter);

        let handles: Vec<_> = (0..8)
            .map(|_| {
                let counter = counter.clone();
                thread::spawn(move|| {
                    for _ in 0..80000 {
                        counter.incr();
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), u32::MAX as u64 - 80000 + 640000);

    }

    #[test]
    fn test_incr_returns_prev() {
        let mut prev = 0;
        let counter = Counter::new();

        for _ in 0..80000 {
            let curr = counter.incr();
            assert_eq!(curr, prev);
            prev += 1;
        }
    }
}
