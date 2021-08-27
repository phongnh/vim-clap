//! Processes the RPC message wrapper in Event type.

pub mod on_init;
pub mod on_move;
pub mod on_typed;

use std::sync::Arc;

use anyhow::Result;
use serde_json::json;

use crate::stdio_server::{
    session::{EventHandler, SessionContext},
    write_response,
};

pub use on_move::{OnMove, OnMoveHandler};

use super::types::Message;

#[derive(Clone)]
pub struct DefaultEventHandler;

#[async_trait::async_trait]
impl EventHandler for DefaultEventHandler {
    async fn handle_on_move(&mut self, msg: Message, context: Arc<SessionContext>) -> Result<()> {
        let msg_id = msg.id;
        if let Err(e) = on_move::OnMoveHandler::create(&msg, &context, None).map(|x| x.handle()) {
            log::error!("Failed to handle OnMove event: {:?}", e);
            write_response(json!({"error": e.to_string(), "id": msg_id }));
        }
        Ok(())
    }

    async fn handle_on_typed(&mut self, msg: Message, context: Arc<SessionContext>) -> Result<()> {
        on_typed::handle_on_typed(msg, context);
        Ok(())
    }
}