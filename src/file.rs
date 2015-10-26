#![cfg(test)]

use std::fs::File;
use std::io::{self, Read}; //, Seek, SeekFrom};
//use std::collections::HashMap;

use {AlignedPool, Lense};

enum PoolPolicy {
    Strict, // Do not allocate more memory when the pool runs out of storage.
}

/*
#[derive(Clone)]
enum CacheEntry {
    Locked(usize),   // A mutable lense is active
    Readonly(usize), // An immutable lense is active
    Unlocked(usize), // An entry exists and is not in use
}*/

pub struct LenseFile<'a, L> where L: 'a + Lense<'a> {
    // Backing file
    file: File,
    // Pool for storing requested entries
    pool: AlignedPool<'a, L>,
    // Cache mapping file indexes to pool entries
//  cache: HashMap<usize, CacheEntry>,
    // Pool policy
    policy: PoolPolicy,
}

impl<'a, L> LenseFile<'a, L> where L: Lense<'a> {
    pub fn from_file(file: File, cap: usize) -> Self {
        LenseFile {
            file: file,
            pool: AlignedPool::with_capacity(cap),
//          cache: HashMap::with_capacity(cap),
            policy: PoolPolicy::Strict,
        }
    }

    // populate the pool with some entries
    pub fn init(&mut self) -> io::Result<usize> {
        match self.policy {
            PoolPolicy::Strict => self.file.read(&mut *self.pool),
        }
    }

    pub fn pool(&mut self) -> &mut AlignedPool<'a, L> {
        &mut self.pool
    }

//  Lock when leasing lenses.
//  Lenses may update the disk state.
//  An unlocked entry can be freely updated without a read first.
//  An entry may be replaced with another - write new, return old.
//  Appending to the statefile is cheap - seek to the end, write.
//
//  Overwriting pool and cached data can only occur in positions that are not locked.
//  If the pool runs out of allocated storage and no positions are unlocked, then either allocate a
//  second pool or complain to the consumer that too many entries are currently active. (Depending
//  on policy)
//
//  The entire state may be maintained in ram with disk writes occurring only on snapshot requests.
//
//  pool_policy    // Can the pool allocate more ram when it has no unused slots and is requested
//                 // for an entry in the persistant store.
//
//  with_pool_size // Configure ram backing pool size
//  cache_policy   // Configure cache policy: Frequency, Sequence (read-a-head)
//                 // Frequency: Store entries frequently requested (weighted)
//                 // Sequence: Expecting sequential reads. N, N+1 .. M
//
//  snapshot       // Save the current state to another file
//
//  [Entry functions] // pool may also implement these for quick snapshot management.
//
//  update_cache // Update ram value
//  update_store // Update persistant value
//
//  flush // Push cache to persistant storage
//  reset // Ignore cache, recover persistant values

}

#[cfg(test)]
mod test{
    use std::fs::File;
    use super::*;

    mk_lense_ty!{struct Alice ref
        a:  u8,
        b: (u8, u16),
        c: [u8; 4],
        d: u64,
    }

    #[test]
//  #[ignore]
    fn read_lense_file() {
        let mut f = LenseFile::<Alice>::from_file(File::open("lense-testing-file.dat").unwrap(), 2);

        println!("Reading... {}", f.init().unwrap());

        for Alice { a, b, c, d } in f.pool().iter() {
            println!("{:?} {:?} {:?} {:?}", a, b, c, d);
        }

        if option_env!("lense_debug").is_some() {
            panic!("Test debug mode enabled")
        }
    }
}
