//! Contain [`KeyboardApiBuilder`]

use hidapi::HidApi;

use super::{KeyboardApi, KeyboardIntrefacesFilter, Res};
use crate::ErrorRoccatVulcanApi;

/// [`KeyboardApi`] builder
pub struct KeyboardApiBuilder<'a> {
    /// List of filter that the api will look for
    models: Vec<KeyboardIntrefacesFilter>,
    /// Optionaly an api can be given
    api: Option<&'a HidApi>,
}

//TODO examples
impl<'a> KeyboardApiBuilder<'a> {
    /// LIist of default KeyboardIntrefacesFilter, it is [`KeyboardIntrefacesFilter::DEFAULT_MODEL`]
    const DEFAULT_MODEL: [KeyboardIntrefacesFilter; 2] = KeyboardIntrefacesFilter::DEFAULT_MODEL;

    /// Create a default builder
    pub const fn new() -> Self {
        Self {
            models: vec![],
            api: None,
        }
    }

    /// Get the vector of model
    pub const fn models(&self) -> &Vec<KeyboardIntrefacesFilter> {
        &self.models
    }

    /// Get a mutable reference to the models
    pub fn models_mut(&mut self) -> &mut Vec<KeyboardIntrefacesFilter> {
        &mut self.models
    }

    /// add a model to the list
    pub fn add_model(&mut self, filter: KeyboardIntrefacesFilter) -> &mut Self {
        self.models.push(filter);
        self
    }

    /// push the default model in the model list
    pub fn add_default_model(&mut self) -> &mut Self {
        for filter in Self::DEFAULT_MODEL {
            self.add_model(filter);
        }
        self
    }

    /// Set the api to use
    pub fn set_api(&mut self, api: &'a HidApi) -> &mut Self {
        self.api = Some(api);
        self
    }

    /// Remove the given api and use the default one
    pub fn remove_api(&mut self) -> &mut Self {
        self.api = None;
        self
    }

    /// Get the final model list, the models if not empty or the default list otherwise
    fn filter_list(&self) -> &[KeyboardIntrefacesFilter] {
        if self.models.is_empty() {
            &Self::DEFAULT_MODEL
        } else {
            &self.models
        }
    }

    /// Terminal methode that build the KeyboardApi.
    /// # Errors
    /// - [`ErrorRoccatVulcanApi::HidApiError`] if the [`HidApi`] could not be initialized.None
    /// - [`ErrorRoccatVulcanApi::KeyboardNotFound`] Keyboard not found,
    /// - [`ErrorRoccatVulcanApi::NoLedDevice`] Led device not found,
    /// - [`ErrorRoccatVulcanApi::LedDeviceError`] Led device error,
    /// - [`ErrorRoccatVulcanApi::NoControlDevice`] Control device not found,
    /// - [`ErrorRoccatVulcanApi::ControlDeviceError`] Control device error,
    /// - [`ErrorRoccatVulcanApi::NoReadDevice`] Read device not found,
    /// - [`ErrorRoccatVulcanApi::ReadDeviceError`] Read device error,
    /// - [`ErrorRoccatVulcanApi::WaitedToMuchTime`] Error while initalizing key board: waited for too long,
    pub fn build(&self) -> Res<KeyboardApi> {
        match self.api {
            Some(api) => KeyboardApi::new_from_model_list(api, self.filter_list()),
            None => KeyboardApi::new_from_model_list(
                &HidApi::new().map_err(ErrorRoccatVulcanApi::HidApiError)?,
                self.filter_list(),
            ),
        }
    }
}

impl<'a> Default for KeyboardApiBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
