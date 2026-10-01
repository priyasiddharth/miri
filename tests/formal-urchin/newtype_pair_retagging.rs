// derived from miri tests/fail/both_borrows/newtype_pair_retagging.rs @ 34d6a7954
// expected: UB — the deallocation through `PTR` would remove the Unique
// item of `_n.0`, the reference inside a struct passed as ScalarPair, which the
// fn-entry retag of `_n` protects for the call (Miri: "… which is
// strongly protected", raised inside std's dealloc)
// rewrites: dropped //@ headers, error annotations and #[rustfmt::skip];
//           the closure -> the named fn `free_it` (closures are not
//           modelled), its captured `ptr` -> the static `PTR`,
//           `impl FnOnce()` -> `fn()`

struct Newtype<'a>(#[allow(dead_code)] &'a mut i32, #[allow(dead_code)] i32);

static mut PTR: *mut i32 = std::ptr::null_mut();

fn dealloc_while_running(_n: Newtype<'_>, dealloc: fn()) {
    dealloc();
}

fn free_it() {
    unsafe { drop(Box::from_raw(PTR)) };
}

// Make sure that we protect references inside structs that are passed as ScalarPair.
fn main() {
    let ptr = Box::into_raw(Box::new(0i32));
    unsafe {
        PTR = ptr;
        dealloc_while_running(Newtype(&mut *ptr, 0), free_it)
    };
}
