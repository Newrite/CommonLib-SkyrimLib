use core::sync::atomic::Ordering;

use crate::re::NiRefObject;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

// РРјРїРѕСЂС‚ РѕСЂРёРіРёРЅР°Р»СЊРЅС‹С… РєРѕРЅСЃС‚Р°РЅС‚ РёР· РѕС„С„СЃРµС‚РѕРІ
use crate::offsets::offsets_rtti::RTTI_BSHandleRefObject;
use crate::offsets::offsets_vtable::VTABLE_BSHandleRefObject;

#[repr(C)]
pub struct BSHandleRefObject {
    // Р’ Rust РјС‹ РїРѕРјРµС‰Р°РµРј Р±Р°Р·РѕРІС‹Р№ РєР»Р°СЃСЃ РєР°Рє РїРµСЂРІРѕРµ РїРѕР»Рµ.
    // РџРѕСЃРєРѕР»СЊРєСѓ NiRefObject РёРјРµРµС‚ repr(C), РµРіРѕ РїРѕР»СЏ Р±СѓРґСѓС‚ РІ РЅР°С‡Р°Р»Рµ.
    pub base: NiRefObject,
}

// РџСЂРѕРІРµСЂРєР° СЂР°Р·РјРµСЂР°: 0x10
const _: () = assert!(core::mem::size_of::<BSHandleRefObject>() == 0x10);

impl RttiType for BSHandleRefObject {
    const RTTI: VariantID = RTTI_BSHandleRefObject;
}

// в”Ђв”Ђв”Ђ Р Р•РђР›РР—РђР¦РРЇ РќРђРЎР›Р•Р”РћР’РђРќРРЇ в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

// РђРІС‚РѕРјР°С‚РёС‡РµСЃРєРё РіРµРЅРµСЂРёСЂСѓРµС‚ Deref, DerefMut Рё AsRef<NiRefObject>
inherit!(BSHandleRefObject : NiRefObject);

impl BSHandleRefObject {
    pub const RTTI: VariantID = RTTI_BSHandleRefObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSHandleRefObject;

    // РљРѕРЅСЃС‚Р°РЅС‚С‹ РјР°СЃРѕРє
    pub const REF_COUNT_MASK: u32 = 0x3FF; // РџРµСЂРІС‹Рµ 10 Р±РёС‚ вЂ” СЌС‚Рѕ СЃР°Рј СЃС‡РµС‚С‡РёРє
    pub const HANDLE_VALID_BIT: u32 = 1 << 10; // 11-Р№ Р±РёС‚ вЂ” РІР°Р»РёРґРЅРѕСЃС‚СЊ С…РµРЅРґР»Р°

    #[inline(always)]
    pub fn q_ref_count(&self) -> u32 {
        // Р§РёС‚Р°РµРј РёР· Р±Р°Р·С‹, РЅРѕ РїСЂРёРјРµРЅСЏРµРј РјР°СЃРєСѓ
        self.base.ref_count.load(Ordering::Acquire) & Self::REF_COUNT_MASK
    }

    #[inline(always)]
    pub fn is_handle_valid(&self) -> bool {
        (self.base.ref_count.load(Ordering::Acquire) & Self::HANDLE_VALID_BIT) != 0
    }

    #[inline(always)]
    pub fn inc_ref_count(&self) {
        // РњС‹ РјРѕР¶РµРј РїСЂРѕСЃС‚Рѕ РїСЂРёР±Р°РІРёС‚СЊ 1. Р”Р°Р¶Рµ РµСЃР»Рё РјС‹ Р·Р°С‚СЂРѕРЅРµРј Р±РёС‚С‹ РІС‹С€Рµ 10-РіРѕ,
        // РјР°СЃРєР° РїСЂРё С‡С‚РµРЅРёРё (QRefCount) СЌС‚Рѕ РѕС‚СЃРµС‡РµС‚.
        // Р’ РѕСЂРёРіРёРЅР°Р»Рµ РЎРєР°Р№СЂРёРјР° РёСЃРїРѕР»СЊР·СѓРµС‚СЃСЏ РѕР±С‹С‡РЅС‹Р№ РёРЅРєСЂРµРјРµРЅС‚.
        self.base.ref_count.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn dec_ref_count(&self) {
        // Р’Р°Р¶РЅС‹Р№ РјРѕРјРµРЅС‚: РґРµРєСЂРµРјРµРЅС‚РёСЂСѓРµРј РІСЃС‘ Р·РЅР°С‡РµРЅРёРµ,
        // РЅРѕ РїСЂРѕРІРµСЂСЏРµРј РЅР° 0 С‚РѕР»СЊРєРѕ С‚Сѓ С‡Р°СЃС‚СЊ, С‡С‚Рѕ РїРѕРґ РјР°СЃРєРѕР№.
        let old_val = self.base.ref_count.fetch_sub(1, Ordering::AcqRel);
        if (old_val - 1) & Self::REF_COUNT_MASK == 0 {
            // Р’С‹Р·С‹РІР°РµРј delete_this, РєРѕС‚РѕСЂС‹Р№ РЅР°Рј РґРѕСЃС‚СѓРїРµРЅ Р±Р»Р°РіРѕРґР°СЂСЏ РјР°РєСЂРѕСЃСѓ inherit!
            self.delete_this();
        }
    }
}

// в”Ђв”Ђв”Ђ РџР•Р Р•РћРџР Р•Р”Р•Р›Р•РќРР• РўР Р•Р™РўРђ NiRef в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

use crate::re::ni_ref_object::NiRef;

// Р­С‚Рѕ РєСЂРёС‚РёС‡РµСЃРєРё РІР°Р¶РЅРѕ: NiPointer<T> РёСЃРїРѕР»СЊР·СѓРµС‚ РјРµС‚РѕРґС‹ СЌС‚РѕРіРѕ С‚СЂРµР№С‚Р°.
// РўР°Рє РєР°Рє Р»РѕРіРёРєР° DecRefCount Р·РґРµСЃСЊ СЃРІРѕСЏ (СЃ РјР°СЃРєРѕР№), РјС‹ РґРѕР»Р¶РЅС‹ РµС‘ РїРµСЂРµРіСЂСѓР·РёС‚СЊ.
impl NiRef for BSHandleRefObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.inc_ref_count();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.dec_ref_count();
    }
}
