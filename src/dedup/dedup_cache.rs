use crate::dedup::dedup_item::DedupItem;
use crate::dedup::dedup_item_key::DedupItemKey;
use crate::dedup::dedup_item_name::DedupItemName;
use ahash::AHashSet;
use rust_extensions::sorted_vec::{EntityWithKey, SortedVec};
use std::num::NonZero;

impl EntityWithKey<DedupItemName> for DedupKeysCache {
    fn get_key(&self) -> &DedupItemName {
        &self.name
    }
}

struct DedupKeysCache {
    name: DedupItemName,
    pub keys: AHashSet<DedupItemKey>,
}

impl DedupKeysCache {
    pub fn new(item: &impl DedupItem, capacity: usize) -> Self {
        let mut keys = AHashSet::with_capacity(capacity);
        keys.insert(item.get_key_str().into());

        Self {
            name: item.get_name_str().into(),
            keys,
        }
    }
}

pub struct DedupCache<TItem: DedupItem> {
    capacity: NonZero<usize>,
    keys_by_names: SortedVec<DedupItemName, DedupKeysCache>,
    item: std::marker::PhantomData<TItem>,
}

impl<TItem: DedupItem> Default for DedupCache<TItem> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: DedupItem> DedupCache<T> {
    pub fn new() -> Self {
        let default_capacity: usize = 1000;

        Self {
            capacity: NonZero::new(default_capacity).unwrap(),
            keys_by_names: SortedVec::new_with_capacity(2),
            item: Default::default(),
        }
    }

    pub fn with_capacity(capacity: NonZero<usize>) -> Self {
        Self {
            capacity,
            keys_by_names: SortedVec::new_with_capacity(2),
            item: Default::default(),
        }
    }

    pub fn insert(&mut self, item: &T) {
        let name = item.get_name_str().into();

        if let Some(keys_cache) = self.keys_by_names.get_mut(&name) {
            if keys_cache.keys.len() >= self.capacity.get() {
                let first_key = keys_cache
                    .keys
                    .iter()
                    .next()
                    .expect("len is not zero")
                    .clone();
                keys_cache.keys.remove(&first_key);
            }
            
            keys_cache.keys.insert(item.get_key_str().into());
        } else {
            self.keys_by_names
                .insert_or_replace(DedupKeysCache::new(item, self.capacity.get()));
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        if let Some(keys_cache) = self.keys_by_names.get(&item.get_name_str().into()) {
            keys_cache.keys.contains(&item.get_key_str().into())
        } else {
            false
        }
    }

    pub fn len_by_name(&self, item_name: &str) -> usize {
        if let Some(keys_cache) = self.keys_by_names.get(&item_name.into()) {
            keys_cache.keys.len()
        } else {
            0
        }
    }
}
