//! Procedural macros backing `libskyrim` and its SDK-facing authoring surface.
//!
//! Most plugin authors do not need to depend on `libskyrim-macros` directly:
//!
//! - hook attributes are re-exported from `libskyrim::sdk::hooks`
//! - event attributes are re-exported from `libskyrim::sdk::events`
//! - `Cosave` is re-exported from `libskyrim::sdk::plugin::serialization`
//!
//! The main direct-use exception is `#[libskyrim_macros::open_enum]`, which is
//! part of the lower-level `re` translation workflow rather than the SDK.
//!
//! Decision guide:
//!
//! - use `function_hook`, `call_hook`, `vtable_hook`, or `vcall_hook` for
//!   high-level hook authoring with generated install glue
//! - use `game_event`, `ui_event`, `dispatcher_event`, `input_event`,
//!   `message_event`, or `bus_event` for event callbacks that should generate
//!   `EventInstaller`-style modules
//! - use `#[derive(Cosave)]` for value-like persistence payloads
//! - use `#[open_enum]` for fieldless integer enums in `re` translations that
//!   should stay open to unknown engine values

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{
    BinOp, Data, DeriveInput, Expr, ExprBinary, ExprCall, ExprCast, ExprGroup, ExprLit, ExprParen,
    ExprPath, ExprUnary, Fields, FnArg, GenericArgument, GenericParam, Ident, ItemEnum, ItemFn,
    Lit, LitInt, LitStr, Meta, Pat, PatIdent, Path, PathArguments, ReturnType, Token, Type,
    TypeGroup, TypeParen, TypePath, UnOp, Visibility, parse_macro_input, parse_quote,
};

/// High-level attribute hook for function-entry detours.
///
/// This macro is re-exported as `libskyrim::sdk::hooks::function_hook`.
///
/// Use it when the plugin is patching one function entry and wants Rust-facing
/// parameters plus a generated sibling hook module with `try_install()`,
/// `INSTALLER`, and batch-install support.
///
/// ```rust,ignore
/// use libskyrim::re::Actor;
/// use libskyrim::relocation::RelocationID;
/// use libskyrim::sdk::hooks;
///
/// #[hooks::function_hook(
///     target = RelocationID::new(123, 456),
///     guard = hooks::guards::default(),
/// )]
/// fn sample_hook(
///     original: hooks::Original<fn(&Actor, bool) -> bool>,
///     actor: &Actor,
///     value: bool,
/// ) -> bool {
///     !original.call(actor, value)
/// }
/// ```
#[proc_macro_attribute]
pub fn function_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Function, attr, item)
}

/// High-level attribute hook for one concrete call site.
///
/// This macro is re-exported as `libskyrim::sdk::hooks::call_hook`.
#[proc_macro_attribute]
pub fn call_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Call, attr, item)
}

/// High-level attribute hook for one vtable slot.
///
/// This macro is re-exported as `libskyrim::sdk::hooks::vtable_hook`.
#[proc_macro_attribute]
pub fn vtable_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Vtable, attr, item)
}

/// High-level attribute hook for one indirect virtual call site.
///
/// This macro is re-exported as `libskyrim::sdk::hooks::vcall_hook`.
#[proc_macro_attribute]
pub fn vcall_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Vcall, attr, item)
}

/// Attribute event installer for gameplay events from
/// `ScriptEventSourceHolder`.
///
/// This macro is re-exported as `libskyrim::sdk::events::game_event`.
#[proc_macro_attribute]
pub fn game_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Game, attr, item)
}

/// Attribute event installer for UI events from the `UI` singleton.
///
/// This macro is re-exported as `libskyrim::sdk::events::ui_event`.
#[proc_macro_attribute]
pub fn ui_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Ui, attr, item)
}

/// Attribute event installer for SKSE dispatcher events.
///
/// This macro is re-exported as `libskyrim::sdk::events::dispatcher_event`.
#[proc_macro_attribute]
pub fn dispatcher_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Dispatcher, attr, item)
}

/// Attribute event installer for the SDK input-chain callback surface.
///
/// This macro is re-exported as `libskyrim::sdk::events::input_event`.
#[proc_macro_attribute]
pub fn input_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Input, attr, item)
}

/// Attribute event installer for SKSE plugin messaging.
///
/// This macro is re-exported as `libskyrim::sdk::events::message_event`.
///
/// It supports `kind = ...`, `plugin_phase = ...`, `game_phase = ...`, or
/// `phase = ...`, plus optional `sender = "PluginName"` filtering.
#[proc_macro_attribute]
pub fn message_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Message, attr, item)
}

/// Attribute event installer for the local synchronous SDK event bus.
///
/// This macro is re-exported as `libskyrim::sdk::events::bus_event`.
#[proc_macro_attribute]
pub fn bus_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Bus, attr, item)
}

/// Derive `CosaveEncode` and `CosaveDecode` for one value-like persistence
/// struct.
///
/// This derive is re-exported as
/// `libskyrim::sdk::plugin::serialization::Cosave`.
///
/// Supported field attributes:
///
/// - `#[cosave(skip)]`
/// - `#[cosave(default)]`
/// - `#[cosave(with = path)]`
#[proc_macro_derive(Cosave, attributes(cosave))]
pub fn derive_cosave(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_cosave_derive(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// Generate `TryFrom<repr>` for a fieldless integer enum without pretending
/// the enum is closed over future engine values.
///
/// This macro is commonly used directly as `#[libskyrim_macros::open_enum]`
/// inside `re` translations.
///
/// Optional configuration:
///
/// - `#[open_enum(ignore(HelperVariant, ...))]`
#[proc_macro_attribute]
pub fn open_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ts = TokenStream2::from(attr);
    let item_enum = parse_macro_input!(item as ItemEnum);
    match expand_open_enum(attr_ts, item_enum) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HookKind {
    Function,
    Call,
    Vtable,
    Vcall,
}

fn expand_open_enum(attr: TokenStream2, item: ItemEnum) -> syn::Result<TokenStream2> {
    let config = parse_open_enum_config(attr)?;

    let repr_ty = parse_open_enum_repr(&item)?;
    let discriminants = collect_open_enum_discriminants(&item)?;
    let enum_ident = &item.ident;
    let enum_item = &item;
    let try_from_impl = build_open_enum_try_from(
        enum_ident,
        &repr_ty,
        &discriminants,
        &config.ignored_variants,
    )?;

    Ok(quote! {
        #enum_item

        impl ::core_util::EnumSetType<#repr_ty> for #enum_ident {
            #[inline(always)]
            fn to_underlying(self) -> #repr_ty {
                self as #repr_ty
            }
        }

        #try_from_impl
    })
}

#[derive(Default)]
struct OpenEnumConfig {
    ignored_variants: BTreeSet<String>,
}

fn parse_open_enum_config(attr: TokenStream2) -> syn::Result<OpenEnumConfig> {
    let mut config = OpenEnumConfig::default();
    if attr.is_empty() {
        return Ok(config);
    }

    let metas = Punctuated::<Meta, Token![,]>::parse_terminated.parse2(attr)?;
    for meta in metas {
        match meta {
            Meta::List(list) if list.path.is_ident("ignore") => {
                let paths =
                    list.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
                for path in paths {
                    let ident = path.get_ident().ok_or_else(|| {
                        syn::Error::new_spanned(
                            &path,
                            "`ignore(...)` entries in `#[open_enum]` must be simple identifiers",
                        )
                    })?;
                    config.ignored_variants.insert(ident.to_string());
                }
            }
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "unsupported `#[open_enum(...)]` argument; expected `ignore(...)`",
                ));
            }
        }
    }

    Ok(config)
}

fn parse_open_enum_repr(item: &ItemEnum) -> syn::Result<Type> {
    for attr in &item.attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }

        let reprs = attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
        for repr in reprs {
            if let Some(ident) = repr.get_ident() {
                match ident.to_string().as_str() {
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                    | "u64" | "u128" | "usize" => {
                        let repr_ty: Type = syn::parse2(quote!(#repr))?;
                        return Ok(repr_ty);
                    }
                    _ => {}
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        &item.ident,
        "`#[open_enum]` requires an integer `#[repr(...)]` on the enum",
    ))
}

fn collect_open_enum_discriminants(item: &ItemEnum) -> syn::Result<Vec<(Ident, i128)>> {
    let mut known = BTreeMap::<String, i128>::new();
    let mut values = Vec::with_capacity(item.variants.len());
    let mut next_implicit = 0i128;

    for variant in &item.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                "`#[open_enum]` supports fieldless enums only",
            ));
        }

        let value = match &variant.discriminant {
            Some((_, expr)) => evaluate_open_enum_expr(expr, &known)?,
            None => next_implicit,
        };

        if known.insert(variant.ident.to_string(), value).is_some() {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                "duplicate enum variant name in `#[open_enum]` expansion",
            ));
        }

        values.push((variant.ident.clone(), value));
        next_implicit = value
            .checked_add(1)
            .ok_or_else(|| syn::Error::new_spanned(&variant.ident, "enum discriminant overflow"))?;
    }

    Ok(values)
}

fn evaluate_open_enum_expr(expr: &Expr, known: &BTreeMap<String, i128>) -> syn::Result<i128> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit), ..
        }) => parse_open_enum_lit_int(lit),
        Expr::Unary(ExprUnary { op, expr, .. }) => match op {
            UnOp::Neg(_) => evaluate_open_enum_expr(expr, known)?
                .checked_neg()
                .ok_or_else(|| syn::Error::new_spanned(expr, "enum discriminant overflow")),
            UnOp::Not(_) => Ok(!evaluate_open_enum_expr(expr, known)?),
            _ => Err(syn::Error::new_spanned(
                op,
                "unsupported unary operator in `#[open_enum]` discriminant",
            )),
        },
        Expr::Paren(ExprParen { expr, .. })
        | Expr::Group(ExprGroup { expr, .. })
        | Expr::Cast(ExprCast { expr, .. }) => evaluate_open_enum_expr(expr, known),
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            let left_value = evaluate_open_enum_expr(left, known)?;
            let right_value = evaluate_open_enum_expr(right, known)?;
            evaluate_open_enum_binary(expr, left_value, op, right_value)
        }
        Expr::Path(ExprPath { path, .. }) => {
            let ident = path.segments.last().ok_or_else(|| {
                syn::Error::new_spanned(path, "empty path in `#[open_enum]` discriminant")
            })?;
            known.get(&ident.ident.to_string()).copied().ok_or_else(|| {
                syn::Error::new_spanned(
                    path,
                    "unsupported path in `#[open_enum]` discriminant; only previously-defined variants are supported",
                )
            })
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "unsupported expression in `#[open_enum]` discriminant",
        )),
    }
}

fn evaluate_open_enum_binary(
    expr: &Expr,
    left: i128,
    op: &BinOp,
    right: i128,
) -> syn::Result<i128> {
    let value = match op {
        BinOp::Add(_) => left.checked_add(right),
        BinOp::Sub(_) => left.checked_sub(right),
        BinOp::Mul(_) => left.checked_mul(right),
        BinOp::Div(_) => left.checked_div(right),
        BinOp::Rem(_) => left.checked_rem(right),
        BinOp::BitXor(_) => Some(left ^ right),
        BinOp::BitAnd(_) => Some(left & right),
        BinOp::BitOr(_) => Some(left | right),
        BinOp::Shl(_) => {
            let shift = u32::try_from(right).ok();
            shift.and_then(|shift| left.checked_shl(shift))
        }
        BinOp::Shr(_) => {
            let shift = u32::try_from(right).ok();
            shift.and_then(|shift| left.checked_shr(shift))
        }
        _ => {
            return Err(syn::Error::new_spanned(
                op,
                "unsupported binary operator in `#[open_enum]` discriminant",
            ));
        }
    };

    value.ok_or_else(|| syn::Error::new_spanned(expr, "enum discriminant overflow"))
}

fn parse_open_enum_lit_int(lit: &LitInt) -> syn::Result<i128> {
    let suffix = lit.suffix();
    let mut raw = lit.to_string();
    if !suffix.is_empty() {
        raw.truncate(raw.len() - suffix.len());
    }

    let raw = raw.replace('_', "");
    let (radix, digits) = if let Some(rest) = raw.strip_prefix("0x") {
        (16, rest)
    } else if let Some(rest) = raw.strip_prefix("0X") {
        (16, rest)
    } else if let Some(rest) = raw.strip_prefix("0o") {
        (8, rest)
    } else if let Some(rest) = raw.strip_prefix("0O") {
        (8, rest)
    } else if let Some(rest) = raw.strip_prefix("0b") {
        (2, rest)
    } else if let Some(rest) = raw.strip_prefix("0B") {
        (2, rest)
    } else {
        (10, raw.as_str())
    };

    i128::from_str_radix(digits, radix).map_err(|_| {
        syn::Error::new_spanned(lit, "failed to parse integer literal in `#[open_enum]`")
    })
}

fn build_open_enum_try_from(
    enum_ident: &Ident,
    repr_ty: &Type,
    discriminants: &[(Ident, i128)],
    ignored_variants: &BTreeSet<String>,
) -> syn::Result<TokenStream2> {
    if discriminants.is_empty() {
        return Err(syn::Error::new_spanned(
            enum_ident,
            "`#[open_enum]` requires at least one enum variant",
        ));
    }

    let mut sorted_values = Vec::with_capacity(discriminants.len());
    let mut sparse_arms = Vec::with_capacity(discriminants.len());
    let mut seen_values = BTreeMap::<i128, Ident>::new();

    for (variant_ident, value) in discriminants {
        if ignored_variants.contains(&variant_ident.to_string()) {
            continue;
        }

        if let Some(previous) = seen_values.insert(*value, variant_ident.clone()) {
            return Err(syn::Error::new_spanned(
                variant_ident,
                format!(
                    "`#[open_enum]` does not support duplicate discriminants: `{}` and `{}` both map to {}",
                    previous, variant_ident, value
                ),
            ));
        }
        sorted_values.push(*value);
        let pattern = open_enum_pattern_literal(*value);
        sparse_arms.push(quote! { #pattern => Ok(Self::#variant_ident), });
    }

    if sorted_values.is_empty() {
        return Err(syn::Error::new_spanned(
            enum_ident,
            "`#[open_enum]` ignored every enum variant; at least one decoded value variant must remain",
        ));
    }

    sorted_values.sort_unstable();
    let is_contiguous = sorted_values
        .windows(2)
        .all(|window| window[1] == window[0] + 1);

    if is_contiguous {
        let min_value = sorted_values[0];
        let max_value = *sorted_values.last().unwrap();
        let min_expr = open_enum_typed_literal(min_value, repr_ty);
        let max_expr = open_enum_typed_literal(max_value, repr_ty);
        Ok(quote! {
            impl ::core::convert::TryFrom<#repr_ty> for #enum_ident {
                type Error = ();

                #[inline(always)]
                fn try_from(value: #repr_ty) -> Result<Self, Self::Error> {
                    if value < #min_expr || value > #max_expr {
                        return Err(());
                    }

                    Ok(unsafe { ::core::mem::transmute::<#repr_ty, Self>(value) })
                }
            }
        })
    } else {
        Ok(quote! {
            impl ::core::convert::TryFrom<#repr_ty> for #enum_ident {
                type Error = ();

                #[inline(always)]
                fn try_from(value: #repr_ty) -> Result<Self, Self::Error> {
                    match value {
                        #(#sparse_arms)*
                        _ => Err(()),
                    }
                }
            }
        })
    }
}

fn open_enum_pattern_literal(value: i128) -> TokenStream2 {
    if value < 0 {
        let magnitude = proc_macro2::Literal::i128_unsuffixed(-value);
        quote! { -#magnitude }
    } else {
        let literal = proc_macro2::Literal::i128_unsuffixed(value);
        quote! { #literal }
    }
}

fn open_enum_typed_literal(value: i128, repr_ty: &Type) -> TokenStream2 {
    if value < 0 {
        let magnitude = proc_macro2::Literal::i128_unsuffixed(-value);
        quote! { (-(#magnitude) as #repr_ty) }
    } else {
        let literal = proc_macro2::Literal::i128_unsuffixed(value);
        quote! { (#literal as #repr_ty) }
    }
}

#[derive(Clone)]
enum GuardPolicy {
    Original,
    Skip,
    Default,
    Return(Expr),
}

#[derive(Clone)]
struct GuardPolicies {
    invalid: GuardPolicy,
    null: Option<GuardPolicy>,
    unresolved: Option<GuardPolicy>,
    convert_fail: Option<GuardPolicy>,
}

impl Default for GuardPolicies {
    fn default() -> Self {
        Self {
            invalid: GuardPolicy::Original,
            null: None,
            unresolved: None,
            convert_fail: None,
        }
    }
}

#[derive(Default)]
struct GuardOverrides {
    invalid: Option<GuardPolicy>,
    null: Option<GuardPolicy>,
    unresolved: Option<GuardPolicy>,
    convert_fail: Option<GuardPolicy>,
}

#[derive(Default)]
struct HookConfig {
    target: Option<Expr>,
    address: Option<Expr>,
    vtable: Option<Expr>,
    offset: Option<Expr>,
    size: Option<Expr>,
    index: Option<Expr>,
    slot: Option<Expr>,
    receiver: Option<Ident>,
    guard_preset: Option<GuardPolicies>,
    guard_overrides: GuardOverrides,
}

impl HookConfig {
    fn resolved_guards(&self) -> GuardPolicies {
        let mut guards = self.guard_preset.clone().unwrap_or_default();

        if let Some(policy) = &self.guard_overrides.invalid {
            guards.invalid = policy.clone();
        }
        if let Some(policy) = &self.guard_overrides.null {
            guards.null = Some(policy.clone());
        }
        if let Some(policy) = &self.guard_overrides.unresolved {
            guards.unresolved = Some(policy.clone());
        }
        if let Some(policy) = &self.guard_overrides.convert_fail {
            guards.convert_fail = Some(policy.clone());
        }

        guards
    }
}

struct ParsedFn {
    function: ItemFn,
    vis: Visibility,
    fn_name: Ident,
    hook_mod_name: Ident,
    hidden_mod_name: Ident,
    ret_ty: Type,
    returns_unit: bool,
    original_param_ty: Option<Type>,
    user_params: Vec<UserParam>,
}

#[derive(Clone)]
struct UserParam {
    ident: Ident,
    ty: Type,
    abi_ty: Type,
}

fn expand_hook(kind: HookKind, attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ts = proc_macro2::TokenStream::from(attr);
    let item_fn = parse_macro_input!(item as ItemFn);

    match expand_hook_impl(kind, attr_ts, item_fn) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn expand_hook_impl(
    kind: HookKind,
    attr: TokenStream2,
    function: ItemFn,
) -> syn::Result<TokenStream2> {
    let config = parse_config(attr)?;
    let parsed = parse_function(function)?;
    let guards = config.resolved_guards();

    validate_config(kind, &config, &guards, &parsed)?;

    let fn_name = &parsed.fn_name;
    let vis = &parsed.vis;
    let hook_mod_name = &parsed.hook_mod_name;
    let hidden_mod_name = &parsed.hidden_mod_name;
    let ret_ty = &parsed.ret_ty;
    let user_params = &parsed.user_params;
    let raw_sig_ty = raw_signature_type(user_params, ret_ty);
    let raw_arg_idents: Vec<Ident> = user_params
        .iter()
        .map(|param| format_ident!("__sdk_raw_{}", param.ident))
        .collect();
    let user_arg_idents: Vec<&Ident> = user_params.iter().map(|param| &param.ident).collect();
    let abi_tys: Vec<&Type> = user_params.iter().map(|param| &param.abi_ty).collect();

    let receiver_index = if kind == HookKind::Vcall {
        let receiver = config
            .receiver
            .as_ref()
            .expect("validated receiver must exist");
        Some(
            user_params
                .iter()
                .position(|param| &param.ident == receiver)
                .expect("validated receiver must exist in params"),
        )
    } else {
        None
    };

    let original_raw_fn =
        build_original_raw_fn(kind, user_params, &raw_arg_idents, ret_ty, receiver_index)?;
    let original_user_fn = build_original_user_fn(&parsed, &raw_arg_idents, ret_ty);
    let conversion_stmts = build_conversion_stmts(
        &guards,
        user_params,
        &raw_arg_idents,
        ret_ty,
        parsed.returns_unit,
    );
    let user_call = if let Some(original_ty) = &parsed.original_param_ty {
        let original_user_fn_name = format_ident!("__sdk_original_user");
        quote! {
            let original: #original_ty =
                ::libskyrim::sdk::hooks::Original::new(#original_user_fn_name as _);
            super::#fn_name(original, #(#user_arg_idents),*)
        }
    } else {
        quote! {
            super::#fn_name(#(#user_arg_idents),*)
        }
    };

    let install_state = build_install_state(kind);
    let install_fns = build_install_fns(kind, &config, &parsed, &raw_sig_ty, receiver_index)?;

    let extra_runtime_items = if kind == HookKind::Vcall {
        quote! {
            static VTABLE_INDEX: ::libskyrim::core_util::Later<usize> =
                ::libskyrim::core_util::Later::new();

            #[inline(always)]
            fn __sdk_vtable_index() -> usize {
                *VTABLE_INDEX
            }
        }
    } else {
        TokenStream2::new()
    };

    let function_item = &parsed.function;

    Ok(quote! {
        #function_item

        #[doc(hidden)]
        mod #hidden_mod_name {
            #[allow(unused_imports)]
            use super::*;

            type __SdkRawSignature = #raw_sig_ty;

            #install_state
            #extra_runtime_items

            #original_raw_fn
            #original_user_fn

            extern "C" fn __sdk_detour(#(#raw_arg_idents: #abi_tys),*) -> #ret_ty {
                #(#conversion_stmts)*
                #user_call
            }

            #install_fns
        }

        #vis use #hidden_mod_name as #hook_mod_name;
    })
}

fn parse_config(attr: TokenStream2) -> syn::Result<HookConfig> {
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let metas = parser.parse2(attr)?;
    let mut config = HookConfig::default();

    for meta in metas {
        match meta {
            Meta::NameValue(nv) if nv.path.is_ident("target") => config.target = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("address") => config.address = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("vtable") => config.vtable = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("offset") => config.offset = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("size") => config.size = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("index") => config.index = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("slot") => config.slot = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("receiver") => {
                config.receiver = Some(parse_receiver_ident(nv.value)?)
            }
            Meta::NameValue(nv) if nv.path.is_ident("guard") => {
                config.guard_preset = Some(parse_guard_preset(nv.value)?)
            }
            Meta::NameValue(nv) if nv.path.is_ident("invalid") => {
                config.guard_overrides.invalid = Some(parse_guard_policy(nv.value)?)
            }
            Meta::NameValue(nv) if nv.path.is_ident("null") => {
                config.guard_overrides.null = Some(parse_guard_policy(nv.value)?)
            }
            Meta::NameValue(nv) if nv.path.is_ident("unresolved") => {
                config.guard_overrides.unresolved = Some(parse_guard_policy(nv.value)?)
            }
            Meta::NameValue(nv) if nv.path.is_ident("convert_fail") => {
                config.guard_overrides.convert_fail = Some(parse_guard_policy(nv.value)?)
            }
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "unsupported hook attribute argument",
                ));
            }
        }
    }

    Ok(config)
}

fn parse_receiver_ident(expr: Expr) -> syn::Result<Ident> {
    match expr {
        Expr::Path(ExprPath { path, .. }) if path.segments.len() == 1 => {
            Ok(path.segments[0].ident.clone())
        }
        other => Err(syn::Error::new_spanned(
            other,
            "`receiver` must be a single parameter identifier",
        )),
    }
}

fn parse_guard_policy(expr: Expr) -> syn::Result<GuardPolicy> {
    match expr {
        Expr::Path(path) => match classify_guard_preset_path(&path).as_deref() {
            Some("original") => Ok(GuardPolicy::Original),
            Some("skip") => Ok(GuardPolicy::Skip),
            Some("default") => Ok(GuardPolicy::Default),
            _ => Err(syn::Error::new_spanned(
                path,
                "guard policy must be one of: original, skip, default, hooks::guards::<policy>(), or return_(expr)",
            )),
        },
        Expr::Call(ExprCall { func, args, .. }) => match *func {
            Expr::Path(path) if path.path.is_ident("return_") && args.len() == 1 => {
                Ok(GuardPolicy::Return(args.into_iter().next().unwrap()))
            }
            Expr::Path(path) if args.is_empty() => {
                match classify_guard_preset_path(&path).as_deref() {
                    Some("original") => Ok(GuardPolicy::Original),
                    Some("skip") => Ok(GuardPolicy::Skip),
                    Some("default") => Ok(GuardPolicy::Default),
                    _ => Err(syn::Error::new_spanned(
                        path,
                        "guard policy must be one of: original, skip, default, hooks::guards::<policy>(), or return_(expr)",
                    )),
                }
            }
            other => Err(syn::Error::new_spanned(
                other,
                "guard policy must be one of: original, skip, default, hooks::guards::<policy>(), or return_(expr)",
            )),
        },
        other => Err(syn::Error::new_spanned(
            other,
            "guard policy must be one of: original, skip, default, hooks::guards::<policy>(), or return_(expr)",
        )),
    }
}

fn parse_guard_preset(expr: Expr) -> syn::Result<GuardPolicies> {
    let preset_name = match expr {
        Expr::Path(path) => classify_guard_preset_path(&path),
        Expr::Call(ExprCall { func, args, .. }) if args.is_empty() => match *func {
            Expr::Path(path) => classify_guard_preset_path(&path),
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "guard preset must be a path or zero-argument call such as hooks::guards::original()",
                ));
            }
        },
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "guard preset must be a path or zero-argument call such as hooks::guards::original()",
            ));
        }
    };

    match preset_name.as_deref() {
        Some("original") => Ok(GuardPolicies {
            invalid: GuardPolicy::Original,
            null: None,
            unresolved: None,
            convert_fail: None,
        }),
        Some("default") => Ok(GuardPolicies {
            invalid: GuardPolicy::Default,
            null: None,
            unresolved: None,
            convert_fail: None,
        }),
        Some("skip") => Ok(GuardPolicies {
            invalid: GuardPolicy::Skip,
            null: None,
            unresolved: None,
            convert_fail: None,
        }),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "unsupported guard preset; use one of hooks::guards::original(), hooks::guards::default(), or hooks::guards::skip()",
        )),
    }
}

fn classify_guard_preset_path(path: &ExprPath) -> Option<String> {
    if path_matches_known_paths(
        &path.path,
        &[
            &["sdk", "hooks", "guards", "original"],
            &["sdk", "hooks", "original"],
            &["original"],
        ],
    ) {
        Some("original".to_owned())
    } else if path_matches_known_paths(
        &path.path,
        &[
            &["sdk", "hooks", "guards", "default"],
            &["sdk", "hooks", "default"],
            &["default"],
        ],
    ) {
        Some("default".to_owned())
    } else if path_matches_known_paths(
        &path.path,
        &[
            &["sdk", "hooks", "guards", "skip"],
            &["sdk", "hooks", "skip"],
            &["skip"],
        ],
    ) {
        Some("skip".to_owned())
    } else {
        None
    }
}

fn parse_function(function: ItemFn) -> syn::Result<ParsedFn> {
    if function.sig.receiver().is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "sdk hook attributes only support free functions",
        ));
    }

    if !function.sig.generics.params.is_empty() || function.sig.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "sdk hook attributes do not support generic hook functions",
        ));
    }

    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.asyncness,
            "sdk hook attributes do not support async hook functions",
        ));
    }

    if function.sig.constness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.constness,
            "sdk hook attributes do not support const hook functions",
        ));
    }

    if function.sig.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.unsafety,
            "sdk hook attributes expect ordinary safe hook functions; use the low-level hook layer for raw unsafe detours",
        ));
    }

    if function.sig.abi.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.abi,
            "sdk hook attributes generate their own ABI wrapper; extern hook functions are not supported",
        ));
    }

    if function.sig.variadic.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.variadic,
            "sdk hook attributes do not support variadic hook functions",
        ));
    }

    let vis = function.vis.clone();
    let fn_name = function.sig.ident.clone();
    let hook_mod_name = format_ident!("{}_hook", fn_name);
    let hidden_mod_name = format_ident!("__sdk_hook_{}", fn_name);
    let ret_ty = match &function.sig.output {
        ReturnType::Default => parse_quote!(()),
        ReturnType::Type(_, ty) => (**ty).clone(),
    };
    ensure_supported_hook_abi_ty(&ret_ty, "hook return type")?;
    let returns_unit = matches!(function.sig.output, ReturnType::Default)
        || matches!(&ret_ty, Type::Tuple(tuple) if tuple.elems.is_empty());

    let mut original_param_ty = None;
    let mut user_params = Vec::new();

    for (index, arg) in function.sig.inputs.iter().enumerate() {
        let typed = match arg {
            FnArg::Typed(typed) => typed,
            FnArg::Receiver(receiver) => {
                return Err(syn::Error::new_spanned(
                    receiver,
                    "sdk hook attributes only support free functions",
                ));
            }
        };

        let ident = match typed.pat.as_ref() {
            Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "hook function parameters must be simple identifiers",
                ));
            }
        };

        if is_original_type(&typed.ty) {
            if index != 0 {
                return Err(syn::Error::new_spanned(
                    &typed.ty,
                    "`Original<_>` parameter is only supported as the first argument",
                ));
            }
            if original_param_ty.is_some() {
                return Err(syn::Error::new_spanned(
                    &typed.ty,
                    "multiple `Original<_>` parameters are not supported",
                ));
            }
            validate_original_param_type(&typed.ty)?;
            original_param_ty = Some((*typed.ty).clone());
            continue;
        }

        let user_ty = (*typed.ty).clone();
        let abi_ty = map_user_ty_to_abi(&user_ty)?;
        ensure_supported_hook_abi_ty(&abi_ty, "hook parameter")?;
        user_params.push(UserParam {
            ident,
            ty: user_ty,
            abi_ty,
        });
    }

    Ok(ParsedFn {
        function,
        vis,
        fn_name,
        hook_mod_name,
        hidden_mod_name,
        ret_ty,
        returns_unit,
        original_param_ty,
        user_params,
    })
}

fn validate_config(
    kind: HookKind,
    config: &HookConfig,
    guards: &GuardPolicies,
    parsed: &ParsedFn,
) -> syn::Result<()> {
    let has_skip = matches!(guards.invalid, GuardPolicy::Skip)
        || matches!(guards.null, Some(GuardPolicy::Skip))
        || matches!(guards.unresolved, Some(GuardPolicy::Skip))
        || matches!(guards.convert_fail, Some(GuardPolicy::Skip));

    if has_skip && !parsed.returns_unit {
        return Err(syn::Error::new(
            Span::call_site(),
            "`skip` guard policy is only allowed for hooks returning `()`; use `default` or `return_(...)` instead",
        ));
    }

    match kind {
        HookKind::Function => {
            require_exactly_one(&config.target, &config.address, "target", "address")?;
        }
        HookKind::Call => {
            require_exactly_one(&config.target, &config.address, "target", "address")?;
            require_present(&config.offset, "offset")?;
            require_present(&config.size, "size")?;
        }
        HookKind::Vtable => {
            require_present(&config.vtable, "vtable")?;
            require_exactly_one_of_three(
                &config.index,
                &config.offset,
                &config.slot,
                "index",
                "offset",
                "slot",
            )?;
        }
        HookKind::Vcall => {
            require_exactly_one(&config.target, &config.address, "target", "address")?;
            require_present(&config.offset, "offset")?;
            require_present(&config.size, "size")?;
            require_present_ident(&config.receiver, "receiver")?;
            require_exactly_one(&config.index, &config.slot, "index", "slot")?;

            let receiver = config.receiver.as_ref().unwrap();
            let Some(receiver_param) = parsed
                .user_params
                .iter()
                .find(|param| &param.ident == receiver)
            else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!("receiver `{receiver}` must name one of the hook function parameters"),
                ));
            };

            if !is_pointer_abi_type(&receiver_param.abi_ty) {
                return Err(syn::Error::new_spanned(
                    &receiver_param.ty,
                    "vcall receiver must adapt to a pointer-like ABI type",
                ));
            }
        }
    }

    Ok(())
}

fn require_present<T>(value: &Option<T>, name: &str) -> syn::Result<()> {
    if value.is_none() {
        Err(syn::Error::new(
            Span::call_site(),
            format!("missing required `{name}` hook argument"),
        ))
    } else {
        Ok(())
    }
}

fn require_present_ident(value: &Option<Ident>, name: &str) -> syn::Result<()> {
    if value.is_none() {
        Err(syn::Error::new(
            Span::call_site(),
            format!("missing required `{name}` hook argument"),
        ))
    } else {
        Ok(())
    }
}

fn require_exactly_one<T, U>(
    left: &Option<T>,
    right: &Option<U>,
    left_name: &str,
    right_name: &str,
) -> syn::Result<()> {
    match (left.is_some(), right.is_some()) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(syn::Error::new(
            Span::call_site(),
            format!("expected one of `{left_name}` or `{right_name}`"),
        )),
        (true, true) => Err(syn::Error::new(
            Span::call_site(),
            format!("only one of `{left_name}` or `{right_name}` may be specified"),
        )),
    }
}

fn require_exactly_one_of_three<T, U, V>(
    first: &Option<T>,
    second: &Option<U>,
    third: &Option<V>,
    first_name: &str,
    second_name: &str,
    third_name: &str,
) -> syn::Result<()> {
    let count = first.is_some() as u8 + second.is_some() as u8 + third.is_some() as u8;
    if count == 1 {
        Ok(())
    } else if count == 0 {
        Err(syn::Error::new(
            Span::call_site(),
            format!("expected one of `{first_name}`, `{second_name}`, or `{third_name}`"),
        ))
    } else {
        Err(syn::Error::new(
            Span::call_site(),
            format!(
                "only one of `{first_name}`, `{second_name}`, or `{third_name}` may be specified"
            ),
        ))
    }
}

fn is_original_type(ty: &Type) -> bool {
    matches!(peel_type(ty), Type::Path(path) if type_path_matches_known_paths(
        path,
        &[
            &["sdk", "hooks", "Original"],
            &["sdk", "hooks", "runtime", "Original"],
            &["Original"],
        ],
    ))
}

fn validate_original_param_type(ty: &Type) -> syn::Result<()> {
    let Type::Path(path) = peel_type(ty) else {
        return Err(syn::Error::new_spanned(
            ty,
            "`Original<_>` parameter must wrap a plain Rust function-pointer type like `Original<fn(&T)>`",
        ));
    };

    let inner = last_type_arg(path)?;
    let Type::BareFn(bare_fn) = peel_type(&inner) else {
        return Err(syn::Error::new_spanned(
            inner,
            "`Original<_>` parameter must wrap a plain Rust function-pointer type like `Original<fn(&T)>`",
        ));
    };

    if bare_fn.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            &bare_fn.unsafety,
            "`Original<_>` must use a safe `fn(...)` signature, not `unsafe fn(...)`",
        ));
    }

    if bare_fn.abi.is_some() {
        return Err(syn::Error::new_spanned(
            &bare_fn.abi,
            "`Original<_>` must use a plain Rust `fn(...)` signature, not an `extern fn(...)` ABI",
        ));
    }

    if bare_fn.variadic.is_some() {
        return Err(syn::Error::new_spanned(
            &bare_fn.variadic,
            "`Original<_>` does not support variadic function-pointer signatures",
        ));
    }

    Ok(())
}

fn map_user_ty_to_abi(ty: &Type) -> syn::Result<Type> {
    if let Some(path) = find_stable_hook_wrapper_type(ty) {
        return Err(syn::Error::new_spanned(
            path,
            "stable SDK pointer wrappers are not supported as hook parameters; use `&T`, `Option<&T>`, `&mut T`, raw pointers, or `Resolved<T>` instead",
        ));
    }

    match peel_type(ty) {
        Type::Ptr(ptr) => Ok(Type::Ptr(ptr.clone())),
        Type::Reference(reference) => {
            let elem = (*reference.elem).clone();
            Ok(parse_quote!(*mut #elem))
        }
        Type::Path(path) => match () {
            _ if type_path_is_option(path) => {
                let inner = option_inner_type(path)?;
                map_option_inner_to_abi(&inner)
            }
            _ if type_path_is_sdk_resolved(path) => {
                let inner = last_type_arg(path)?;
                Ok(parse_quote!(*mut #inner))
            }
            _ if type_path_is_sdk_resolved_handle(path) => {
                let inner = last_type_arg(path)?;
                Ok(inner)
            }
            _ => Ok(Type::Path(path.clone())),
        },
        other => Err(syn::Error::new_spanned(
            other,
            "unsupported high-level hook argument type",
        )),
    }
}

fn map_option_inner_to_abi(inner: &Type) -> syn::Result<Type> {
    match peel_type(inner) {
        Type::Reference(reference) => {
            let elem = (*reference.elem).clone();
            Ok(parse_quote!(*mut #elem))
        }
        Type::Path(path) if type_path_is_sdk_resolved(path) => {
            let inner = last_type_arg(path)?;
            Ok(parse_quote!(*mut #inner))
        }
        Type::Path(path) if type_path_is_sdk_resolved_handle(path) => last_type_arg(path),
        other => Err(syn::Error::new_spanned(
            other,
            "unsupported `Option<_>` hook argument type; use Option<&T>, Option<&mut T>, Option<Resolved<T>>, or Option<ResolvedHandle<H>>",
        )),
    }
}

fn find_stable_hook_wrapper_type(ty: &Type) -> Option<&TypePath> {
    match peel_type(ty) {
        Type::Reference(reference) => find_stable_hook_wrapper_type(&reference.elem),
        Type::Ptr(ptr) => find_stable_hook_wrapper_type(&ptr.elem),
        Type::Path(path) => {
            if type_path_is_sdk_game_ref(path) || type_path_is_sdk_game_ptr(path) {
                return Some(path);
            }

            path.path
                .segments
                .iter()
                .find_map(|segment| match &segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        args.args.iter().find_map(|arg| match arg {
                            GenericArgument::Type(ty) => find_stable_hook_wrapper_type(ty),
                            _ => None,
                        })
                    }
                    _ => None,
                })
        }
        _ => None,
    }
}

fn is_nontrivial_cpp_value_type(ty: &Type) -> bool {
    matches!(peel_type(ty), Type::Path(path) if type_path_matches_known_paths(
        path,
        &[
            &["re", "ActorHandle"],
            &["re", "bs_pointer_handle", "ActorHandle"],
            &["re", "ObjectRefHandle"],
            &["re", "bs_pointer_handle", "ObjectRefHandle"],
            &["re", "ProjectileHandle"],
            &["re", "bs_pointer_handle", "ProjectileHandle"],
            &["re", "BSFixedString"],
            &["re", "bs_fixed_string", "BSFixedString"],
            &["re", "BSString"],
            &["re", "BSStringT"],
            &["re", "BSStaticStringT"],
            &["re", "NiPointer"],
            &["re", "ni_smart_pointer", "NiPointer"],
            &["re", "BSTSmartPointer"],
            &["re", "bst_smart_pointer", "BSTSmartPointer"],
            &["re", "GPtr"],
            &["re", "g_ptr", "GPtr"],
            &["re", "hkRefPtr"],
            &["re", "hk_ref_ptr", "hkRefPtr"],
            &["re", "BSTArray"],
            &["re", "bst_array", "BSTArray"],
            &["re", "BSScrapArray"],
            &["re", "bs_scrap_array", "BSScrapArray"],
            &["re", "BSTSmallArray"],
            &["re", "bst_small_array", "BSTSmallArray"],
            &["re", "BSStaticArray"],
            &["re", "bs_static_array", "BSStaticArray"],
            &["re", "BSTSmallSharedArray"],
            &["re", "bst_small_shared_array", "BSTSmallSharedArray"],
            &["re", "BSTScatterTable"],
            &["re", "bst_scatter_table", "BSTScatterTable"],
            &["re", "BSTHashMap"],
            &["re", "BSTSet"],
            &["re", "BSTFixedHashMap"],
            &["re", "BSTScrapHashMap"],
            &["re", "BSTStaticHashMap"],
            &["re", "BSTArrayHeapAllocator"],
            &["re", "BSTSmallArrayHeapAllocator"],
            &["re", "BSScrapArrayAllocator"],
            &["re", "BSTScatterTableHeapAllocator"],
            &["re", "BSTScatterTableScrapAllocator"],
            &["re", "BSTStaticHashMapAllocator"],
            &["ActorHandle"],
            &["ObjectRefHandle"],
            &["ProjectileHandle"],
            &["BSFixedString"],
            &["BSString"],
            &["BSStringT"],
            &["BSStaticStringT"],
            &["NiPointer"],
            &["BSTSmartPointer"],
            &["GPtr"],
            &["hkRefPtr"],
            &["BSTArray"],
            &["BSScrapArray"],
            &["BSTSmallArray"],
            &["BSStaticArray"],
            &["BSTSmallSharedArray"],
            &["BSTScatterTable"],
            &["BSTHashMap"],
            &["BSTSet"],
            &["BSTFixedHashMap"],
            &["BSTScrapHashMap"],
            &["BSTStaticHashMap"],
            &["BSTArrayHeapAllocator"],
            &["BSTSmallArrayHeapAllocator"],
            &["BSScrapArrayAllocator"],
            &["BSTScatterTableHeapAllocator"],
            &["BSTScatterTableScrapAllocator"],
            &["BSTStaticHashMapAllocator"],
        ],
    ))
}

fn ensure_supported_hook_abi_ty(ty: &Type, context: &str) -> syn::Result<()> {
    if is_nontrivial_cpp_value_type(ty) {
        return Err(syn::Error::new_spanned(
            ty,
            format!(
                "{context} uses a non-trivial C++ value type by value; SDK hook attributes do not support this ABI. Use the low-level hook layer with an explicit out-param signature, pointer/reference ABI, or a C++ bridge instead"
            ),
        ));
    }

    Ok(())
}

fn peel_type(ty: &Type) -> &Type {
    match ty {
        Type::Group(TypeGroup { elem, .. }) => peel_type(elem),
        Type::Paren(TypeParen { elem, .. }) => peel_type(elem),
        other => other,
    }
}

fn path_matches_known_paths(path: &Path, expected_paths: &[&[&str]]) -> bool {
    let normalized = normalized_path_segments(path);
    expected_paths.iter().any(|expected| {
        normalized_segments_match(&normalized, expected)
            || bare_ident_matches_expected_tail(&normalized, expected)
    })
}

fn type_path_matches_known_paths(path: &TypePath, expected_paths: &[&[&str]]) -> bool {
    path_matches_known_paths(&path.path, expected_paths)
}

fn normalized_path_segments(path: &Path) -> Vec<String> {
    let mut segments: Vec<String> = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();

    while matches!(
        segments.first().map(String::as_str),
        Some("crate" | "self" | "super" | "libskyrim")
    ) {
        segments.remove(0);
    }

    segments
}

fn type_path_is_option(path: &TypePath) -> bool {
    type_path_matches_known_paths(
        path,
        &[
            &["Option"],
            &["core", "option", "Option"],
            &["std", "option", "Option"],
        ],
    )
}

fn type_path_is_sdk_resolved(path: &TypePath) -> bool {
    type_path_matches_known_paths(
        path,
        &[
            &["sdk", "core", "Resolved"],
            &["sdk", "core", "handles", "Resolved"],
            &["sdk", "hooks", "Resolved"],
            &["Resolved"],
        ],
    )
}

fn type_path_is_sdk_resolved_handle(path: &TypePath) -> bool {
    type_path_matches_known_paths(
        path,
        &[
            &["sdk", "core", "ResolvedHandle"],
            &["sdk", "core", "handles", "ResolvedHandle"],
            &["sdk", "hooks", "ResolvedHandle"],
            &["ResolvedHandle"],
        ],
    )
}

fn type_path_is_sdk_game_ref(path: &TypePath) -> bool {
    type_path_matches_known_paths(
        path,
        &[
            &["sdk", "core", "GameRef"],
            &["sdk", "core", "refs", "GameRef"],
            &["sdk", "core", "ptr", "GameRef"],
            &["sdk", "papyrus", "types", "GameRef"],
            &["GameRef"],
        ],
    )
}

fn type_path_is_sdk_game_ptr(path: &TypePath) -> bool {
    type_path_matches_known_paths(
        path,
        &[
            &["sdk", "core", "GamePtr"],
            &["sdk", "core", "refs", "GamePtr"],
            &["sdk", "core", "ptr", "GamePtr"],
            &["sdk", "papyrus", "types", "GamePtr"],
            &["GamePtr"],
        ],
    )
}

fn last_type_arg(path: &TypePath) -> syn::Result<Type> {
    let segment = path
        .path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new_spanned(path, "missing type arguments"))?;
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => args
            .args
            .iter()
            .filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            })
            .last()
            .ok_or_else(|| syn::Error::new_spanned(path, "missing type argument")),
        _ => Err(syn::Error::new_spanned(path, "missing type arguments")),
    }
}

fn option_inner_type(path: &TypePath) -> syn::Result<Type> {
    let segment = path
        .path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new_spanned(path, "missing Option type argument"))?;
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => args
            .args
            .iter()
            .find_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            })
            .ok_or_else(|| syn::Error::new_spanned(path, "missing Option type argument")),
        _ => Err(syn::Error::new_spanned(
            path,
            "missing Option type argument",
        )),
    }
}

fn is_pointer_abi_type(ty: &Type) -> bool {
    matches!(peel_type(ty), Type::Ptr(_))
}

fn raw_signature_type(user_params: &[UserParam], ret_ty: &Type) -> Type {
    let abi_tys: Vec<Type> = user_params
        .iter()
        .map(|param| param.abi_ty.clone())
        .collect();
    parse_quote!(extern "C" fn(#(#abi_tys),*) -> #ret_ty)
}

fn build_original_raw_fn(
    kind: HookKind,
    user_params: &[UserParam],
    raw_arg_idents: &[Ident],
    ret_ty: &Type,
    receiver_index: Option<usize>,
) -> syn::Result<TokenStream2> {
    let abi_tys: Vec<&Type> = user_params.iter().map(|param| &param.abi_ty).collect();
    let call_raw_args = quote! { #(#raw_arg_idents),* };

    let body = match kind {
        HookKind::Function | HookKind::Call | HookKind::Vtable => {
            quote! {
                let original = unsafe { original_relocation().get() };
                original(#call_raw_args)
            }
        }
        HookKind::Vcall => {
            let receiver_raw = raw_arg_idents
                .get(receiver_index.expect("vcall receiver index"))
                .expect("receiver raw arg");
            quote! {
                let original: __SdkRawSignature = unsafe {
                    ::libskyrim::relocation::virtual_function(
                        #receiver_raw as *const _,
                        __sdk_vtable_index(),
                    )
                };
                original(#call_raw_args)
            }
        }
    };

    Ok(quote! {
        #[inline(always)]
        fn __sdk_original_raw(#(#raw_arg_idents: #abi_tys),*) -> #ret_ty {
            #body
        }
    })
}

fn build_original_user_fn(
    parsed: &ParsedFn,
    _raw_arg_idents: &[Ident],
    ret_ty: &Type,
) -> TokenStream2 {
    if parsed.original_param_ty.is_none() {
        return TokenStream2::new();
    }

    let user_arg_idents: Vec<&Ident> = parsed
        .user_params
        .iter()
        .map(|param| &param.ident)
        .collect();
    let user_arg_tys: Vec<&Type> = parsed.user_params.iter().map(|param| &param.ty).collect();
    let abi_tys: Vec<&Type> = parsed
        .user_params
        .iter()
        .map(|param| &param.abi_ty)
        .collect();

    let into_abi_args = user_arg_idents
        .iter()
        .zip(user_arg_tys.iter())
        .zip(abi_tys.iter())
        .map(|((ident, user_ty), abi_ty)| {
            quote! {
                <#user_ty as ::libskyrim::sdk::hooks::HookArg<'_, #abi_ty>>::into_abi(#ident)
            }
        });

    quote! {
        #[inline(always)]
        fn __sdk_original_user(#(#user_arg_idents: #user_arg_tys),*) -> #ret_ty {
            __sdk_original_raw(#(#into_abi_args),*)
        }
    }
}

fn build_conversion_stmts(
    guards: &GuardPolicies,
    user_params: &[UserParam],
    raw_arg_idents: &[Ident],
    ret_ty: &Type,
    returns_unit: bool,
) -> Vec<TokenStream2> {
    user_params
        .iter()
        .zip(raw_arg_idents.iter())
        .map(|(param, raw_ident)| {
            let ident = &param.ident;
            let user_ty = &param.ty;
            let abi_ty = &param.abi_ty;
            let on_failure = build_guard_dispatch(guards, ret_ty, returns_unit, raw_arg_idents);
            quote! {
                let #ident = match <#user_ty as ::libskyrim::sdk::hooks::HookArg<'_, #abi_ty>>::from_abi(#raw_ident) {
                    Ok(value) => value,
                    Err(failure) => #on_failure,
                };
            }
        })
        .collect()
}

fn build_guard_dispatch(
    guards: &GuardPolicies,
    ret_ty: &Type,
    returns_unit: bool,
    raw_arg_idents: &[Ident],
) -> TokenStream2 {
    let null_policy = guards.null.as_ref().unwrap_or(&guards.invalid);
    let unresolved_policy = guards.unresolved.as_ref().unwrap_or(&guards.invalid);
    let convert_fail_policy = guards.convert_fail.as_ref().unwrap_or(&guards.invalid);

    let null_arm = build_policy_tokens(null_policy, ret_ty, returns_unit, raw_arg_idents);
    let unresolved_arm =
        build_policy_tokens(unresolved_policy, ret_ty, returns_unit, raw_arg_idents);
    let convert_arm =
        build_policy_tokens(convert_fail_policy, ret_ty, returns_unit, raw_arg_idents);

    quote! {
        match failure {
            ::libskyrim::sdk::hooks::HookGuardFailure::Null => #null_arm,
            ::libskyrim::sdk::hooks::HookGuardFailure::Unresolved => #unresolved_arm,
            ::libskyrim::sdk::hooks::HookGuardFailure::ConvertFail => #convert_arm,
        }
    }
}

fn build_policy_tokens(
    policy: &GuardPolicy,
    ret_ty: &Type,
    returns_unit: bool,
    raw_arg_idents: &[Ident],
) -> TokenStream2 {
    match policy {
        GuardPolicy::Original => quote! {{
            return __sdk_original_raw(#(#raw_arg_idents),*);
        }},
        GuardPolicy::Skip => {
            debug_assert!(returns_unit);
            quote! {{
                return;
            }}
        }
        GuardPolicy::Default => quote! {{
            return <#ret_ty as ::core::default::Default>::default();
        }},
        GuardPolicy::Return(expr) => quote! {{
            return (#expr);
        }},
    }
}

fn build_install_state(kind: HookKind) -> TokenStream2 {
    match kind {
        HookKind::Vcall => quote! {
            static INSTALLED: ::libskyrim::core_util::Later<()> =
                ::libskyrim::core_util::Later::new();
        },
        _ => quote! {
            static ORIGINAL: ::libskyrim::core_util::Later<
                ::libskyrim::relocation::Relocation<__SdkRawSignature>
            > = ::libskyrim::core_util::Later::new();

            #[inline(always)]
            fn original_relocation() -> ::libskyrim::relocation::Relocation<__SdkRawSignature> {
                *ORIGINAL
            }
        },
    }
}

fn build_install_fns(
    kind: HookKind,
    config: &HookConfig,
    parsed: &ParsedFn,
    _raw_sig_ty: &Type,
    _receiver_index: Option<usize>,
) -> syn::Result<TokenStream2> {
    let fn_name = &parsed.fn_name;
    let installed_check = match kind {
        HookKind::Vcall => quote! { INSTALLED.is_init() },
        _ => quote! { ORIGINAL.is_init() },
    };

    let install_body = match kind {
        HookKind::Function => {
            let target = config
                .target
                .as_ref()
                .or(config.address.as_ref())
                .expect("validated function target must exist");
            quote! {
                let orig_addr = ::libskyrim::relocation::try_write_function_hook_universal(
                    #target,
                    __sdk_detour as *const () as usize,
                )?;
                ORIGINAL.init(::libskyrim::relocation::Relocation::from_address(orig_addr));
            }
        }
        HookKind::Call => {
            let target = config
                .target
                .as_ref()
                .or(config.address.as_ref())
                .expect("validated call target must exist");
            let offset = config
                .offset
                .as_ref()
                .expect("validated call offset must exist");
            let size = config
                .size
                .as_ref()
                .expect("validated call size must exist");
            quote! {
                let target_addr =
                    ::libskyrim::relocation::TryIntoAddress::try_into_address(#target)?
                    + ::libskyrim::relocation::TryIntoOffset::try_into_offset(#offset)?;
                let orig_addr = ::libskyrim::relocation::try_write_call(
                    target_addr,
                    __sdk_detour as *const () as usize,
                    #size,
                )?;
                ORIGINAL.init(::libskyrim::relocation::Relocation::from_address(orig_addr));
            }
        }
        HookKind::Vtable => {
            let vtable = config
                .vtable
                .as_ref()
                .expect("validated vtable target must exist");
            let index_expr = build_install_index_expr(config)?;
            quote! {
                let index = #index_expr;
                let orig_addr = ::libskyrim::relocation::try_write_vfunc(
                    #vtable,
                    index,
                    __sdk_detour as *const () as usize,
                )?;
                ORIGINAL.init(::libskyrim::relocation::Relocation::from_address(orig_addr));
            }
        }
        HookKind::Vcall => {
            let target = config
                .target
                .as_ref()
                .or(config.address.as_ref())
                .expect("validated vcall target must exist");
            let offset = config
                .offset
                .as_ref()
                .expect("validated vcall offset must exist");
            let size = config
                .size
                .as_ref()
                .expect("validated vcall size must exist");
            let index_expr = build_install_index_expr(config)?;
            quote! {
                let target_addr =
                    ::libskyrim::relocation::TryIntoAddress::try_into_address(#target)?
                    + ::libskyrim::relocation::TryIntoOffset::try_into_offset(#offset)?;
                let index = #index_expr;
                let _ = ::libskyrim::relocation::try_write_call(
                    target_addr,
                    __sdk_detour as *const () as usize,
                    #size,
                )?;
                VTABLE_INDEX.init(index);
                INSTALLED.init(());
            }
        }
    };

    Ok(quote! {
        #[inline(always)]
        pub fn is_installed() -> bool {
            #installed_check
        }

        pub fn try_install() -> Result<(), ::libskyrim::sdk::hooks::HookInstallError> {
            if is_installed() {
                return Err(::libskyrim::sdk::hooks::HookInstallError::AlreadyInstalled(
                    stringify!(#fn_name),
                ));
            }

            #install_body
            Ok(())
        }

        #[inline(always)]
        pub fn install() -> Result<(), ::libskyrim::sdk::hooks::HookInstallError> {
            try_install()
        }

        pub const INSTALLER: ::libskyrim::sdk::hooks::HookInstaller =
            ::libskyrim::sdk::hooks::HookInstaller::new(
                stringify!(#fn_name),
                try_install as ::libskyrim::sdk::hooks::HookInstallFn,
            );

        #[inline(always)]
        pub const fn installer() -> ::libskyrim::sdk::hooks::HookInstaller {
            INSTALLER
        }

        #[inline(always)]
        pub fn install_or_fatal() {
            if let Err(error) = try_install() {
                error.install_or_fatal(stringify!(#fn_name));
            }
        }
    })
}

fn build_install_index_expr(config: &HookConfig) -> syn::Result<TokenStream2> {
    if let Some(index) = &config.index {
        Ok(quote! {
            ::libskyrim::relocation::TryIntoOffset::try_into_offset(#index)?
        })
    } else if let Some(offset) = &config.offset {
        Ok(quote! {
            ::libskyrim::relocation::TryIntoOffset::try_into_offset(#offset)?
        })
    } else if let Some(slot) = &config.slot {
        Ok(quote! {
            ::libskyrim::relocation::try_vtable_index_from_slot(#slot)?
        })
    } else {
        Err(syn::Error::new(
            Span::call_site(),
            "missing vtable index/offset/slot specification",
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EventDomain {
    Game,
    Ui,
    Dispatcher,
    Input,
    Message,
    Bus,
}

#[derive(Default)]
struct EventConfig {
    event_ty: Option<Type>,
    bus: Option<Expr>,
    kind: Option<Expr>,
    plugin_phase: Option<Expr>,
    game_phase: Option<Expr>,
    phase: Option<Expr>,
    sender: Option<LitStr>,
    priority: Option<Expr>,
    prepend: bool,
}

struct ParsedEventFn {
    function: ItemFn,
    vis: Visibility,
    fn_name: Ident,
    event_mod_name: Ident,
    hidden_mod_name: Ident,
    param_kind: EventParamKind,
    bus_event_ty: Option<Type>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EventParamKind {
    None,
    Ref,
    OptionRef,
    InputEvents,
    MessageRawRef,
    MessageRef,
    BusRef,
    BusMutRef,
}

fn expand_event(domain: EventDomain, attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ts = proc_macro2::TokenStream::from(attr);
    let item_fn = parse_macro_input!(item as ItemFn);

    match expand_event_impl(domain, attr_ts, item_fn) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn expand_event_impl(
    domain: EventDomain,
    attr: TokenStream2,
    function: ItemFn,
) -> syn::Result<TokenStream2> {
    let config = parse_event_config(domain, attr)?;
    let parsed = parse_event_function(domain, &config, function)?;

    let fn_name = &parsed.fn_name;
    let vis = &parsed.vis;
    let event_mod_name = &parsed.event_mod_name;
    let hidden_mod_name = &parsed.hidden_mod_name;
    let function_item = &parsed.function;
    let callback_fn = build_event_callback(domain, &parsed, &config)?;
    let install_body = build_event_install_body(domain, &parsed, &config)?;

    Ok(quote! {
        #function_item

        #[doc(hidden)]
        mod #hidden_mod_name {
            #[allow(unused_imports)]
            use super::*;

            #callback_fn

            pub fn try_install<'a>(
                batch: &mut ::libskyrim::sdk::events::EventBatch<'a>,
            ) -> Result<(), ::libskyrim::sdk::events::EventBatchError> {
                #install_body
                Ok(())
            }

            #[inline(always)]
            pub fn install<'a>(
                batch: &mut ::libskyrim::sdk::events::EventBatch<'a>,
            ) -> Result<(), ::libskyrim::sdk::events::EventBatchError> {
                try_install(batch)
            }

            pub const INSTALLER: ::libskyrim::sdk::events::EventInstaller =
                ::libskyrim::sdk::events::EventInstaller::new(
                    stringify!(#fn_name),
                    try_install as ::libskyrim::sdk::events::EventInstallFn,
                );

            #[inline(always)]
            pub const fn installer() -> ::libskyrim::sdk::events::EventInstaller {
                INSTALLER
            }

            #[inline(always)]
            pub fn install_or_fatal<'a>(
                batch: &mut ::libskyrim::sdk::events::EventBatch<'a>,
            ) {
                if let Err(error) = try_install(batch) {
                    error.install_or_fatal();
                }
            }
        }

        #vis use #hidden_mod_name as #event_mod_name;
    })
}

fn parse_event_config(domain: EventDomain, attr: TokenStream2) -> syn::Result<EventConfig> {
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let metas = parser.parse2(attr)?;
    let mut config = EventConfig::default();

    for meta in metas {
        match meta {
            Meta::NameValue(nv) if nv.path.is_ident("event") => {
                config.event_ty = Some(parse_event_type(nv.value)?)
            }
            Meta::NameValue(nv) if nv.path.is_ident("bus") => config.bus = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("kind") => config.kind = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("plugin_phase") => {
                config.plugin_phase = Some(nv.value)
            }
            Meta::NameValue(nv) if nv.path.is_ident("game_phase") => {
                config.game_phase = Some(nv.value)
            }
            Meta::NameValue(nv) if nv.path.is_ident("phase") => config.phase = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("priority") => config.priority = Some(nv.value),
            Meta::NameValue(nv) if nv.path.is_ident("sender") => {
                config.sender = Some(parse_sender_lit(nv.value)?)
            }
            Meta::Path(path) if path.is_ident("first") => assign_event_priority(
                &mut config,
                parse_quote!(::libskyrim::sdk::events::SubscriberPriority::FIRST),
                &path,
            )?,
            Meta::Path(path) if path.is_ident("early") => assign_event_priority(
                &mut config,
                parse_quote!(::libskyrim::sdk::events::SubscriberPriority::EARLY),
                &path,
            )?,
            Meta::Path(path) if path.is_ident("late") => assign_event_priority(
                &mut config,
                parse_quote!(::libskyrim::sdk::events::SubscriberPriority::LATE),
                &path,
            )?,
            Meta::Path(path) if path.is_ident("last") => assign_event_priority(
                &mut config,
                parse_quote!(::libskyrim::sdk::events::SubscriberPriority::LAST),
                &path,
            )?,
            Meta::Path(path) if path.is_ident("prepend") => config.prepend = true,
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "unsupported event attribute argument",
                ));
            }
        }
    }

    match domain {
        EventDomain::Game | EventDomain::Ui | EventDomain::Dispatcher => {
            if config.event_ty.is_none() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "missing required `event = TypePath` argument",
                ));
            }
            if config.kind.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`kind = ...` is only valid for `message_event`",
                ));
            }
            if config.bus.is_some() || config.priority.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`bus = ...` and `priority = ...` are only valid for `bus_event`",
                ));
            }
            if config.plugin_phase.is_some()
                || config.game_phase.is_some()
                || config.phase.is_some()
                || config.sender.is_some()
            {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`plugin_phase`, `game_phase`, `phase`, and `sender` are only valid for `message_event`",
                ));
            }
        }
        EventDomain::Input => {
            if config.event_ty.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`input_event` does not accept `event = ...`",
                ));
            }
            if config.kind.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`kind = ...` is only valid for `message_event`",
                ));
            }
            if config.bus.is_some() || config.priority.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`bus = ...` and `priority = ...` are only valid for `bus_event`",
                ));
            }
            if config.plugin_phase.is_some()
                || config.game_phase.is_some()
                || config.phase.is_some()
                || config.sender.is_some()
            {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`plugin_phase`, `game_phase`, `phase`, and `sender` are only valid for `message_event`",
                ));
            }
        }
        EventDomain::Message => {
            let mode_count = config.kind.is_some() as u8
                + config.plugin_phase.is_some() as u8
                + config.game_phase.is_some() as u8
                + config.phase.is_some() as u8;

            if mode_count == 0 {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "missing required `kind = ...`, `plugin_phase = ...`, `game_phase = ...`, or `phase = ...` argument",
                ));
            }
            if mode_count > 1 {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`message_event` accepts exactly one of `kind`, `plugin_phase`, `game_phase`, or `phase`",
                ));
            }
            if config.event_ty.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`message_event` does not accept `event = ...`",
                ));
            }
            if config.bus.is_some() || config.priority.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`bus = ...` and `priority = ...` are only valid for `bus_event`",
                ));
            }
            if config.prepend {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`prepend` is not supported for `message_event`",
                ));
            }
        }
        EventDomain::Bus => {
            if config.bus.is_none() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "missing required `bus = ...` argument",
                ));
            }
            if config.event_ty.is_some() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`bus_event` infers its payload type from the callback parameter and does not accept `event = ...`",
                ));
            }
            if config.kind.is_some()
                || config.plugin_phase.is_some()
                || config.game_phase.is_some()
                || config.phase.is_some()
                || config.sender.is_some()
            {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`kind`, `plugin_phase`, `game_phase`, `phase`, and `sender` are only valid for `message_event`",
                ));
            }
            if config.prepend {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "`prepend` is not supported for `bus_event`; use `priority = ...` or `first/early/late/last` instead",
                ));
            }
        }
    }

    Ok(config)
}

fn assign_event_priority(config: &mut EventConfig, priority: Expr, path: &Path) -> syn::Result<()> {
    if config.priority.is_some() {
        return Err(syn::Error::new_spanned(
            path,
            "event priority was already specified",
        ));
    }
    config.priority = Some(priority);
    Ok(())
}

fn parse_sender_lit(expr: Expr) -> syn::Result<LitStr> {
    match expr {
        Expr::Lit(expr_lit) => match expr_lit.lit {
            syn::Lit::Str(value) => {
                if value.value().is_empty() {
                    return Err(syn::Error::new_spanned(
                        value,
                        "`sender = ...` must not be empty",
                    ));
                }
                Ok(value)
            }
            other => Err(syn::Error::new_spanned(
                other,
                "`sender = ...` must be a string literal",
            )),
        },
        other => Err(syn::Error::new_spanned(
            other,
            "`sender = ...` must be a string literal",
        )),
    }
}

fn parse_event_type(expr: Expr) -> syn::Result<Type> {
    match expr {
        Expr::Path(path) => Ok(Type::Path(TypePath {
            qself: path.qself,
            path: path.path,
        })),
        other => Err(syn::Error::new_spanned(
            other,
            "`event = ...` must be a type path",
        )),
    }
}

fn parse_event_function(
    domain: EventDomain,
    config: &EventConfig,
    function: ItemFn,
) -> syn::Result<ParsedEventFn> {
    if function.sig.receiver().is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "sdk event attributes only support free functions",
        ));
    }

    if !function.sig.generics.params.is_empty() || function.sig.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "sdk event attributes do not support generic functions",
        ));
    }

    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.asyncness,
            "sdk event attributes do not support async functions",
        ));
    }

    if function.sig.constness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.constness,
            "sdk event attributes do not support const functions",
        ));
    }

    if function.sig.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.unsafety,
            "sdk event attributes expect ordinary safe functions",
        ));
    }

    if function.sig.abi.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.abi,
            "sdk event attributes generate their own wrapper; extern functions are not supported",
        ));
    }

    if function.sig.variadic.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.variadic,
            "sdk event attributes do not support variadic functions",
        ));
    }

    let vis = function.vis.clone();
    let fn_name = function.sig.ident.clone();
    let event_mod_name = format_ident!("{}_event", fn_name);
    let hidden_mod_name = format_ident!("__sdk_event_{}", fn_name);

    if domain == EventDomain::Bus && function.sig.inputs.len() != 1 {
        return Err(syn::Error::new_spanned(
            &function.sig.inputs,
            "`bus_event` requires exactly one callback parameter of type `&Payload` or `&mut Payload`",
        ));
    }

    let param_kind = match function.sig.inputs.len() {
        0 => EventParamKind::None,
        1 => {
            let arg = match function.sig.inputs.first().unwrap() {
                FnArg::Typed(typed) => typed,
                FnArg::Receiver(receiver) => {
                    return Err(syn::Error::new_spanned(
                        receiver,
                        "sdk event attributes only support free functions",
                    ));
                }
            };

            match domain {
                EventDomain::Game | EventDomain::Ui | EventDomain::Dispatcher => {
                    parse_event_callback_param(&arg.ty, config.event_ty.as_ref().unwrap())?
                }
                EventDomain::Input => parse_input_callback_param(&arg.ty)?,
                EventDomain::Message => parse_message_callback_param(&arg.ty)?,
                EventDomain::Bus => parse_bus_callback_param(&arg.ty)?.0,
            }
        }
        _ => {
            return Err(syn::Error::new_spanned(
                &function.sig.inputs,
                "sdk event attributes support at most one callback parameter",
            ));
        }
    };

    let bus_event_ty = match domain {
        EventDomain::Bus => {
            let arg = match function.sig.inputs.first().unwrap() {
                FnArg::Typed(typed) => typed,
                FnArg::Receiver(_) => unreachable!(),
            };
            Some(parse_bus_callback_param(&arg.ty)?.1)
        }
        _ => None,
    };

    Ok(ParsedEventFn {
        function,
        vis,
        fn_name,
        event_mod_name,
        hidden_mod_name,
        param_kind,
        bus_event_ty,
    })
}

fn parse_event_callback_param(ty: &Type, event_ty: &Type) -> syn::Result<EventParamKind> {
    if is_option_ref_to_type(ty, event_ty) {
        Ok(EventParamKind::OptionRef)
    } else if is_ref_to_type(ty, event_ty) {
        Ok(EventParamKind::Ref)
    } else {
        Err(syn::Error::new_spanned(
            ty,
            "event callback parameter must be `&Event`, `Option<&Event>`, or omitted",
        ))
    }
}

fn parse_input_callback_param(ty: &Type) -> syn::Result<EventParamKind> {
    if type_matches_known_path(
        ty,
        &[
            &["sdk", "events", "InputEvents"],
            &["sdk", "events", "input", "InputEvents"],
        ],
    ) {
        Ok(EventParamKind::InputEvents)
    } else {
        Err(syn::Error::new_spanned(
            ty,
            "input event callback parameter must be `InputEvents<'_>`",
        ))
    }
}

fn parse_message_callback_param(ty: &Type) -> syn::Result<EventParamKind> {
    if type_matches_known_path(ty, &[&["sdk", "events", "skse", "messages", "MessageRef"]]) {
        Ok(EventParamKind::MessageRef)
    } else if is_ref_to_known_path(ty, &[&["skse", "Message"]]) {
        Ok(EventParamKind::MessageRawRef)
    } else {
        Err(syn::Error::new_spanned(
            ty,
            "message event callback parameter must be `MessageRef<'_>` or `&Message`",
        ))
    }
}

fn parse_bus_callback_param(ty: &Type) -> syn::Result<(EventParamKind, Type)> {
    let Type::Reference(reference) = peel_type_groups(ty) else {
        return Err(syn::Error::new_spanned(
            ty,
            "bus event callback parameter must be `&Payload` or `&mut Payload`",
        ));
    };

    let payload_ty = (*reference.elem).clone();
    let kind = if reference.mutability.is_some() {
        EventParamKind::BusMutRef
    } else {
        EventParamKind::BusRef
    };
    Ok((kind, payload_ty))
}

fn build_event_callback(
    domain: EventDomain,
    parsed: &ParsedEventFn,
    config: &EventConfig,
) -> syn::Result<TokenStream2> {
    let fn_name = &parsed.fn_name;

    match domain {
        EventDomain::Game | EventDomain::Ui | EventDomain::Dispatcher => {
            let event_ty = config.event_ty.as_ref().unwrap();
            let body = match parsed.param_kind {
                EventParamKind::None => quote! {
                    let _ = event;
                    ::libskyrim::sdk::events::IntoEventFlow::into_event_flow(super::#fn_name())
                },
                EventParamKind::OptionRef => quote! {
                    ::libskyrim::sdk::events::IntoEventFlow::into_event_flow(super::#fn_name(event))
                },
                EventParamKind::Ref => quote! {
                    let Some(event) = event else {
                        return ::libskyrim::sdk::events::EventFlow::Continue;
                    };
                    ::libskyrim::sdk::events::IntoEventFlow::into_event_flow(super::#fn_name(event))
                },
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "invalid internal event callback shape",
                    ));
                }
            };

            Ok(quote! {
                #[inline(always)]
                fn __sdk_callback(event: Option<&#event_ty>) -> ::libskyrim::sdk::events::EventFlow {
                    #body
                }
            })
        }
        EventDomain::Input => {
            let body = match parsed.param_kind {
                EventParamKind::None => quote! {
                    let _ = event;
                    ::libskyrim::sdk::events::IntoEventFlow::into_event_flow(super::#fn_name())
                },
                EventParamKind::InputEvents => quote! {
                    ::libskyrim::sdk::events::IntoEventFlow::into_event_flow(super::#fn_name(event))
                },
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "invalid internal input callback shape",
                    ));
                }
            };

            Ok(quote! {
                #[inline(always)]
                fn __sdk_callback(
                    event: ::libskyrim::sdk::events::InputEvents<'_>,
                ) -> ::libskyrim::sdk::events::EventFlow {
                    #body
                }
            })
        }
        EventDomain::Message => {
            let body = match parsed.param_kind {
                EventParamKind::None => quote! {
                    let _ = message;
                    let _ = super::#fn_name();
                },
                EventParamKind::MessageRawRef => quote! {
                    let _ = super::#fn_name(message.raw());
                },
                EventParamKind::MessageRef => quote! {
                    let _ = super::#fn_name(message);
                },
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "invalid internal message callback shape",
                    ));
                }
            };

            Ok(quote! {
                #[inline(always)]
                fn __sdk_callback(
                    message: ::libskyrim::sdk::events::skse::messages::MessageRef<'_>,
                ) {
                    #body
                }
            })
        }
        EventDomain::Bus => {
            let payload_ty = parsed.bus_event_ty.as_ref().ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    "invalid internal bus callback shape: missing payload type",
                )
            })?;
            let body = match parsed.param_kind {
                EventParamKind::BusRef | EventParamKind::BusMutRef => quote! {
                    ::libskyrim::sdk::events::IntoEventFlow::into_event_flow(super::#fn_name(event))
                },
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "invalid internal bus callback shape",
                    ));
                }
            };

            Ok(quote! {
                #[inline(always)]
                fn __sdk_callback(
                    event: &mut #payload_ty,
                ) -> ::libskyrim::sdk::events::EventFlow {
                    #body
                }
            })
        }
    }
}

fn build_event_install_body(
    domain: EventDomain,
    parsed: &ParsedEventFn,
    config: &EventConfig,
) -> syn::Result<TokenStream2> {
    let fn_name = &parsed.fn_name;
    match domain {
        EventDomain::Game => {
            let event_ty = config.event_ty.as_ref().unwrap();
            if config.prepend {
                Ok(quote! {
                    batch.prepend_game::<#event_ty, _, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            } else {
                Ok(quote! {
                    batch.game::<#event_ty, _, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            }
        }
        EventDomain::Ui => {
            let event_ty = config.event_ty.as_ref().unwrap();
            if config.prepend {
                Ok(quote! {
                    batch.prepend_ui::<#event_ty, _, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            } else {
                Ok(quote! {
                    batch.ui::<#event_ty, _, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            }
        }
        EventDomain::Dispatcher => {
            let event_ty = config.event_ty.as_ref().unwrap();
            if config.prepend {
                Ok(quote! {
                    batch.prepend_dispatcher::<#event_ty, _, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            } else {
                Ok(quote! {
                    batch.dispatcher::<#event_ty, _, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            }
        }
        EventDomain::Input => {
            if config.prepend {
                Ok(quote! {
                    batch.prepend_input::<_, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            } else {
                Ok(quote! {
                    batch.input::<_, _>(stringify!(#fn_name), __sdk_callback)?;
                })
            }
        }
        EventDomain::Message => {
            match (
                config.kind.as_ref(),
                config.plugin_phase.as_ref(),
                config.game_phase.as_ref(),
                config.phase.as_ref(),
                config.sender.as_ref(),
            ) {
                (Some(kind), None, None, None, None) => Ok(quote! {
                    batch.message(stringify!(#fn_name), #kind, __sdk_callback);
                }),
                (Some(kind), None, None, None, Some(sender)) => Ok(quote! {
                    batch.message_sender_str(stringify!(#fn_name), #kind, #sender, __sdk_callback);
                }),
                (None, Some(phase), None, None, None) => Ok(quote! {
                    batch.message_plugin_phase(stringify!(#fn_name), #phase, __sdk_callback);
                }),
                (None, Some(phase), None, None, Some(sender)) => Ok(quote! {
                    batch.message_plugin_phase_sender_str(
                        stringify!(#fn_name),
                        #phase,
                        #sender,
                        __sdk_callback,
                    );
                }),
                (None, None, Some(phase), None, None) => Ok(quote! {
                    batch.message_game_lifecycle(stringify!(#fn_name), #phase, __sdk_callback);
                }),
                (None, None, Some(phase), None, Some(sender)) => Ok(quote! {
                    batch.message_game_lifecycle_sender_str(
                        stringify!(#fn_name),
                        #phase,
                        #sender,
                        __sdk_callback,
                    );
                }),
                (None, None, None, Some(phase), None) => Ok(quote! {
                    batch.message_lifecycle(stringify!(#fn_name), #phase, __sdk_callback);
                }),
                (None, None, None, Some(phase), Some(sender)) => Ok(quote! {
                    batch.message_lifecycle_sender_str(
                        stringify!(#fn_name),
                        #phase,
                        #sender,
                        __sdk_callback,
                    );
                }),
                _ => Err(syn::Error::new(
                    Span::call_site(),
                    "invalid internal message event configuration",
                )),
            }
        }
        EventDomain::Bus => {
            let bus = config.bus.as_ref().unwrap();
            if let Some(priority) = config.priority.as_ref() {
                Ok(quote! {
                    batch.bus_with_priority(stringify!(#fn_name), #bus, #priority, __sdk_callback)?;
                })
            } else {
                Ok(quote! {
                    batch.bus(stringify!(#fn_name), #bus, __sdk_callback)?;
                })
            }
        }
    }
}

fn is_ref_to_type(ty: &Type, target: &Type) -> bool {
    match ty {
        Type::Reference(reference) => types_equal(reference.elem.as_ref(), target),
        Type::Group(TypeGroup { elem, .. }) | Type::Paren(TypeParen { elem, .. }) => {
            is_ref_to_type(elem.as_ref(), target)
        }
        _ => false,
    }
}

fn is_option_ref_to_type(ty: &Type, target: &Type) -> bool {
    let Type::Path(path) = peel_type_groups(ty) else {
        return false;
    };
    if !type_path_is_option(path) {
        return false;
    }
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };
    let Some(GenericArgument::Type(inner_ty)) = arguments.args.first() else {
        return false;
    };
    is_ref_to_type(inner_ty, target)
}

fn is_ref_to_known_path(ty: &Type, expected_paths: &[&[&str]]) -> bool {
    let Type::Reference(reference) = peel_type_groups(ty) else {
        return false;
    };
    type_matches_known_path(reference.elem.as_ref(), expected_paths)
}

fn type_matches_known_path(ty: &Type, expected_paths: &[&[&str]]) -> bool {
    let Type::Path(path) = peel_type_groups(ty) else {
        return false;
    };

    let normalized = normalized_path_segments(&path.path);
    expected_paths.iter().any(|expected| {
        normalized_segments_match(&normalized, expected)
            || bare_ident_matches_expected_tail(&normalized, expected)
    })
}

fn normalized_segments_match(normalized: &[String], expected: &[&str]) -> bool {
    normalized.len() == expected.len()
        && normalized
            .iter()
            .map(String::as_str)
            .zip(expected.iter().copied())
            .all(|(left, right)| left == right)
}

fn bare_ident_matches_expected_tail(normalized: &[String], expected: &[&str]) -> bool {
    match (normalized, expected.last().copied()) {
        ([ident], Some(expected_tail)) => ident == expected_tail,
        _ => false,
    }
}

fn bare_or_normalized_type_paths_equal(left: &TypePath, right: &TypePath) -> bool {
    let left_segments = normalized_path_segments(&left.path);
    let right_segments = normalized_path_segments(&right.path);

    if left_segments == right_segments {
        return true;
    }

    match (left_segments.as_slice(), right_segments.as_slice()) {
        ([left_ident], right) => right
            .last()
            .is_some_and(|right_ident| left_ident == right_ident),
        (left, [right_ident]) => left
            .last()
            .is_some_and(|left_ident| left_ident == right_ident),
        _ => false,
    }
}

fn peel_type_groups(ty: &Type) -> &Type {
    match ty {
        Type::Group(TypeGroup { elem, .. }) | Type::Paren(TypeParen { elem, .. }) => {
            peel_type_groups(elem.as_ref())
        }
        other => other,
    }
}

fn types_equal(left: &Type, right: &Type) -> bool {
    let left = peel_type_groups(left);
    let right = peel_type_groups(right);

    if quote!(#left).to_string() == quote!(#right).to_string() {
        return true;
    }

    match (left, right) {
        (Type::Path(left_path), Type::Path(right_path)) => {
            bare_or_normalized_type_paths_equal(left_path, right_path)
        }
        _ => false,
    }
}

fn expand_cosave_derive(input: DeriveInput) -> syn::Result<TokenStream2> {
    #[derive(Default)]
    struct FieldAttrs {
        skip: bool,
        default: bool,
        with: Option<Path>,
    }

    fn parse_field_attrs(field: &syn::Field) -> syn::Result<FieldAttrs> {
        let mut attrs = FieldAttrs::default();

        for attr in &field.attrs {
            if !attr.path().is_ident("cosave") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    if attrs.skip {
                        return Err(meta.error("duplicate `skip` in `#[cosave(...)]`"));
                    }
                    attrs.skip = true;
                    return Ok(());
                }

                if meta.path.is_ident("default") {
                    if attrs.default {
                        return Err(meta.error("duplicate `default` in `#[cosave(...)]`"));
                    }
                    attrs.default = true;
                    return Ok(());
                }

                if meta.path.is_ident("with") {
                    if attrs.with.is_some() {
                        return Err(meta.error("duplicate `with = ...` in `#[cosave(...)]`"));
                    }

                    let value = meta.value()?;
                    attrs.with = Some(value.parse()?);
                    return Ok(());
                }

                Err(meta.error(
                    "unsupported `#[cosave(...)]` option; expected `skip`, `default`, or `with = path`",
                ))
            })?;
        }

        if attrs.skip && attrs.default {
            return Err(syn::Error::new_spanned(
                field,
                "`#[cosave(skip)]` cannot be combined with `#[cosave(default)]`",
            ));
        }

        if attrs.skip && attrs.with.is_some() {
            return Err(syn::Error::new_spanned(
                field,
                "`#[cosave(skip)]` cannot be combined with `#[cosave(with = ...)]`",
            ));
        }

        Ok(attrs)
    }

    for param in &input.generics.params {
        if matches!(param, GenericParam::Lifetime(_)) {
            return Err(syn::Error::new_spanned(
                param,
                "`#[derive(Cosave)]` does not currently support lifetime parameters",
            ));
        }
    }

    let name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => data.fields,
        Data::Enum(data) => {
            return Err(syn::Error::new_spanned(
                data.enum_token,
                "`#[derive(Cosave)]` currently supports structs only",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new_spanned(
                data.union_token,
                "`#[derive(Cosave)]` does not support unions",
            ));
        }
    };

    let field_attrs = fields
        .iter()
        .map(parse_field_attrs)
        .collect::<syn::Result<Vec<_>>>()?;

    let mut generics = input.generics;
    {
        let where_clause = generics.make_where_clause();
        for (field, attrs) in fields.iter().zip(field_attrs.iter()) {
            let ty = &field.ty;
            if attrs.skip {
                where_clause
                    .predicates
                    .push(parse_quote!(#ty: ::core::default::Default));
                continue;
            }

            if attrs.with.is_none() {
                where_clause.predicates.push(parse_quote!(
                    #ty:
                        ::libskyrim::sdk::plugin::serialization::CosaveEncode
                        + ::libskyrim::sdk::plugin::serialization::CosaveDecode
                ));
            }

            if attrs.default {
                where_clause
                    .predicates
                    .push(parse_quote!(#ty: ::core::default::Default));
            }
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode_fields = fields.iter().enumerate().zip(field_attrs.iter()).map(
        |((index, field), attrs)| {
            let (access, label) = match &field.ident {
                Some(ident) => (
                    quote!(self.#ident),
                    LitStr::new(&ident.to_string(), ident.span()),
                ),
                None => {
                    let tuple_index = syn::Index::from(index);
                    (
                        quote!(self.#tuple_index),
                        LitStr::new(&index.to_string(), Span::call_site()),
                    )
                }
            };

            if attrs.skip {
                return quote! {};
            }

            if let Some(with) = &attrs.with {
                quote! {
                    #with::encode(&#access, writer).map_err(|source| {
                        ::libskyrim::sdk::plugin::serialization::SaveError::field(#label, source)
                    })?;
                }
            } else {
                quote! {
                    ::libskyrim::sdk::plugin::serialization::CosaveEncode::encode(
                        &#access,
                        writer,
                    )
                    .map_err(|source| {
                        ::libskyrim::sdk::plugin::serialization::SaveError::field(#label, source)
                    })?;
                }
            }
        },
    );

    let decode_expr = match &fields {
        Fields::Named(named) => {
            let field_inits = named
                .named
                .iter()
                .zip(field_attrs.iter())
                .map(|(field, attrs)| {
                let ident = field.ident.as_ref().expect("named field must have ident");
                let ty = &field.ty;
                let label = LitStr::new(&ident.to_string(), ident.span());

                let decode_value = if attrs.skip {
                    quote!(::core::default::Default::default())
                } else {
                    let decode_core = if let Some(with) = &attrs.with {
                        quote!(#with::decode(reader))
                    } else {
                        quote!(<#ty as ::libskyrim::sdk::plugin::serialization::CosaveDecode>::decode(reader))
                    };

                    if attrs.default {
                        quote! {
                            if reader.is_empty() {
                                ::core::default::Default::default()
                            } else {
                                #decode_core.map_err(|source| {
                                    ::libskyrim::sdk::plugin::serialization::LoadError::field(#label, source)
                                })?
                            }
                        }
                    } else {
                        quote! {
                            #decode_core.map_err(|source| {
                                ::libskyrim::sdk::plugin::serialization::LoadError::field(#label, source)
                            })?
                        }
                    }
                };

                quote! { #ident: #decode_value }
            });
            quote! {
                Self {
                    #(#field_inits),*
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            let field_values = unnamed
                .unnamed
                .iter()
                .zip(field_attrs.iter())
                .enumerate()
                .map(|(index, (field, attrs))| {
                let ty = &field.ty;
                let label = LitStr::new(&index.to_string(), Span::call_site());

                if attrs.skip {
                    return quote!(::core::default::Default::default());
                }

                let decode_core = if let Some(with) = &attrs.with {
                    quote!(#with::decode(reader))
                } else {
                    quote!(<#ty as ::libskyrim::sdk::plugin::serialization::CosaveDecode>::decode(reader))
                };

                if attrs.default {
                    quote! {
                        if reader.is_empty() {
                            ::core::default::Default::default()
                        } else {
                            #decode_core.map_err(|source| {
                                ::libskyrim::sdk::plugin::serialization::LoadError::field(#label, source)
                            })?
                        }
                    }
                } else {
                    quote! {
                        #decode_core.map_err(|source| {
                            ::libskyrim::sdk::plugin::serialization::LoadError::field(#label, source)
                        })?
                    }
                }
            });
            quote! {
                Self(
                    #(#field_values),*
                )
            }
        }
        Fields::Unit => quote!(Self),
    };

    Ok(quote! {
        impl #impl_generics ::libskyrim::sdk::plugin::serialization::CosaveEncode
            for #name #ty_generics
        #where_clause
        {
            fn encode(
                &self,
                writer: &mut ::libskyrim::sdk::plugin::serialization::RecordWriter,
            ) -> Result<(), ::libskyrim::sdk::plugin::serialization::SaveError> {
                #(#encode_fields)*
                Ok(())
            }
        }

        impl #impl_generics ::libskyrim::sdk::plugin::serialization::CosaveDecode
            for #name #ty_generics
        #where_clause
        {
            fn decode(
                reader: &mut ::libskyrim::sdk::plugin::serialization::RecordReader<'_>,
            ) -> Result<Self, ::libskyrim::sdk::plugin::serialization::LoadError> {
                Ok(#decode_expr)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use syn::parse_quote;

    fn ty_tokens(ty: &Type) -> String {
        ty.to_token_stream().to_string()
    }

    #[test]
    fn map_shared_ref_hook_arg_to_raw_pointer() {
        let ty: Type = parse_quote!(&Actor);
        let abi = map_user_ty_to_abi(&ty).expect("&Actor should stay supported");
        assert_eq!(ty_tokens(&abi), "* mut Actor");
    }

    #[test]
    fn reject_ref_to_game_ref_hook_arg() {
        let ty: Type = parse_quote!(&GameRef<PlayerCharacter>);
        let err = map_user_ty_to_abi(&ty).expect_err("&GameRef<_> must be rejected");
        assert!(
            err.to_string()
                .contains("stable SDK pointer wrappers are not supported as hook parameters")
        );
    }

    #[test]
    fn reject_option_ref_to_game_ptr_hook_arg() {
        let ty: Type = parse_quote!(Option<&GamePtr<Actor>>);
        let err = map_user_ty_to_abi(&ty).expect_err("Option<&GamePtr<_>> must be rejected");
        assert!(
            err.to_string()
                .contains("stable SDK pointer wrappers are not supported as hook parameters")
        );
    }

    #[test]
    fn reject_raw_pointer_to_game_ref_hook_arg() {
        let ty: Type = parse_quote!(*mut GameRef<Actor>);
        let err = map_user_ty_to_abi(&ty).expect_err("*mut GameRef<_> must be rejected");
        assert!(
            err.to_string()
                .contains("stable SDK pointer wrappers are not supported as hook parameters")
        );
    }

    #[test]
    fn reject_bs_fixed_string_hook_param() {
        let ty: Type = parse_quote!(BSFixedString);
        let err = ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect_err("BSFixedString by value must be rejected");
        assert!(err.to_string().contains("non-trivial C++ value type"));
    }

    #[test]
    fn reject_ni_pointer_hook_param() {
        let ty: Type = parse_quote!(NiPointer<NiNode>);
        let err = ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect_err("NiPointer by value must be rejected");
        assert!(err.to_string().contains("non-trivial C++ value type"));
    }

    #[test]
    fn reject_bst_smart_pointer_hook_param() {
        let ty: Type = parse_quote!(BSTSmartPointer<Object>);
        let err = ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect_err("BSTSmartPointer by value must be rejected");
        assert!(err.to_string().contains("non-trivial C++ value type"));
    }

    #[test]
    fn reject_gptr_hook_param() {
        let ty: Type = parse_quote!(GPtr<IMenu>);
        let err =
            ensure_supported_hook_abi_ty(&ty, "hook parameter").expect_err("GPtr must be rejected");
        assert!(err.to_string().contains("non-trivial C++ value type"));
    }

    #[test]
    fn reject_hk_ref_ptr_hook_param() {
        let ty: Type = parse_quote!(hkRefPtr<hkReferencedObject>);
        let err = ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect_err("hkRefPtr must be rejected");
        assert!(err.to_string().contains("non-trivial C++ value type"));
    }

    #[test]
    fn reject_namespaced_sdk_nontrivial_hook_param() {
        let ty: Type = parse_quote!(crate::re::g_ptr::GPtr<IMenu>);
        let err = ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect_err("SDK GPtr path must be rejected");
        assert!(err.to_string().contains("non-trivial C++ value type"));
    }

    #[test]
    fn do_not_reject_foreign_terminal_ident_nontrivial_hook_param() {
        let ty: Type = parse_quote!(other_crate::GPtr<IMenu>);
        ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect("foreign terminal-ident match must not be rejected");
    }

    #[test]
    fn do_not_reject_foreign_terminal_ident_bs_fixed_string_hook_param() {
        let ty: Type = parse_quote!(other_crate::BSFixedString);
        ensure_supported_hook_abi_ty(&ty, "hook parameter")
            .expect("foreign BSFixedString terminal-ident match must not be rejected");
    }

    #[test]
    fn types_equal_accepts_bare_imported_ident_against_sdk_path() {
        let left: Type = parse_quote!(TESHitEvent);
        let right: Type = parse_quote!(crate::re::TESHitEvent);
        assert!(types_equal(&left, &right));
    }

    #[test]
    fn types_equal_accepts_normalized_sdk_paths() {
        let left: Type = parse_quote!(libskyrim::re::TESHitEvent);
        let right: Type = parse_quote!(crate::re::TESHitEvent);
        assert!(types_equal(&left, &right));
    }

    #[test]
    fn types_equal_rejects_only_terminal_ident_match() {
        let left: Type = parse_quote!(other_crate::TESHitEvent);
        let right: Type = parse_quote!(crate::re::TESHitEvent);
        assert!(!types_equal(&left, &right));
    }

    #[test]
    fn parse_event_callback_param_rejects_foreign_terminal_ident_match() {
        let callback_ty: Type = parse_quote!(&other_crate::TESHitEvent);
        let event_ty: Type = parse_quote!(crate::re::TESHitEvent);
        let err = match parse_event_callback_param(&callback_ty, &event_ty) {
            Ok(_) => panic!("foreign terminal-ident match must be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains(
                "event callback parameter must be `&Event`, `Option<&Event>`, or omitted"
            )
        );
    }

    #[test]
    fn parse_event_callback_param_rejects_foreign_option_path_match() {
        let callback_ty: Type = parse_quote!(other_crate::Option<&crate::re::TESHitEvent>);
        let event_ty: Type = parse_quote!(crate::re::TESHitEvent);
        let err = match parse_event_callback_param(&callback_ty, &event_ty) {
            Ok(_) => panic!("foreign Option path must be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains(
                "event callback parameter must be `&Event`, `Option<&Event>`, or omitted"
            )
        );
    }

    #[test]
    fn parse_input_callback_param_accepts_sdk_paths() {
        let bare: Type = parse_quote!(InputEvents<'_>);
        let namespaced: Type = parse_quote!(crate::sdk::events::input::InputEvents<'_>);
        assert!(matches!(
            parse_input_callback_param(&bare),
            Ok(EventParamKind::InputEvents)
        ));
        assert!(matches!(
            parse_input_callback_param(&namespaced),
            Ok(EventParamKind::InputEvents)
        ));
    }

    #[test]
    fn parse_input_callback_param_rejects_foreign_terminal_ident_match() {
        let foreign: Type = parse_quote!(other_crate::InputEvents<'_>);
        let err = match parse_input_callback_param(&foreign) {
            Ok(_) => panic!("foreign InputEvents must be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string()
                .contains("input event callback parameter must be `InputEvents<'_>`")
        );
    }

    #[test]
    fn parse_message_callback_param_accepts_sdk_paths() {
        let bare_ref: Type = parse_quote!(MessageRef<'_>);
        let namespaced_ref: Type = parse_quote!(crate::sdk::events::skse::messages::MessageRef<'_>);
        let bare_raw: Type = parse_quote!(&Message);
        let namespaced_raw: Type = parse_quote!(&crate::skse::Message);

        assert!(matches!(
            parse_message_callback_param(&bare_ref),
            Ok(EventParamKind::MessageRef)
        ));
        assert!(matches!(
            parse_message_callback_param(&namespaced_ref),
            Ok(EventParamKind::MessageRef)
        ));
        assert!(matches!(
            parse_message_callback_param(&bare_raw),
            Ok(EventParamKind::MessageRawRef)
        ));
        assert!(matches!(
            parse_message_callback_param(&namespaced_raw),
            Ok(EventParamKind::MessageRawRef)
        ));
    }

    #[test]
    fn parse_message_callback_param_rejects_foreign_terminal_ident_matches() {
        let foreign_ref: Type = parse_quote!(other_crate::MessageRef<'_>);
        let foreign_raw: Type = parse_quote!(&other_crate::Message);

        let err = match parse_message_callback_param(&foreign_ref) {
            Ok(_) => panic!("foreign MessageRef must be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains(
                "message event callback parameter must be `MessageRef<'_>` or `&Message`"
            )
        );

        let err = match parse_message_callback_param(&foreign_raw) {
            Ok(_) => panic!("foreign Message must be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains(
                "message event callback parameter must be `MessageRef<'_>` or `&Message`"
            )
        );
    }

    #[test]
    fn foreign_original_type_is_not_treated_as_sdk_original() {
        let ty: Type = parse_quote!(other_crate::Original<fn(&Actor)>);
        assert!(!is_original_type(&ty));
    }

    #[test]
    fn validate_original_param_type_accepts_plain_fn_pointer() {
        let ty: Type = parse_quote!(Original<fn(&Actor, u32) -> bool>);
        validate_original_param_type(&ty).expect("plain fn pointer should be accepted");
    }

    #[test]
    fn validate_original_param_type_rejects_unsafe_fn_pointer() {
        let ty: Type = parse_quote!(Original<unsafe fn(&Actor) -> bool>);
        let err = validate_original_param_type(&ty).expect_err("unsafe fn must be rejected");
        assert!(
            err.to_string()
                .contains("must use a safe `fn(...)` signature")
        );
    }

    #[test]
    fn validate_original_param_type_rejects_extern_fn_pointer() {
        let ty: Type = parse_quote!(Original<extern "C" fn(&Actor) -> bool>);
        let err = validate_original_param_type(&ty).expect_err("extern fn must be rejected");
        assert!(
            err.to_string()
                .contains("must use a plain Rust `fn(...)` signature")
        );
    }

    #[test]
    fn validate_original_param_type_rejects_non_function_inner_type() {
        let ty: Type = parse_quote!(Original<u32>);
        let err = validate_original_param_type(&ty)
            .expect_err("non-function inner type must be rejected");
        assert!(
            err.to_string()
                .contains("must wrap a plain Rust function-pointer type")
        );
    }

    #[test]
    fn foreign_resolved_type_is_not_special_cased() {
        let ty: Type = parse_quote!(other_crate::Resolved<Actor>);
        let abi = map_user_ty_to_abi(&ty).expect("foreign Resolved should not be special-cased");
        assert_eq!(ty_tokens(&abi), "other_crate :: Resolved < Actor >");
    }

    #[test]
    fn foreign_game_ref_is_not_rejected_as_sdk_wrapper() {
        let ty: Type = parse_quote!(&other_crate::GameRef<Actor>);
        let abi = map_user_ty_to_abi(&ty).expect("foreign GameRef should not be rejected");
        assert_eq!(ty_tokens(&abi), "* mut other_crate :: GameRef < Actor >");
    }

    #[test]
    fn parse_guard_preset_rejects_foreign_original_call() {
        let expr: Expr = parse_quote!(other_crate::original());
        let err = match parse_guard_preset(expr) {
            Ok(_) => panic!("foreign original() preset must be rejected"),
            Err(err) => err,
        };
        assert!(err.to_string().contains("unsupported guard preset"));
    }

    #[test]
    fn parse_guard_policy_accepts_namespaced_original_call() {
        let expr: Expr = parse_quote!(crate::sdk::hooks::guards::original());
        match parse_guard_policy(expr) {
            Ok(GuardPolicy::Original) => {}
            Ok(_) => panic!("expected original guard policy"),
            Err(err) => panic!("expected namespaced guard policy to be accepted: {err}"),
        }
    }

    #[test]
    fn parse_guard_policy_accepts_namespaced_default_path() {
        let expr: Expr = parse_quote!(crate::sdk::hooks::guards::default);
        match parse_guard_policy(expr) {
            Ok(GuardPolicy::Default) => {}
            Ok(_) => panic!("expected default guard policy"),
            Err(err) => panic!("expected namespaced guard path to be accepted: {err}"),
        }
    }

    #[test]
    fn parse_guard_policy_rejects_foreign_namespaced_original_call() {
        let expr: Expr = parse_quote!(other_crate::original());
        let err = match parse_guard_policy(expr) {
            Ok(_) => panic!("foreign original() guard policy must be rejected"),
            Err(err) => err,
        };
        assert!(err.to_string().contains("guard policy must be one of"));
    }
}
