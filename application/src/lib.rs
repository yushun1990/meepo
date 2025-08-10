pub mod app;
pub mod app_router;
pub mod app_state;
pub mod common;
pub mod db;
pub mod domains;

mod server;

pub use app_state::AppState;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
