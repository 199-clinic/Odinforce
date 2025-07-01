mod providers;
mod registry;
mod store;

use gpui::App;

pub use crate::providers::rustdoc::*;
pub use crate::registry::*;
pub use crate::store::*;

pub fn init(cx: &mut App) {
    IndexedDocsRegistry::init_global(cx);
}
