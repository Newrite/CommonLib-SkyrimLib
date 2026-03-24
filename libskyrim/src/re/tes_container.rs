use alloc::boxed::Box;
use core::ffi::c_void;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESContainer;
use crate::offsets::offsets_vtable::VTABLE_TESContainer;
use crate::re::BSContainerForEachResult;
use crate::re::ContainerItemExtra;
use crate::re::TESBoundObject;
use crate::re::TESForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct ContainerObject {
    pub count: i32,                          // 00
    pub pad04: u32,                          // 04
    pub obj: *mut TESBoundObject,            // 08
    pub item_extra: *mut ContainerItemExtra, // 10
}

const _: () = assert!(core::mem::size_of::<ContainerObject>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ContainerObject, count) == 0x00);
const _: () = assert!(core::mem::offset_of!(ContainerObject, obj) == 0x08);
const _: () = assert!(core::mem::offset_of!(ContainerObject, item_extra) == 0x10);

impl Default for ContainerObject {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerObject {
    #[inline]
    pub const fn new() -> Self {
        Self {
            count: 0,
            pad04: 0,
            obj: core::ptr::null_mut(),
            item_extra: core::ptr::null_mut(),
        }
    }

    #[inline]
    pub const fn with_count(obj: *mut TESBoundObject, count: i32) -> Self {
        Self {
            count,
            pad04: 0,
            obj,
            item_extra: core::ptr::null_mut(),
        }
    }

    #[inline]
    pub fn with_owner(obj: *mut TESBoundObject, count: i32, owner: *mut TESForm) -> Self {
        let item_extra = if owner.is_null() {
            core::ptr::null_mut()
        } else {
            Box::into_raw(Box::new(ContainerItemExtra::new_with_owner(owner)))
        };

        Self {
            count,
            pad04: 0,
            obj,
            item_extra,
        }
    }
}

#[repr(C)]
pub struct TESContainer {
    pub base: BaseFormComponent,                      // 00
    pub container_objects: *mut *mut ContainerObject, // 08
    pub num_container_objects: u32,                   // 10
    pub allow_stolen_items: bool,                     // 14
    pub pad15: [u8; 3],                               // 15
}

const _: () = assert!(core::mem::size_of::<TESContainer>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TESContainer, container_objects) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESContainer, num_container_objects) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESContainer, allow_stolen_items) == 0x14);

impl RttiType for TESContainer {
    const RTTI: VariantID = RTTI_TESContainer;
}

impl AsRef<TESContainer> for TESContainer {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESContainer> for TESContainer {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TESContainer : BaseFormComponent);

impl TESContainer {
    pub const RTTI: VariantID = RTTI_TESContainer;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESContainer;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    #[inline]
    pub fn container_objects_slice(&self) -> &[*mut ContainerObject] {
        if self.container_objects.is_null() || self.num_container_objects == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(
                    self.container_objects,
                    self.num_container_objects as usize,
                )
            }
        }
    }

    pub fn for_each_container_object<F>(&self, mut callback: F)
    where
        F: FnMut(&mut ContainerObject) -> BSContainerForEachResult,
    {
        for &entry in self.container_objects_slice() {
            let Some(entry) = (unsafe { entry.as_mut() }) else {
                continue;
            };

            if callback(entry).is_stop() {
                break;
            }
        }
    }

    fn copy_object_list(&mut self, copied_data: &[*mut ContainerObject]) {
        let old_data = self.container_objects;
        let new_size = copied_data.len();
        let new_data = if new_size == 0 {
            core::ptr::null_mut()
        } else {
            unsafe {
                crate::ffi::commonlib_calloc(new_size, core::mem::size_of::<*mut ContainerObject>())
                    as *mut *mut ContainerObject
            }
        };

        if new_size != 0 {
            assert!(
                !new_data.is_null(),
                "TESContainer::copy_object_list allocation failed"
            );
            unsafe {
                core::ptr::copy_nonoverlapping(copied_data.as_ptr(), new_data, new_size);
            }
        }

        self.num_container_objects = new_size as u32;
        self.container_objects = new_data;

        if !old_data.is_null() {
            unsafe {
                crate::ffi::commonlib_free(old_data.cast::<c_void>());
            }
        }
    }

    pub fn get_container_object_at(&self, index: u32) -> Option<*mut ContainerObject> {
        self.container_objects_slice().get(index as usize).copied()
    }

    pub fn get_container_object_index(
        &self,
        object: *mut TESBoundObject,
        count: i32,
    ) -> Option<u32> {
        self.container_objects_slice()
            .iter()
            .position(|&entry| unsafe {
                !entry.is_null() && (*entry).obj == object && (*entry).count == count
            })
            .map(|index| index as u32)
    }

    pub fn add_object_to_container(
        &mut self,
        object: *mut TESBoundObject,
        count: i32,
        owner: *mut TESForm,
    ) -> bool {
        for &entry in self.container_objects_slice() {
            if entry.is_null() {
                continue;
            }

            let entry_ref = unsafe { &mut *entry };
            if entry_ref.obj == object {
                entry_ref.count += count;
                return true;
            }
        }

        let mut copied_data = self.container_objects_slice().to_vec();
        copied_data.push(Box::into_raw(Box::new(ContainerObject::with_owner(
            object, count, owner,
        ))));
        self.copy_object_list(&copied_data);
        true
    }

    pub fn add_objects_to_container<I>(&mut self, objects: I, owner: *mut TESForm) -> bool
    where
        I: IntoIterator<Item = (*mut TESBoundObject, i32)>,
    {
        let mut copied_data = self.container_objects_slice().to_vec();

        for (object, count) in objects {
            let mut merged = false;
            for &entry in &copied_data {
                if entry.is_null() {
                    continue;
                }

                let entry_ref = unsafe { &mut *entry };
                if entry_ref.obj == object {
                    entry_ref.count += count;
                    merged = true;
                    break;
                }
            }

            if !merged {
                copied_data.push(Box::into_raw(Box::new(ContainerObject::with_owner(
                    object, count, owner,
                ))));
            }
        }

        self.copy_object_list(&copied_data);
        true
    }

    pub fn get_object_count(&self, object: *const TESBoundObject) -> i32 {
        let mut count = 0;
        self.for_each_container_object(|entry| {
            if core::ptr::eq(entry.obj as *const TESBoundObject, object) {
                count += entry.count;
            }
            BSContainerForEachResult::Continue
        });
        count
    }

    pub fn remove_object_from_container(
        &mut self,
        object: *mut TESBoundObject,
        count: i32,
    ) -> bool {
        let Some(index) = self.get_container_object_index(object, count) else {
            return false;
        };

        let mut copied_data = self.container_objects_slice().to_vec();
        copied_data.remove(index as usize);
        self.copy_object_list(&copied_data);
        true
    }
}

pub trait TESContainerExt {
    fn container_objects_slice(&self) -> &[*mut ContainerObject];
    fn for_each_container_object<F>(&self, callback: F)
    where
        F: FnMut(&mut ContainerObject) -> BSContainerForEachResult;
    fn get_container_object_at(&self, index: u32) -> Option<*mut ContainerObject>;
    fn get_container_object_index(&self, object: *mut TESBoundObject, count: i32) -> Option<u32>;
    fn add_object_to_container(
        &mut self,
        object: *mut TESBoundObject,
        count: i32,
        owner: *mut TESForm,
    ) -> bool;
    fn add_objects_to_container<I>(&mut self, objects: I, owner: *mut TESForm) -> bool
    where
        I: IntoIterator<Item = (*mut TESBoundObject, i32)>;
    fn get_object_count(&self, object: *const TESBoundObject) -> i32;
    fn remove_object_from_container(&mut self, object: *mut TESBoundObject, count: i32) -> bool;
}

impl<T: AsRef<TESContainer> + AsMut<TESContainer>> TESContainerExt for T {
    #[inline(always)]
    fn container_objects_slice(&self) -> &[*mut ContainerObject] {
        self.as_ref().container_objects_slice()
    }

    #[inline(always)]
    fn for_each_container_object<F>(&self, callback: F)
    where
        F: FnMut(&mut ContainerObject) -> BSContainerForEachResult,
    {
        self.as_ref().for_each_container_object(callback)
    }

    #[inline(always)]
    fn get_container_object_at(&self, index: u32) -> Option<*mut ContainerObject> {
        self.as_ref().get_container_object_at(index)
    }

    #[inline(always)]
    fn get_container_object_index(&self, object: *mut TESBoundObject, count: i32) -> Option<u32> {
        self.as_ref().get_container_object_index(object, count)
    }

    #[inline(always)]
    fn add_object_to_container(
        &mut self,
        object: *mut TESBoundObject,
        count: i32,
        owner: *mut TESForm,
    ) -> bool {
        TESContainer::add_object_to_container(self.as_mut(), object, count, owner)
    }

    #[inline(always)]
    fn add_objects_to_container<I>(&mut self, objects: I, owner: *mut TESForm) -> bool
    where
        I: IntoIterator<Item = (*mut TESBoundObject, i32)>,
    {
        TESContainer::add_objects_to_container(self.as_mut(), objects, owner)
    }

    #[inline(always)]
    fn get_object_count(&self, object: *const TESBoundObject) -> i32 {
        self.as_ref().get_object_count(object)
    }

    #[inline(always)]
    fn remove_object_from_container(&mut self, object: *mut TESBoundObject, count: i32) -> bool {
        TESContainer::remove_object_from_container(self.as_mut(), object, count)
    }
}
