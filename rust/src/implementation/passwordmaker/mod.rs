mod emittingsender;
mod thread_messages;
mod settings;
mod helperthread;

pub use self::settings::Settings;
use passwordmaker_rs::{UrlParsing, ProtocolUsageMode, SettingsError, GenerationError};
use crate::interface::ProfilesTrait;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::implementation::pwm_macros::EnumVariantCount;

use mockall_double::double;

#[double]
use crate::interface::PasswordMakerEmitter;
use crate::interface::PasswordMakerTrait;
use super::Profiles;
use self::emittingsender::EmittingSender;
use self::thread_messages::{GeneratePasswordTask, HelperToUi, UiToHelper, GenerationIssue};

#[derive(EnumVariantCount, Clone, Copy)]
enum GeneratorState{
    MissingTextToUse,
    MissingMasterPassword,
    CharsetError,
    GenerationCompleted,
    Busy
}

pub struct PasswordMaker {
    emit : PasswordMakerEmitter,
    profiles : Profiles,
    settings : Settings,
    url : RefCell<String>,
    used_text : RefCell<String>,
    master_password : RefCell<String>,
    generated_password : RefCell<String>,
    generator_state : RefCell<GeneratorState>,
    from_helper : Receiver<HelperToUi>,
    to_helper : Sender<UiToHelper>,
    helper_thread : Option<std::thread::JoinHandle<()>>
}

#[derive(Debug)]
struct GeneratorStateFromIntError;
impl Display for GeneratorStateFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Invalid value for GeneratorStateFromIntError.")
    }
}
impl Error for GeneratorStateFromIntError{}

impl TryFrom<u8> for GeneratorState {
    type Error = GeneratorStateFromIntError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GeneratorState::GenerationCompleted),
            1 => Ok(GeneratorState::Busy),
            2 => Ok(GeneratorState::MissingTextToUse),
            3 => Ok(GeneratorState::MissingMasterPassword),
            4 => Ok(GeneratorState::CharsetError),
            5..=255 => Err(GeneratorStateFromIntError)
        }
    }
}
impl From<GeneratorState> for u8 {
    fn from(s : GeneratorState) -> Self {
        match s {
            GeneratorState::MissingTextToUse => 2,
            GeneratorState::GenerationCompleted => 0,
            GeneratorState::Busy => 1,
            GeneratorState::MissingMasterPassword => 3,
            GeneratorState::CharsetError => 4,
        }
    }
}

impl PasswordMakerTrait for PasswordMaker {
    fn new(emit : PasswordMakerEmitter, profiles : Profiles, settings : Settings) -> Self {
        let stored_settings_data = settings.load();
        if let Ok(x) = &stored_settings_data {
            #[allow(clippy::cast_possible_truncation)]
            profiles.set_current_profile(x.current_profile_index as u32);
        }
        let settings = if let Ok(x) = stored_settings_data { x.settings } else { settings };

        let emit_clone = emit.clone();
        let (to_ui,from_helper) = EmittingSender::emitting_channel(
            move || {emit_clone.i_say_sexy_things_to_myself_while_im_dancing_changed();}
        );
        let (to_helper, from_ui) = channel();
        let helper_thread = Some(std::thread::spawn(move || helperthread::run(&to_ui, &from_ui).expect("UI Thread hung up.") ));
        PasswordMaker{emit, profiles, settings, from_helper, to_helper, helper_thread,
             url: RefCell::new(String::new()), used_text: RefCell::new(String::new()), master_password: RefCell::new(String::new()), generated_password: RefCell::new(String::new()), generator_state: RefCell::new(GeneratorState::MissingTextToUse) }
    }
    fn emit(&self) -> &PasswordMakerEmitter {
        &self.emit
    }
    fn profiles(&self) -> &Profiles {
        &self.profiles
    }
    fn settings(&self) -> &Settings {
        &self.settings
    }
    fn store_settings(&self) -> bool {
        self.settings.store(self.profiles.current_profile() as usize).is_ok()
    }
    fn i_say_sexy_things_to_myself_while_im_dancing(&self) -> bool {
        unreachable!()
    }
    fn set_i_say_sexy_things_to_myself_while_im_dancing(&self, _ : bool) {
        if let Ok(m) = self.from_helper.try_recv() {
            self.handle_message_from_helper(m);
        } else {
            println!("Spurious wakeup from helper thread. No message queued. Please investigate.");
        }
    }

    fn generated_password<F>(&self, setter: F) where F: FnOnce(&str) {
        setter(&*self.generated_password.borrow());
    }

    fn master_password<F>(&self, setter: F) where F: FnOnce(&str) {
        setter(&*self.master_password.borrow());
    }

    fn set_master_password(&self, value: String) {
        let different = { 
            let mut b = self.master_password.borrow_mut();
            let different = *b != value;
            if different {*b = value;}
            different
        };
        if different {
            self.update_generated_password();
            self.emit().master_password_changed();
        }
    }

    fn url<F>(&self, setter: F) where F: FnOnce(&str) {
        setter(&*self.url.borrow());
    }

    fn set_url(&self, value: String) {
        let different = {
            let mut b = self.url.borrow_mut();
            let different = *b != value;
            if different { *b = value; }
            different
        };
        if different {
            self.update_used_text_from_url();
            self.emit().url_changed();
        }
    }

    fn used_text<F>(&self, setter: F) where F: FnOnce(&str) {
        setter(&*self.used_text.borrow());
    }

    fn set_used_text(&self, value: String) {
        let different = {
            let mut b = self.used_text.borrow_mut();
            let different = *b != value;
            if different { *b = value; }
            different
        };
        if different {
            self.update_generated_password();
            self.emit().used_text_changed();
        }
    }

    fn generator_state(&self) -> u8 {
        (*self.generator_state.borrow()).into()
    }

    fn profile_changed(&self) {
        self.update_used_text_from_url();
    }
}

impl Drop for PasswordMaker
{
    fn drop(&mut self) {
        if self.to_helper.send(UiToHelper::Shutdown).is_err() {
            eprintln!("Failed to tell worker thread to quit. Might need to kill the process.");
        }
        if let Some(j) = self.helper_thread.take() {
            if j.join().is_err() {
                eprintln!("Helper Thread crashed. This should not happen, so please report a bug.");
            }
        } else {
            eprintln!("Somehow the information about the helper thread got lost, so we can't wait for it to quit. Might need to kill the process.");
        }
    }
}

impl PasswordMaker{
    fn handle_message_from_helper(&self, message : HelperToUi){
        //unless everything is terribly wrong, we are in the UI thread here.
        match message {
            HelperToUi::Generated{ password } => {
                //println!("Password generated.");
                {
                    *self.generator_state.borrow_mut() = GeneratorState::GenerationCompleted;
                    *self.generated_password.borrow_mut() = password;
                }
                self.emit().generator_state_changed();
                self.emit().generated_password_changed();
            }
            HelperToUi::GenerationFailed { error  } => {
                //println!("No password generated due to missing input.");
                {
                    *self.generator_state.borrow_mut() = match error {
                        GenerationIssue::Settings(SettingsError::InsufficientCharset) => GeneratorState::CharsetError,
                        GenerationIssue::Input(GenerationError::MissingMasterPassword) => GeneratorState::MissingMasterPassword,
                        GenerationIssue::Input(GenerationError::MissingTextToUse) => GeneratorState::MissingTextToUse,
                    };
                    self.generated_password.borrow_mut().clear();
                }
                self.emit().generator_state_changed();
                self.emit().generated_password_changed();
            },
            HelperToUi::GenerationStarted => {
                //println!("Setting password generator state as busy.");
                {
                    *self.generator_state.borrow_mut() = GeneratorState::Busy;
                    self.generated_password.borrow_mut().clear();
                }
                self.emit().generator_state_changed();
                self.emit().generated_password_changed();
            },
        }
    }
    fn update_used_text_from_url(&self) {
        let used_text = {
            self.profiles.do_with_current_url_parsing_settings(
                |settings| {
                    //have to convert saved data to corresponding runtime data.
                    let use_protocol = match (settings.use_protocol, settings.use_undefined_as_protocol_fallback) {
                        (true, false) => ProtocolUsageMode::Used,
                        (true, true) => ProtocolUsageMode::UsedWithUndefinedIfEmpty,
                        (false, _) => ProtocolUsageMode::Ignored,
                    };
                    let url_parsing = UrlParsing::new(
                        use_protocol,
                        settings.use_userinfo,
                        settings.use_subdomains,
                        settings.use_domain,
                        settings.use_port_path);
                    url_parsing.parse(&self.url.borrow())
                }
            )
        };
        { *self.used_text.borrow_mut() = used_text.unwrap_or_else(|e| e.to_string()); }
        self.emit().used_text_changed();
        self.update_generated_password(); //intentionally unconditional.
    }
    fn update_generated_password(&self) {
        let generation_settings = self.profiles.get_copy_current_generation_settings();
        match generation_settings {
            Ok(generation_settings) => {
                self.to_helper.send(
                    UiToHelper::GeneratePassword(
                        GeneratePasswordTask{
                            input: self.used_text.borrow().clone(),
                            master_password: self.master_password.borrow().clone(),
                            generation_settings }
                    )
                ).expect("Helper thread no longer listening. Unrecoverable.");
            }
            Err(error) => {
                {*self.generated_password.borrow_mut() = error.to_string();}
                {*self.generator_state.borrow_mut() = GeneratorState::GenerationCompleted;}
                self.emit().generator_state_changed();
                self.emit().generated_password_changed();
            }
        }
    }
}

#[cfg(test)]
mod passwordmaker_tests {
    use std::convert::TryInto;

    use super::*;
    #[test]
    fn generator_state_reciprocity() {
        for i in 0..(GeneratorState::variant_count() as u8) {
            let g : GeneratorState = i.try_into().unwrap();
            let j : u8 = g.into();
            assert_eq!(j,i);
        }
        for i in (GeneratorState::variant_count() as u8)..=255 {
            let g : Result<GeneratorState,_> = i.try_into();
            assert!(g.is_err());
        }
    }
}
