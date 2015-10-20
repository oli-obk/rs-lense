
use Lense;

pub struct AlignedPool<'a, L> where L: 'a + Lense<'a> {
    buf: Vec<u64>,
    _ty: ::std::marker::PhantomData<&'a L>,
}

impl<'a, L> AlignedPool<'a, L> where L: Lense<'a> {
    pub fn with_capacity(mut size: usize) -> Self {
        size = size.next_power_of_two();
        AlignedPool {
            buf: vec![0u64; (L::size() * size) / 8],
            _ty: ::std::marker::PhantomData,
        }
    }

//  pub fn as_mut_vec(&mut self) -> &mut Vec<u64> {
//      &mut self.buf
//  }
}

impl<'a, L> ::std::ops::Deref for AlignedPool<'a, L> where L: Lense<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            ::std::slice::from_raw_parts(self.buf.as_ptr() as *const u8, self.buf.len() * 8)
        }
    }
}

impl<'a, L> ::std::ops::DerefMut for AlignedPool<'a, L> where L: Lense<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            ::std::slice::from_raw_parts_mut(self.buf.as_ptr() as *mut u8, self.buf.len() * 8)
        }
    }
}
