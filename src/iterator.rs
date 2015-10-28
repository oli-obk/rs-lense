use {LenseRef, LenseMut, Dice, DiceMut, AlignedPool, Aligned};

pub struct Iter<'a, L: 'a + LenseRef<'a>> {
    pool: Aligned<&'a [u8]>,
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, L> Iter<'a, L> where L: LenseRef<'a> {
    pub fn from_aligned_pool(pool: &'a mut AlignedPool<'a, L>) -> Self {
        Iter {
            pool: Aligned::new(&**pool),
            _ty: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for Iter<'a, L> where L: LenseRef<'a> {
    type Item = L::Ref;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pool.size() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(L::slice(&mut self.pool)),
        }
    }
}

impl<'a, L> ExactSizeIterator for Iter<'a, L> where L: LenseRef<'a> {
    fn len(&self) -> usize {
        self.pool.size() / L::size()
    }
}

pub struct IterMut<'a, L: 'a + LenseMut<'a>> {
    pool: Aligned<&'a mut [u8]>,
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, L> IterMut<'a, L> where L: LenseMut<'a> {
    pub fn from_aligned_pool(pool: &'a mut AlignedPool<'a, L>) -> Self {
        IterMut {
            pool: Aligned::new(&mut **pool),
            _ty: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for IterMut<'a, L> where L: LenseMut<'a> {
    type Item = L::Mut;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pool.size() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(L::slice_mut(&mut self.pool)),
        }
    }
}

impl<'a, L> ExactSizeIterator for IterMut<'a, L> where L: LenseMut<'a> {
    fn len(&self) -> usize {
        self.pool.size() / L::size()
    }
}
