use serde::Deserialize;
use std::fmt::Debug;

use crate::db::{Postgres, QuestDB};

use super::Tracing;

pub trait Config: Clone + Postgres + QuestDB + Tracing + Deserialize<'static> {}
impl<T> Config for T where T: Clone + Postgres + QuestDB + Tracing + Deserialize<'static> {}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub timeout_secs: u64,
    pub enable_swagger: bool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            timeout_secs: Default::default(),
            enable_swagger: true,
        }
    }
}
