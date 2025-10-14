use teloxide::prelude::*;

use crate::db::pool::DbPool;
use crate::types::HandlerResult;

pub async fn khp_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult { Ok(()) }

pub async fn mps_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult { Ok(()) }

pub async fn mph_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult { Ok(()) }

pub async fn knots_handler(bot: Bot, callback: CallbackQuery, db: DbPool) -> HandlerResult { Ok(()) }
