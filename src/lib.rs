#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::fmt;
use std::hash;

mod baseline;
mod specialized;
mod table;

#[derive(Clone)]
enum State {
    Baseline(baseline::State),
    Specialized(specialized::State),
}

#[derive(Clone)]
pub struct Hasher {
    state: State,
}

impl Hasher {
    pub fn new() -> Self {
        Self::internal_new_specialized().unwrap_or_else(|| Self::internal_new_baseline())
    }

    #[doc(hidden)]
    pub fn internal_new_baseline() -> Self {
        Hasher {
            state: State::Baseline(baseline::State::new()),
        }
    }

    #[doc(hidden)]
    pub fn internal_new_specialized() -> Option<Self> {
        {
            if let Some(state) = specialized::State::new() {
                return Some(Hasher {
                    state: State::Specialized(state),
                });
            }
        }
        None
    }

    pub fn update(&mut self, buf: &[u8]) {
        match self.state {
            State::Baseline(ref mut state) => state.update(buf),
            State::Specialized(ref mut state) => state.update(buf),
        }
    }

    pub fn finalize(self) -> u32 {
        match self.state {
            State::Baseline(state) => state.finalize(),
            State::Specialized(state) => state.finalize(),
        }
    }
}

impl fmt::Debug for Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("crc32fast::Hasher").finish()
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl hash::Hasher for Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes)
    }

    fn finish(&self) -> u64 {
        self.clone().finalize() as u64
    }
}
