use std::sync::Arc;
use passwordmaker_rs::{GenerationError,SettingsError};
use crate::implementation::profiles::GenerationSettings;

pub(super) enum HelperToUi {
    Generated {
        password : String,
    },
    GenerationFailed {
        error : GenerationIssue
    },
    GenerationStarted
}

pub(super) struct GeneratePasswordTask{
    pub(crate) input : String,
    pub(crate) master_password : String,
    pub(crate) generation_settings : Arc<GenerationSettings>
}

pub(super) enum UiToHelper {
    Shutdown,
    GeneratePassword(GeneratePasswordTask),
}

pub(super) enum GenerationIssue {
    Settings(SettingsError),
    Input(GenerationError),
}

impl From<SettingsError> for GenerationIssue {
    fn from(s: SettingsError) -> Self {
        GenerationIssue::Settings(s)
    }
}

impl From<GenerationError> for GenerationIssue {
    fn from(g: GenerationError) -> Self {
        GenerationIssue::Input(g)
    }
}