use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use sdl2::rect::Rect;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct CollisionPair {
    a: usize,
    b: usize
}

impl CollisionPair {
    pub fn new(a: usize, b: usize) -> Self {
        if a < b {
            CollisionPair {
                a, b
            }
        }
        else {
            CollisionPair {
                a: b, b: a
            }
        }
    }
}

pub struct Collision {
    collision_cache: HashMap<CollisionPair, Option<Rect>>,
    collider_ids: HashMap<Rect, usize>,
    _id_gen: AtomicUsize
}

impl Collision {
    pub fn new() -> Self {
        Collision { collision_cache: HashMap::new(), collider_ids: HashMap::new(), _id_gen: AtomicUsize::new(0) }
    }

    pub fn clear(&mut self) {
        self.collider_ids.clear();
        self.collision_cache.clear();
    }

    pub fn check_collision(&mut self, a: Rect, b: Rect) -> (bool, Option<Rect>) {
        let pair = self.make_pair(a, b);
        if let Some(found) = self.check_cached(pair) {
            return (true, found);
        }
        let intersection = a.intersection(b);
        self.collision_cache.insert(pair, intersection);
        (false, intersection)
    }

    fn check_cached(&self, pair: CollisionPair) -> Option<Option<Rect>> {
        match self.collision_cache.get(&pair) {
            Some(col) => Some(*col),
            None => None
        }       
    }

    pub fn store(&mut self, rect: Rect) -> usize {
        if let Some(id) = self.collider_ids.get(&rect) {
            return *id;
        }
        let id = self._id_gen.fetch_add(1, Ordering::SeqCst);
        self.collider_ids.insert(rect, id);
        id
    }

    fn make_pair(&mut self, a: Rect, b: Rect) -> CollisionPair {
        let a_part = {
            match self.collider_ids.get(&a) {
                Some(id) => *id,
                None => self.store(a)
            }
        };
        let b_part = {
            match self.collider_ids.get(&b) {
                Some(id) => *id,
                None => self.store(b)
            }
        };
        CollisionPair::new(a_part, b_part)
    }
}