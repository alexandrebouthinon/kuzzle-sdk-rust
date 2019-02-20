use crate::types::KuzzleError;
use serde_json::Value;
use std::collections::HashMap;

/// A KuzzleResponse is a standardized result.
/// This format is shared by all  API routes, including routes added by controller plugins.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct KuzzleResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    status: u16,
    error: Option<KuzzleError>,
    controller: Option<String>,
    action: Option<String>,
    collection: Option<String>,
    index: Option<String>,
    volatile: Option<HashMap<String, Value>>,
    result: Value,

    #[serde(rename = "room")]
    room_id: Option<String>,
    channel: Option<String>,
}

impl KuzzleResponse {
    /// KuzzleResponse request_id getter.
    pub fn request_id(&self) -> &String {
        &self.request_id
    }

    /// KuzzleResponse status getter.
    pub fn status(&self) -> &u16 {
        &self.status
    }

    /// KuzzleResponse error getter.
    pub fn error(&self) -> &Option<KuzzleError> {
        &self.error
    }

    /// KuzzleResponse controller getter.
    pub fn controller(&self) -> String {
        match &self.controller {
            Some(controller) => controller.clone(),
            None => String::new(),
        }
    }

    /// KuzzleResponse action getter.
    pub fn action(&self) -> String {
        match &self.action {
            Some(action) => action.clone(),
            None => String::new(),
        }
    }

    /// KuzzleResponse index getter.
    pub fn index(&self) -> String {
        match &self.index {
            Some(index) => index.clone(),
            None => String::new(),
        }
    }

    /// KuzzleResponse collection getter.
    pub fn collection(&self) -> String {
        match &self.collection {
            Some(collection) => collection.clone(),
            None => String::new(),
        }
    }

    /// KuzzleResponse result getter.
    pub fn result(&self) -> &Value {
        &self.result
    }

    /// KuzzleResponse volatile getter.
    pub fn volatile(&self) -> HashMap<String, Value> {
        match &self.volatile {
            Some(volatile) => volatile.clone(),
            None => HashMap::new(),
        }
    }

    /// KuzzleResponse room_id getter.
    pub fn room_id(&self) -> String {
        match &self.room_id {
            Some(id) => id.clone(),
            None => String::new(),
        }
    }

    /// KuzzleResponse channel getter.
    pub fn channel(&self) -> String {
        match &self.channel {
            Some(channel) => channel.clone(),
            None => String::new(),
        }
    }
}
