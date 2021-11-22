//! # Handle W30 messages.

use crate::context::Context;
use crate::message::MsgId;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Status Update ID.
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct StatusUpdateId(u32);

impl Context {
    pub async fn send_w30_status_update(&self, _msg_id: MsgId, json: String) -> Result<()> {
        Ok(())
    }

    pub async fn get_w30_status_update(
        &self,
        _msg_id: MsgId,
        _status_update_id: StatusUpdateId,
    ) -> Result<String> {
        Ok("".to_string())
    }
}
