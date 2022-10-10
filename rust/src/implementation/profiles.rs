use std::convert::{From, Into, TryFrom, TryInto};
use std::cell::RefCell;
use std::fmt::Display;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use mockall_double::double;
#[cfg(test)]
use crate::implementation::pwm_macros::*;

#[double]
use crate::interface::{ProfilesEmitter, ProfilesList};
use crate::interface::ProfilesTrait;

use crate::implementation::{LoadError, StoreError};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(test, derive(EnumVariantCount))]
pub(crate) enum HashAlgorithm {
    Md4,
    HmacMd4,
    Md5,
    Md5Version06,
    HmacMd5,
    HmacMd5Version06,
    Sha1,
    HmacSha1,
    Sha256,
    HmacSha256,
    Ripemd160,
    HmacRipemd160,
}

impl Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgorithm::Md4 => write!(f,"MD4"),
            HashAlgorithm::HmacMd4 => write!(f, "MD4-HMAC"),
            HashAlgorithm::Md5 => write!(f, "MD5"),
            HashAlgorithm::Md5Version06 => write!(f, "MD5 PWM Version 0.6"),
            HashAlgorithm::HmacMd5 => write!(f, "MD5-HMAC"),
            HashAlgorithm::HmacMd5Version06 => write!(f, "MD5-HMAC PWM Version 0.6"),
            HashAlgorithm::Sha1 => write!(f,"SHA-1"),
            HashAlgorithm::HmacSha1 => write!(f, "SHA-1-HMAC"),
            HashAlgorithm::Sha256 => write!(f, "SHA-256"),
            HashAlgorithm::HmacSha256 => write!(f, "SHA-256-HMAC"),
            HashAlgorithm::Ripemd160 => write!(f, "RIPEMD-160"),
            HashAlgorithm::HmacRipemd160 => write!(f, "RIPEMD-160-HMAC"),
        }
    }
}

impl From<HashAlgorithm> for u8 {
    fn from(h : HashAlgorithm) -> u8 {
        h as u8
    }
}
impl TryFrom<u8> for HashAlgorithm {
    type Error = ();
    fn try_from(i : u8) -> Result<HashAlgorithm,()> {
        match i {
            0 => { Ok(HashAlgorithm::Md4) }
            1 => { Ok(HashAlgorithm::HmacMd4) }
            2 => { Ok(HashAlgorithm::Md5) }
            3 => { Ok(HashAlgorithm::Md5Version06) }
            4 => { Ok(HashAlgorithm::HmacMd5) }
            5 => { Ok(HashAlgorithm::HmacMd5Version06) }
            6 => { Ok(HashAlgorithm::Sha1) }
            7 => { Ok(HashAlgorithm::HmacSha1) }
            8 => { Ok(HashAlgorithm::Sha256) }
            9 => { Ok(HashAlgorithm::HmacSha256) }
            10 => {Ok(HashAlgorithm::Ripemd160) }
            11 => {Ok(HashAlgorithm::HmacRipemd160)}
            _ => { Err(()) }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(EnumVariantCount))]
pub(crate) enum LeetLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<LeetLevel> for u8 {
    fn from(l : LeetLevel) -> u8 {
        l as u8 + 1
    }
}
impl TryFrom<u8> for LeetLevel {
    type Error = ();
    fn try_from(i : u8) -> Result<LeetLevel,()> {
        match i {
            1 => { Ok(LeetLevel::One) }
            2 => { Ok(LeetLevel::Two) }
            3 => { Ok(LeetLevel::Three) }
            4 => { Ok(LeetLevel::Four) }
            5 => { Ok(LeetLevel::Five) }
            6 => { Ok(LeetLevel::Six) }
            7 => { Ok(LeetLevel::Seven) }
            8 => { Ok(LeetLevel::Eight) }
            9 => { Ok(LeetLevel::Nine) }
            _ => { Err(()) }
        }
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "UseLeet")]
pub(crate) enum UseLeetWhenGenerating {
    NotAtAll,
    Before {
        level : LeetLevel,
    },
    After {
        level : LeetLevel,
    },
    BeforeAndAfter {
        level : LeetLevel,
    },
}

impl UseLeetWhenGenerating {
    pub(crate) fn get_leet_level(&self) -> Option<LeetLevel> {
        match self {
            UseLeetWhenGenerating::NotAtAll => { None }
            UseLeetWhenGenerating::Before { level }
            | UseLeetWhenGenerating::After { level }
            | UseLeetWhenGenerating::BeforeAndAfter { level } => { Some(*level) }
        }
    }
    pub(crate) fn set_leet_level(&mut self, lvl : LeetLevel) -> Result<(),()> {
        match self {
            UseLeetWhenGenerating::NotAtAll => { Err(()) }
            UseLeetWhenGenerating::Before { level } 
            | UseLeetWhenGenerating::After { level }
            | UseLeetWhenGenerating::BeforeAndAfter { level} => { *level = lvl; Ok(()) }
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Clone)]
pub(super) struct UrlParsingSettings {
    pub(super) use_protocol : bool,
    pub(super) use_userinfo : bool,
    pub(super) use_subdomains : bool,
    pub(super) use_domain : bool,
    pub(super) use_port_path : bool,
    pub(super) use_undefined_as_protocol_fallback : bool
}

impl std::default::Default for UrlParsingSettings {
    fn default() -> Self {
        Self { 
            use_protocol: false, 
            use_userinfo : false, 
            use_subdomains: false, 
            use_domain: true, 
            use_port_path: false , 
            use_undefined_as_protocol_fallback: true
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct GenerationSettings {
    pub(crate) password_length : u32,
    pub(crate) username : String,
    pub(crate) modifier : String,
    pub(crate) characters : String,
    pub(crate) prefix : String,
    pub(crate) suffix : String,
    pub(crate) hash_algorithm : HashAlgorithm,
    pub(crate) leet : UseLeetWhenGenerating,
}

impl std::default::Default for GenerationSettings {
    fn default() -> Self {
        Self { 
            leet : UseLeetWhenGenerating::NotAtAll,
            hash_algorithm : HashAlgorithm::Md5,
            password_length : 8,
            username : String::new(),
            modifier : String::new(),
            characters : String::from(
                r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\:";'<>?,./"#
                ),
            prefix : String::new(),
            suffix : String::new(),
         }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct ProfileSettings {
    generation_settings : Arc<GenerationSettings>,
    url_parsing_settings : UrlParsingSettings,
}

#[derive(Serialize, Deserialize)]
struct Profile {
    name : String,
    settings : ProfileSettings,
}

#[derive(Serialize, Deserialize)]
struct StoredProfiles {
    profiles : Vec<Profile>,
}

impl std::default::Default for StoredProfiles {
    fn default() -> Self {
        Self {
            profiles : vec![Profile { 
                name : String::from("Default"),
                settings : ProfileSettings::default() 
            }]
        }
    }
}

impl StoredProfiles {
    fn load() -> Result<Self, LoadError> {
        super::get_config_folder()
            .map(|p| p.join("profiles"))
            .ok_or(LoadError::Xdg)
            .and_then(|p| std::fs::read_to_string(p).map_err(Into::into))
            .and_then(|s| toml::from_str(&s).map_err(Into::into))
    }

    fn store(&self) -> Result<(), StoreError> {
        toml::to_string(self).map_err(Into::into)
            .and_then(|s| Self::write_serialized_profile_data(&s))
    }
    fn write_serialized_profile_data(data : &str) -> Result<(), StoreError> {
        super::get_config_folder()
            .ok_or(StoreError::Xdg)
            .and_then(|p| std::fs::create_dir_all(p.clone()).map_err(Into::into).map(|()| p))
            .map(|p| p.join("profiles"))
            .and_then(|f| std::fs::write(f, data).map_err(Into::into))
    }
}

pub struct Profiles {
    emit : ProfilesEmitter,
    model : ProfilesList,

    data : RefCell<StoredProfiles>,

    current_profile_idx : RefCell<usize>,
}

#[derive(Clone, Debug)]
pub(crate) enum ProfileAccessError {
    CurrentProfileOutOfBounds
}
impl Display for ProfileAccessError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileAccessError::CurrentProfileOutOfBounds => write!(f, "error reading current profile, please reselect profile")
        }
    }
}
impl std::error::Error for ProfileAccessError{}

impl Profiles {
    fn notify_current_profile_changed(&self) {
        self.emit().current_profile_changed();
        self.emit().current_profile_name_changed();
    }

    fn model(&self) -> &ProfilesList {
        #[cfg(debug_assertions)]
        assert!(!self.is_any_borrowed());
        &self.model
    }

    pub(super) fn do_with_current_url_parsing_settings<F, R>(&self, operation : F) -> Result<R, ProfileAccessError>
        where F : FnOnce(&UrlParsingSettings) -> R
    {
        self.data.borrow().profiles.get(*self.current_profile_idx.borrow())
            .map(|p| &p.settings.url_parsing_settings).map(operation)
            .ok_or(ProfileAccessError::CurrentProfileOutOfBounds)
    }

    pub(crate) fn get_copy_current_generation_settings(&self) -> Result<Arc<GenerationSettings>, ProfileAccessError> {
        self.data.borrow().profiles.get(*self.current_profile_idx.borrow())
            .map(|p| p.settings.generation_settings.clone())
            .ok_or(ProfileAccessError::CurrentProfileOutOfBounds)
    }

    #[cfg(debug_assertions)]
    fn is_any_borrowed(&self) -> bool {
        self.data.try_borrow_mut().is_err() ||
            self.current_profile_idx.try_borrow_mut().is_err()
    }
}

impl ProfilesTrait for Profiles {
    fn new(emit : ProfilesEmitter, model : ProfilesList) -> Self {
        let data = RefCell::new(StoredProfiles::load().unwrap_or_default());
        Profiles { emit, model, data, current_profile_idx : RefCell::new(0) }
    }
    fn emit(&self) -> &ProfilesEmitter {
        #[cfg(debug_assertions)]
        assert!(!self.is_any_borrowed());
        &self.emit
    }

    fn row_count(&self) -> usize {
        self.data.borrow().profiles.len()
    }
    fn insert_rows(&self, row : usize, count : usize) -> bool {
        let max_index = self.row_count();
        let valid_index = row <= self.data.borrow().profiles.len();
        if valid_index {
            self.model().begin_insert_rows(row, row + count - 1);
            let new_profiles = (0..count)
                .map(|c| format!("New Profile {}", max_index + c))
                .map(|name| Profile { name, settings : ProfileSettings::default() });
            self.data.borrow_mut().profiles.splice(row..row,new_profiles);
            self.model().end_insert_rows();
            //if the current profile is at or beyond row, we need to update it, so it still points
            //to the same profile
            if *self.current_profile_idx.borrow() >= row {
                *self.current_profile_idx.borrow_mut() += count;
                //no need to notify about field changes - those are still the same. Only the
                //internal index changed.
                self.emit().current_profile_changed();
            }
        }
        valid_index
    }
    fn remove_rows(&self, row : usize, count : usize) -> bool {
        let valid_index = row + count <= self.row_count();
        let at_least_one_left = self.row_count() > count;
        let operation_allowed = valid_index && at_least_one_left;
        if operation_allowed {
            self.model().begin_remove_rows(row, row + count - 1);
            self.data.borrow_mut().profiles.drain(row..(row+count));
            //we have to update the current profile index before we complete the row removal.
            //this is to ensure the index is not going invalid.
            //here we can have a destructive operation: the current index might have been removed.
            //in that case set it to 0.
            let current_idx = *self.current_profile_idx.borrow();
            if row + count <= current_idx {
                *self.current_profile_idx.borrow_mut() -= count;
                //same as in the insert_rows case here: The field values are still the same, only
                //the index changed.
                self.emit().current_profile_changed();
            }
            else if row <= current_idx {
                *self.current_profile_idx.borrow_mut() = 0;
                self.notify_current_profile_changed();
            }
            self.model().end_remove_rows();
        }
        operation_allowed
    }

    fn characters<F : FnOnce(&str)>(&self, index : usize, setter : F) {
        setter(self.data.borrow().profiles.get(index)
            .map(|p| &*p.settings.generation_settings.characters)
            .unwrap_or_default());
    }
    fn set_characters(&self, index : usize, val : String) -> bool {
        if let Some(chars) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut Arc::make_mut(&mut p.settings.generation_settings).characters)
            .filter(|c| **c != val) {
                *chars = val;
                true
        } else {
            false
        }
    }
    fn hash_algorithm(&self, index : usize) -> u8 {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.generation_settings.hash_algorithm.into())
            .unwrap_or_default()
    }
    fn set_hash_algorithm(&self, index : usize, val : u8) -> bool {
        let hash = val.try_into().ok();
        let mut borrow = self.data.borrow_mut();
        let profile = borrow.profiles.get_mut(index);
        if let Some((p, h)) = profile.zip(hash)
            .map(|(p, h)| (&mut Arc::make_mut(&mut p.settings.generation_settings).hash_algorithm, h))
            .filter(|(p, h)| *p != h) {
                *p = h;
                true
        } else {
            false
        }
    }
    fn leet_level(&self, index : usize) -> u8 {
        self.data.borrow().profiles.get(index)
            .and_then(|p| p.settings.generation_settings.leet.get_leet_level())
            .map(Into::into)
            .unwrap_or_default()
    }
    fn set_leet_level(&self, index : usize, level : u8) -> bool {
        let level = level.try_into().ok();
        let mut borrow = self.data.borrow_mut();
        let profile = borrow.profiles.get_mut(index);
        if let Some((p, l)) = profile.zip(level)
            .map(|(p,l)| (&mut Arc::make_mut(&mut p.settings.generation_settings).leet, l))
            .filter(|(p,l)| p.get_leet_level() != Some(*l)) {
                p.set_leet_level(l).is_ok()
        } else {
            false
        }
    }
    fn modifier<F : FnOnce(&str)>(&self, index : usize, setter : F) {
        setter(self.data.borrow().profiles.get(index)
            .map(|p| &*p.settings.generation_settings.modifier)
            .unwrap_or_default());
    }
    fn set_modifier(&self, index : usize, val : String) -> bool {
        if let Some(m) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut Arc::make_mut(&mut p.settings.generation_settings).modifier)
            .filter(|m| **m != val) {
                *m = val;
                true
        } else {
            false
        }
    }
    fn name<F : FnOnce(&str)>(&self, index : usize, setter : F) {
        setter(self.data.borrow().profiles.get(index)
            .map_or("profile index invalid",|p| &*p.name));
    }
    fn set_name(&self, index : usize, val : String) -> bool {
        let changed = { 
            if let Some(n) = self.data.borrow_mut().profiles.get_mut(index)
                .map(|p| &mut p.name)
                .filter(|n| **n != val) {
                    *n = val;
                    true
            } else {
                false 
            }
        };
        if changed && index == *self.current_profile_idx.borrow() {
            self.emit().current_profile_name_changed();
        }
        changed
    }
    fn password_length(&self, index : usize) -> u32 {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.generation_settings.password_length)
            .unwrap_or_default()
    }
    fn set_password_length(&self, index : usize, len : u32) -> bool {
        if let Some(pl) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut Arc::make_mut(&mut p.settings.generation_settings).password_length)
            .filter(|pl| **pl != len) {
                *pl = len;
                true
        } else {
            false
        }
    }
    fn prefix<F : FnOnce(&str)>(&self, index : usize, setter : F) {
        setter(self.data.borrow().profiles.get(index)
            .map(|p| &*p.settings.generation_settings.prefix)
            .unwrap_or_default());
    }
    fn set_prefix(&self, index : usize, pref : String) -> bool {
        if let Some(p) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut Arc::make_mut(&mut p.settings.generation_settings).prefix)
            .filter(|p| **p != pref) {
                *p = pref;
                true
        } else {
            false
        }
    }
    fn suffix<F : FnOnce(&str)>(&self, index : usize, setter : F) {
        setter(self.data.borrow_mut().profiles.get(index)
            .map(|p| &*p.settings.generation_settings.suffix)
            .unwrap_or_default());
    }
    fn set_suffix(&self, index : usize, suf : String) -> bool {
        if let Some(s) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut Arc::make_mut(&mut p.settings.generation_settings).suffix)
            .filter(|s| **s != suf) {
                *s=suf;
                true
        } else {
            false
        }
    }
    fn use_domain(&self, index : usize) -> bool {
        self.data.borrow().profiles.get(index)
            .map_or(true,|p| p.settings.url_parsing_settings.use_domain)
    }
    fn set_use_domain(&self, index : usize, val : bool) -> bool {
        if let Some(d) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut p.settings.url_parsing_settings.use_domain)
            .filter(|d| **d != val) {
                *d = val;
                true
        } else {
            false
        }
    }
    fn use_leet(&self, index : usize) -> u8 {
        self.data.borrow().profiles.get(index)
            .map(|p| match p.settings.generation_settings.leet {
                UseLeetWhenGenerating::NotAtAll => { 0 }
                UseLeetWhenGenerating::Before{..} => { 1 }
                UseLeetWhenGenerating::After {..} => { 2 }
                UseLeetWhenGenerating::BeforeAndAfter{..} => { 3 }
            })
            .unwrap_or_default()
    }
    fn set_use_leet(&self, index : usize, l : u8) -> bool {
        let mut borrow = self.data.borrow_mut();
        let profile = borrow.profiles.get_mut(index);
        let level = |profile : Option<&Profile>| {
            profile.as_ref().and_then(|p| p.settings.generation_settings.leet.get_leet_level())
            .unwrap_or(LeetLevel::One)
        };
        let use_leet = match l {
            0 => { Some(UseLeetWhenGenerating::NotAtAll) }
            1 => { Some(UseLeetWhenGenerating::Before { level : level(profile.as_deref()) }) }
            2 => { Some(UseLeetWhenGenerating::After { level : level(profile.as_deref()) }) }
            3 => { Some(UseLeetWhenGenerating::BeforeAndAfter{ level : level(profile.as_deref()) }) }
            _ => { None }
        };
        if let Some((p, l)) = profile.zip(use_leet)
            .map(|(p,l)| (&mut Arc::make_mut(&mut p.settings.generation_settings).leet, l))
            .filter(|(p,l)| *p != l) {
                *p = l;
                true
        } else {
            false
        }
    }
    fn use_port_path(&self, index : usize) -> bool {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.url_parsing_settings.use_port_path)
            .unwrap_or_default()
    }
    fn set_use_port_path(&self, index : usize, val : bool) -> bool {
        if let Some(pp) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut p.settings.url_parsing_settings.use_port_path)
            .filter(|pp| **pp != val) {
                *pp = val;
                true
        } else {
            false
        }
    }
    fn use_protocol(&self, index : usize) -> bool {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.url_parsing_settings.use_protocol)
            .unwrap_or_default()
    }
    fn set_use_protocol(&self, index : usize, val : bool) -> bool {
        if let Some(pr) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut p.settings.url_parsing_settings.use_protocol)
            .filter(|pr| **pr != val) {
                *pr = val;
                true
        } else {
            false
        }
    }
    fn use_subdomains(&self, index : usize) -> bool {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.url_parsing_settings.use_subdomains)
            .unwrap_or_default()
    }
    fn set_use_subdomains(&self, index : usize, val : bool) -> bool {
        if let Some(sd) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut p.settings.url_parsing_settings.use_subdomains)
            .filter(|sd| **sd != val) {
                *sd = val;
                true
        } else {
            false
        }
    }
    fn username<F : FnOnce(&str)>(&self, index : usize, setter : F) {
        setter(self.data.borrow().profiles.get(index)
            .map(|p| &*p.settings.generation_settings.username)
            .unwrap_or_default());
    }
    fn set_username(&self, index : usize, name : String) -> bool {
        if let Some(u) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut Arc::make_mut(&mut p.settings.generation_settings).username)
            .filter(|u| **u != name) {
                *u = name;
                true
        } else {
            false
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn current_profile(&self) -> u32 {
        *self.current_profile_idx.borrow() as u32
    }
    fn set_current_profile(&self, value : u32) {
        let valid_index = self.row_count() > value as usize;
        let does_change = *self.current_profile_idx.borrow() != value as usize;
        if valid_index && does_change {
            *self.current_profile_idx.borrow_mut() = value as usize;
            self.notify_current_profile_changed();
        }
    }
    fn current_profile_name<F : FnOnce(&str)>(&self, setter : F) {
        self.name(*self.current_profile_idx.borrow(), setter);
    }

    fn store(&self) -> bool {
        self.data.borrow().store()
            .map_err(|e| println!("{}", e))
            .is_ok()
    }

    fn use_undefined_as_protocol_fallback(&self, index: usize) -> bool {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.url_parsing_settings.use_undefined_as_protocol_fallback)
            .unwrap_or_default()
    }

    fn set_use_undefined_as_protocol_fallback(&self, index: usize, val: bool) -> bool {
        if let Some(sd) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut p.settings.url_parsing_settings.use_undefined_as_protocol_fallback)
            .filter(|sd| **sd != val) {
                *sd = val;
                true
        } else {
            false
        }
    }

    fn use_user_info(&self, index: usize) -> bool {
        self.data.borrow().profiles.get(index)
            .map(|p| p.settings.url_parsing_settings.use_userinfo)
            .unwrap_or_default()
    }

    fn set_use_user_info(&self, index: usize, val: bool) -> bool {
        if let Some(sd) = self.data.borrow_mut().profiles.get_mut(index)
            .map(|p| &mut p.settings.url_parsing_settings.use_userinfo)
            .filter(|sd| **sd != val) {
                *sd = val;
                true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod profiles_tests {
    use super::*;
    fn get_test_profiles_three_entries(emit : ProfilesEmitter, model : ProfilesList) -> Profiles {
        Profiles { 
            emit,
            model,
            data : RefCell::new( StoredProfiles {
                profiles : vec![
                    Profile {
                        name : String::from("First"),
                        settings : ProfileSettings::default(),
                    },
                    Profile {
                        name : String::from("Second"),
                        settings : ProfileSettings::default(),
                    },
                    Profile {
                        name : String::from("Third"),
                        settings : ProfileSettings::default(),
                    },
                ],
            }),
            current_profile_idx : RefCell::new(1),
        }
    }
    #[test]
    fn hash_algo_reciprocicity_test() {
        for i in 0..(HashAlgorithm::variant_count() as u8) {
            let x : HashAlgorithm = i.try_into().unwrap();
            let y = x.into();
            assert_eq!(i,y);
        }
        let z: Result<HashAlgorithm,_> = (HashAlgorithm::variant_count() as u8).try_into();
        assert!(z.is_err());
    }
    #[test]
    fn leet_level_reciprocicity_test() {
        for i in 1..=(LeetLevel::variant_count() as u8) {
            let x : LeetLevel = i.try_into().unwrap();
            let y = x.into();
            assert_eq!(i,y);
        }
        let z : Result<LeetLevel,_> = (1+LeetLevel::variant_count() as u8).try_into();
        assert!(z.is_err());
        let a : Result<LeetLevel,_> = 0.try_into();
        assert!(a.is_err());
    }
    #[test]
    fn set_name_test() {
        let (emit_nothing, model_doesnt_emit) = expect_no_emission();
        let profiles_assert_on_emit = get_test_profiles_three_entries(emit_nothing, model_doesnt_emit);
        //profiles_assert_on_emit has profile 1 set as active. We can therefore modify the name of 0 (or 2) without emission.
        let should_change = profiles_assert_on_emit.set_name(0, "SomethingIJustMadeUp".into());
        assert!(should_change);
        profiles_assert_on_emit.name(0, |x| {assert_eq!(x, "SomethingIJustMadeUp")});
    }
    #[test]
    fn set_current_profile_test() {
        {
            //setting to same index should not do anything.
            let (emit_nothing, model_doesnt_emit) = expect_no_emission();
            let profiles_assert_on_emit = get_test_profiles_three_entries(emit_nothing, model_doesnt_emit);
            profiles_assert_on_emit.set_current_profile(profiles_assert_on_emit.current_profile());
        }
        {
            //setting to different index should emit both name and index changed.
            let mut emit = ProfilesEmitter::new();
            emit.expect_clone().never();
            emit.expect_current_profile_changed().return_once(||{()}).once();
            emit.expect_current_profile_name_changed().return_once(||{()}).once();
            emit.expect_new_data_ready().never();
            let mut list = ProfilesList::new();
            list.expect_layout_about_to_be_changed().never();
            list.expect_layout_changed().never();
            list.expect_data_changed().never();
            list.expect_begin_reset_model().never();
            list.expect_end_reset_model().never();
            list.expect_begin_insert_rows().never();
            list.expect_end_insert_rows().never();
            list.expect_begin_move_rows().never();
            list.expect_end_move_rows().never();
            list.expect_begin_remove_rows().never();
            list.expect_end_remove_rows().never();
            let profiles_wants_one_name_change = get_test_profiles_three_entries(emit, list);
            let previous_profile = profiles_wants_one_name_change.current_profile();
            profiles_wants_one_name_change.set_current_profile(previous_profile + 1);
            assert_eq!(previous_profile + 1, profiles_wants_one_name_change.current_profile());
        }
        {
            //Last, but not least, even if the _name_ of the profiles is the same, we WANT to emit current_profile_name_changed.
            //This is because the UI is also functioning as Controller, so every profile change (even if it's the same name) should refresh the
            //corresponding UI.

            let mut emit = ProfilesEmitter::new();
            emit.expect_clone().never();
            emit.expect_current_profile_changed().return_once(||{()}).once();
            emit.expect_current_profile_name_changed().return_once(||{()}).once();
            emit.expect_new_data_ready().never();
            let mut list = ProfilesList::new();
            list.expect_layout_about_to_be_changed().never();
            list.expect_layout_changed().never();
            list.expect_data_changed().never();
            list.expect_begin_reset_model().never();
            list.expect_end_reset_model().never();
            list.expect_begin_insert_rows().never();
            list.expect_end_insert_rows().never();
            list.expect_begin_move_rows().never();
            list.expect_end_move_rows().never();
            list.expect_begin_remove_rows().never();
            list.expect_end_remove_rows().never();
            let p = Profiles { 
                emit,
                model :list,
                data : RefCell::new( StoredProfiles {
                    profiles : vec![
                        Profile {
                            name : String::from("Twin"),
                            settings : ProfileSettings::default(),
                        },
                        Profile {
                            name : String::from("Twin"),
                            settings : ProfileSettings::default(),
                        },
                    ],
                }),
                current_profile_idx : RefCell::new(1),
            };
            p.set_current_profile(0);
            assert_eq!(p.current_profile(), 0);
        }
    }
    #[test]
    fn current_profile_name_test() {
        //let's check if setting the same value we already have emits. Hopefully not. Set the index 1 profile's name to "Second"
        {
            let (emit_nothing, model_doesnt_emit) = expect_no_emission();
            let profiles_assert_on_emit = get_test_profiles_three_entries(emit_nothing, model_doesnt_emit);
            let should_not_change = profiles_assert_on_emit.set_name(1,"Second".into());
            assert!(!should_not_change);
        }
        
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().never();
        emit.expect_current_profile_name_changed().return_once(||{()}).once();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().never();
        list.expect_end_insert_rows().never();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().never();
        list.expect_end_remove_rows().never();
        let profiles_wants_one_name_change = get_test_profiles_three_entries(emit, list);
        let should_change2 = profiles_wants_one_name_change.set_name(1, "Agadlagugu".into());
        assert!(should_change2);
        profiles_wants_one_name_change.name(1, |x| {assert_eq!(x, "Agadlagugu")});
    }
    #[test]
    fn insert_rows_before_current_test() {
        //if we add a row before the current, only, current_profile should emit. No need to emit profile_name, as the data is still the same.
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().return_once(||{()}).once();
        emit.expect_current_profile_name_changed().never();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().return_once(|_,_|{()}).once();
        list.expect_end_insert_rows().return_once(||{()}).once();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().never();
        list.expect_end_remove_rows().never();
        //must make sure that index changes, but data remains same
        let profile_tests_insert_before_current = get_test_profiles_three_entries(emit, list);
        let get_current_name_copy = || { let mut res = String::new(); profile_tests_insert_before_current.current_profile_name(|x| res = String::from(x)); res};
        let old_name = get_current_name_copy();
        let old_index = profile_tests_insert_before_current.current_profile();
        let worked = profile_tests_insert_before_current.insert_rows(1, 1);
        assert!(worked);
        let new_name = get_current_name_copy();
        let new_index = profile_tests_insert_before_current.current_profile();
        assert_eq!(old_name, new_name);
        assert_eq!(new_index, old_index +1);
    }
    #[test]
    fn insert_rows_after_current_test() {
        //if we add a row after current, no signals other than insert_rows related ones should emit:
        //if we add a row before the current, only, current_profile should emit. No need to emit profile_name, as the data is still the same.
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().never();
        emit.expect_current_profile_name_changed().never();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().return_once(|_,_|{()}).once();
        list.expect_end_insert_rows().return_once(||{()}).once();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().never();
        list.expect_end_remove_rows().never();
        //must make sure that index changes, but data remains same
        let profile_tests_insert_before_current = get_test_profiles_three_entries(emit, list);
        let get_current_name_copy = || { let mut res = String::new(); profile_tests_insert_before_current.current_profile_name(|x| res = String::from(x)); res};
        let old_name = get_current_name_copy();
        let old_index = profile_tests_insert_before_current.current_profile();
        let worked = profile_tests_insert_before_current.insert_rows(3, 1);
        assert!(worked);
        let new_name = get_current_name_copy();
        let new_index = profile_tests_insert_before_current.current_profile();
        assert_eq!(old_name, new_name);
        assert_eq!(new_index, old_index);
    }
    #[test]
    fn remove_rows_before_current_test() {
        //if we remove a row before the current, only current_profile should emit. No need to emit profile_name, as the data is still the same.
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().return_once(||{()}).once();
        emit.expect_current_profile_name_changed().never();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().never();
        list.expect_end_insert_rows().never();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().return_once(|_,_|{()}).once();
        list.expect_end_remove_rows().return_once(||{()}).once();
        //must make sure that index changes, but data remains same
        let profile_tests_remove_before_current = get_test_profiles_three_entries(emit, list);
        let get_current_name_copy = || { let mut res = String::new(); profile_tests_remove_before_current.current_profile_name(|x| res = String::from(x)); res};
        let old_name = get_current_name_copy();
        let old_index = profile_tests_remove_before_current.current_profile();
        let worked = profile_tests_remove_before_current.remove_rows(0, 1);
        assert!(worked);
        let new_name = get_current_name_copy();
        let new_index = profile_tests_remove_before_current.current_profile();
        assert_eq!(old_name, new_name);
        assert_eq!(new_index, old_index - 1);
    }
    #[test]
    fn remove_rows_containign_current_test() { 
        //if we remove the current row, both, index and name will change and index should be zero.
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().return_once(||{()}).once();
        emit.expect_current_profile_name_changed().return_once(||{()}).once();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().never();
        list.expect_end_insert_rows().never();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().return_once(|_,_|{()}).once();
        list.expect_end_remove_rows().return_once(||{()}).once();
        //must make sure that index changes, but data remains same
        let profile_tests_remove_current = get_test_profiles_three_entries(emit, list);
        let get_current_name_copy = || { let mut res = String::new(); profile_tests_remove_current.current_profile_name(|x| res = String::from(x)); res};
        let old_name = get_current_name_copy();
        let old_index = profile_tests_remove_current.current_profile();
        let worked = profile_tests_remove_current.remove_rows(1, 1);
        assert!(worked);
        let new_name = get_current_name_copy();
        let new_index = profile_tests_remove_current.current_profile();
        assert_ne!(old_name, new_name);
        assert_eq!(new_index, 0);
        assert_ne!(new_index, old_index);
    }
    #[test]
    fn remove_rows_after_current_test() {
        //only remove rows emit, no profile related emit
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().never();
        emit.expect_current_profile_name_changed().never();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().never();
        list.expect_end_insert_rows().never();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().return_once(|_,_|{()}).once();
        list.expect_end_remove_rows().return_once(||{()}).once();
        //must make sure that index changes, but data remains same
        let profile_tests_remove_after_current = get_test_profiles_three_entries(emit, list);
        let get_current_name_copy = || { let mut res = String::new(); profile_tests_remove_after_current.current_profile_name(|x| res = String::from(x)); res};
        let old_name = get_current_name_copy();
        let old_index = profile_tests_remove_after_current.current_profile();
        let worked = profile_tests_remove_after_current.remove_rows(2, 1);
        assert!(worked);
        let new_name = get_current_name_copy();
        let new_index = profile_tests_remove_after_current.current_profile();
        assert_eq!(old_name, new_name);
        assert_eq!(new_index, old_index);
    }
    // Leet level needs its own test, as setting leet level depends on use_leet.
    #[test]
    fn set_leet_level_test() {
        let (emit, model) = expect_no_emission();
        //we're not going through the new() method, to avoid touching the filesystem.
        let profiles = get_test_profiles_three_entries(emit, model);

        //remember the old values in other rows to confimr they aren't touched
        let oldvals = [profiles.leet_level(0), profiles.leet_level(2)];

        //make sure that leet isn't enabled.
        assert_eq!(profiles.use_leet(1), u8::default());
        //make sure we are changing something.
        assert_ne!(profiles.leet_level(1), 2);
        //assure that nothing happens if we try to change leet level with leet disabled.
        let changed = profiles.set_leet_level(1,2);
        assert!(!changed);
        assert_eq!(profiles.use_leet(1), u8::default());

        //enable leet.
        let changed = profiles.set_use_leet(1,1);
        assert!(changed);
        //make sure we are changing something.
        assert_ne!(profiles.leet_level(1), 2);
        //actually change something:
        let changed = profiles.set_leet_level(1,2);
        assert!(changed);
        assert_eq!(profiles.leet_level(1), 2);
        //check that other values are unaffected
        assert_eq!(profiles.leet_level(0), oldvals[0]);
        assert_eq!(profiles.leet_level(2), oldvals[1]);
        //IMPORTANT: check that assigning the same value returns not-changed.
        let changed = profiles.set_leet_level(1,2);
        assert!(!changed);
        //disable leet again and make sure leet_level returns 0 again.
        let changed = profiles.set_use_leet(1,0);
        assert!(changed);
        assert_eq!(profiles.leet_level(1), u8::default());
    }

    // The remaining properties should (at the time of writing) not emit.
    // This means we can use one and the same test for all of them :-)
    // Except that complex types have a different getter syntax...
    // Oh well.
    fn expect_no_emission() -> (ProfilesEmitter, ProfilesList) {
        let mut emit = ProfilesEmitter::new();
        emit.expect_clone().never();
        emit.expect_current_profile_changed().never();
        emit.expect_current_profile_name_changed().never();
        emit.expect_new_data_ready().never();
        let mut list = ProfilesList::new();
        list.expect_layout_about_to_be_changed().never();
        list.expect_layout_changed().never();
        list.expect_data_changed().never();
        list.expect_begin_reset_model().never();
        list.expect_end_reset_model().never();
        list.expect_begin_insert_rows().never();
        list.expect_end_insert_rows().never();
        list.expect_begin_move_rows().never();
        list.expect_end_move_rows().never();
        list.expect_begin_remove_rows().never();
        list.expect_end_remove_rows().never();
        (emit, list)
    }
    macro_rules! simple_setter_tests {
        ( $( $test_name:ident, $setter:ident, $getter:ident, $val:literal ),* ) => {
            $(#[test]
                fn $test_name() {
                    let (emit, model) = expect_no_emission();
                    //we're not going through the new() method, to avoid touching the filesystem.
                    let profiles = get_test_profiles_three_entries(emit, model);

                    //remember the old values in other rows to confimr they aren't touched
                    let oldvals = [profiles.$getter(0), profiles.$getter(2)];

                    //make sure we are changing something.
                    assert_ne!(profiles.$getter(1), $val);
                    //assure that something changed
                    let changed = profiles.$setter(1,$val);
                    assert!(changed);
                    assert_eq!(profiles.$getter(1), $val);
                    //check that other values are unaffected
                    assert_eq!(profiles.$getter(0), oldvals[0]);
                    assert_eq!(profiles.$getter(2), oldvals[1]);
                    //IMPORTANT: check that assigning the same value returns not-changed.
                    let changed = profiles.$setter(1,$val);
                    assert!(!changed);
            })*
        };
    }
    macro_rules! string_setter_tests {
        ( $( $test_name:ident, $setter:ident, $getter:ident, $val:literal ),* ) => {
            $(#[test]
                fn $test_name() {
                    let (emit, model) = expect_no_emission();
                    //we're not going through the new() method, to avoid touching the filesystem.
                    let profiles = get_test_profiles_three_entries(emit, model);

                    //remember the old values in other rows to confirm they aren't touched.
                    let mut oldvals = [String::new(), String::new()];
                    profiles.$getter(0, |x| { oldvals[0] = String::from(x); });
                    profiles.$getter(2, |x| { oldvals[1] = String::from(x); });
                    //make sure we are changing something.
                    profiles.$getter(1, |x| { assert_ne!(x, $val); });
                    //assure that something changed:
                    let changed = profiles.$setter(1,String::from($val));
                    assert!(changed);
                    profiles.$getter(1, |x| { assert_eq!(x, $val); });
                    //check that the other values are unaffected.
                    profiles.$getter(0, |x| { assert_eq!(oldvals[0], x); });
                    profiles.$getter(2, |x| { assert_eq!(oldvals[1], x); });
                    //IMPORTANT: check that assigning the same value returns not-changed.
                    let changed = profiles.$setter(1,String::from($val));
                    assert!(!changed);
            })*
        };
    }
    string_setter_tests!(
        set_characters_test,set_characters,characters,"ABCDEFG",
        set_modifier_test, set_modifier, modifier, "ab",
        set_prefix_test, set_prefix, prefix, "xy",
        set_suffix_test, set_suffix, suffix, "zw",
        set_username_test, set_username, username, "Zargothrax"
    );
    simple_setter_tests!(
        set_hash_algorithm_test, set_hash_algorithm, hash_algorithm, 5,
        set_password_length_test, set_password_length, password_length, 12,
        set_use_domain_test, set_use_domain, use_domain, false,
        set_use_leet_test, set_use_leet, use_leet, 1,
        set_use_port_path_test, set_use_port_path, use_port_path, true,
        set_use_protocol_test, set_use_protocol, use_protocol, true,
        set_use_subdomains_test, set_use_subdomains, use_subdomains, true,
        set_use_user_info_test, set_use_user_info, use_user_info, true,
        set_use_undefined_as_protocol_fallback_test, set_use_undefined_as_protocol_fallback, use_undefined_as_protocol_fallback, false
    );
}
