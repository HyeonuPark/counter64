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
            let _ =
                self.n1.fetch_add(1, O::Release) == usize::MAX &&
                self.n2.fetch_add(1, O::Release) == usize::MAX;

            self.get()
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
            let _ =
                self.n1.fetch_add(1, O::Release) == usize::MAX &&
                self.n2.fetch_add(1, O::Release) == usize::MAX &&
                self.n3.fetch_add(1, O::Release) == usize::MAX &&
                self.n4.fetch_add(1, O::Release) == usize::MAX;

            self.get()
        }
    }
}

// Probably not even needed, but why not?
#[cfg(target_pointer_width = "8")]
mod counter {
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering as O};
    use std::usize;

    #[derive(Debug)]
    pub struct Counter {
        n1: AtomicUsize,
        n2: AtomicUsize,
        n3: AtomicUsize,
        n4: AtomicUsize,
        n5: AtomicUsize,
        n6: AtomicUsize,
        n7: AtomicUsize,
        n8: AtomicUsize,
    }

    pub const COUNTER_INIT: Counter = Counter {
        n1: ATOMIC_USIZE_INIT,
        n2: ATOMIC_USIZE_INIT,
        n3: ATOMIC_USIZE_INIT,
        n4: ATOMIC_USIZE_INIT,
        n5: ATOMIC_USIZE_INIT,
        n6: ATOMIC_USIZE_INIT,
        n7: ATOMIC_USIZE_INIT,
        n8: ATOMIC_USIZE_INIT,
    };

    impl Counter {
        /// Create new counter from 0
        pub fn new() -> Self {
            COUNTER_INIT
        }

        /// Get counter's current value
        pub fn get(&self) -> u64 {
            loop {
                let n1 = self.n1.load(O::SeqCst);
                let n2 = self.n2.load(O::SeqCst);
                let n3 = self.n3.load(O::SeqCst);
                let n4 = self.n4.load(O::SeqCst);
                let n5 = self.n1.load(O::SeqCst);
                let n6 = self.n2.load(O::SeqCst);
                let n7 = self.n3.load(O::SeqCst);
                let n8 = self.n4.load(O::SeqCst);

                if n1 == self.n1.load(O::SeqCst) {
                    let mut count = 0u64;
                    count += n8;
                    count <<= 8;
                    count += n7;
                    count <<= 8;
                    count += n6;
                    count <<= 8;
                    count += n5;
                    count <<= 8;
                    count += n4;
                    count <<= 8;
                    count += n3;
                    count <<= 8;
                    count += n2;
                    count <<= 8;
                    count += n1;

                    return count;
                }
            }
        }

        /// Increase counter by 1, and return previous value
        pub fn incr(&self) -> u64 {
            let _ =
                self.n1.fetch_add(1, O::Release) == usize::MAX &&
                self.n2.fetch_add(1, O::Release) == usize::MAX &&
                self.n3.fetch_add(1, O::Release) == usize::MAX &&
                self.n4.fetch_add(1, O::Release) == usize::MAX &&
                self.n5.fetch_add(1, O::Release) == usize::MAX &&
                self.n6.fetch_add(1, O::Release) == usize::MAX &&
                self.n7.fetch_add(1, O::Release) == usize::MAX &&
                self.n8.fetch_add(1, O::Release) == usize::MAX;

            self.get()
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
        let counter = Arc::new(Counter::new());

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

        assert_eq!(counter.get(), 640000);

    }
}
