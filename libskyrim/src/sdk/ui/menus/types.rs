use crate::re::{
    BGSLocation, BSUIMessageData, BSUIScaleformData, FaderData, HUDData, LoadingMenuData,
    UIMessageType,
};
use crate::sdk::core::GamePtr;

/// Small trait for menu-like types with a stable UI registration name.
///
/// Users can implement this for their own custom menu markers even before the
/// concrete `RE::*Menu` type exists in `libskyrim`.
pub trait NamedMenu {
    const MENU_NAME: &'static str;
}

/// Marker trait for menu message-data types constructible through the engine's
/// `UIMessageDataFactory`.
pub trait TypedMenuMessageData {
    const CLASS_NAME: &'static str;
}

macro_rules! impl_named_menu {
    ($($ty:path),+ $(,)?) => {
        $(
            impl NamedMenu for $ty {
                const MENU_NAME: &'static str = <$ty>::MENU_NAME;
            }
        )+
    };
}

impl_named_menu!(
    crate::re::BarterMenu,
    crate::re::BookMenu,
    crate::re::Console,
    crate::re::ConsoleNativeUIMenu,
    crate::re::ContainerMenu,
    crate::re::CraftingMenu,
    crate::re::CreationClubMenu,
    crate::re::CreditsMenu,
    crate::re::CursorMenu,
    crate::re::DialogueMenu,
    crate::re::FavoritesMenu,
    crate::re::FaderMenu,
    crate::re::GiftMenu,
    crate::re::HUDMenu,
    crate::re::InventoryMenu,
    crate::re::JournalMenu,
    crate::re::KinectMenu,
    crate::re::LevelUpMenu,
    crate::re::LoadingMenu,
    crate::re::LoadWaitSpinner,
    crate::re::LockpickingMenu,
    crate::re::MagicMenu,
    crate::re::MainMenu,
    crate::re::MapMenu,
    crate::re::MessageBoxMenu,
    crate::re::MistMenu,
    crate::re::ModManagerMenu,
    crate::re::RaceSexMenu,
    crate::re::SafeZoneMenu,
    crate::re::SleepWaitMenu,
    crate::re::StatsMenu,
    crate::re::TitleSequenceMenu,
    crate::re::TrainingMenu,
    crate::re::TutorialMenu,
    crate::re::TweenMenu,
    crate::re::WSActivateRollover,
);

macro_rules! impl_typed_menu_message_data {
    ($($ty:path),+ $(,)?) => {
        $(
            impl TypedMenuMessageData for $ty {
                const CLASS_NAME: &'static str = <$ty>::CLASS_NAME;
            }
        )+
    };
}

impl_typed_menu_message_data!(
    BSUIMessageData,
    BSUIScaleformData,
    FaderData,
    HUDData,
    LoadingMenuData,
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FadeRequest {
    pub min_duration: f32,
    pub fade_duration: f32,
    pub message_type: UIMessageType,
    pub is_fading_out: bool,
    pub is_black: bool,
    pub pauses_game: bool,
}

impl FadeRequest {
    #[inline(always)]
    pub const fn to_black(min_duration: f32, fade_duration: f32, pauses_game: bool) -> Self {
        Self {
            min_duration,
            fade_duration,
            message_type: UIMessageType::Show,
            is_fading_out: true,
            is_black: true,
            pauses_game,
        }
    }

    #[inline(always)]
    pub const fn from_black(fade_duration: f32, pauses_game: bool) -> Self {
        Self {
            min_duration: 0.0,
            fade_duration,
            message_type: UIMessageType::Show,
            is_fading_out: false,
            is_black: true,
            pauses_game,
        }
    }
}

impl Default for FadeRequest {
    #[inline(always)]
    fn default() -> Self {
        Self::to_black(0.0, 0.25, false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadingMenuRequest {
    pub current_location: GamePtr<BGSLocation>,
    pub message_type: UIMessageType,
    pub show_loading_text: bool,
}

impl LoadingMenuRequest {
    #[inline(always)]
    pub const fn show(current_location: GamePtr<BGSLocation>, show_loading_text: bool) -> Self {
        Self {
            current_location,
            message_type: UIMessageType::Show,
            show_loading_text,
        }
    }
}

impl Default for LoadingMenuRequest {
    #[inline(always)]
    fn default() -> Self {
        Self::show(GamePtr::null(), false)
    }
}
