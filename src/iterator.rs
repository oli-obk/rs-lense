use {Lense, slice_lense_chunk};

pub struct LenseIterator<'a, L: 'a + Lense<'a>> {
    buf: &'a mut [u8],
    _lense: ::std::marker::PhantomData<&'a L>,
}

#[cfg(feature = "remaining")]
impl<'a, L> LenseIterator<'a, L> where L: Lense<'a> {
    fn remaining(self) -> Option<&'a mut [u8]> {
        if self.buf.len() == 0 {
            None
        } else {
            Some(self.buf)
        }
    }
}    

pub trait LenseIteratable<'a>: Lense<'a> + Sized {
    fn from_buf(mut buf: &'a mut [u8]) -> LenseIterator<'a, Self> {
        LenseIterator {
            buf: buf,
            _lense: ::std::marker::PhantomData,
        }
    }
}

impl<'a, L> Iterator for LenseIterator<'a, L> where L: Lense<'a> {
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.len() {
            0 => None,
            x if x < L::size() => None,
            _ => Some(slice_lense_chunk(&mut self.buf)),
        }
    }
}

impl<'a, L> ExactSizeIterator for LenseIterator<'a, L> where L: Lense<'a> {
    fn len(&self) -> usize {
        self.buf.len() / L::size()
    }
}
