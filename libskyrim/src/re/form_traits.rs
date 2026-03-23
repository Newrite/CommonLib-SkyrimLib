use crate::re::form_type::FormType;
pub trait FormCastable {
    const TARGET_FORM_TYPE: FormType;
}
