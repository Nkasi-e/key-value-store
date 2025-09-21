//! Database trait definition

/// Database trait for key-value storage operations
pub trait Database<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&mut self, key: K, value: V) -> Option<V>;
    fn delete(&mut self, key: &K) -> Option<V>;
    fn exists(&self, key: &K) -> bool;
    fn keys(&self) -> Vec<K>;
    fn len(&self) -> usize;
    fn clear(&mut self);
}
