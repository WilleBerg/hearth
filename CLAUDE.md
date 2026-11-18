# Hearth — engineering conventions

## Comments

Avoid comments that just restate what the code already shows. If a
reasonably careful reader can tell what's happening from the code itself
(names, structure, types), don't add a comment saying so — that includes
placeholder/temporary code, not just code meant to last. Only comment when
there's a non-obvious *why*: a hidden constraint, an invariant, a workaround,
or something that would surprise a reader. When a comment is warranted, keep
it short — a line or two, not a paragraph.

## Magic numbers

Magic numbers are heavily discouraged. If a literal value is reused, or is
something someone would plausibly want to tune, give it a name (a `const`)
instead of repeating the bare literal — even in code that's explicitly
temporary or a placeholder.

## Reviewing PRs

When reviewing a PR (yours or someone else's), check for the following in
addition to the comment and magic-number rules above:

**Correctness & error handling**
- No `Result`/`Option` is silently dropped. Watch especially for
  `.map_err(...)` or `.map(...)` called for a side effect (e.g. logging)
  where the returned `Result` is then discarded — the error path goes
  silent. Use `if let Err(e) = ...` or propagate with `?` instead.
- `unwrap()`/`expect()`/`panic!` are not used on paths that can fail at
  runtime from user input, I/O, or external state (config files, subprocess
  spawning, network). They're fine in tests or for invariants that are
  truly unreachable — say why in a comment if it's not obvious.
- Errors carry enough context to be debuggable (custom error enums with
  `Display`/`Error` impls beat bubbling up opaque strings).

**Simplicity & maintainability**
- Flag code that's more complex than the problem needs — a manual loop
  where an iterator adapter reads more clearly, a helper that's only
  called once and doesn't clarify anything, an abstraction built for a
  hypothetical future case. Prefer the simpler version.
- No dead code, unused imports, or leftover debug prints.
- Function and variable names say what they hold/do without needing the
  reader to check the implementation.
- New logic that isn't trivial has test coverage, or a clear reason why
  not (e.g. it's a thin wrapper over something already tested).

**Rust-specific efficiency**
- No unnecessary `.clone()` — check whether a borrow (`&T`) would do
  instead, especially when the clone immediately gets passed to something
  that only reads the value. Cloning is fine when ownership genuinely
  needs to move across a boundary (e.g. into a thread or async task) —
  just don't reach for it as a default over fighting the borrow checker.
- Prefer borrowing (`&str`, `&[T]`) over owned types (`String`, `Vec<T>`)
  in function signatures unless ownership is actually needed.
- Prefer iterator chains over manual index loops where they're at least as
  readable.
- Watch for avoidable allocations in hot paths (e.g. building a `Vec` just
  to iterate it once, `format!` where a `write!` to an existing buffer
  would do).

**Consistency**
- New code follows the patterns already established nearby (error
  handling style, module layout, naming) rather than introducing a new
  convention for the same kind of problem.
- `cargo clippy` and `cargo fmt` are clean before merging.
