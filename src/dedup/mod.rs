pub mod dedup_cache;
pub mod dedup_item_key;
pub mod dedup_item_name;

#[cfg(test)]
mod tests {
    use crate::dedup::dedup_cache::{DedupCache, DedupItem};
    use crate::dedup::dedup_item_key::DedupItemKey;
    use crate::dedup::dedup_item_name::DedupItemName;
    use std::num::NonZero;

    #[test]
    fn inserts_one() {
        let mut dedup_cache = DedupCache::new();
        let item = TestDedupItem {
            id: "1".to_string(),
        };
        dedup_cache.insert(&item);

        assert!(dedup_cache.contains(&item));
        assert_eq!(dedup_cache.items_len(&item.get_name()), 1);
    }

    #[test]
    fn do_not_contains() {
        let mut dedup_cache = DedupCache::new();
        let item = TestDedupItem {
            id: "1".to_string(),
        };
        let item2 = TestDedupItem {
            id: "2".to_string(),
        };
        dedup_cache.insert(&item);

        assert!(!dedup_cache.contains(&item2));
    }

    #[test]
    fn replaces_first() {
        let mut dedup_cache = DedupCache::with_capacity(NonZero::new(2).unwrap());
        let item1 = TestDedupItem {
            id: "1".to_string(),
        };
        let item2 = TestDedupItem {
            id: "2".to_string(),
        };
        let item3 = TestDedupItem {
            id: "3".to_string(),
        };
        dedup_cache.insert(&item1);
        dedup_cache.insert(&item2);
        dedup_cache.insert(&item3);

        assert_eq!(dedup_cache.items_len(&item1.get_name()), 2);
        assert!(!dedup_cache.contains(&item1));
        assert!(dedup_cache.contains(&item2));
        assert!(dedup_cache.contains(&item3));
    }

    struct TestDedupItem {
        pub id: String,
    }

    impl DedupItem for TestDedupItem {
        fn get_key(&self) -> DedupItemKey {
            self.id.clone().into()
        }

        fn get_name(&self) -> DedupItemName {
            "test-item".into()
        }
    }
}
