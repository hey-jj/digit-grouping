//! Gate 6: the Buffer path performs zero heap allocations.
//!
//! This file holds exactly one test so no parallel test can allocate while
//! the counter window is open.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

// The library forbids unsafe code; this test binary needs it only to
// install the counting allocator around the system one.
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[test]
fn buffer_path_never_allocates() {
    use digit_grouping::{Buffer, Locale};

    let locales: Vec<Locale> = Locale::available_names()
        .iter()
        .map(|name| Locale::from_name(name).unwrap())
        .collect();
    let mut buffer = Buffer::new();

    // Warm up.
    for locale in &locales {
        buffer.write_formatted(i128::MIN, locale);
        buffer.write_formatted(u128::MAX, locale);
    }

    let before = ALLOCATIONS.load(Ordering::SeqCst);
    for locale in &locales {
        for i in 0..10_000u64 {
            let value = i.wrapping_mul(0x9E37_79B9_7F4A_7C15) as i64;
            buffer.write_formatted(value, locale);
        }
    }
    let after = ALLOCATIONS.load(Ordering::SeqCst);
    assert_eq!(after - before, 0, "Buffer formatting allocated");
}
