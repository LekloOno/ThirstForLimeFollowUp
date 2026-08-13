use std::collections::HashMap;

use crate::generator::{Registry, RegistryConfig};
use crate::generators::minors_table::MinorsTable;
use crate::generators::patch_notes::PatchNotes;
use crate::generators::roadmaps::Roadmaps;
use crate::generators::title::Title;
use crate::generators::toc::Toc;

pub mod title;
pub mod minors_table;
pub mod patch_notes;
pub mod roadmaps;
pub mod toc;

pub fn build_registry() -> Registry {
    let mut registry: Registry = HashMap::new();
    registry.register::<Title>();
    registry.register::<MinorsTable>();
    registry.register::<PatchNotes>();
    registry.register::<Roadmaps>();
    registry.register::<Toc>();
    registry
}