use {Lense, SliceRef, SliceMut, Dice, AlignedPool, Aligned};

// Want a random access state machine that locks pool positions and to enable partitioning for
// variable length data.

pub struct SeekablePool<L: Lense> {
    // Backing u64 pool
    pool: AlignedPool<L>,
    // Index cur is in pool
    cur: usize,
    // Lock state
    state: Vec<bool>,
}

impl<'a, L: SliceMut<'a>> SeekablePool<L> {
    fn new() -> Self {
        SeekablePool {
            pool: AlignedPool::with_capacity(3),
            cur: 0,
            state: Vec::with_capacity(3),
        }
    }
}

impl<'a, L: SliceMut<'a>> Iterator for SeekablePool<L> {
    type Item = L::Mut;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = ((self.pool.as_ptr() as usize) + (L::size() * self.cur)) as *mut u8;
        self.cur += 1;

        unsafe {
            Some(L::slice_mut(&mut ::std::slice::from_raw_parts_mut(ptr, L::size())))
        }
    }
}

pub struct Entry<L> {
    entry: L,
    index: usize,
}
