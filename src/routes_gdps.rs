use serde_json::Value;

use crate::client::Client;

#[derive(Clone)]
pub struct GDPSRoutes {
    pub config: ConfigRoutes,
    pub analytics: AnalyticsRoutes,
    pub gauntlets: GauntletsRoutes,
    pub levels: LevelsRoutes,
    pub mappacks: MapPacksRoutes,
    pub masonlab: MasonLabRoutes,
    pub music: MusicRoutes,
    pub player: PlayerRoutes,
    pub players: PlayersRoutes,
    pub public: PublicRoutes,
    pub quests: QuestsRoutes,
    pub roles: RolesRoutes,
    pub server: ServerRoutes,
}

impl GDPSRoutes {
    pub fn new(client: Client) -> Self {
        Self {
            config: ConfigRoutes {
                client: client.clone(),
            },
            analytics: AnalyticsRoutes {
                client: client.clone(),
            },
            gauntlets: GauntletsRoutes {
                client: client.clone(),
            },
            levels: LevelsRoutes {
                client: client.clone(),
            },
            mappacks: MapPacksRoutes {
                client: client.clone(),
            },
            masonlab: MasonLabRoutes {
                client: client.clone(),
            },
            music: MusicRoutes {
                client: client.clone(),
            },
            player: PlayerRoutes::new(client.clone()),
            players: PlayersRoutes {
                client: client.clone(),
            },
            public: PublicRoutes {
                client: client.clone(),
            },
            quests: QuestsRoutes {
                client: client.clone(),
            },
            roles: RolesRoutes {
                client: client.clone(),
            },
            server: ServerRoutes { client },
        }
    }
}

#[derive(Clone)]
pub struct ConfigRoutes {
    client: Client,
}

impl ConfigRoutes {
    pub fn get(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "config", "get"], Some(data))
    }

    pub fn update_chests(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "config", "updateChest"], Some(data))
    }

    pub fn update_security(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "config", "updateSecurity"], Some(data))
    }

    pub fn update_server(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "config", "updateServer"], Some(data))
    }
}

#[derive(Clone)]
pub struct AnalyticsRoutes {
    client: Client,
}

impl AnalyticsRoutes {
    pub fn overview(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "analytics", "overview"], Some(data))
    }
}

#[derive(Clone)]
pub struct GauntletsRoutes {
    client: Client,
}

impl GauntletsRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "gauntlets", "list"], Some(data))
    }

    pub fn create(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "gauntlets", "create"], Some(data))
    }
}

#[derive(Clone)]
pub struct LevelsRoutes {
    client: Client,
}

impl LevelsRoutes {
    pub fn search(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "levels", "search"], Some(data))
    }

    pub fn get(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "levels", "details"], Some(data))
    }

    pub fn update_metadata(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "levels", "updateMetadata"], Some(data))
    }

    pub fn update_music(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "levels", "updateMusic"], Some(data))
    }

    pub fn update_rating(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "levels", "updateRating"], Some(data))
    }
}

#[derive(Clone)]
pub struct MapPacksRoutes {
    client: Client,
}

impl MapPacksRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "mappacks", "list"], Some(data))
    }

    pub fn create(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "mappacks", "create"], Some(data))
    }
}

#[derive(Clone)]
pub struct MasonLabRoutes {
    client: Client,
}

impl MasonLabRoutes {
    pub fn get(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "masonlab", "get"], Some(data))
    }

    pub fn save(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "masonlab", "save"], Some(data))
    }
}

#[derive(Clone)]
pub struct MusicRoutes {
    client: Client,
}

impl MusicRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "music", "list"], Some(data))
    }

    pub fn create_from_newgrounds(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(
            &["rpc", "gdps", "music", "createFromNewgrounds"],
            Some(data),
        )
    }

    pub fn create_from_url(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "music", "createFromUrl"], Some(data))
    }

    pub fn update_metadata(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "music", "updateMetadata"], Some(data))
    }

    pub fn delete_all(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "music", "deleteAll"], Some(data))
    }

    pub fn toggle_ban(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "music", "toggleBan"], Some(data))
    }
}

#[derive(Clone)]
pub struct PlayerSongsRoutes {
    client: Client,
}

impl PlayerSongsRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "player", "songs", "list"], Some(data))
    }

    pub fn create(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "player", "songs", "create"], Some(data))
    }

    pub fn create_from_newgrounds(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(
            &["rpc", "gdps", "player", "songs", "createFromNewgrounds"],
            Some(data),
        )
    }

    pub fn create_from_url(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(
            &["rpc", "gdps", "player", "songs", "createFromUrl"],
            Some(data),
        )
    }
}

#[derive(Clone)]
pub struct PlayerRoutes {
    client: Client,
    pub songs: PlayerSongsRoutes,
}

impl PlayerRoutes {
    pub fn new(client: Client) -> Self {
        Self {
            songs: PlayerSongsRoutes {
                client: client.clone(),
            },
            client,
        }
    }

    pub fn login(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "player", "login"], Some(data))
    }

    pub fn profile(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "player", "profile"], Some(data))
    }
}

#[derive(Clone)]
pub struct PlayersRoutes {
    client: Client,
}

impl PlayersRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "players", "list"], Some(data))
    }

    pub fn assign(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "players", "assign"], Some(data))
    }
}

#[derive(Clone)]
pub struct PublicRoutes {
    client: Client,
}

impl PublicRoutes {
    pub fn page(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "public", "page"], Some(data))
    }
}

#[derive(Clone)]
pub struct QuestsRoutes {
    client: Client,
}

impl QuestsRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "quests", "list"], Some(data))
    }

    pub fn create(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "quests", "create"], Some(data))
    }

    pub fn remove(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "quests", "remove"], Some(data))
    }
}

#[derive(Clone)]
pub struct RolesRoutes {
    client: Client,
}

impl RolesRoutes {
    pub fn list(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "roles", "list"], Some(data))
    }

    pub fn create(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "roles", "create"], Some(data))
    }

    pub fn update(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "roles", "update"], Some(data))
    }

    pub fn remove(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "roles", "delete"], Some(data))
    }
}

#[derive(Clone)]
pub struct ServerRoutes {
    client: Client,
}

impl ServerRoutes {
    pub fn list_members(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "listMembers"], Some(data))
    }

    pub fn list_invites(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "listInvites"], Some(data))
    }

    pub fn create_invite(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "createInvite"], Some(data))
    }

    pub fn revoke_invite(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "revokeInvite"], Some(data))
    }

    pub fn join(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "join"], Some(data))
    }

    pub fn update_member(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "updateMember"], Some(data))
    }

    pub fn remove_member(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "removeMember"], Some(data))
    }

    pub fn leave(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "leave"], Some(data))
    }

    pub fn list(&self) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "list"], None)
    }

    pub fn mysrvs(&self) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "mysrvs"], None)
    }

    pub fn createsrv(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "createsrv"], Some(data))
    }

    pub fn deletesrv(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "deletesrv"], Some(data))
    }

    pub fn getinfo(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client.call(&["rpc", "gdps", "getinfo"], Some(data))
    }

    pub fn toggle_public(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "togglePublic"], Some(data))
    }

    pub fn submit_external(&self, data: Value) -> Result<Value, crate::SDKError> {
        self.client
            .call(&["rpc", "gdps", "submitExternal"], Some(data))
    }
}
