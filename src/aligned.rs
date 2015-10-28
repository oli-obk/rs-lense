use {Lense, Dice, DiceMut};

pub struct Aligned<B> {
    state: B,
    len: usize,
}

impl<'a, B> Aligned<B> where B: Dice<'a> {
    pub fn new(b: B) -> Self {
        Aligned {
            state: b,
            len: 0,
        }
    }

    fn align_to<L: Lense<'a>>(&mut self) {
        let offset = self.len % L::size();
        if cfg!(feature = "strict_alignment") {
            debug_assert!(self.len == 0 || offset == 0,
                          "Poorly aligned struct found. Offset {}", offset)
        }
        match offset {
            0 => { println!("Padding applied") },
            1 => { self.slice::<[u8; 1]>(); },
            2 => { self.slice::<[u8; 2]>(); },
            3 => { self.slice::<[u8; 3]>(); },
            4 => { self.slice::<[u8; 4]>(); },
            5 => { self.slice::<[u8; 5]>(); },
            6 => { self.slice::<[u8; 6]>(); },
            7 => { self.slice::<[u8; 7]>(); },
            _ => panic!("Unimplemented offset correction: {}", offset),
        }
    }
}

impl<'a, B> DiceMut<'a> for Aligned<B> where B: DiceMut<'a> {
    fn slice_mut<L: Lense<'a>>(&mut self) -> &'a mut L {
        self.align_to::<L>();
        self.len += L::size();
        self.state.slice_mut()
    }
}

impl<'a, B> Dice<'a> for Aligned<B> where B: Dice<'a> {
    fn slice<L: Lense<'a>>(&mut self) -> &'a L {
        self.align_to::<L>();
        self.len += L::size();
        self.state.slice()
    }

    fn size(&self) -> usize {
        self.state.size()
    }
}
