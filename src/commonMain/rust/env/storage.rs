use futures::future;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::ClassType;
use serde::{Deserialize, Serialize};

use objc2_foundation::{NSString, NSUserDefaults};
use stremio_core::runtime::{EnvError, EnvFutureExt, TryEnvFuture};

pub struct Storage {
    suite_name: Option<String>,
}
//TODO: This implimentation probably have race condition. Proper implimentation needed
impl Storage {
    /// Stores in the `suite_name` user defaults (an app group, so app extensions can share the
    /// app's storage) when given, or in the standard user defaults otherwise.
    pub fn new(suite_name: Option<String>) -> Result<Self, &'static str> {
        Ok(Self { suite_name })
    }

    fn user_defaults(suite_name: Option<&str>) -> Retained<NSUserDefaults> {
        if let Some(suite_name) = suite_name {
            let suite_name = NSString::from_str(suite_name);
            let user_defaults = unsafe {
                NSUserDefaults::initWithSuiteName(NSUserDefaults::alloc(), Some(&suite_name))
            };
            if let Some(user_defaults) = user_defaults {
                return user_defaults;
            }
        }
        unsafe { NSUserDefaults::standardUserDefaults() }
    }

    pub fn get<T: for<'de> Deserialize<'de> + Send + 'static>(
        &self,
        key: &str,
    ) -> TryEnvFuture<Option<T>> {
        let key = key.to_owned();
        let suite_name = self.suite_name.to_owned();
        Box::pin(future::lazy(move |_| {
            let nskey = &NSString::from_str(&key);

            let user_defaults = &Storage::user_defaults(suite_name.as_deref());
            let optional_value = unsafe { &NSUserDefaults::stringForKey(user_defaults, nskey) };

            Ok(match optional_value {
                Some(value) => {
                    let deserialized_value: T =
                        serde_json::from_str(&value.to_string()).map_err(EnvError::from)?; // Adjust error handling as needed
                    Some(deserialized_value)
                }
                None => None,
            })
        }))
        .boxed_env()
    }
    pub fn set<T: Serialize>(&self, key: &str, value: Option<&T>) -> TryEnvFuture<()> {
        let nskey = &NSString::from_str(&key);
        let user_defaults = &Storage::user_defaults(self.suite_name.as_deref());
        match value {
            Some(value) => {
                // Convert the value to a JSON string
                let nsvalue = match serde_json::to_string(value) {
                    Ok(value) => NSString::from_str(&value),
                    Err(error) => {
                        return future::err(EnvError::Serde(error.to_string())).boxed_env()
                    }
                };
                let nsvalue_object: Option<&AnyObject> = Some(&nsvalue);
                unsafe { NSUserDefaults::setObject_forKey(user_defaults, nsvalue_object, nskey) };
            }
            None => unsafe { NSUserDefaults::removeObjectForKey(user_defaults, nskey) },
        }
        future::lazy(move |_| {
            Ok(()) // Return the serialized value
        })
        .boxed_env()
    }
}
