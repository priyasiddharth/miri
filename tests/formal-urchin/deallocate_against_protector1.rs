// derived from miri tests/fail/both_borrows/deallocate_against_protector1.rs @ 34d6a7954
// expected: UB — `f` deallocates the allocation `x` points to while `x` is
// protected by `inner`'s fn-entry retag (Miri: "deallocating while item
// [Unique for …] is strongly protected", raised inside std's dealloc)
// rewrites: dropped //@ headers and error annotations;
//           the closure -> the named fn `free_it` (closures are not modelled);
//           (upstream `drop(Box::from_raw(raw))` restored 2026-10-01 with
//           Box drop glue; upstream `Box::leak(Box::new(0))` 2026-09-29)



fn inner(x: &mut i32, f: fn(&mut i32)) {
    // `f` may mutate, but it may not deallocate!
    f(x)
}

fn free_it(x: &mut i32) {
    let raw = x as *mut _;
    drop(unsafe { Box::from_raw(raw) });
}

fn main() {
    inner(Box::leak(Box::new(0)), free_it);
}
