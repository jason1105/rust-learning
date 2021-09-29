use std::collections::HashMap;

#[derive(Debug)]
struct LRUCache {
    capacity: i32,
    cache: HashMap<i32, i32>,
    keys: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity,
            cache: HashMap::new(),
            keys: Vec::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) -> i32 {
        self.cache.insert(key, value);
        self.update(key);
        -1
    }

    fn update(&mut self, key: i32) {
        self.keys = self.keys.iter().map(|x| *x).filter(|x| *x != key).collect();
        self.keys.insert(0, key);
        if (self.keys.len() as i32) > self.capacity {
            for i in (self.capacity)..(self.keys.len() as i32) {
                self.cache.remove(&self.keys[i as usize]);
            }
            // self.keys.resize(self.capacity as usize, -1);
            self.keys = self.keys[0..(self.capacity as usize)].to_vec();
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.keys.contains(&key) {
            self.update(key);
            self.cache.get(&key).unwrap().clone()
        } else {
            -1
        }
    }
}

#[warn(unused_doc_comments)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let a = [1, 2, 3];

        //assert_eq!(a.iter().filter(|x| **x == 1).position(|&x| x == 2), Some(1));

        /**
         * Your LRUCache object will be instantiated and called as such:
         */
        use crate::*;
        let mut obj = LRUCache::new(2);
        assert_eq!(-1, obj.put(1, 1));
        println!("{:?}", obj);
        assert_eq!(-1, obj.put(2, 2));
        println!("{:?}", obj);
        assert_eq!(1, obj.get(1));
        println!("{:?}", obj);
        assert_eq!(-1, obj.put(3, 3));
        println!("{:?}", obj);
        assert_eq!(-1, obj.get(2));
        println!("{:?}", obj);
        assert_eq!(-1, obj.put(4, 4));
        println!("{:?}", obj);
        assert_eq!(-1, obj.get(1));
        println!("{:?}", obj);
        assert_eq!(3, obj.get(3));
        println!("{:?}", obj);
        assert_eq!(4, obj.get(4));
        println!("{:?}", obj);
    }
}
