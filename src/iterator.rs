
use {LenseRef, LenseMut, Dice, DiceMut};

pub struct Iter<'a, B: Dice<'a>, L: 'a + LenseRef<'a>> {
    buf: B,
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, B, L> Iter<'a, B, L> where L: LenseRef<'a>, B: Dice<'a> {
    pub fn new(buf: B) -> Self {
        Iter {
            buf: buf,
            _ty: ::std::marker::PhantomData,
        }
    }
}

impl<'a, B, L> Iterator for Iter<'a, B, L> where L: LenseRef<'a>, B: Dice<'a> {
    type Item = L::Ref;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.len() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(L::slice(&mut self.buf)),
        }
    }
}

impl<'a, B, L> ExactSizeIterator for Iter<'a, B, L> where L: LenseRef<'a>, B: Dice<'a> {
    fn len(&self) -> usize {
        self.buf.len() / L::size()
    }
}


pub struct IterMut<'a, B: DiceMut<'a>, L: 'a + LenseMut<'a>> {
    buf: B,
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, B, L> IterMut<'a, B, L> where L: LenseMut<'a>, B: DiceMut<'a> {
    pub fn new(buf: B) -> Self {
        IterMut {
            buf: buf,
            _ty: ::std::marker::PhantomData,
        }
    }
}

impl<'a, B, L> Iterator for IterMut<'a, B, L> where L: LenseMut<'a>, B: DiceMut<'a> {
    type Item = L::Mut;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.len() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(L::slice_mut(&mut self.buf)),
        }
    }
}

impl<'a, B, L> ExactSizeIterator for IterMut<'a, B, L> where L: LenseMut<'a>, B: DiceMut<'a> {
    fn len(&self) -> usize {
        self.buf.len() / L::size()
    }
}
