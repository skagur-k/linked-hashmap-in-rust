use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_NBUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize, // # of items in the bucket
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // If the bucket has 0 items or the # of items in the buck is more than the 3/4 of the bucket size, -> resize
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = (hasher.finish() % self.buckets.len() as u64) as usize; // index of the bucket to put the (K,V) into.
        let bucket = &mut self.buckets[bucket]; // mutable reference to the bucket (shadowed)

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            // get mutable iterator and find&replace the existing value with the new value
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }

        bucket.push((key, value));
        None
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => n * 2,
        };

        let mut new_buckets = vec![Vec::new(); target_size];

        for (key, value) in self.buckets.drain(..) {
            // Drain the original buckets and fill the old key-value pairs into the new buckets
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % self.buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        mem::replace(&mut self.buckets, new_buckets); // replace the old bucket with the new one
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let map = HashMap::new();
        map.insert("foo", 42);
    }
}
