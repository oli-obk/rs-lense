use {LenseRef, LenseMut, Dice, DiceMut, AlignedPool};

pub struct Iter<'a, L: 'a + LenseRef<'a>> {
    pool: &'a [u8],
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, L> Iter<'a, L> where L: LenseRef<'a> {
    pub fn new(pool: &'a mut AlignedPool<'a, L>) -> Self {
        Iter {
            pool: &**pool,
            _ty: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for Iter<'a, L> where L: LenseRef<'a> {
    type Item = L::Ref;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pool.len() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(L::slice(&mut self.pool)),
        }
    }
}

impl<'a, L> ExactSizeIterator for Iter<'a, L> where L: LenseRef<'a> {
    fn len(&self) -> usize {
        self.pool.len() / L::size()
    }
}

pub struct IterMut<'a, L: 'a + LenseMut<'a>> {
    pool: &'a mut [u8],
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, L> IterMut<'a, L> where L: LenseMut<'a> {
    pub fn new(pool: &'a mut AlignedPool<'a, L>) -> Self {
        IterMut {
            pool: &mut **pool,
            _ty: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for IterMut<'a, L> where L: LenseMut<'a> {
    type Item = L::Mut;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pool.len() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(L::slice_mut(&mut self.pool)),
        }
    }
}

impl<'a, L> ExactSizeIterator for IterMut<'a, L> where L: LenseMut<'a> {
    fn len(&self) -> usize {
        self.pool.len() / L::size()
    }
}
