use crate::context::Context;
use crate::error::Result;
use std::collections::HashMap;
 
/// Something that can fill in a `<!-- generated:KEY:start/end -->` block.
pub trait Generator {
    /// What the marker's KEY must match to be routed to this generator.
    /// It is also used to register generators in a dynamic registry.
    fn key() -> &'static str
    where
        Self: Sized;

    /// Whether this generator's output can depend on document structure
    /// that other generators might change.
    /// 
    /// For example, a toc that includes generated content.
    fn depends_on_structure() -> bool
    where
        Self: Sized;

    fn new() -> Box<dyn Generator>
    where
        Self: Sized;
 
    /// Produce the generated content for the block, *not* including the
    /// marker comment lines themselves.
    /// 
    /// An empty string is a valid output.
    fn generate(&self, ctx: &Context) -> Result<String>;
}
 
pub type Registry = HashMap<&'static str, Box<dyn Generator>>;

pub trait RegistryConfig
{
    fn register<T>(&mut self) where T: Generator;
}

impl RegistryConfig for Registry
{
    fn register<T>(&mut self) where T: Generator {
        self.insert(T::key(), T::new());
    }
}