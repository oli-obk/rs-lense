
use {Lense, Dice, DiceMut};

pub struct Aligned<B> {
    state: B,
    len: usize,
}

impl<'a, B> Aligned<B> where B: Dice<'a> {
    pub fn new(b: B) -> Self {
        Aligned { state: b, len: 0 }
    }

    fn align_to(&mut self, size: usize) {
        let offset = self.len % size;

//      debug_assert!(self.align >= size,
//          "Pooly ordered struct found. {} > {}, {}", self.align, size, self.len);

        if offset > 0 {
            debug_assert!(!cfg!(feature = "strict_alignment"),
                "Poorly aligned struct found. {} % {} = {}", self.len, size, offset);

            self.len += offset;

            // Todo advance the pointer without slicing
            match offset {
                1 => unsafe { self.state.dice::<[u8; 1]>(); },
                2 => unsafe { self.state.dice::<[u8; 2]>(); },
                3 => unsafe { self.state.dice::<[u8; 3]>(); },
                4 => unsafe { self.state.dice::<[u8; 4]>(); },
                5 => unsafe { self.state.dice::<[u8; 5]>(); },
                6 => unsafe { self.state.dice::<[u8; 6]>(); },
                7 => unsafe { self.state.dice::<[u8; 7]>(); },
                _ => panic!("Unimplemented offset correction: {}", offset),
            }
        }
    }
}

impl<'a, B> DiceMut<'a> for Aligned<B> where B: DiceMut<'a> {
    #[inline]
    unsafe fn dice_mut<L: Lense>(&mut self) -> &'a mut L {
        self.align_to(L::size());
        self.state.dice_mut()
    }
}

impl<'a, B> Dice<'a> for Aligned<B> where B: Dice<'a> {
    #[inline]
    unsafe fn dice<L: Lense>(&mut self) -> &'a L {
        self.align_to(L::size());
        self.state.dice()
    }

    #[inline]
    fn size(&self) -> usize {
        self.state.size()
    }
}
