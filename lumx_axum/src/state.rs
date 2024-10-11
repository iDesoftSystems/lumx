use std::sync::Arc;

use lumx_core::program::Program;

#[derive(Clone)]
pub struct AppState {
    /// App Registry Ref
    pub app: Arc<Program>,
}
