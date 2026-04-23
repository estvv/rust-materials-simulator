//! Material registry for storing and retrieving materials.
//!
//! The registry serves as a centralized store for all material definitions
//! loaded from configuration, allowing lookup by ID or iteration over all materials.

use crate::core::material::Material;

/// A registry that stores and manages all available material types.
///
/// Materials are loaded from configuration files at startup and registered here.
/// The registry provides efficient lookup by ID and iteration over all materials.
#[derive(Clone)]
pub struct MaterialRegistry {
    materials: Vec<Material>,
}

impl MaterialRegistry {
    /// Creates a new empty material registry.
    pub fn new() -> Self {
        MaterialRegistry {
            materials: Vec::new(),
        }
    }

    /// Registers a new material in the registry.
    ///
    /// Materials are appended to the internal list and can be retrieved by ID.
    pub fn register(&mut self, material: Material) {
        self.materials.push(material);
    }

    /// Retrieves a material by its unique ID.
    ///
    /// Returns `None` if no material with the given ID exists in the registry.
    pub fn get(&self, id: u32) -> Option<&Material> {
        for m in &self.materials {
            if m.id == id {
                return Some(m);
            }
        }
        None
    }

    /// Returns the total number of registered materials.
    pub fn len(&self) -> usize {
        self.materials.len()
    }

    /// Returns an iterator over all registered materials.
    pub fn all(&self) -> impl Iterator<Item = &Material> {
        self.materials.iter()
    }

    pub fn get_behavior(&self, id: u32) -> Option<&crate::core::behavior::Behavior> {
        self.get(id).map(|m| &m.behavior)
    }
}
