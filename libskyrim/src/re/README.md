# RE API Notes

## Enum Seams

`re` currently contains a large number of enum declarations, but only a much
smaller subset are actually risky from a Rust validity perspective.

Local audit on `2026-03-31`:

- `359` enum declarations under `libskyrim/src/re`
- `222` files containing at least one enum declaration
- explicit unsafe raw-to-enum decode hotspots were concentrated in a few files:
  - `actor_state.rs`
  - `gfx_resource.rs`
  - `cfilter.rs`

This is the key distinction:

- **closed enum surface**
  - parameters we pass into the engine
  - named constants used purely in Rust
  - flags or values where the storage is authored by our code or source-backed
    code proves the value set is exhaustive for the read path
- **open enum seam**
  - values decoded from engine bitfields
  - values read from raw struct storage written by the engine
  - values returned from native code where the underlying integer can be
    unknown, incomplete in our translation, modded, or runtime-extended

For the second category, a plain Rust enum must **not** be the only storage
representation. External integers can be valid engine data while still being
invalid Rust enum discriminants.

## Existing Building Block

The project already has a suitable low-level wrapper:

- `core_util::Enum<E, U>`

This maps directly to `REX::Enum<E, U>` and is the preferred representation for
open enum seams.

Useful surface:

- `Enum::from_underlying(raw)`
- `Enum::underlying()`
- `Enum::get() -> Option<E>`
- `Enum::is_known() -> bool`
- `Enum::get_or(default)`
- `Enum::get_or_else(...)`
- `Enum::map_or(...)`
- `Enum::map_or_else(...)`

## Recommended Pattern

For raw-backed enum storage or bitfield extraction, prefer this API shape:

1. raw/storage accessor

```rust
pub const fn collision_layer_storage(&self) -> Enum<ColLayer, i32>
```

2. typed optional accessor

```rust
pub fn try_get_collision_layer(&self) -> Option<ColLayer>
```

3. optional lossy convenience only when there is an honest sentinel

```rust
pub fn get_collision_layer(&self) -> ColLayer
```

This keeps three distinct contracts available:

- exact raw integer storage
- typed known-value interpretation
- convenience fallback for call sites that already want a sentinel

Prefer the shared enum attribute when possible:

- `#[libskyrim_macros::open_enum]`
- `#[libskyrim_macros::open_enum(ignore(HelperVariant, ...))]`

It reads the enum's `#[repr(...)]`, inspects discriminants, and generates:

- `impl core_util::EnumSetType<repr>`
- `impl TryFrom<repr>`

The attribute automatically chooses:

- contiguous range decoding when every value in `min..=max` is present
- sparse match decoding when the enum has gaps

Use it only for seam enums whose variants all represent genuine decoded values.
If a translated enum also contains helper constants, aliases, masks, shifts, or
other non-value sentinels, prefer an explicit sparse mapping helper instead of
blind auto-derivation.

The `ignore(...)` form exists specifically for mixed enums such as parts of the
GFx/Scaleform layer where most variants are genuine decoded values but a small
number of variants are helper constants like `*_Mask` or `*_Shift`.

If the type is more deeply mixed, for example a value code plus modifier bits
plus masks, do not force it through `open_enum`. Use a raw storage/newtype view
and expose typed interpretation helpers on top.

Keep the lower-level macro helpers as fallback for edge cases where the
attribute cannot infer the intended mapping cleanly:

- contiguous seams:
  - `core_util::impl_enum_try_from_contiguous!(EnumTy => StorageTy, MinVariant, MaxVariant);`
- sparse seams:
  - `core_util::impl_enum_try_from_sparse!(EnumTy => StorageTy { raw => Variant, ... });`

This keeps the seam policy reusable without requiring every enum to grow its
own local `from_raw(...)` implementation.

## Minimal-Refactor Strategy

Do **not** refactor all enums in `re`.

Instead:

- keep ordinary closed enums as ordinary Rust enums
- add `TryFrom<raw>` only for enums that sit on an open seam
- migrate only the decode sites, not every enum declaration
- prefer `*_storage()` + `try_*()` accessors over direct `transmute`
- for virtual/native methods that return an enum-like value, prefer a raw
  `*_raw()` return plus `*_storage()` / `try_*()` / optional lossy `get_*()`
  wrapper instead of exposing the vfunc itself as a closed Rust enum return

This lets the library stay ergonomic without pretending every engine-provided
integer is already a valid Rust enum.

## First Migration Targets

After `cfilter`, the next obvious raw-decode candidates are:

- `actor_state.rs`
  - bitfield slices currently decoded through private `from_raw(...)` helpers
- `gfx_resource.rs`
  - `resource_type_code` decoding currently uses direct `transmute`

Both should move toward the same storage-first pattern instead of adding more
literal raw-to-enum transmutes.
