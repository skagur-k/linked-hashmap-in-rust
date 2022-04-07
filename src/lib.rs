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
        // create new hashmap<K,V>
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
    fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize // index of the bucket to put the (K,V) into.
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.bucket(key); // index of the bucket to get
        self.buckets[bucket]
            .iter() //iterate through the bucket
            .find(|&(ref ekey, _)| ekey == key) // when bucket with key found, return Some / or None if not found
            .map(|&(_, ref v)| v) // Return the value
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.bucket(key);
        let bucket = &mut self.buckets[bucket];
        let i = bucket.iter().position(|&(ref ekey, _)| ekey == key)?;
        Some(bucket.swap_remove(i).1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // If the bucket has 0 items or the # of items in the buck is more than the 3/4 of the bucket size, -> resize
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key);
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

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            // Drain the original buckets and fill the old key-value pairs into the new buckets
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
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
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.get(&"foo"), None);
    }
}
