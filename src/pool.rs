use {Lense, SliceRef, SliceMut, Dice, Iter}; //, IterMut};

pub struct AlignedPool<L: Lense> {
    buf: Vec<u64>,
    len: usize,
    marker: ::std::marker::PhantomData<*const L>,
}

fn div_up(n: usize, m: usize) -> usize {
    if n % m == 0 {
        n / m
    } else {
        n / m + 1
    }
}

impl<L> AlignedPool<L> where L: Lense {
    pub fn with_capacity(mut cap: usize) -> Self {
        cap *= L::size();

        AlignedPool {
            buf: vec![0u64; div_up(cap, 8)],
            len: cap,
            marker: ::std::marker::PhantomData,
        }
    }

    pub fn iter<'a>(&'a mut self) -> Iter<L>
        where L: SliceRef<'a>
    {
        Iter::from_aligned_pool(self)
    }

//  pub fn iter_mut<'a>(&mut self) -> IterMut<'a, L>
//      where L: SliceMut<'a>
//  {
//      IterMut::from_aligned_pool(self)
//  }
}

impl<L> ::std::ops::Deref for AlignedPool<L> where L: Lense {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.buf.as_ptr() as *const u8, self.len) }
    }
}

impl<L> ::std::ops::DerefMut for AlignedPool<L> where L: Lense {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { ::std::slice::from_raw_parts_mut(self.buf.as_ptr() as *mut u8, self.len) }
    }
}
