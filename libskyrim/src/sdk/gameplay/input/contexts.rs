use alloc::vec::Vec;

use crate::re::{ControlMap, INPUT_CONTEXT_ID};
use crate::sdk::core::snapshot_contiguous_copied_named;

use super::access::control_map;
use super::shared::{validate_context, validate_context_query};
use super::types::ContextGuard;

#[inline(always)]
pub fn push_context(context: INPUT_CONTEXT_ID) {
    if !validate_context("push_context", context) {
        return;
    }

    ControlMap::push_input_context(context);
}

#[inline(always)]
pub fn pop_context(context: INPUT_CONTEXT_ID) {
    if !validate_context("pop_context", context) {
        return;
    }

    ControlMap::pop_input_context(context);
}

#[inline(always)]
pub fn push_context_scoped(context: INPUT_CONTEXT_ID) -> ContextGuard {
    if !validate_context("push_context_scoped", context) {
        return ContextGuard::new(context, false);
    }

    push_context(context);
    ContextGuard::new(context, true)
}

#[inline(always)]
pub fn context_stack() -> Vec<INPUT_CONTEXT_ID> {
    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    snapshot_contiguous_copied_named(
        &runtime_data.context_priority_stack,
        "sdk::gameplay::input::context_stack()",
        Default::default(),
    )
}

#[inline(always)]
pub fn context_stack_depth() -> usize {
    control_map().runtime_data().context_priority_stack.len() as usize
}

#[inline(always)]
pub fn top_context() -> Option<INPUT_CONTEXT_ID> {
    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    snapshot_contiguous_copied_named(
        &runtime_data.context_priority_stack,
        "sdk::gameplay::input::top_context()",
        Default::default(),
    )
    .last()
    .copied()
}

#[inline(always)]
pub fn has_context(context: INPUT_CONTEXT_ID) -> bool {
    if !validate_context_query("has_context", context) {
        return false;
    }

    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    snapshot_contiguous_copied_named(
        &runtime_data.context_priority_stack,
        "sdk::gameplay::input::has_context()",
        Default::default(),
    )
    .contains(&context)
}

#[inline(always)]
pub fn top_context_is(context: INPUT_CONTEXT_ID) -> bool {
    top_context().is_some_and(|current| current == context)
}

#[inline(always)]
pub fn is_gameplay_context(context: INPUT_CONTEXT_ID) -> bool {
    context == INPUT_CONTEXT_ID::kGameplay
}

#[inline(always)]
pub fn is_menu_like_context(context: INPUT_CONTEXT_ID) -> bool {
    context == INPUT_CONTEXT_ID::kMenuMode
        || context == INPUT_CONTEXT_ID::kConsole
        || context == INPUT_CONTEXT_ID::kItemMenu
        || context == INPUT_CONTEXT_ID::kInventory
        || context == INPUT_CONTEXT_ID::kDebugText
        || context == INPUT_CONTEXT_ID::kFavorites
        || context == INPUT_CONTEXT_ID::kMap
        || context == INPUT_CONTEXT_ID::kStats
        || context == INPUT_CONTEXT_ID::kCursor
        || context == INPUT_CONTEXT_ID::kBook
        || context == INPUT_CONTEXT_ID::kDebugOverlay
        || context == INPUT_CONTEXT_ID::kJournal
        || context == INPUT_CONTEXT_ID::kMapDebug
        || context == INPUT_CONTEXT_ID::kLockpicking
        || INPUT_CONTEXT_ID::kMarketplace().is_some_and(|marketplace| context == marketplace)
}

#[inline(always)]
pub fn has_menu_like_context() -> bool {
    context_stack().into_iter().any(is_menu_like_context)
}

#[inline(always)]
pub fn top_context_is_gameplay() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kGameplay)
}

#[inline(always)]
pub fn top_context_is_cursor() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kCursor)
}

#[inline(always)]
pub fn top_context_is_console() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kConsole)
}

#[inline(always)]
pub fn top_context_is_item_menu() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kItemMenu)
}

#[inline(always)]
pub fn top_context_is_menu_like() -> bool {
    top_context().is_some_and(is_menu_like_context)
}
