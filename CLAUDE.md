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
