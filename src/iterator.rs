// Aligned iterators

use {Lense, SliceRef, SliceMut, Dice, AlignedPool, Aligned};

pub struct Iter<'a, L: Lense> {
    pool: Aligned<&'a [u8]>,
    marker: ::std::marker::PhantomData<*const L>,
}

impl<'a, L> Iter<'a, L> where L: SliceRef<'a> {
    pub fn from_aligned_pool(pool: &'a mut AlignedPool<L>) -> Self {
        Iter {
            pool: Aligned::new(&**pool),
            marker: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for Iter<'a, L> where L: SliceRef<'a> {
    type Item = L::Ref;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.pool.size() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(unsafe { L::slice(&mut self.pool) }),
        }
    }
}

impl<'a, L> ExactSizeIterator for Iter<'a, L> where L: SliceRef<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.pool.size() / L::size()
    }
}


pub struct IterMut<'a, L: Lense> {
    pool: Aligned<&'a mut [u8]>,
    marker: ::std::marker::PhantomData<*const L>,
}

impl<'a, L> IterMut<'a, L> where L: SliceMut<'a> {
    pub fn from_aligned_pool(pool: &'a mut AlignedPool<L>) -> Self {
        IterMut {
            pool: Aligned::new(&mut **pool),
            marker: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for IterMut<'a, L> where L: SliceMut<'a> {
    type Item = L::Mut;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.pool.size() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(unsafe { L::slice_mut(&mut self.pool) }),
        }
    }
}

impl<'a, L> ExactSizeIterator for IterMut<'a, L> where L: SliceMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.pool.size() / L::size()
    }
}
