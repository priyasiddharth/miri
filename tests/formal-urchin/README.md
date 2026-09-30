# formal-urchin conformance tests

Rewritten copies of upstream Miri tests used by the formal-urchin
Stacked Borrows conformance suite
(https://github.com/priyasiddharth/formal-urchin, `conformance/`), for
the tests whose rewrite REPLACES standard-library code with user-written
code: a local `Option` with std's method bodies, a local trait standing in
for a std operator trait, a named `fn` for a closure, `alloc::dealloc` for
a `Box` drop. Each file's header names the upstream test and every
rewrite.

Why here: user-written replacements are ordinary program code, so Charon
translates them and Miri records their frames; keeping them in a Miri
checkout next to the upstream tests they derive from makes every change a
`git diff` against the pinned upstream commit.

Only this directory differs from the pinned upstream commit (the
formal-urchin bootstrap refuses any other difference), so the Miri built
from this checkout is upstream Miri at that commit. Miri's own test
harness does not run this directory; formal-urchin's `conformance/
scripts/live.py` runs each file under the pinned Miri and checks its
verdict and UB line against the suite's manifest.
