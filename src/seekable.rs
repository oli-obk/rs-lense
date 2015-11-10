use std::cell::Cell;

use {Lense, SliceRef, SliceMut, Dice};
use aligned::Aligned;

// Want a random access state machine that locks pool positions and to enable partitioning for
// variable length data.

pub struct SeekablePool<'a, L: 'a + Lense> {
    // Backing u64 pool
    pool: Vec<u64>,
    // Lock state
    state: Vec<Cell<bool>>,
    // Lifetime and lense type
    marker: ::std::marker::PhantomData<&'a L>,
}

fn div_up(n: usize, m: usize) -> usize {
    if n % m == 0 {
        n / m
    } else {
        n / m + 1
    }
}

impl<'a, L: Lense> SeekablePool<'a, L> {
    pub fn with_capacity(cap: usize) -> Self {
        SeekablePool {
            pool: vec![0u64; div_up(cap * L::size(), 8)],
            state: vec![Cell::new(false); cap],
            marker: ::std::marker::PhantomData,
        }
    }

    fn lense(&mut self, pos: usize) -> Option<Guard<L::Ref>> where L: SliceRef<'a> {
        match self.state.get_mut(pos) {
            Some(ref mut lock) if !lock.get() => {
                let ref mut ptr = Aligned::new(unsafe { // &self[L::size() * pos .. L::size()]
                    ::std::slice::from_raw_parts(
                        (self.pool.as_ptr() as *const u8).offset((L::size() * pos) as isize),
                        L::size())
                });

                lock.set(true);

                Some(Guard(lock.clone(), L::slice(ptr)))
            }
            Some(..) => None,
            None => panic!("Invalid index! {}", pos),
        }
    }

    fn lense_mut(&mut self, pos: usize) -> Option<Guard<L::Mut>> where L: SliceMut<'a> {
        match self.state.get_mut(pos) {
            Some(ref mut lock) if !lock.get() => {
                let ref mut ptr = Aligned::new(unsafe { // &mut self[L::size() * pos .. L::size()]
                    ::std::slice::from_raw_parts_mut(
                        (self.pool.as_mut_ptr() as *mut u8).offset((L::size() * pos) as isize),
                        L::size())
                });

                lock.set(true);

                Some(Guard(lock.clone(), L::slice_mut(ptr)))
            }
            Some(..) => None,
            None => panic!("Invalid index! {}", pos),
        }
    }

    pub fn iter(&'a mut self) -> IterRef<'a, L> where L: SliceRef<'a> {
        IterRef { pool: self, cur: 0 }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, L> where L: SliceMut<'a> {
        IterMut { pool: self, cur: 0 }
    }
}

impl<'a, L: Lense> ::std::ops::Deref for SeekablePool<'a, L> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { // Vec<u64> -> &[u8]
            ::std::slice::from_raw_parts(self.pool.as_ptr() as *const u8,
                                         self.state.capacity() * L::size())
        }
    }
}

impl<'a, L: Lense> ::std::ops::DerefMut for SeekablePool<'a, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { // Vec<u64> -> &mut [u8]
            ::std::slice::from_raw_parts_mut(self.pool.as_mut_ptr() as *mut u8,
                                             self.state.capacity() * L::size())
        }
    }
}

// Guard the lense such that when it drops, we free the position again.

pub struct Guard<T>(Cell<bool>, T);

impl<T> Drop for Guard<T> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

impl<T> ::std::ops::Deref for Guard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> ::std::ops::DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

// Should iterators be reserved for lense_vector?

pub struct IterRef<'a, L: 'a + SliceRef<'a>> {
    pool: &'a mut SeekablePool<'a, L>,
    cur: usize,
}

impl<'a, L: SliceRef<'a>> Iterator for IterRef<'a, L> {
    type Item = Guard<L::Ref>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.pool.state.len() {
            self.pool.lense(self.cur).map(|ret| {
                self.cur += 1;
                ret
            })
        } else { None }
    }
}

impl<'a, L: SliceRef<'a>> ExactSizeIterator for IterRef<'a, L> {
    fn len(&self) -> usize {
        self.pool.state.capacity()
    }
}

pub struct IterMut<'a, L: 'a + SliceMut<'a>> {
    pool: &'a mut SeekablePool<'a, L>,
    cur: usize,
}

impl<'a, L: SliceMut<'a>> Iterator for IterMut<'a, L> {
    type Item = Guard<L::Mut>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.pool.state.len() {
            self.pool.lense_mut(self.cur).map(|ret| {
                self.cur += 1;
                ret
            })
        } else { None }
    }
}

impl<'a, L: SliceMut<'a>> ExactSizeIterator for IterMut<'a, L> {
    fn len(&self) -> usize {
        self.pool.state.capacity()
    }
}
