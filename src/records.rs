use pagurus::failure::OrFail;
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

use crate::model::ModelCommand;

#[derive(Debug, Serialize, Deserialize)]
pub enum Record {
    Open(OpenRecord),
    Model(ModelCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenRecord {
    pub timestamp: UnixTimestamp,
    pub version: String,
    pub port: u16,
    pub uuid: uuid::Uuid,
}

impl OpenRecord {
    pub fn new(port: u16) -> pagurus::Result<Self> {
        Ok(Self {
            timestamp: UnixTimestamp::now()?,
            version: env!("CARGO_PKG_VERSION").to_string(),
            port,
            uuid: uuid::Uuid::new_v4(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
pub struct UnixTimestamp(u64);

impl UnixTimestamp {
    pub fn now() -> pagurus::Result<Self> {
        Ok(Self(UNIX_EPOCH.elapsed().or_fail()?.as_secs()))
    }
}
