use {Lense, Dice, DiceMut};

pub struct Aligned<D> {
    state: D,
    len: usize,
}

impl<'a, D> Aligned<D> where D: Dice<'a> {
    #[cfg(not(feature = "automatic_padding"))]
    pub fn new(b: D) -> D {
        b
    }

    #[cfg(feature = "automatic_padding")]
    pub fn new(b: D) -> Aligned<D> {
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
                1 => { self.dice::<[u8; 1]>(); }
                2 => { self.dice::<[u8; 2]>(); }
                3 => { self.dice::<[u8; 3]>(); }
                4 => { self.dice::<[u8; 4]>(); }
                5 => { self.dice::<[u8; 5]>(); }
                6 => { self.dice::<[u8; 6]>(); }
                7 => { self.dice::<[u8; 7]>(); }
                _ => panic!("Unimplemented offset correction: {}", offset),
            }
        }
    }

    fn waste(&self) -> usize {
        0
    }
}

impl<'a, D> DiceMut<'a> for Aligned<D> where D: DiceMut<'a> {
    #[inline]
    fn dice_mut<L: Lense>(&mut self) -> &'a mut L {
        self.align_to(L::size());
        self.state.dice_mut()
    }
}

impl<'a, D> Dice<'a> for Aligned<D> where D: Dice<'a> {
    #[inline]
    fn dice<L: Lense>(&mut self) -> &'a L {
        self.align_to(L::size());
        self.state.dice()
    }

    #[inline]
    fn size(&self) -> usize {
        self.state.size() + self.waste()
    }
}
