mod message_parsing;
mod hashers;
mod profile_to_domain;
use std::sync::mpsc::{Receiver, SendError};
use passwordmaker_rs::PasswordMaker;
use profile_to_domain::{convert_hash_algorithm, convert_leet};
use super::emittingsender::EmittingSender;
use super::thread_messages::{HelperToUi, UiToHelper};
use message_parsing::receive_and_get_newest_or_important_command;
use hashers::PassFishHashers;

pub(super) fn run<T: Fn()>(to_ui : &EmittingSender<HelperToUi, T>, from_ui : &Receiver<UiToHelper>) -> Result<(), SendError<HelperToUi>>{
    println!("Helper Thread starting up.");

    while let Ok(m) = receive_and_get_newest_or_important_command(from_ui)
    {
        //m is a product type, because we gather the latest command of each type. Easiest to deal with those would be using a group of ifs.
        //for now the number of alternatives is rather small though, so let's match instead.
        match m.0 {
            UiToHelper::Shutdown => break,
            UiToHelper::GeneratePassword(task) => {
                
                to_ui.send(HelperToUi::GenerationStarted)?;
                let hash_algorithm = convert_hash_algorithm(&task.generation_settings.hash_algorithm);
                let use_leet = convert_leet(&task.generation_settings.leet);
                let characters = &task.generation_settings.characters;
                let username = &task.generation_settings.username;
                let modifier = &task.generation_settings.modifier;
                let password_length = task.generation_settings.password_length as usize;
                let suffix = &task.generation_settings.suffix;
                let prefix = &task.generation_settings.prefix;
                type Pwm<'a> = PasswordMaker<'a, PassFishHashers>;
                let pwm = Pwm::new(hash_algorithm, use_leet, characters, username, modifier, password_length, prefix, suffix);
                let input = task.input;
                let master_password = task.master_password;
                let pwm = pwm.map_err(Into::into);
                let password = 
                    pwm.and_then(|pwm| pwm.generate(input, master_password).map_err(Into::into));

                match password {
                    Ok(password) => to_ui.send(HelperToUi::Generated{password})?,
                    Err(error) => to_ui.send(HelperToUi::GenerationFailed{ error })?,
                }
            }
        }
    }
    println!("Helper Thread shutting down.");
    Ok(())
}
