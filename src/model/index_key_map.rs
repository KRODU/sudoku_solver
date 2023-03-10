use std::ops::Index;

pub trait IndexKey {
    fn index(&self) -> u16;
}

pub struct IndexKeyMap<K, V>
where
    K: IndexKey,
{
    arr: Vec<Option<(K, V)>>,
}

impl<K, V> IndexKeyMap<K, V>
where
    K: IndexKey,
{
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut arr: Vec<Option<(K, V)>> = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            arr.push(None);
        }

        Self { arr }
    }

    pub fn insert_new(&mut self, k: K, v: V) {
        let index = k.index() as usize;

        if self.arr.len() <= index {
            while self.arr.len() < index {
                self.arr.push(None);
            }

            self.arr.push(Some((k, v)));
        } else {
            let element = &mut self.arr[index];
            if element.is_some() {
                panic!("already contains the same key");
            }

            *element = Some((k, v));
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        let index = k.index() as usize;

        if self.arr.len() <= index {
            while self.arr.len() < index {
                self.arr.push(None);
            }

            self.arr.push(Some((k, v)));
        } else {
            self.arr[index] = Some((k, v));
        }
    }

    pub fn get(&self, k: K) -> Option<&V> {
        let index = k.index() as usize;

        if let Some((_, v)) = self.arr.get(index)? {
            Some(v)
        } else {
            None
        }
    }

    pub fn contains(&self, k: K) -> bool {
        let index = k.index() as usize;
        let Some(option) = self.arr.get(index) else {
            return false;
        };

        option.is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> + '_ {
        self.arr.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> + '_ {
        self.arr.iter_mut().flatten()
    }
}

impl<K, V> Index<K> for IndexKeyMap<K, V>
where
    K: IndexKey,
{
    type Output = V;

    fn index(&self, k: K) -> &Self::Output {
        let index = k.index() as usize;

        if let Some((_, v)) = &self.arr[index] {
            v
        } else {
            panic!("key not found in ZoneKeyMap");
        }
    }
}

impl<'a, K, V> IntoIterator for &'a IndexKeyMap<K, V>
where
    K: IndexKey,
{
    type Item = &'a (K, V);
    type IntoIter = std::iter::Flatten<core::slice::Iter<'a, Option<(K, V)>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter().flatten()
    }
}

impl<K, V> Default for IndexKeyMap<K, V>
where
    K: IndexKey,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct IndexKeySet<K>
where
    K: IndexKey,
{
    map: IndexKeyMap<K, ()>,
}

impl<K> IndexKeySet<K>
where
    K: IndexKey,
{
    pub fn new() -> Self {
        Self {
            map: IndexKeyMap::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            map: IndexKeyMap::new_with_capacity(capacity),
        }
    }

    pub fn insert_new(&mut self, k: K) {
        self.map.insert_new(k, ());
    }

    pub fn insert(&mut self, k: K) {
        self.map.insert(k, ());
    }

    pub fn contains(&self, k: K) -> bool {
        self.map.contains(k)
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> + '_ {
        self.map.iter().map(|(k, _)| k)
    }
}

impl<'a, K> IntoIterator for &'a IndexKeySet<K>
where
    K: IndexKey,
{
    type Item = &'a K;
    type IntoIter = IndexKeySetIter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        IndexKeySetIter {
            iter: self.map.arr.iter().flatten(),
        }
    }
}

impl<K> Default for IndexKeySet<K>
where
    K: IndexKey,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct IndexKeySetIter<'a, K> {
    iter: std::iter::Flatten<core::slice::Iter<'a, Option<(K, ())>>>,
}

impl<'a, K> Iterator for IndexKeySetIter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((k, _)) = self.iter.next() {
            Some(k)
        } else {
            None
        }
    }
}
