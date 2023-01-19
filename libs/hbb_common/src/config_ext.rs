use serde_derive::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    path::PathBuf,
    sync::{Arc, Mutex},
};

lazy_static::lazy_static!(
    static ref USER_CONFIGS: Arc<Mutex<UserConfig>> = Arc::new(Mutex::new(UserConfig::new("vMvigXXqve9by1GQBt5RVPK+JwGBWwXo9DvEZkDd+Fc=", "device_id", "hyper-token","rs-sg.rustdesk.com,rs-ny.rustdesk.com,rs-cn.rustdesk.com")));
);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserConfig {
    key: String,
    uuid: String,
    token: String,
    server: String,
}

impl UserConfig {
    pub fn new<T: Into<String>>(key: T, uuid: T, token: T, server: T) -> Self {
        UserConfig {
            key: key.into(),
            uuid: uuid.into(),
            token: token.into(),
            server: server.into(),
        }
    }

    pub fn get_global() -> Self {
        UserConfig::default()
    }

    pub fn is_empty(&self) -> bool {
        return self.key.is_empty() || self.uuid.is_empty() || self.token.is_empty();
    }

    pub fn set_global(&self) {
        *USER_CONFIGS.lock().unwrap() = self.clone();
    }

    pub fn get(field: &str) -> Option<String> {
        match field {
            "key" => return Some(USER_CONFIGS.lock().unwrap().key.clone()),
            "uuid" => return Some(USER_CONFIGS.lock().unwrap().uuid.clone()),
            "token" => return Some(USER_CONFIGS.lock().unwrap().token.clone()),
            "server" => return Some(USER_CONFIGS.lock().unwrap().server.clone()),
            _ => return None,
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig::from((&*USER_CONFIGS.lock().unwrap()).clone())
    }
}

impl TryFrom<&str> for UserConfig {
    type Error = serde_json::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl TryFrom<PathBuf> for UserConfig {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(std::fs::read(path)?.as_slice())?)
    }
}

#[test]
fn test_user_config() {
    let jv = "{\"key\":\"handshake_key\",\"token\":\"token\",\"uuid\":\"uuid\"}";
    let v = UserConfig::try_from(jv).unwrap();
    println!("{:?}", v);
    v.set_global();
    println!("{:?}", UserConfig::default());
}
