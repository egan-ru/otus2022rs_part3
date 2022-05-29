//! generic Double linked list type
//! do not contain atomic primitives,
//! so if used in multi-threaded code,
//! access should be protected by mutex
//! crate uses default, that have issues in rust
//! <https://github.com/rust-lang/rust/issues/73014>
#![feature(default_free_fn)]
use core::default::default;
use core::ptr::null_mut;

/// double linked list element
///
/// -`next`  - next element
/// -`prev`  - prev element
/// -`data`  - linked list payload
#[repr(C)]
pub struct Dll<T> {
    pub next: *mut Dll<T>,
    pub prev: *mut Dll<T>,
    pub data: T,
}

impl<T: Clone + Default> Dll<T> {
    /// Double linked list ctor
    /// data field will be zeroed
    /// -`return`  - new double linked list instance
    /// care, for complex types must be relinked,
    /// after higher level ctor
    pub fn new() -> Self {
        let mut the_dll: Self = Self {
            data: default(),
            next: null_mut(),
            prev: null_mut(),
        };
        /* link entry to self */
        the_dll.next = &mut the_dll;
        the_dll.prev = &mut the_dll;

        the_dll
    }

    /// Double linked list ctor
    /// data field will be taken from arg
    /// -`data`    - data to init
    /// -`return`  - new double linked list instance
    pub fn from(init_data: T) -> Self {
        let mut the_dll: Self = Self {
            data: init_data,
            /* can't here take address of the_dll */
            next: null_mut(),
            prev: null_mut(),
        };
        /* link entry to self */
        the_dll.next = &mut the_dll;
        the_dll.prev = &mut the_dll;

        the_dll
    }

    /// Return true, if linked list node
    /// is last
    ///
    pub fn is_last(&mut self) -> bool {
        self.next == self.prev
    }

    /// Add node to head
    ///
    /// -`nhead`  - node to add into head
    pub fn addh(&mut self, nhead: &mut Self) {
        let ohead: *mut Self = self.next;

        unsafe {
            nhead.next = ohead;
            nhead.prev = self;
            (*ohead).prev = nhead;
            self.next = nhead;
        }
    }

    /// Add node to tail
    ///
    /// -`ntail`    - node to add into tail
    pub fn addt(&mut self, ntail: &mut Self) {
        let otail: *mut Self = self.prev;

        unsafe {
            ntail.next = self;
            ntail.prev = otail;
            (*otail).next = ntail;
            self.prev = ntail;
        }
    }

    /// Delete node, must be called in drop,
    /// to keep the chain linked
    pub fn delete(self) {
        unsafe {
            (*(self.prev)).next = self.next;
            (*(self.next)).prev = self.prev;
        }
    }

    /// Unlinke node
    pub fn unlink(&mut self) {
        unsafe {
            (*(self.prev)).next = self.next;
            (*(self.next)).prev = self.prev;
            /* link entry to self */
            self.next = self;
            self.prev = self;
        }
    }

    /// Relink node
    pub fn relink(&mut self) {
        /* link entry to self */
        self.next = self;
        self.prev = self;
    }
}

impl<T> Drop for Dll<T> {
    fn drop(&mut self) {
        /* keep the chain linked */
        unsafe {
            (*(self.prev)).next = self.next;
            (*(self.next)).prev = self.prev;
        }
    }
}

/* linked list have special clone rules */
impl<T: Clone> Clone for Dll<T> {
    fn clone(&self) -> Self {
        let mut new_dll: Self = Self {
            data: self.data.clone(),
            /* can't here take address of new_dll */
            next: null_mut(),
            prev: null_mut(),
        };
        new_dll.next = &mut new_dll;
        new_dll.prev = &mut new_dll;

        new_dll
    }
}

impl<T: Clone + Default> Default for Dll<T> {
    fn default() -> Self {
        let mut the_dll: Self = Self {
            data: default(),
            next: null_mut(),
            prev: null_mut(),
        };
        /* link entry to self */
        the_dll.next = &mut the_dll;
        the_dll.prev = &mut the_dll;

        the_dll
    }
}

#[cfg(test)]
mod tests {
    use crate::Dll;
    #[test]
    fn linked_list_tests() {
        /* generate nurse */
        let mut nurse: Dll<u32> = Dll::new();
        nurse.data = 0x00;

        /* generate 4 nums */
        let mut ll_1: Dll<u32> = Dll::new();
        ll_1.data = 0x01;
        let mut ll_2: Dll<u32> = Dll::new();
        ll_2.data = 0x02;
        let mut ll_3: Dll<u32> = Dll::new();
        ll_3.data = 0x03;
        let mut ll_4: Dll<u32> = Dll::new();
        ll_4.data = 0x04;

        nurse.addh(&mut ll_1);
        nurse.addh(&mut ll_2);
        nurse.addh(&mut ll_3);
        nurse.addh(&mut ll_4);

        ll_2.unlink();
    }
}
