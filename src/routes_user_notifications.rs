use serde_json::Value;

use crate::client::Client;

#[derive(Clone)]
pub struct NotificationRoutes {
    client: Client,
}

impl NotificationRoutes {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "notifications", "list"], None)
    }

    pub fn mark_as_read(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "notifications", "markAsRead"], Some(data))
    }

    pub fn mark_all_as_read(&self) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "notifications", "markAllAsRead"], None)
    }

    pub fn delete(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "notifications", "delete"], Some(data))
    }
}

#[derive(Clone)]
pub struct UserRoutes {
    client: Client,
}

impl UserRoutes {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn me(&self) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "user", "me"], None)
    }

    pub fn update_profile(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "user", "updateProfile"], Some(data))
    }

    pub fn list_sessions(&self) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "user", "listSessions"], None)
    }

    pub fn revoke_session(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "user", "revokeSession"], Some(data))
    }

    pub fn revoke_all_sessions(&self) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "user", "revokeAllSessions"], None)
    }

    pub fn delete_account(&self) -> Result<Value, crate::SDKError> {
        self.client.call(
            &["rpc", "user", "deleteAccount"],
            Some(serde_json::json!({ "confirmation": "delete" })),
        )
    }
}
