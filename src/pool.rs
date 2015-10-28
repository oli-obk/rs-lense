use {Lense, LenseRef, LenseMut, Dice, Iter, IterMut};

pub struct AlignedPool<'a, L> where L: 'a + Lense<'a> {
    buf: Vec<u64>,
    len: usize,
    _ty: ::std::marker::PhantomData<&'a L>,
}

fn div_up(n: usize, m: usize) -> usize {
    if n % m == 0 {
        n / m
    } else {
        n / m + 1
    }
}

impl<'a, L> AlignedPool<'a, L> where L: Lense<'a> {
    pub fn with_capacity(mut cap: usize) -> Self {
        debug_assert!(cap * L::size() % 8 == 0,
                      "Implementation limitation: expected capacity of {}, implementation", cap);
        cap *= L::size();
        AlignedPool {
            buf: vec![0u64; div_up(cap, 8)],
            len: cap,
            _ty: ::std::marker::PhantomData,
        }
    }

    pub fn iter(&'a mut self) -> Iter<'a, L> where L: LenseRef<'a> {
        Iter::from_aligned_pool(self)
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, L> where L: LenseMut<'a> {
        IterMut::from_aligned_pool(self)
    }
}

impl<'a, L> ::std::ops::Deref for AlignedPool<'a, L> where L: Lense<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            ::std::slice::from_raw_parts(self.buf.as_ptr() as *const u8, self.len)
        }
    }
}

impl<'a, L> ::std::ops::DerefMut for AlignedPool<'a, L> where L: Lense<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            ::std::slice::from_raw_parts_mut(self.buf.as_ptr() as *mut u8, self.len)
        }
    }
}
