use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    barrier: Condvar,
    lock: Mutex<u32>,
}

impl Semaphore {
    /// Create a new semaphore representing `count` resources.
    pub fn new(count: u32) -> Semaphore {
        Semaphore {
            barrier: Condvar::new(),
            lock: Mutex::new(count),
        }
    }

    /// Consumes one resource from the semaphore.
    /// If no resources are available, blocks the current thread until
    /// one becomes present.
    pub fn down(&self) {
        let mut has_resource = false;
        while !has_resource {
            let mut guard = self.lock.lock().unwrap();
            has_resource = *guard > 0;
            if has_resource {
                *guard -= 1;
            } else {
                let _unused = self.barrier.wait(guard);
            }
        }
    }

    /// Provides one resource back to the semaphore.
    pub fn up(&self) {
        *self.lock.lock().unwrap() += 1;
        self.barrier.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::UnsafeCell, sync::Arc};

    use super::*;

    #[test]
    fn test_single_thread_bin_semaphore() {
        let s = Semaphore::new(1);
        s.down();
        s.up();
    }

    #[test]
    fn test_two_thread_bin_semaphore() {
        let s1 = Arc::new(Semaphore::new(1));
        let s2 = s1.clone();
        let mut r = UnsafeCell::new(0);
        let p: usize = r.get() as usize;
        let t1 = std::thread::spawn(move || {
            s1.down();
            let p: *mut i32 = p as *mut i32;
            unsafe {
                *p += 1;
            }
            s1.up();
        });
        let t2 = std::thread::spawn(move || {
            s2.down();
            let p: *mut i32 = p as *mut i32;
            unsafe {
                *p += 1;
            }
            s2.up();
        });
        let _ = t1.join();
        let _ = t2.join();
        assert!(*r.get_mut() == 2);
    }
}
