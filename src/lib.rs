mod client;
mod routes_gdps;
mod routes_user_notifications;

pub use client::{Client, SDKError};
pub use routes_gdps::GDPSRoutes;
pub use routes_user_notifications::{NotificationRoutes, UserRoutes};

#[derive(Clone)]
pub struct RigbySDK {
    pub client: Client,
    pub gdps: GDPSRoutes,
    pub notifications: NotificationRoutes,
    pub user: UserRoutes,
}

impl RigbySDK {
    pub fn new(token: impl Into<String>) -> Self {
        let client = Client::new(token.into(), None);
        Self::from_client(client)
    }

    pub fn with_base_url(token: impl Into<String>, base_url: impl Into<String>) -> Self {
        let client = Client::new(token.into(), Some(base_url.into()));
        Self::from_client(client)
    }

    pub fn from_client(client: Client) -> Self {
        Self {
            gdps: GDPSRoutes::new(client.clone()),
            notifications: NotificationRoutes::new(client.clone()),
            user: UserRoutes::new(client.clone()),
            client,
        }
    }
}
