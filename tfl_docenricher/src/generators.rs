use std::collections::HashMap;

use crate::generator::{Registry, RegistryConfig};
use crate::generators::majors_brf_table::MajorsBrfTable;
use crate::generators::majors_rmp_table::MajorsRmpTable;
use crate::generators::minors_rmp_table::MinorsRmpTable;
use crate::generators::patch_notes::PatchNotes;
use crate::generators::roadmaps::Roadmaps;
use crate::generators::title::Title;
use crate::generators::toc::Toc;

pub mod title;
pub mod roadmaps;
pub mod minors_rmp_table;
pub mod patch_notes;
pub mod majors_rmp_table;
pub mod majors_brf_table;
pub mod toc;

pub fn build_registry() -> Registry {
    let mut registry: Registry = HashMap::new();
    registry.register::<Title>();
    registry.register::<Roadmaps>();
    registry.register::<MinorsRmpTable>();
    registry.register::<MajorsRmpTable>();
    registry.register::<MajorsBrfTable>();
    registry.register::<PatchNotes>();
    registry.register::<Roadmaps>();
    registry.register::<Toc>();
    registry
}