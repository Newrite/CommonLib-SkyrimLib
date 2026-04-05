//! User-facing HUD notification helpers.
//!
//! This is a thin SDK layer over the engine's `HUDData` + `HUD Menu`
//! UI-message path. It intentionally stays close to the source-backed contract:
//! callers build a `HudNotificationRequest`, and the SDK turns that into the
//! same `HUDData` payload the engine uses for subtitles, quest toasts, mode
//! pushes, and other HUD-visible messages.

use crate::re::{
    HUD_MESSAGE_TYPE, HUDData, HUDMenu, MARKER_TYPE, ObjectRefHandle, TESQuest, TESWordOfPower,
    UIMessageType,
};
use crate::rex::EnumSet;
use crate::sdk::core::GamePtr;
use crate::sdk::ui::menus;

pub type HudMessageType = HUD_MESSAGE_TYPE;

/// Default `UIMessageQueue` message kind used for HUD notification updates.
///
/// Most callers should leave notification requests on this value. Override it
/// through [`HudNotificationRequest::with_ui_message_type`] only when a plugin
/// intentionally needs a non-default HUD queue path.
pub const DEFAULT_HUD_UI_MESSAGE_TYPE: UIMessageType = UIMessageType::Update;

/// Source-backed payload for HUD-visible toasts and subtitle-style updates.
///
/// This is the SDK-side builder for the engine's `HUDData` payload. It is most
/// useful when a plugin needs something slightly richer than the convenience
/// `queue_*` helpers below, such as:
///
/// - attaching a quest pointer to quest/objective toasts;
/// - attaching a word-of-power pointer to word-learning messages;
/// - routing the request through a different `UIMessageType`;
/// - toggling the `show` flag for mode push/pop flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HudNotificationRequest<'a> {
    /// Localized or preformatted text copied into `HUDData::text`.
    pub text: &'a str,
    /// Engine HUD message discriminator written into `HUDData::type_`.
    pub message_type: HudMessageType,
    /// `UIMessageQueue` delivery kind used to submit the `HUDData` payload.
    pub ui_message_type: UIMessageType,
    /// Mode-toggle boolean and general-purpose HUD visibility flag.
    pub show: bool,
    /// Optional crosshair reference for messages that want a focused target.
    pub crosshair_ref: ObjectRefHandle,
    /// Optional quest pointer for quest/objective HUD toasts.
    pub quest: GamePtr<TESQuest>,
    /// Optional word-of-power pointer for shout-learning HUD toasts.
    pub word_of_power: GamePtr<TESWordOfPower>,
    /// Optional discovery marker for location discovery notifications.
    pub discovery: Option<MARKER_TYPE>,
}

impl<'a> HudNotificationRequest<'a> {
    /// Creates a raw HUD request with the given message discriminator.
    ///
    /// Prefer the named constructors such as [`Self::notification`] or
    /// [`Self::subtitle`] when one already matches the desired HUD flow.
    #[inline(always)]
    pub const fn new(text: &'a str, message_type: HudMessageType) -> Self {
        Self {
            text,
            message_type,
            ui_message_type: DEFAULT_HUD_UI_MESSAGE_TYPE,
            show: true,
            crosshair_ref: ObjectRefHandle::new(),
            quest: GamePtr::null(),
            word_of_power: GamePtr::null(),
            discovery: None,
        }
    }

    /// Creates a generic top-left HUD notification toast.
    #[inline(always)]
    pub const fn notification(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kNotification)
    }

    /// Creates a subtitle request that shows dialogue-style HUD text.
    #[inline(always)]
    pub const fn subtitle(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kShowSubtitle)
    }

    /// Creates a subtitle-hide request.
    ///
    /// This mirrors the engine path where `kHideSubtitle` is paired with a
    /// cleared `show` flag.
    #[inline(always)]
    pub const fn hide_subtitle() -> Self {
        Self::new("", HudMessageType::kHideSubtitle).with_show(false)
    }

    /// Creates a hint-text request.
    #[inline(always)]
    pub const fn hint(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kShowHintText)
    }

    /// Creates a location-name request.
    #[inline(always)]
    pub const fn location(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kShowLocationName)
    }

    /// Creates a location discovery request with an explicit discovery marker.
    #[inline(always)]
    pub const fn location_discovery(text: &'a str, discovery: MARKER_TYPE) -> Self {
        Self::new(text, HudMessageType::kLocationDiscovery).with_discovery(discovery)
    }

    /// Creates a skill-increase toast.
    #[inline(always)]
    pub const fn skill_increase(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kSkillIncrease)
    }

    /// Creates a dragon-soul-absorbed toast.
    #[inline(always)]
    pub const fn dragon_soul_absorbed(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kDragonSoulAbsorbed)
    }

    /// Creates a HUD mode push/pop request.
    ///
    /// Use [`push_hud_mode`] and [`pop_hud_mode`] for the common wrapper path.
    #[inline(always)]
    pub const fn hud_mode(mode: &'a str, push: bool) -> Self {
        Self::new(mode, HudMessageType::kSetMode).with_show(push)
    }

    /// Overrides the `UIMessageQueue` delivery kind used for this request.
    #[inline(always)]
    pub const fn with_ui_message_type(mut self, ui_message_type: UIMessageType) -> Self {
        self.ui_message_type = ui_message_type;
        self
    }

    /// Overrides the engine `show` flag written into `HUDData`.
    #[inline(always)]
    pub const fn with_show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }

    /// Attaches a crosshair reference handle to the request payload.
    #[inline(always)]
    pub const fn with_crosshair_ref(mut self, crosshair_ref: ObjectRefHandle) -> Self {
        self.crosshair_ref = crosshair_ref;
        self
    }

    /// Attaches a quest pointer to the request payload.
    #[inline(always)]
    pub fn with_quest(mut self, quest: impl Into<GamePtr<TESQuest>>) -> Self {
        self.quest = quest.into();
        self
    }

    /// Attaches a word-of-power pointer to the request payload.
    #[inline(always)]
    pub fn with_word_of_power(mut self, word_of_power: impl Into<GamePtr<TESWordOfPower>>) -> Self {
        self.word_of_power = word_of_power.into();
        self
    }

    /// Attaches a discovery marker to the request payload.
    #[inline(always)]
    pub const fn with_discovery(mut self, discovery: MARKER_TYPE) -> Self {
        self.discovery = Some(discovery);
        self
    }

    /// Clears any attached discovery marker.
    #[inline(always)]
    pub const fn without_discovery(mut self) -> Self {
        self.discovery = None;
        self
    }
}

#[inline(always)]
fn discovery_storage(discovery: Option<MARKER_TYPE>) -> EnumSet<MARKER_TYPE, u32> {
    match discovery {
        Some(discovery) => EnumSet::from_underlying(discovery as u32),
        None => EnumSet::from_underlying(0),
    }
}

#[inline(always)]
fn write_request_to_hud_data(data: &mut HUDData, request: HudNotificationRequest<'_>) {
    data.set_type(request.message_type);
    data.text = crate::re::BSString::from_str(request.text);
    data.crosshair_ref = request.crosshair_ref;
    data.quest = request.quest.as_ptr();
    data.word_of_power = request.word_of_power.as_ptr();
    data.show = request.show;
    data.discovery = discovery_storage(request.discovery);
}

/// Queues a prebuilt HUD request through the `HUD Menu` update path.
///
/// This is the central low-level entrypoint for this module. Most plugins will
/// prefer the more specific `queue_*` helpers unless they need to customize the
/// request payload directly.
pub fn queue_hud_request(request: HudNotificationRequest<'_>) -> bool {
    let queued =
        menus::queue_named_message_with::<HUDMenu, HUDData>(request.ui_message_type, |data| {
            write_request_to_hud_data(data, request);
        });

    if !queued {
        crate::defensive_sdk_warn!(
            "sdk::ui::notifications::queue_hud_request() failed for {:?}",
            request.message_type
        );
    }

    queued
}

/// Queues a generic HUD notification toast.
#[inline(always)]
pub fn queue_notification(text: &str) -> bool {
    queue_hud_request(HudNotificationRequest::notification(text))
}

/// Queues a subtitle update.
#[inline(always)]
pub fn queue_subtitle(text: &str) -> bool {
    queue_hud_request(HudNotificationRequest::subtitle(text))
}

/// Queues a subtitle-hide request.
#[inline(always)]
pub fn hide_subtitle() -> bool {
    queue_hud_request(HudNotificationRequest::hide_subtitle())
}

/// Queues a hint-text HUD request.
#[inline(always)]
pub fn queue_hint(text: &str) -> bool {
    queue_hud_request(HudNotificationRequest::hint(text))
}

/// Queues a location-name HUD request.
#[inline(always)]
pub fn queue_location(text: &str) -> bool {
    queue_hud_request(HudNotificationRequest::location(text))
}

/// Queues a location discovery toast with the given discovery marker.
#[inline(always)]
pub fn queue_location_discovery(text: &str, discovery: MARKER_TYPE) -> bool {
    queue_hud_request(HudNotificationRequest::location_discovery(text, discovery))
}

/// Queues a quest-started toast for the given quest.
#[inline(always)]
pub fn queue_quest_started(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kQuestStarted).with_quest(quest),
    )
}

/// Queues a quest-completed toast for the given quest.
#[inline(always)]
pub fn queue_quest_completed(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kQuestComplete).with_quest(quest),
    )
}

/// Queues a quest-failed toast for the given quest.
#[inline(always)]
pub fn queue_quest_failed(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kQuestFailed).with_quest(quest),
    )
}

/// Queues an objective-started toast for the given quest.
#[inline(always)]
pub fn queue_objective_started(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kObjectiveStarted).with_quest(quest),
    )
}

/// Queues an objective-completed toast for the given quest.
#[inline(always)]
pub fn queue_objective_completed(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kObjectiveComplete).with_quest(quest),
    )
}

/// Queues an objective-failed toast for the given quest.
#[inline(always)]
pub fn queue_objective_failed(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kObjectiveFailed).with_quest(quest),
    )
}

/// Queues a skill-increase toast.
#[inline(always)]
pub fn queue_skill_increase(text: &str) -> bool {
    queue_hud_request(HudNotificationRequest::skill_increase(text))
}

/// Queues a word-of-power learned toast for the given word.
#[inline(always)]
pub fn queue_word_of_power_learned(
    text: &str,
    word_of_power: impl Into<GamePtr<TESWordOfPower>>,
) -> bool {
    queue_hud_request(
        HudNotificationRequest::new(text, HudMessageType::kWordOfPowerLearned)
            .with_word_of_power(word_of_power),
    )
}

/// Queues a dragon-soul absorbed toast.
#[inline(always)]
pub fn queue_dragon_soul_absorbed(text: &str) -> bool {
    queue_hud_request(HudNotificationRequest::dragon_soul_absorbed(text))
}

/// Queues a HUD mode push/pop request.
///
/// Pass `true` to push the mode and `false` to pop it. The convenience wrappers
/// [`push_hud_mode`] and [`pop_hud_mode`] usually read better at call sites.
#[inline(always)]
pub fn queue_hud_mode(mode: &str, push: bool) -> bool {
    queue_hud_request(HudNotificationRequest::hud_mode(mode, push))
}

/// Pushes a named HUD mode.
#[inline(always)]
pub fn push_hud_mode(mode: &str) -> bool {
    queue_hud_mode(mode, true)
}

/// Pops a named HUD mode.
#[inline(always)]
pub fn pop_hud_mode(mode: &str) -> bool {
    queue_hud_mode(mode, false)
}

#[cfg(test)]
mod tests {
    use super::{
        HudMessageType, HudNotificationRequest, discovery_storage, write_request_to_hud_data,
    };
    use crate::re::{
        BSString, HUDData, IUIMessageData, MARKER_TYPE, ObjectRefHandle, TESQuest, TESWordOfPower,
        UIMessageType,
    };
    use crate::rex::EnumSet;
    use crate::sdk::core::GamePtr;

    fn empty_hud_data() -> HUDData {
        HUDData {
            base: IUIMessageData {
                vtable: core::ptr::null(),
                unk08: 0,
                pad0a: 0,
                pad0c: 0,
            },
            type_: EnumSet::from_underlying(0),
            pad14: 0,
            text: BSString::new(),
            crosshair_ref: ObjectRefHandle::new(),
            pad2c: 0,
            quest: core::ptr::null_mut(),
            word_of_power: core::ptr::null_mut(),
            show: false,
            pad41: 0,
            pad42: 0,
            discovery: EnumSet::from_underlying(0),
        }
    }

    #[test]
    fn notification_request_defaults_to_hud_update_message() {
        let request = HudNotificationRequest::notification("Hello");

        assert_eq!(request.text, "Hello");
        assert_eq!(request.message_type, HudMessageType::kNotification);
        assert_eq!(request.ui_message_type, UIMessageType::Update);
        assert!(request.show);
        assert_eq!(request.crosshair_ref, ObjectRefHandle::new());
        assert!(request.quest.is_null());
        assert!(request.word_of_power.is_null());
        assert_eq!(request.discovery, None);
    }

    #[test]
    fn set_mode_request_uses_show_flag_for_push_pop() {
        let push = HudNotificationRequest::hud_mode("InventoryLessHUD", true);
        let pop = HudNotificationRequest::hud_mode("InventoryLessHUD", false);

        assert_eq!(push.message_type, HudMessageType::kSetMode);
        assert!(push.show);
        assert!(!pop.show);
    }

    #[test]
    fn write_request_to_hud_data_copies_all_payload_fields() {
        let quest = unsafe { GamePtr::from_raw(0x1000usize as *mut TESQuest) };
        let word = unsafe { GamePtr::from_raw(0x2000usize as *mut TESWordOfPower) };
        let request = HudNotificationRequest::location_discovery("Rorikstead", MARKER_TYPE::kTown)
            .with_ui_message_type(UIMessageType::Show)
            .with_show(false)
            .with_crosshair_ref(ObjectRefHandle {
                handle: 0xDEAD_BEEF,
            })
            .with_quest(quest)
            .with_word_of_power(word);
        let mut data = empty_hud_data();

        write_request_to_hud_data(&mut data, request);

        assert_eq!(data.get_type(), Some(HudMessageType::kLocationDiscovery));
        assert_eq!(data.text.as_str(), "Rorikstead");
        assert_eq!(data.crosshair_ref.handle, 0xDEAD_BEEF);
        assert_eq!(data.quest, quest.as_ptr());
        assert_eq!(data.word_of_power, word.as_ptr());
        assert!(!data.show);
        assert_eq!(data.discovery, MARKER_TYPE::kTown);
    }

    #[test]
    fn discovery_storage_is_empty_without_marker() {
        assert!(discovery_storage(None).is_empty());
        assert_eq!(
            discovery_storage(Some(MARKER_TYPE::kQuestTarget)),
            MARKER_TYPE::kQuestTarget
        );
    }
}
