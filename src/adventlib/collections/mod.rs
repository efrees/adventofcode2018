extern crate slab;

use self::slab::Slab;

pub struct CircleList<T> {
    nodes: Slab<CircleListNode<T>>,
    pub last: Option<CircleListPointer>,
}

struct CircleListNode<T> {
    value: T,
    next_key: usize,
    prev_key: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct CircleListPointer(usize);

impl<T> CircleList<T> {
    pub fn new() -> CircleList<T> {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> CircleList<T> {
        CircleList::<T> {
            nodes: Slab::with_capacity(capacity),
            last: Option::None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.last {
            None => {
                let next_slot = self.nodes.vacant_entry();
                let next_key = next_slot.key();
                next_slot.insert(CircleListNode {
                    value,
                    next_key: next_key,
                    prev_key: next_key,
                });
                self.last = Some(CircleListPointer(next_key));
            }
            Some(cur_ptr) => self.insert_after(cur_ptr, value),
        };
    }

    pub fn insert_after(&mut self, cur_ptr: CircleListPointer, value: T) {
        let next_key = self.nodes[cur_ptr.0].next_key;
        let prev_key = cur_ptr.0;
        let new_key = self.nodes.insert(CircleListNode {
            value,
            next_key: next_key,
            prev_key: prev_key,
        });

        self.nodes[next_key].prev_key = new_key;
        self.nodes[prev_key].next_key = new_key;
        self.last = Some(CircleListPointer(new_key));
    }

    pub fn next_node(&self, cur_ptr: CircleListPointer) -> CircleListPointer {
        let cur_node = &self.nodes[cur_ptr.0];
        CircleListPointer(cur_node.next_key)
    }

    pub fn prev_node(&self, cur_ptr: CircleListPointer) -> CircleListPointer {
        let cur_node = &self.nodes[cur_ptr.0];
        CircleListPointer(cur_node.prev_key)
    }

    pub fn remove(&mut self, cur_ptr: CircleListPointer) -> T {
        let rem_node = self.nodes.remove(cur_ptr.0);
        if rem_node.prev_key != rem_node.next_key {
            self.nodes[rem_node.next_key].prev_key = rem_node.prev_key;
            self.nodes[rem_node.prev_key].next_key = rem_node.next_key;
            self.last = Some(CircleListPointer(rem_node.next_key));
        } else {
            self.last = None;
        }

        return rem_node.value;
    }
}

impl<T> CircleList<T>
where
    T: Copy,
{
    pub fn get_value(&self, cur_ptr: CircleListPointer) -> Option<T> {
        return match self.nodes.get(cur_ptr.0) {
            Some(node) => Some(node.value),
            None => None,
        };
    }
}
