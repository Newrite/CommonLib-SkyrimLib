// AUTO-STUB: Full translation pending
// TODO: VERIFY - replace with full translation when layout is needed
core_util::abstract_type! { pub type Array; }

impl crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable for Array {
    #[inline(always)]
    fn bst_inc_ref(&self) {}

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {}
}
