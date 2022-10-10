/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

#[cfg(test)]
use mockall::automock;

use crate::implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> COption<T> {
    #![allow(dead_code)]
    fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct PasswordMakerQObject {}

pub struct PasswordMakerEmitter {
    qobject: Arc<AtomicPtr<PasswordMakerQObject>>,
    generated_password_changed: extern fn(*mut PasswordMakerQObject),
    generator_state_changed: extern fn(*mut PasswordMakerQObject),
    i_say_sexy_things_to_myself_while_im_dancing_changed: extern fn(*mut PasswordMakerQObject),
    master_password_changed: extern fn(*mut PasswordMakerQObject),
    url_changed: extern fn(*mut PasswordMakerQObject),
    used_text_changed: extern fn(*mut PasswordMakerQObject),
}

unsafe impl Send for PasswordMakerEmitter {}

#[cfg_attr(test, automock)]
#[cfg_attr(test,allow(dead_code))]
impl PasswordMakerEmitter {
    /// Clone the emitter
    pub fn clone(&self) -> Self {
        Self {
            qobject: self.qobject.clone(),
            generated_password_changed: self.generated_password_changed,
            generator_state_changed: self.generator_state_changed,
            i_say_sexy_things_to_myself_while_im_dancing_changed: self.i_say_sexy_things_to_myself_while_im_dancing_changed,
            master_password_changed: self.master_password_changed,
            url_changed: self.url_changed,
            used_text_changed: self.used_text_changed,
        }
    }
    fn clear(&self) {
        let n: *const PasswordMakerQObject = null();
        self.qobject.store(n as *mut PasswordMakerQObject, Ordering::SeqCst);
    }
    pub fn generated_password_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.generated_password_changed)(ptr);
        }
    }
    pub fn generator_state_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.generator_state_changed)(ptr);
        }
    }
    pub fn i_say_sexy_things_to_myself_while_im_dancing_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.i_say_sexy_things_to_myself_while_im_dancing_changed)(ptr);
        }
    }
    pub fn master_password_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.master_password_changed)(ptr);
        }
    }
    pub fn url_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.url_changed)(ptr);
        }
    }
    pub fn used_text_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.used_text_changed)(ptr);
        }
    }
}

pub trait PasswordMakerTrait {
    #[cfg(not(test))]
    fn new(emit: PasswordMakerEmitter,
        profiles: Profiles,
        settings: Settings) -> Self;
    #[cfg(not(test))]
    fn emit(&self) -> &PasswordMakerEmitter;
    #[cfg(test)]
    fn new(emit: MockPasswordMakerEmitter,
        profiles: Profiles,
        settings: Settings) -> Self;
    #[cfg(test)]
    fn emit(&self) -> &MockPasswordMakerEmitter;
    fn generated_password<F>(&self, setter: F) where F: FnOnce(&str);
    fn generator_state(&self) -> u8;
    fn i_say_sexy_things_to_myself_while_im_dancing(&self) -> bool;
    fn set_i_say_sexy_things_to_myself_while_im_dancing(&self, value: bool);
    fn master_password<F>(&self, setter: F) where F: FnOnce(&str);
    fn set_master_password(&self, value: String);
    fn profiles(&self) -> &Profiles;
    fn settings(&self) -> &Settings;
    fn url<F>(&self, setter: F) where F: FnOnce(&str);
    fn set_url(&self, value: String);
    fn used_text<F>(&self, setter: F) where F: FnOnce(&str);
    fn set_used_text(&self, value: String);
    fn profile_changed(&self) -> ();
    fn store_settings(&self) -> bool;
}
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn password_maker_new(
    password_maker: *mut PasswordMakerQObject,
    password_maker_generated_password_changed: extern fn(*mut PasswordMakerQObject),
    password_maker_generator_state_changed: extern fn(*mut PasswordMakerQObject),
    password_maker_i_say_sexy_things_to_myself_while_im_dancing_changed: extern fn(*mut PasswordMakerQObject),
    password_maker_master_password_changed: extern fn(*mut PasswordMakerQObject),
    profiles: *mut ProfilesQObject,
    profiles_current_profile_changed: extern fn(*mut ProfilesQObject),
    profiles_current_profile_name_changed: extern fn(*mut ProfilesQObject),
    profiles_new_data_ready: extern fn(*mut ProfilesQObject),
    profiles_layout_about_to_be_changed: extern fn(*mut ProfilesQObject),
    profiles_layout_changed: extern fn(*mut ProfilesQObject),
    profiles_data_changed: extern fn(*mut ProfilesQObject, usize, usize),
    profiles_begin_reset_model: extern fn(*mut ProfilesQObject),
    profiles_end_reset_model: extern fn(*mut ProfilesQObject),
    profiles_begin_insert_rows: extern fn(*mut ProfilesQObject, usize, usize),
    profiles_end_insert_rows: extern fn(*mut ProfilesQObject),
    profiles_begin_move_rows: extern fn(*mut ProfilesQObject, usize, usize, usize),
    profiles_end_move_rows: extern fn(*mut ProfilesQObject),
    profiles_begin_remove_rows: extern fn(*mut ProfilesQObject, usize, usize),
    profiles_end_remove_rows: extern fn(*mut ProfilesQObject),
    settings: *mut SettingsQObject,
    settings_clear_generated_password_seconds_changed: extern fn(*mut SettingsQObject),
    settings_clear_master_password_seconds_changed: extern fn(*mut SettingsQObject),
    settings_hide_generated_password_changed: extern fn(*mut SettingsQObject),
    password_maker_url_changed: extern fn(*mut PasswordMakerQObject),
    password_maker_used_text_changed: extern fn(*mut PasswordMakerQObject),
) -> *mut PasswordMaker {
    let profiles_emit = ProfilesEmitter {
        qobject: Arc::new(AtomicPtr::new(profiles)),
        current_profile_changed: profiles_current_profile_changed,
        current_profile_name_changed: profiles_current_profile_name_changed,
        new_data_ready: profiles_new_data_ready,
    };
    let model = ProfilesList {
        qobject: profiles,
        layout_about_to_be_changed: profiles_layout_about_to_be_changed,
        layout_changed: profiles_layout_changed,
        data_changed: profiles_data_changed,
        begin_reset_model: profiles_begin_reset_model,
        end_reset_model: profiles_end_reset_model,
        begin_insert_rows: profiles_begin_insert_rows,
        end_insert_rows: profiles_end_insert_rows,
        begin_move_rows: profiles_begin_move_rows,
        end_move_rows: profiles_end_move_rows,
        begin_remove_rows: profiles_begin_remove_rows,
        end_remove_rows: profiles_end_remove_rows,
    };
    let d_profiles = Profiles::new(profiles_emit, model);
    let settings_emit = SettingsEmitter {
        qobject: Arc::new(AtomicPtr::new(settings)),
        clear_generated_password_seconds_changed: settings_clear_generated_password_seconds_changed,
        clear_master_password_seconds_changed: settings_clear_master_password_seconds_changed,
        hide_generated_password_changed: settings_hide_generated_password_changed,
    };
    let d_settings = Settings::new(settings_emit);
    let password_maker_emit = PasswordMakerEmitter {
        qobject: Arc::new(AtomicPtr::new(password_maker)),
        generated_password_changed: password_maker_generated_password_changed,
        generator_state_changed: password_maker_generator_state_changed,
        i_say_sexy_things_to_myself_while_im_dancing_changed: password_maker_i_say_sexy_things_to_myself_while_im_dancing_changed,
        master_password_changed: password_maker_master_password_changed,
        url_changed: password_maker_url_changed,
        used_text_changed: password_maker_used_text_changed,
    };
    let d_password_maker = PasswordMaker::new(password_maker_emit,
        d_profiles,
        d_settings);
    Box::into_raw(Box::new(d_password_maker))
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_free(ptr: *mut PasswordMaker) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_generated_password_get(
ptr: *const PasswordMaker,
p: *mut QString,
set: extern fn(*mut QString, *const c_char, c_int),
) {
let o = &*ptr;
o.generated_password(|v| {
    let s: *const c_char = v.as_ptr() as *const c_char;
    set(p, s, to_c_int(v.len()));
});
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_generator_state_get(ptr: *const PasswordMaker) -> u8 {
    (&*ptr).generator_state()
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_i_say_sexy_things_to_myself_while_im_dancing_get(ptr: *const PasswordMaker) -> bool {
    (&*ptr).i_say_sexy_things_to_myself_while_im_dancing()
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_i_say_sexy_things_to_myself_while_im_dancing_set(ptr: *const PasswordMaker, v: bool) {
    (&*ptr).set_i_say_sexy_things_to_myself_while_im_dancing(v);
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_master_password_get(
ptr: *const PasswordMaker,
p: *mut QString,
set: extern fn(*mut QString, *const c_char, c_int),
) {
let o = &*ptr;
o.master_password(|v| {
    let s: *const c_char = v.as_ptr() as *const c_char;
    set(p, s, to_c_int(v.len()));
});
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_master_password_set(ptr: *const PasswordMaker, v: *const c_ushort, len: c_int) {
    let o = &*ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_master_password(s);
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_profiles_get(ptr: *const PasswordMaker) -> *const Profiles {
    (&*ptr).profiles()
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_settings_get(ptr: *const PasswordMaker) -> *const Settings {
    (&*ptr).settings()
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_url_get(
ptr: *const PasswordMaker,
p: *mut QString,
set: extern fn(*mut QString, *const c_char, c_int),
) {
let o = &*ptr;
o.url(|v| {
    let s: *const c_char = v.as_ptr() as *const c_char;
    set(p, s, to_c_int(v.len()));
});
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_url_set(ptr: *const PasswordMaker, v: *const c_ushort, len: c_int) {
    let o = &*ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_url(s);
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_used_text_get(
ptr: *const PasswordMaker,
p: *mut QString,
set: extern fn(*mut QString, *const c_char, c_int),
) {
let o = &*ptr;
o.used_text(|v| {
    let s: *const c_char = v.as_ptr() as *const c_char;
    set(p, s, to_c_int(v.len()));
});
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_used_text_set(ptr: *const PasswordMaker, v: *const c_ushort, len: c_int) {
    let o = &*ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_used_text(s);
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_profile_changed(ptr: *const PasswordMaker) {
    let o = &*ptr;
    o.profile_changed()
}

#[no_mangle]
pub unsafe extern "C" fn password_maker_store_settings(ptr: *const PasswordMaker) -> bool {
    let o = &*ptr;
    o.store_settings()
}

pub struct ProfilesQObject {}

pub struct ProfilesEmitter {
    qobject: Arc<AtomicPtr<ProfilesQObject>>,
    current_profile_changed: extern fn(*mut ProfilesQObject),
    current_profile_name_changed: extern fn(*mut ProfilesQObject),
    new_data_ready: extern fn(*mut ProfilesQObject),
}

unsafe impl Send for ProfilesEmitter {}

#[cfg_attr(test, automock)]
#[cfg_attr(test,allow(dead_code))]
impl ProfilesEmitter {
    /// Clone the emitter
    pub fn clone(&self) -> Self {
        Self {
            qobject: self.qobject.clone(),
            current_profile_changed: self.current_profile_changed,
            current_profile_name_changed: self.current_profile_name_changed,
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const ProfilesQObject = null();
        self.qobject.store(n as *mut ProfilesQObject, Ordering::SeqCst);
    }
    pub fn current_profile_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.current_profile_changed)(ptr);
        }
    }
    pub fn current_profile_name_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.current_profile_name_changed)(ptr);
        }
    }
    pub fn new_data_ready(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct ProfilesList {
    qobject: *mut ProfilesQObject,
    layout_about_to_be_changed: extern fn(*mut ProfilesQObject),
    layout_changed: extern fn(*mut ProfilesQObject),
    data_changed: extern fn(*mut ProfilesQObject, usize, usize),
    begin_reset_model: extern fn(*mut ProfilesQObject),
    end_reset_model: extern fn(*mut ProfilesQObject),
    begin_insert_rows: extern fn(*mut ProfilesQObject, usize, usize),
    end_insert_rows: extern fn(*mut ProfilesQObject),
    begin_move_rows: extern fn(*mut ProfilesQObject, usize, usize, usize),
    end_move_rows: extern fn(*mut ProfilesQObject),
    begin_remove_rows: extern fn(*mut ProfilesQObject, usize, usize),
    end_remove_rows: extern fn(*mut ProfilesQObject),
}


#[cfg_attr(test, automock)]
impl ProfilesList {
    pub fn layout_about_to_be_changed(&self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait ProfilesTrait {
    #[cfg(not(test))]
    fn new(emit: ProfilesEmitter, model: ProfilesList) -> Self;
    #[cfg(not(test))]
    fn emit(&self) -> &ProfilesEmitter;
    #[cfg(test)]
    fn new(emit: MockProfilesEmitter, model: MockProfilesList) -> Self;
    #[cfg(test)]
    fn emit(&self) -> &MockProfilesEmitter;
    fn current_profile(&self) -> u32;
    fn set_current_profile(&self, value: u32);
    fn current_profile_name<F>(&self, setter: F) where F: FnOnce(&str);
    fn store(&self) -> bool;
    fn row_count(&self) -> usize;
    fn insert_rows(&self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&self) {}
    fn sort(&self, _: u8, _: SortOrder) {}
    fn characters<F: FnOnce(&str)>(&self, index: usize, setter: F);
    fn set_characters(&self, index: usize, _: String) -> bool;
    fn hash_algorithm(&self, index: usize) -> u8;
    fn set_hash_algorithm(&self, index: usize, _: u8) -> bool;
    fn leet_level(&self, index: usize) -> u8;
    fn set_leet_level(&self, index: usize, _: u8) -> bool;
    fn modifier<F: FnOnce(&str)>(&self, index: usize, setter: F);
    fn set_modifier(&self, index: usize, _: String) -> bool;
    fn name<F: FnOnce(&str)>(&self, index: usize, setter: F);
    fn set_name(&self, index: usize, _: String) -> bool;
    fn password_length(&self, index: usize) -> u32;
    fn set_password_length(&self, index: usize, _: u32) -> bool;
    fn prefix<F: FnOnce(&str)>(&self, index: usize, setter: F);
    fn set_prefix(&self, index: usize, _: String) -> bool;
    fn suffix<F: FnOnce(&str)>(&self, index: usize, setter: F);
    fn set_suffix(&self, index: usize, _: String) -> bool;
    fn use_domain(&self, index: usize) -> bool;
    fn set_use_domain(&self, index: usize, _: bool) -> bool;
    fn use_leet(&self, index: usize) -> u8;
    fn set_use_leet(&self, index: usize, _: u8) -> bool;
    fn use_port_path(&self, index: usize) -> bool;
    fn set_use_port_path(&self, index: usize, _: bool) -> bool;
    fn use_protocol(&self, index: usize) -> bool;
    fn set_use_protocol(&self, index: usize, _: bool) -> bool;
    fn use_subdomains(&self, index: usize) -> bool;
    fn set_use_subdomains(&self, index: usize, _: bool) -> bool;
    fn use_undefined_as_protocol_fallback(&self, index: usize) -> bool;
    fn set_use_undefined_as_protocol_fallback(&self, index: usize, _: bool) -> bool;
    fn use_user_info(&self, index: usize) -> bool;
    fn set_use_user_info(&self, index: usize, _: bool) -> bool;
    fn username<F: FnOnce(&str)>(&self, index: usize, setter: F);
    fn set_username(&self, index: usize, _: String) -> bool;
}
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn profiles_new(
    profiles: *mut ProfilesQObject,
    profiles_current_profile_changed: extern fn(*mut ProfilesQObject),
    profiles_current_profile_name_changed: extern fn(*mut ProfilesQObject),
    profiles_new_data_ready: extern fn(*mut ProfilesQObject),
    profiles_layout_about_to_be_changed: extern fn(*mut ProfilesQObject),
    profiles_layout_changed: extern fn(*mut ProfilesQObject),
    profiles_data_changed: extern fn(*mut ProfilesQObject, usize, usize),
    profiles_begin_reset_model: extern fn(*mut ProfilesQObject),
    profiles_end_reset_model: extern fn(*mut ProfilesQObject),
    profiles_begin_insert_rows: extern fn(*mut ProfilesQObject, usize, usize),
    profiles_end_insert_rows: extern fn(*mut ProfilesQObject),
    profiles_begin_move_rows: extern fn(*mut ProfilesQObject, usize, usize, usize),
    profiles_end_move_rows: extern fn(*mut ProfilesQObject),
    profiles_begin_remove_rows: extern fn(*mut ProfilesQObject, usize, usize),
    profiles_end_remove_rows: extern fn(*mut ProfilesQObject),
) -> *mut Profiles {
    let profiles_emit = ProfilesEmitter {
        qobject: Arc::new(AtomicPtr::new(profiles)),
        current_profile_changed: profiles_current_profile_changed,
        current_profile_name_changed: profiles_current_profile_name_changed,
        new_data_ready: profiles_new_data_ready,
    };
    let model = ProfilesList {
        qobject: profiles,
        layout_about_to_be_changed: profiles_layout_about_to_be_changed,
        layout_changed: profiles_layout_changed,
        data_changed: profiles_data_changed,
        begin_reset_model: profiles_begin_reset_model,
        end_reset_model: profiles_end_reset_model,
        begin_insert_rows: profiles_begin_insert_rows,
        end_insert_rows: profiles_end_insert_rows,
        begin_move_rows: profiles_begin_move_rows,
        end_move_rows: profiles_end_move_rows,
        begin_remove_rows: profiles_begin_remove_rows,
        end_remove_rows: profiles_end_remove_rows,
    };
    let d_profiles = Profiles::new(profiles_emit, model);
    Box::into_raw(Box::new(d_profiles))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_free(ptr: *mut Profiles) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn profiles_current_profile_get(ptr: *const Profiles) -> u32 {
    (&*ptr).current_profile()
}

#[no_mangle]
pub unsafe extern "C" fn profiles_current_profile_set(ptr: *const Profiles, v: u32) {
    (&*ptr).set_current_profile(v);
}

#[no_mangle]
pub unsafe extern "C" fn profiles_current_profile_name_get(
ptr: *const Profiles,
p: *mut QString,
set: extern fn(*mut QString, *const c_char, c_int),
) {
let o = &*ptr;
o.current_profile_name(|v| {
    let s: *const c_char = v.as_ptr() as *const c_char;
    set(p, s, to_c_int(v.len()));
});
}

#[no_mangle]
pub unsafe extern "C" fn profiles_store(ptr: *const Profiles) -> bool {
    let o = &*ptr;
    o.store()
}

#[no_mangle]
pub unsafe extern "C" fn profiles_row_count(ptr: *const Profiles) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn profiles_insert_rows(ptr: *const Profiles, row: c_int, count: c_int) -> bool {
    (&*ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn profiles_remove_rows(ptr: *const Profiles, row: c_int, count: c_int) -> bool {
    (&*ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn profiles_can_fetch_more(ptr: *const Profiles) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn profiles_fetch_more(ptr: *const Profiles) {
    (&*ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn profiles_sort(
    ptr: *const Profiles,
    column: u8,
    order: SortOrder,
) {
    (&*ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_characters(
    ptr: *const Profiles, row: c_int,
    d: *mut QString,
    set: extern fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    o.characters(to_usize(row), |data| {
        let s: *const c_char = data.as_ptr() as *const c_char;
        set(d, s, to_c_int(data.len()));
    })
    }

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_characters(
    ptr: *const Profiles, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &*ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_characters(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_hash_algorithm(ptr: *const Profiles, row: c_int) -> u8 {
    let o = &*ptr;
    o.hash_algorithm(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_hash_algorithm(
    ptr: *const Profiles, row: c_int,
    v: u8,
) -> bool {
    (&*ptr).set_hash_algorithm(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_leet_level(ptr: *const Profiles, row: c_int) -> u8 {
    let o = &*ptr;
    o.leet_level(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_leet_level(
    ptr: *const Profiles, row: c_int,
    v: u8,
) -> bool {
    (&*ptr).set_leet_level(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_modifier(
    ptr: *const Profiles, row: c_int,
    d: *mut QString,
    set: extern fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    o.modifier(to_usize(row), |data| {
        let s: *const c_char = data.as_ptr() as *const c_char;
        set(d, s, to_c_int(data.len()));
    })
    }

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_modifier(
    ptr: *const Profiles, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &*ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_modifier(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_name(
    ptr: *const Profiles, row: c_int,
    d: *mut QString,
    set: extern fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    o.name(to_usize(row), |data| {
        let s: *const c_char = data.as_ptr() as *const c_char;
        set(d, s, to_c_int(data.len()));
    })
    }

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_name(
    ptr: *const Profiles, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &*ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_name(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_password_length(ptr: *const Profiles, row: c_int) -> u32 {
    let o = &*ptr;
    o.password_length(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_password_length(
    ptr: *const Profiles, row: c_int,
    v: u32,
) -> bool {
    (&*ptr).set_password_length(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_prefix(
    ptr: *const Profiles, row: c_int,
    d: *mut QString,
    set: extern fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    o.prefix(to_usize(row), |data| {
        let s: *const c_char = data.as_ptr() as *const c_char;
        set(d, s, to_c_int(data.len()));
    })
    }

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_prefix(
    ptr: *const Profiles, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &*ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_prefix(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_suffix(
    ptr: *const Profiles, row: c_int,
    d: *mut QString,
    set: extern fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    o.suffix(to_usize(row), |data| {
        let s: *const c_char = data.as_ptr() as *const c_char;
        set(d, s, to_c_int(data.len()));
    })
    }

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_suffix(
    ptr: *const Profiles, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &*ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_suffix(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_domain(ptr: *const Profiles, row: c_int) -> bool {
    let o = &*ptr;
    o.use_domain(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_domain(
    ptr: *const Profiles, row: c_int,
    v: bool,
) -> bool {
    (&*ptr).set_use_domain(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_leet(ptr: *const Profiles, row: c_int) -> u8 {
    let o = &*ptr;
    o.use_leet(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_leet(
    ptr: *const Profiles, row: c_int,
    v: u8,
) -> bool {
    (&*ptr).set_use_leet(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_port_path(ptr: *const Profiles, row: c_int) -> bool {
    let o = &*ptr;
    o.use_port_path(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_port_path(
    ptr: *const Profiles, row: c_int,
    v: bool,
) -> bool {
    (&*ptr).set_use_port_path(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_protocol(ptr: *const Profiles, row: c_int) -> bool {
    let o = &*ptr;
    o.use_protocol(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_protocol(
    ptr: *const Profiles, row: c_int,
    v: bool,
) -> bool {
    (&*ptr).set_use_protocol(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_subdomains(ptr: *const Profiles, row: c_int) -> bool {
    let o = &*ptr;
    o.use_subdomains(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_subdomains(
    ptr: *const Profiles, row: c_int,
    v: bool,
) -> bool {
    (&*ptr).set_use_subdomains(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_undefined_as_protocol_fallback(ptr: *const Profiles, row: c_int) -> bool {
    let o = &*ptr;
    o.use_undefined_as_protocol_fallback(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_undefined_as_protocol_fallback(
    ptr: *const Profiles, row: c_int,
    v: bool,
) -> bool {
    (&*ptr).set_use_undefined_as_protocol_fallback(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_use_user_info(ptr: *const Profiles, row: c_int) -> bool {
    let o = &*ptr;
    o.use_user_info(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_use_user_info(
    ptr: *const Profiles, row: c_int,
    v: bool,
) -> bool {
    (&*ptr).set_use_user_info(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn profiles_data_username(
    ptr: *const Profiles, row: c_int,
    d: *mut QString,
    set: extern fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    o.username(to_usize(row), |data| {
        let s: *const c_char = data.as_ptr() as *const c_char;
        set(d, s, to_c_int(data.len()));
    })
    }

#[no_mangle]
pub unsafe extern "C" fn profiles_set_data_username(
    ptr: *const Profiles, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &*ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_username(to_usize(row), v)
}

pub struct SettingsQObject {}

pub struct SettingsEmitter {
    qobject: Arc<AtomicPtr<SettingsQObject>>,
    clear_generated_password_seconds_changed: extern fn(*mut SettingsQObject),
    clear_master_password_seconds_changed: extern fn(*mut SettingsQObject),
    hide_generated_password_changed: extern fn(*mut SettingsQObject),
}

unsafe impl Send for SettingsEmitter {}

#[cfg_attr(test, automock)]
#[cfg_attr(test,allow(dead_code))]
impl SettingsEmitter {
    /// Clone the emitter
    pub fn clone(&self) -> Self {
        Self {
            qobject: self.qobject.clone(),
            clear_generated_password_seconds_changed: self.clear_generated_password_seconds_changed,
            clear_master_password_seconds_changed: self.clear_master_password_seconds_changed,
            hide_generated_password_changed: self.hide_generated_password_changed,
        }
    }
    fn clear(&self) {
        let n: *const SettingsQObject = null();
        self.qobject.store(n as *mut SettingsQObject, Ordering::SeqCst);
    }
    pub fn clear_generated_password_seconds_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.clear_generated_password_seconds_changed)(ptr);
        }
    }
    pub fn clear_master_password_seconds_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.clear_master_password_seconds_changed)(ptr);
        }
    }
    pub fn hide_generated_password_changed(&self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.hide_generated_password_changed)(ptr);
        }
    }
}

pub trait SettingsTrait {
    #[cfg(not(test))]
    fn new(emit: SettingsEmitter) -> Self;
    #[cfg(not(test))]
    fn emit(&self) -> &SettingsEmitter;
    #[cfg(test)]
    fn new(emit: MockSettingsEmitter) -> Self;
    #[cfg(test)]
    fn emit(&self) -> &MockSettingsEmitter;
    fn clear_generated_password_seconds(&self) -> Option<u32>;
    fn set_clear_generated_password_seconds(&self, value: Option<u32>);
    fn clear_master_password_seconds(&self) -> Option<u32>;
    fn set_clear_master_password_seconds(&self, value: Option<u32>);
    fn hide_generated_password(&self) -> bool;
    fn set_hide_generated_password(&self, value: bool);
}
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn settings_new(
    settings: *mut SettingsQObject,
    settings_clear_generated_password_seconds_changed: extern fn(*mut SettingsQObject),
    settings_clear_master_password_seconds_changed: extern fn(*mut SettingsQObject),
    settings_hide_generated_password_changed: extern fn(*mut SettingsQObject),
) -> *mut Settings {
    let settings_emit = SettingsEmitter {
        qobject: Arc::new(AtomicPtr::new(settings)),
        clear_generated_password_seconds_changed: settings_clear_generated_password_seconds_changed,
        clear_master_password_seconds_changed: settings_clear_master_password_seconds_changed,
        hide_generated_password_changed: settings_hide_generated_password_changed,
    };
    let d_settings = Settings::new(settings_emit);
    Box::into_raw(Box::new(d_settings))
}

#[no_mangle]
pub unsafe extern "C" fn settings_free(ptr: *mut Settings) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn settings_clear_generated_password_seconds_get(ptr: *const Settings) -> COption<u32> {
    match (&*ptr).clear_generated_password_seconds() {
        Some(value) => COption { data: value, some: true },
        None => COption { data: u32::default(), some: false}
    }
}

#[no_mangle]
pub unsafe extern "C" fn settings_clear_generated_password_seconds_set(ptr: *const Settings, v: u32) {
    (&*ptr).set_clear_generated_password_seconds(Some(v));
}

#[no_mangle]
pub unsafe extern "C" fn settings_clear_generated_password_seconds_set_none(ptr: *const Settings) {
    let o = &*ptr;
    o.set_clear_generated_password_seconds(None);
}

#[no_mangle]
pub unsafe extern "C" fn settings_clear_master_password_seconds_get(ptr: *const Settings) -> COption<u32> {
    match (&*ptr).clear_master_password_seconds() {
        Some(value) => COption { data: value, some: true },
        None => COption { data: u32::default(), some: false}
    }
}

#[no_mangle]
pub unsafe extern "C" fn settings_clear_master_password_seconds_set(ptr: *const Settings, v: u32) {
    (&*ptr).set_clear_master_password_seconds(Some(v));
}

#[no_mangle]
pub unsafe extern "C" fn settings_clear_master_password_seconds_set_none(ptr: *const Settings) {
    let o = &*ptr;
    o.set_clear_master_password_seconds(None);
}

#[no_mangle]
pub unsafe extern "C" fn settings_hide_generated_password_get(ptr: *const Settings) -> bool {
    (&*ptr).hide_generated_password()
}

#[no_mangle]
pub unsafe extern "C" fn settings_hide_generated_password_set(ptr: *const Settings, v: bool) {
    (&*ptr).set_hide_generated_password(v);
}
