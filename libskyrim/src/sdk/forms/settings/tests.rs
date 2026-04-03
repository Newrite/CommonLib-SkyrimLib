use super::{
    SettingValue, SettingValueRef, set_bool, set_setting_value, setting_value, setting_value_ref,
    try_bool, try_float, try_string,
};
use crate::re::{Setting, SettingType};

#[test]
fn value_views_follow_runtime_setting_type() {
    let bool_setting = Setting::new_bool("bTest", true);
    let float_setting = Setting::new_float("fTest", 2.5);
    let string_setting = Setting::new_string("STest", "value");

    assert_eq!(bool_setting.get_type(), SettingType::Bool);
    assert_eq!(
        setting_value_ref(&bool_setting),
        SettingValueRef::Bool(true)
    );
    assert_eq!(setting_value(&float_setting), SettingValue::Float(2.5));
    assert_eq!(try_bool(&bool_setting), Some(true));
    assert_eq!(try_float(&float_setting), Some(2.5));
    assert_eq!(try_string(&string_setting), Some("value"));
}

#[test]
fn typed_setters_reject_mismatched_values() {
    let mut bool_setting = Setting::new_bool("bTest", false);
    let mut int_setting = Setting::new_integer("iTest", 7);

    assert!(set_bool(&mut bool_setting, true));
    assert_eq!(setting_value(&bool_setting), SettingValue::Bool(true));

    assert!(!set_bool(&mut int_setting, true));
    assert_eq!(setting_value(&int_setting), SettingValue::Integer(7));

    assert!(set_setting_value(
        &mut int_setting,
        SettingValueRef::Integer(42)
    ));
    assert_eq!(setting_value(&int_setting), SettingValue::Integer(42));
    assert!(!set_setting_value(
        &mut int_setting,
        SettingValueRef::String("nope")
    ));
}
