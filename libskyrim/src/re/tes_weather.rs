use crate::offsets::offsets_rtti::RTTI_TESWeather;
use crate::offsets::offsets_vtable::VTABLE_TESWeather;
use crate::re::{FormCastable, FormType};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESWeather; }

impl RttiType for TESWeather {
    const RTTI: VariantID = RTTI_TESWeather;
}

impl FormCastable for TESWeather {
    const TARGET_FORM_TYPE: FormType = FormType::Weather;
}

impl TESWeather {
    pub const RTTI: VariantID = RTTI_TESWeather;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESWeather;
    pub const FORMTYPE: FormType = FormType::Weather;
}

// TODO: `TESClimate` only needs pointer-typed `TESWeather` support through `WeatherType`.
// Replace this opaque stand-in with a real translation when a caller needs `TESWeather` layout
// or helper methods, instead of only source-backed RTTI/VTABLE/form-type metadata.
