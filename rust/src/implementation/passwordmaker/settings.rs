use crate::interface::SettingsTrait;
use crate::implementation::{LoadError, StoreError};

use serde::{Serialize, Deserialize};
use std::default::Default;
use std::cell::RefCell;

use mockall_double::double;

#[double]
use crate::interface::SettingsEmitter;

#[derive(Serialize, Deserialize)]
struct SettingsSaveData {
    current_profile_index : usize,
    settings : SettingsData
}

#[derive(Serialize, Deserialize, Clone)]
struct SettingsData{
    clear_generated_password_seconds : Option<u32>,
    clear_master_password_seconds : Option<u32>,
    hide_generated_password : bool
}

impl Default for SettingsData {
    fn default() -> Self {
        SettingsData {
            clear_generated_password_seconds : Some(60),
            clear_master_password_seconds : Some(300),
            hide_generated_password : false
        }
    }
}

pub struct Settings {
    emit : SettingsEmitter,
    data : RefCell<SettingsData>
}

pub struct SettingLoadResult {
    pub settings : Settings,
    pub current_profile_index : usize
}

impl Settings {
    /// Loads the data from disk and merges them with the current Settings object. Only use during startup, as it does NOT emit.
    pub(super) fn load(&self) -> Result<SettingLoadResult, LoadError>{
         super::super::get_config_folder()
            .map(|p| p.join("settings"))
            .ok_or(LoadError::Xdg)
            .and_then(|p| std::fs::read_to_string(p).map_err(Into::into))
            .and_then(|s| toml::from_str(&s).map_err(Into::into))
            .map(|sd| self.merge_with_loaded_data_no_emit(sd))
    }
    pub(super) fn store(&self, current_profile_index : usize) -> Result<(), StoreError>{
        toml::to_string(&SettingsSaveData{current_profile_index, settings : self.data.borrow().clone()})
            .map_err(Into::into)
            .and_then(|s| Self::write_serialized_settings_data(&s))
    }

    ///Private. Should only be used during load().
    fn merge_with_loaded_data_no_emit(&self, save_data: SettingsSaveData) -> SettingLoadResult {
        SettingLoadResult {
            settings : Settings { emit : self.emit.clone(), data : RefCell::new(save_data.settings) },
            current_profile_index : save_data.current_profile_index
        }
    }
    fn write_serialized_settings_data(data : &str) -> Result<(), StoreError> {
        super::super::get_config_folder()
            .ok_or(StoreError::Xdg)
            .and_then(|p| std::fs::create_dir_all(p.clone()).map_err(Into::into).map(|()| p))
            .map(|p| p.join("settings"))
            .and_then(|f| std::fs::write(f, data).map_err(Into::into))
    }
}

impl SettingsTrait for Settings {
    fn new(emit: SettingsEmitter) -> Self {
        Settings {emit, data : RefCell::new(SettingsData::default())}
    }
    fn emit(&self) -> &SettingsEmitter{
        &self.emit
    }
    fn clear_generated_password_seconds(&self) -> Option<u32> {
        self.data.borrow().clear_generated_password_seconds
    }
    fn set_clear_generated_password_seconds(&self, value: Option<u32>) {
        let changed = self.data.borrow().clear_generated_password_seconds != value;
        self.data.borrow_mut().clear_generated_password_seconds = value;
        if changed {
            self.emit().clear_generated_password_seconds_changed();
        }
    }
    fn clear_master_password_seconds(&self) -> Option<u32> {
        self.data.borrow().clear_master_password_seconds
    }
    fn set_clear_master_password_seconds(&self, value: Option<u32>) {
        let changed = self.data.borrow().clear_master_password_seconds != value;
        self.data.borrow_mut().clear_master_password_seconds = value;
        if changed {
            self.emit().clear_master_password_seconds_changed();
        }
    }

    fn hide_generated_password(&self) -> bool {
        self.data.borrow().hide_generated_password
    }

    fn set_hide_generated_password(&self, value: bool) {
        let changed = {
            let mut v = self.data.borrow_mut();
            let changed = v.hide_generated_password != value;
            v.hide_generated_password = value;
            changed
        };
        if changed {
            self.emit().hide_generated_password_changed();
        }
    }
}

#[cfg(test)]
mod settings_test{
    use super::*;

    #[test]
    fn merge_with_loaded_data_test(){
        let mut emit = SettingsEmitter::new();
        emit.expect_clone().return_once(||{
            let mut e2 = SettingsEmitter::new();
            e2.expect_clone().never();
            e2.expect_clear_generated_password_seconds_changed().never();
            e2.expect_clear_master_password_seconds_changed().never();
            e2.expect_hide_generated_password_changed().never();
            e2
        }).once();
        emit.expect_clear_generated_password_seconds_changed().never();
        emit.expect_clear_master_password_seconds_changed().never();
        emit.expect_hide_generated_password_changed().never();
        let old_settings = Settings {
            emit,
            data: RefCell::new(SettingsData::default()),
        };
        let save_data = SettingsSaveData{
            current_profile_index: 2,
            settings: SettingsData {
                clear_generated_password_seconds: Some(600),
                clear_master_password_seconds: None,
                hide_generated_password: true
            },
        };
        let result = old_settings.merge_with_loaded_data_no_emit(save_data);
        assert_eq!(result.current_profile_index, 2);
        assert_eq!(result.settings.data.borrow().clear_generated_password_seconds, Some(600));
        assert_eq!(result.settings.data.borrow().clear_master_password_seconds, None);
        assert_eq!(result.settings.data.borrow().hide_generated_password, true);
    }
}