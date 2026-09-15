//! Typed generational handles (`slotmap`, P0-02).
//!
//! UUIDs remain the persistent/external identity of assets (see
//! `petunia_project::Asset::id`). Generational handles complement them for
//! session-local lookups where stale references must fail loudly instead of
//! silently aliasing a recycled slot, e.g. viewport picking caches and gizmo
//! drag sessions.
//!
//! The `slotmap` types never cross this boundary: callers only see
//! [`Handle`] and [`HandleTable`].

use slotmap::{SlotMap, new_key_type};

new_key_type! {
    /// Session-local generational handle. `Copy`, comparable, never recycled
    /// silently: a handle obtained before removal never resolves after it.
    pub struct Handle;
}

/// Generational arena mapping [`Handle`]s to values.
///
/// Removal invalidates the handle; re-insertion yields a *different* handle
/// even if the slot is recycled internally.
#[derive(Clone, Debug, Default)]
pub struct HandleTable<T> {
    inner: SlotMap<Handle, T>,
}

impl<T> HandleTable<T> {
    /// Creates an empty table.
    pub fn new() -> Self {
        Self {
            inner: SlotMap::with_key(),
        }
    }

    /// Inserts a value, returning a fresh generational handle.
    pub fn insert(&mut self, value: T) -> Handle {
        self.inner.insert(value)
    }

    /// Resolves a handle, or `None` when unknown or stale (removed).
    pub fn get(&self, handle: Handle) -> Option<&T> {
        self.inner.get(handle)
    }

    /// Mutably resolves a handle, or `None` when unknown or stale.
    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        self.inner.get_mut(handle)
    }

    /// Removes the entry, returning it. The handle stays invalid afterwards.
    pub fn remove(&mut self, handle: Handle) -> Option<T> {
        self.inner.remove(handle)
    }

    /// Number of live entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether no live entries exist.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_handles_never_resolve() {
        let mut table = HandleTable::new();
        let a = table.insert("mesh-a");
        let b = table.insert("mesh-b");
        assert_eq!(table.get(a), Some(&"mesh-a"));
        assert_eq!(table.len(), 2);

        assert_eq!(table.remove(a), Some("mesh-a"));
        assert_eq!(table.get(a), None);
        assert_eq!(table.get(b), Some(&"mesh-b"));

        // Slot recycling must not resurrect the stale handle.
        let c = table.insert("mesh-c");
        assert_ne!(a, c);
        assert_eq!(table.get(a), None);
        assert_eq!(table.get(c), Some(&"mesh-c"));
    }

    #[test]
    fn get_mut_edits_live_entries_only() {
        let mut table = HandleTable::new();
        let live = table.insert(1_u32);
        let dead = table.insert(2_u32);
        table.remove(dead);

        *table.get_mut(live).expect("live handle resolves") += 10;
        assert_eq!(table.get(live), Some(&11));
        assert_eq!(table.get_mut(dead), None);
        assert!(!table.is_empty());
    }
}
