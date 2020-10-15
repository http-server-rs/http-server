use std::fs::Metadata;
use std::time::SystemTime;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Creates an Entity Tag from a `File` `Metadata`
pub fn make_entity_tag(meta: &Metadata) -> String {
  let mut created_hasher = DefaultHasher::new();
  let mut modified_hasher = DefaultHasher::new();
  let mut size_hasher = DefaultHasher::new();

  meta.len().hash(&mut size_hasher);
  meta.created().unwrap_or(SystemTime::now()).hash(&mut created_hasher);
  meta.modified().unwrap_or(SystemTime::now()).hash(&mut modified_hasher);

  format!("{0:x}{1:x}{2:x}", created_hasher.finish(), modified_hasher.finish(), size_hasher.finish())
}
