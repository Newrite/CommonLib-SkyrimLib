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

pub const DEFAULT_HUD_UI_MESSAGE_TYPE: UIMessageType = UIMessageType::Update;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HudNotificationRequest<'a> {
    pub text: &'a str,
    pub message_type: HudMessageType,
    pub ui_message_type: UIMessageType,
    pub show: bool,
    pub crosshair_ref: ObjectRefHandle,
    pub quest: GamePtr<TESQuest>,
    pub word_of_power: GamePtr<TESWordOfPower>,
    pub discovery: Option<MARKER_TYPE>,
}

impl<'a> HudNotificationRequest<'a> {
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

    #[inline(always)]
    pub const fn notification(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kNotification)
    }

    #[inline(always)]
    pub const fn subtitle(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kShowSubtitle)
    }

    #[inline(always)]
    pub const fn hide_subtitle() -> Self {
        Self::new("", HudMessageType::kHideSubtitle).with_show(false)
    }

    #[inline(always)]
    pub const fn hint_text(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kShowHintText)
    }

    #[inline(always)]
    pub const fn location_name(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kShowLocationName)
    }

    #[inline(always)]
    pub const fn location_discovery(text: &'a str, discovery: MARKER_TYPE) -> Self {
        Self::new(text, HudMessageType::kLocationDiscovery).with_discovery(discovery)
    }

    #[inline(always)]
    pub const fn skill_increase(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kSkillIncrease)
    }

    #[inline(always)]
    pub const fn dragon_soul_absorbed(text: &'a str) -> Self {
        Self::new(text, HudMessageType::kDragonSoulAbsorbed)
    }

    #[inline(always)]
    pub const fn set_mode(mode: &'a str, push: bool) -> Self {
        Self::new(mode, HudMessageType::kSetMode).with_show(push)
    }

    #[inline(always)]
    pub const fn with_ui_message_type(mut self, ui_message_type: UIMessageType) -> Self {
        self.ui_message_type = ui_message_type;
        self
    }

    #[inline(always)]
    pub const fn with_show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }

    #[inline(always)]
    pub const fn with_crosshair_ref(mut self, crosshair_ref: ObjectRefHandle) -> Self {
        self.crosshair_ref = crosshair_ref;
        self
    }

    #[inline(always)]
    pub fn with_quest(mut self, quest: impl Into<GamePtr<TESQuest>>) -> Self {
        self.quest = quest.into();
        self
    }

    #[inline(always)]
    pub fn with_word_of_power(mut self, word_of_power: impl Into<GamePtr<TESWordOfPower>>) -> Self {
        self.word_of_power = word_of_power.into();
        self
    }

    #[inline(always)]
    pub const fn with_discovery(mut self, discovery: MARKER_TYPE) -> Self {
        self.discovery = Some(discovery);
        self
    }

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

pub fn queue_hud_notification(request: HudNotificationRequest<'_>) -> bool {
    let queued =
        menus::queue_named_message_with::<HUDMenu, HUDData>(request.ui_message_type, |data| {
            write_request_to_hud_data(data, request);
        });

    if !queued {
        crate::defensive_sdk_warn!(
            "sdk::ui::notifications::queue_hud_notification() failed for {:?}",
            request.message_type
        );
    }

    queued
}

#[inline(always)]
pub fn show_notification(text: &str) -> bool {
    queue_hud_notification(HudNotificationRequest::notification(text))
}

#[inline(always)]
pub fn show_subtitle(text: &str) -> bool {
    queue_hud_notification(HudNotificationRequest::subtitle(text))
}

#[inline(always)]
pub fn hide_subtitle() -> bool {
    queue_hud_notification(HudNotificationRequest::hide_subtitle())
}

#[inline(always)]
pub fn show_hint_text(text: &str) -> bool {
    queue_hud_notification(HudNotificationRequest::hint_text(text))
}

#[inline(always)]
pub fn show_location_name(text: &str) -> bool {
    queue_hud_notification(HudNotificationRequest::location_name(text))
}

#[inline(always)]
pub fn show_location_discovery(text: &str, discovery: MARKER_TYPE) -> bool {
    queue_hud_notification(HudNotificationRequest::location_discovery(text, discovery))
}

#[inline(always)]
pub fn show_quest_started(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kQuestStarted).with_quest(quest),
    )
}

#[inline(always)]
pub fn show_quest_complete(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kQuestComplete).with_quest(quest),
    )
}

#[inline(always)]
pub fn show_quest_failed(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kQuestFailed).with_quest(quest),
    )
}

#[inline(always)]
pub fn show_objective_started(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kObjectiveStarted).with_quest(quest),
    )
}

#[inline(always)]
pub fn show_objective_complete(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kObjectiveComplete).with_quest(quest),
    )
}

#[inline(always)]
pub fn show_objective_failed(text: &str, quest: impl Into<GamePtr<TESQuest>>) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kObjectiveFailed).with_quest(quest),
    )
}

#[inline(always)]
pub fn show_skill_increase(text: &str) -> bool {
    queue_hud_notification(HudNotificationRequest::skill_increase(text))
}

#[inline(always)]
pub fn show_word_of_power_learned(
    text: &str,
    word_of_power: impl Into<GamePtr<TESWordOfPower>>,
) -> bool {
    queue_hud_notification(
        HudNotificationRequest::new(text, HudMessageType::kWordOfPowerLearned)
            .with_word_of_power(word_of_power),
    )
}

#[inline(always)]
pub fn show_dragon_soul_absorbed(text: &str) -> bool {
    queue_hud_notification(HudNotificationRequest::dragon_soul_absorbed(text))
}

#[inline(always)]
pub fn set_hud_mode(mode: &str, push: bool) -> bool {
    queue_hud_notification(HudNotificationRequest::set_mode(mode, push))
}

#[inline(always)]
pub fn push_hud_mode(mode: &str) -> bool {
    set_hud_mode(mode, true)
}

#[inline(always)]
pub fn pop_hud_mode(mode: &str) -> bool {
    set_hud_mode(mode, false)
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
        let push = HudNotificationRequest::set_mode("InventoryLessHUD", true);
        let pop = HudNotificationRequest::set_mode("InventoryLessHUD", false);

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
