# Triage drafts (maintainer only)

Drafts to post manually — nothing here is posted automatically.

Principles: nobody gets ghosted; every superseded contribution gets a
`Co-authored-by:` trailer on the superseding commit plus a CHANGELOG
name-check; every closure comes with a reason and a pointer. Post everything
in one sitting so the backlog reads as a plan, not an apology tour.

Roadmap the drafts assume (matches what is actually built):

- **0.1.2** — tagged `v0.1.2`, compatible fixes: generics/lifetimes/const
  generics (fixes issue #1's compile errors for `'static` uses), spanned
  `syn::Error` diagnostics, hygienic codegen + `#[iterable(crate = "...")]`,
  tests/CI/docs, dual license.
- **0.2.0** — on `main`, next release: `#![no_std]` + zero allocation (#7),
  `IterableMut` (`field_at_mut`/`field_mut`/`visit_fields_mut`, #2),
  `IterableAs` + `#[iterable(to(dyn Trait))]` for non-`'static` structs (#5),
  tuple/unit structs, `skip`/`rename` attributes.

---

## Issue #1 — lifetime-parameterized struct fails to compile

Labels: `bug`. Keep open until 0.2.0 is published, then close with a docs
link.

```markdown
Thanks for the report, and sorry it sat unanswered for so long. Status, in
two parts:

**Fixed in 0.1.2** (a compatible bugfix release): `#[derive(Iterable)]` now
handles generic, lifetime-parameterized, and const-generic structs, so the
compile errors from this issue are gone. The generated impl carries an `Any`
bound per field type, which means it applies whenever the field types are
`'static` (e.g. `MyStruct<'static>`, `Wrapper<String>`). And when something
genuinely can't work, you now get a spanned compile error explaining exactly
why, instead of the old wall of confusion.

The underlying constraint hasn't moved, though: `iter()` hands out
`&dyn Any`, and `Any` requires `'static`, so a field borrowing from a
shorter `'a` can never be erased to `dyn Any`. For genuinely borrowed data,
**0.2.0** (implemented on `main`, next release) has the real answers:

- `#[iterable(skip)]` on the borrowed field — the rest of the struct keeps
  the normal `dyn Any` iteration.
- `#[iterable(to(dyn Trait))]` — a new `IterableAs` mechanism that erases
  fields to a trait object of your choosing instead of `dyn Any`. Your trait
  doesn't carry `Any`'s `'static` requirement, so this works for
  non-`'static` structs.

I'll keep this open until 0.2.0 is published, then close it with a pointer
to the docs.
```

## Issue #2 — iter_mut (plus @mjhouse's two closed, unmerged PRs)

Labels: `enhancement`; milestone 0.2.0. When the release commit lands, it
carries `Co-authored-by: mjhouse <email-from-their-commits>` and the
CHANGELOG says "based on work by @mjhouse".

```markdown
This is happening: mutable field access is implemented on `main` and ships
in 0.2.0. @mjhouse — thank you again for the two earlier PRs; closing them
back then without a proper resolution wasn't fair, and I want to fix that.
The implementing commit carries `Co-authored-by:` credit for you, and the
changelog name-checks the original PRs.

One design note, because the shape is deliberately not a literal
`iter_mut()`: 0.2.0 adds an `IterableMut` trait (emitted by the same derive,
no second derive needed) with three entry points — `field_at_mut(index)`,
`field_mut(name)`, and `visit_fields_mut(closure)`. An `Iterator` yielding
`(&'static str, &mut dyn Any)` can't be built on top of a safe,
manually-implementable trait without `unsafe` aliasing contracts, and the
crate is now `#![forbid(unsafe_code)]`. The visitor covers the "loop over
every field mutably" pattern, the name/index accessors cover targeted
mutation, and the downcast story is the same as `iter()` — just
`downcast_mut` instead of `downcast_ref`. If a sound lending iterator ever
becomes expressible in safe Rust, it can be added on top of this without
breaking anything.
```

## Issue #5 — iterate fields as `&dyn MyCustomTrait`

Labels: `enhancement`; milestone 0.2.0. Post before closing PR #6 so the
closure comment has somewhere to point.

```markdown
This ships in 0.2.0 (implemented on `main`), and it exists because this
thread and @kaimast's #6 proved the demand. What landed:

- `#[iterable(to(dyn MyTrait))]` on the struct generates an
  `IterableAs<dyn MyTrait>` impl: `iter_as()` yields
  `(&'static str, &dyn MyTrait)`. Every non-skipped field must implement the
  trait — enforced at compile time, with the error spanned to the offending
  field.
- The attribute is repeatable, so one struct can expose several traits;
  `iter_fields_as::<dyn Trait>()` disambiguates at the call site.
- It's strictly additive: `iter()` still yields `&dyn Any`, so no existing
  `downcast_ref` call site changes behavior. (`#[iterable(no_any)]` opts out
  of the `dyn Any` impls if you don't want them at all.)
- Because `&dyn MyTrait` doesn't require `'static`, this works for structs
  borrowing non-`'static` data — the thing the `dyn Any` path fundamentally
  can't do.

@srid @kaimast — I'd genuinely value your eyes on this before the release
freezes the shape: does it cover what you needed? Happy to adjust while it's
still unreleased.
```

## PR #6 (@kaimast) — `#[iterable(ToString)]` attribute

Close with this comment — after the issue #5 comment above is posted, never
before. Credit plan: `Co-authored-by: kaimast <email-from-their-commits>` on
the implementation commit if any of the attribute-parsing shape survives,
plus a CHANGELOG name-check regardless.

```markdown
I'm going to close this one, but to be clear: this is a "superseded", not a
"rejected" — this PR is the reason custom-trait iteration is now
implemented, and it ships in 0.2.0. What blocked merging it as-is: (1) it
changes the return type of `iter()` for everyone, which breaks existing
`downcast_ref::<T>()` callers the moment an attribute appears; (2) the
associated-type design means a struct can only ever expose one trait; and
(3) it would have inherited pre-existing bugs (panics instead of spanned
errors, generics being ignored) that had to be fixed underneath first — none
of which is your fault; it was the state of the macro.

The design that grew out of your idea is on `main`: an additive `IterableAs`
trait plus a repeatable `#[iterable(to(dyn Trait))]` attribute, leaving
`iter()`'s `dyn Any` behavior untouched — and it works for non-`'static`
structs, which the original couldn't. I'd love your review before 0.2.0 goes
out (see the discussion on #5). You'll be credited in the commit and
changelog — this feature exists because you pushed on it.
```

## Issue #7 (@Kezii) — no_std

Labels: `bug` (unnecessary platform restriction); milestone 0.2.0.

```markdown
Confirmed and fixed on `main`; ships in 0.2.0. `Any` lives in `core`, so
nothing about the crate's concept ever required std — the `std::` paths in
the trait and the generated code were just historical.

The facade is now `#![no_std]` with **no `alloc` feature and no allocation
at all**: Cortex-M targets often have no allocator, so a feature-gated `Vec`
wouldn't have actually served you. One heads-up that falls out of this:
`iter()` stopped allocating for everyone — it returns a lightweight
`FieldIter` instead of building a `Vec` per call. It's still
`ExactSizeIterator` + `DoubleEndedIterator`, so iterator-adapter code keeps
working; only code that named the old concrete iterator type would notice.

CI now builds a `#![no_std]` fixture crate (and the facade itself) for
`thumbv7em-none-eabihf`, so this can't regress.
```

## Pinned roadmap issue

Create as a new issue, pin it. Suggested title:
**Roadmap: 0.1.2 (compatible fixes) → 0.2.0 (no_std + mutation + custom traits)**.
Replace `#N` references with real links when posting.

```markdown
This issue is the map of where the crate is going, so the backlog reads as
an intentional plan. Two releases:

**0.1.2 — compatible fixes (out now/soon, tag `v0.1.2`)**

- `#[derive(Iterable)]` works on generic, lifetime-parameterized, and
  const-generic structs — the compile errors from #1, fixed for `'static`
  uses.
- Spanned `syn::Error` diagnostics instead of "proc macro panicked".
- Hygienic generated code (fully qualified paths), with
  `#[iterable(crate = "...")]` as the escape hatch for renamed or
  re-exported facades.
- Tests (integration + trybuild UI + expansion snapshots), CI, runnable
  examples, rewritten docs; dual-licensed MIT OR Apache-2.0.
- No API changes: a drop-in replacement for 0.1.1.

**0.2.0 — no_std, mutation, custom trait objects (implemented on `main`,
next release)**

- `#![no_std]` and zero allocation — no `alloc` feature needed; CI proves it
  on `thumbv7em-none-eabihf` (#7).
- Mutable access: `IterableMut` with `field_at_mut` / `field_mut` /
  `visit_fields_mut` (#2, based on work by @mjhouse).
- Custom trait objects: `IterableAs` + repeatable
  `#[iterable(to(dyn Trait))]` — additive to `iter()`, and it works for
  non-`'static` structs (#5, prompted by @kaimast's #6).
- Tuple structs, unit structs, `#[iterable(skip)]` and
  `#[iterable(rename = "...")]` field attributes.
- `iter()` now returns an allocation-free `FieldIter` instead of a
  `Vec` iterator; `field_count`, `field_at`, and by-name `field()` lookup
  added.

**Migration safety:** these are 0.x releases, so cargo never auto-upgrades
anyone across them — `struct_iterable = "0.1"` keeps resolving to 0.1.x
forever; you move to 0.2 by editing your `Cargo.toml`, and the CHANGELOG
carries a migration table for the differences. Depend only on
`struct_iterable`; the `_derive`/`_internal` crates are implementation
details versioned in lockstep.

Comments welcome here or on the linked issues — #5 in particular could use
review from people who wanted custom traits, before 0.2.0 freezes the shape.
```

## Checklist

Suggested labels per item:

| Item | Labels | Milestone / action |
|---|---|---|
| Issue #1 | `bug` | keep open; close after 0.2.0 publishes |
| Issue #2 | `enhancement` | 0.2.0; Co-authored-by @mjhouse on the commit |
| Issue #5 | `enhancement` | 0.2.0; ask @srid + @kaimast for pre-release review |
| PR #6 | — | close as superseded (after the #5 comment exists) |
| Issue #7 | `bug` | 0.2.0 |
| Roadmap issue | `documentation` | pin it |

Publish day, in order:

1. **Publish 0.1.2 from tag `v0.1.2` first** (`git checkout v0.1.2`):
   `cargo publish -p struct_iterable_derive`, then
   `cargo publish -p struct_iterable`. (No internal shim republish at
   0.1.2.)
2. **Then publish 0.2.0 from `main`**, in dependency order:
   `struct_iterable_derive` → `struct_iterable` → `struct_iterable_internal`
   (the shim depends on the facade, so it goes last). Tag `v0.2.0`.
3. Stamp the release dates in `CHANGELOG.md` (the `0.1.2` and `0.2.0`
   headings) on `main`.
4. Create the GitHub releases for `v0.1.2` and `v0.2.0` from the tags, with
   the CHANGELOG sections as release notes.
5. Post the comments above (#1, #2, #5, #7, the PR #6 closure, the pinned
   roadmap issue) in one sitting.
6. Enable private vulnerability reporting (repo Settings → Security →
   "Private vulnerability reporting") so SECURITY.md's instructions work.
7. Add repo topics: `rust`, `derive-macro`, `proc-macro`, `reflection`,
   `struct-fields`, `no-std`.
