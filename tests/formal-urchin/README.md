# formal-urchin conformance tests

Rewritten copies of upstream Miri tests used by the formal-urchin
Stacked Borrows conformance suite
(https://github.com/priyasiddharth/formal-urchin, `conformance/`), for
the tests whose rewrite REPLACES standard-library code with user-written
code: a local `Option` with std's method bodies, a local trait standing in
for a std operator trait, `alloc::dealloc` for a `Box` drop. (Closures
are lowered since 2026-10-08, so no test here rewrites one any more.) Each file's header names the upstream test and every
rewrite.

Why here: user-written replacements are ordinary program code, so Charon
translates them and Miri records their frames; keeping them in a Miri
checkout next to the upstream tests they derive from makes every change a
`git diff` against the pinned upstream commit.

Apart from this directory, the branch differs from the pinned upstream
commit only by the observation-only certificate event stream in
`src/machine.rs` and `src/concurrency/scheduler.rs` (written only when
`FORMAL_URCHIN_EVENTS` is set; the formal-urchin bootstrap refuses any
other difference), so its verdicts are upstream Miri's at that commit. Miri's own test
harness does not run this directory; formal-urchin's `conformance/
scripts/live.py` runs each file under the pinned Miri and checks its
verdict and UB line against the suite's manifest.
