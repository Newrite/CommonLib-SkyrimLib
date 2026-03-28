use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{
    Data, DeriveInput, Expr, ExprCall, ExprPath, Fields, FnArg, GenericArgument, GenericParam,
    Ident, ItemFn, LitStr, Meta, Pat, PatIdent, Path, PathArguments, ReturnType, Token, Type,
    TypeGroup, TypeParen, TypePath, Visibility, parse_macro_input, parse_quote,
};

#[proc_macro_attribute]
pub fn function_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Function, attr, item)
}

#[proc_macro_attribute]
pub fn call_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Call, attr, item)
}

#[proc_macro_attribute]
pub fn vtable_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Vtable, attr, item)
}

#[proc_macro_attribute]
pub fn vcall_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_hook(HookKind::Vcall, attr, item)
}

#[proc_macro_attribute]
pub fn game_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Game, attr, item)
}

#[proc_macro_attribute]
pub fn ui_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Ui, attr, item)
}

#[proc_macro_attribute]
pub fn dispatcher_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Dispatcher, attr, item)
}

#[proc_macro_attribute]
pub fn input_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Input, attr, item)
}

#[proc_macro_attribute]
pub fn message_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Message, attr, item)
}

#[proc_macro_attribute]
pub fn bus_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_event(EventDomain::Bus, attr, item)
}

#[proc_macro_derive(Cosave, attributes(cosave))]
pub fn derive_cosave(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_cosave_derive(input) {
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
        Expr::Path(path) if path.path.is_ident("original") => Ok(GuardPolicy::Original),
        Expr::Path(path) if path.path.is_ident("skip") => Ok(GuardPolicy::Skip),
        Expr::Path(path) if path.path.is_ident("default") => Ok(GuardPolicy::Default),
        Expr::Call(ExprCall { func, args, .. }) => match *func {
            Expr::Path(path) if path.path.is_ident("return_") && args.len() == 1 => {
                Ok(GuardPolicy::Return(args.into_iter().next().unwrap()))
            }
            other => Err(syn::Error::new_spanned(
                other,
                "expected `return_(expr)` for explicit guard return value",
            )),
        },
        other => Err(syn::Error::new_spanned(
            other,
            "guard policy must be one of: original, skip, default, return_(expr)",
        )),
    }
}

fn parse_guard_preset(expr: Expr) -> syn::Result<GuardPolicies> {
    let preset_name = match expr {
        Expr::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        Expr::Call(ExprCall { func, args, .. }) if args.is_empty() => match *func {
            Expr::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
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
            original_param_ty = Some((*typed.ty).clone());
            continue;
        }

        let user_ty = (*typed.ty).clone();
        let abi_ty = map_user_ty_to_abi(&user_ty)?;
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
    matches!(peel_type(ty), Type::Path(path) if path_last_ident(path).is_some_and(|ident| ident == "Original"))
}

fn map_user_ty_to_abi(ty: &Type) -> syn::Result<Type> {
    match peel_type(ty) {
        Type::Ptr(ptr) => Ok(Type::Ptr(ptr.clone())),
        Type::Reference(reference) => {
            let elem = (*reference.elem).clone();
            Ok(parse_quote!(*mut #elem))
        }
        Type::Path(path) => {
            let Some(last_ident) = path_last_ident(path) else {
                return Err(syn::Error::new_spanned(
                    path,
                    "unsupported hook argument type",
                ));
            };

            match () {
                _ if last_ident == "Option" => {
                    let inner = option_inner_type(path)?;
                    map_option_inner_to_abi(&inner)
                }
                _ if last_ident == "GameRef"
                    || last_ident == "GameRefMut"
                    || last_ident == "Resolved" =>
                {
                    let inner = last_type_arg(path)?;
                    Ok(parse_quote!(*mut #inner))
                }
                _ if last_ident == "ResolvedHandle" => {
                    let inner = last_type_arg(path)?;
                    Ok(inner)
                }
                _ => Ok(Type::Path(path.clone())),
            }
        }
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
        Type::Path(path) if path_last_ident(path).is_some_and(|ident| ident == "Resolved") => {
            let inner = last_type_arg(path)?;
            Ok(parse_quote!(*mut #inner))
        }
        Type::Path(path)
            if path_last_ident(path).is_some_and(|ident| ident == "ResolvedHandle") =>
        {
            last_type_arg(path)
        }
        other => Err(syn::Error::new_spanned(
            other,
            "unsupported `Option<_>` hook argument type; use Option<&T>, Option<&mut T>, Option<Resolved<T>>, or Option<ResolvedHandle<H>>",
        )),
    }
}

fn peel_type(ty: &Type) -> &Type {
    match ty {
        Type::Group(TypeGroup { elem, .. }) => peel_type(elem),
        Type::Paren(TypeParen { elem, .. }) => peel_type(elem),
        other => other,
    }
}

fn path_last_ident(path: &TypePath) -> Option<&Ident> {
    path.path.segments.last().map(|segment| &segment.ident)
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
    GameRef,
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
    } else if is_game_ref_to_type(ty, event_ty) {
        Ok(EventParamKind::GameRef)
    } else {
        Err(syn::Error::new_spanned(
            ty,
            "event callback parameter must be `&Event`, `Option<&Event>`, or `GameRef<'_, Event>`",
        ))
    }
}

fn parse_input_callback_param(ty: &Type) -> syn::Result<EventParamKind> {
    if path_last_ident_is(ty, "InputEvents") {
        Ok(EventParamKind::InputEvents)
    } else {
        Err(syn::Error::new_spanned(
            ty,
            "input event callback parameter must be `InputEvents<'_>`",
        ))
    }
}

fn parse_message_callback_param(ty: &Type) -> syn::Result<EventParamKind> {
    if path_last_ident_is(ty, "MessageRef") {
        Ok(EventParamKind::MessageRef)
    } else if is_ref_to_terminal_ident(ty, "Message") {
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
                EventParamKind::GameRef => quote! {
                    let event = ::libskyrim::sdk::core::GameRef::from(event);
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
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    if segment.ident != "Option" {
        return false;
    }
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };
    let Some(GenericArgument::Type(inner_ty)) = arguments.args.first() else {
        return false;
    };
    is_ref_to_type(inner_ty, target)
}

fn is_game_ref_to_type(ty: &Type, target: &Type) -> bool {
    let Type::Path(path) = peel_type_groups(ty) else {
        return false;
    };
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    if segment.ident != "GameRef" {
        return false;
    }
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };

    arguments
        .args
        .iter()
        .any(|arg| matches!(arg, GenericArgument::Type(inner_ty) if types_equal(inner_ty, target)))
}

fn is_ref_to_terminal_ident(ty: &Type, ident: &str) -> bool {
    let Type::Reference(reference) = peel_type_groups(ty) else {
        return false;
    };
    path_last_ident_is(reference.elem.as_ref(), ident)
}

fn path_last_ident_is(ty: &Type, ident: &str) -> bool {
    let Type::Path(path) = peel_type_groups(ty) else {
        return false;
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident == ident)
        .unwrap_or(false)
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
        (Type::Path(left_path), Type::Path(right_path)) => left_path
            .path
            .segments
            .last()
            .zip(right_path.path.segments.last())
            .map(|(left_segment, right_segment)| left_segment.ident == right_segment.ident)
            .unwrap_or(false),
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
