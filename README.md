# semrs
![Build](https://github.com/obicons/semrs/actions/workflows/rust.yml/badge.svg)
![crates.io](https://github.com/obicons/semrs/actions/workflows/rust.yml/badge.svg)


This is a dependency-free, pure Rust implementation of counting semaphores.

Crates.io: [https://crates.io/crates/semrs](https://crates.io/crates/semrs).

GitHub repository: [https://github.com/obicons/semrs](https://github.com/obicons/semrs).

## About
Unfortunately, the use of semaphores often requires unsafe
blocks. Fundamentally, a semaphore does not provide exclusive
access. It provides *limited* access. It's up to the programmer to
ensure that the access is ultimately free from
dataraces. 

The semaphore is a useful concurrency primitive despite this severe
limitation. For example, semaphores can be used to implement a bounded
buffer to solve the readers/writers problem without
busy-waiting. Semaphores are useful for implementing safe concurrency
primitives.

## Usage
```
// Two references to a binary semaphore.
let s1 = Arc::new(Semaphore::new(1));
let s2 = s1.clone();

// A shared resource, r, and a pointer to it, p.
let mut r = UnsafeCell::new(0);
let p: usize = r.get() as usize;

// Thread 1.
let t1 = std::thread::spawn(move || {
    s1.down();
    let p: *mut i32 = p as *mut i32;
    unsafe { *p += 1; }
    s1.up();
});

// Thread 2.
let t2 = std::thread::spawn(move || {
    s2.down();
    let p: *mut i32 = p as *mut i32;
    unsafe { *p += 1; }
    s2.up();
});

let _ = t1.join();
let _ = t2.join();

// The access was sychronized.
assert!(*r.get_mut() == 2);
```
