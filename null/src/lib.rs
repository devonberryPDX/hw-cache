//! # Assignment
//!
//! * Fix the `NullCache` type to be able to carry
//!   its item type. This will involve the use of `PhantomData`.
//!
//! * Fix the `capacity()` method to correctly report the
//!   capacity. Be careful here!

use cache::*;
use std::marker::PhantomData;

/// A "cache" that does not cache anything. Useful
/// for testing behavior with cache misses.
pub struct NullCache<I>(PhantomData<I>);

impl<I> Default for NullCache<I> {
    fn default() -> Self {
        NullCache { 0: PhantomData }
    }
}

impl<K, I> Cache<K> for NullCache<I> {
    type Item = I;

    /// Insert an item in the cache at the given key. Does
    /// not actually alter the cache, since values are not
    /// cached.
    fn insert(&mut self, _key: K, _item: I) {
        // XXX Do nothing.
    }

    /// Fail to retrieve an item from the cache.
    fn retrieve(&mut self, _key: &K) -> Option<&mut I> {
        None
    }

    /// This cache has zero capacity.
    fn capacity(&self) -> Option<usize> {
        Some(0)
    }
}

#[test]
fn test_null_cache() {
    let cache = Box::new(NullCache::default());
    cache_tests::test_fib_cache(cache);
}
