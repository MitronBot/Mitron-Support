use crate::commands::moderation::basic::*;
use crate::commands::welcomer::welcome::*;
use serenity::framework::standard::macros::group;

// All commands related with moderation
#[group]
#[commands(kick, ban, unban)]
#[summary = "Moderation Commands"]
pub struct Moderation;

// All utility commands.
#[group]
#[commands(welcome, welcome_disable)]
#[summary = "Utility Commands"]
pub struct Utility;
