use rand_core::{RngCore, SeedableRng};

#[derive(Clone, Copy, Debug)]
pub struct Mixer32 {
    state: u32,
}

impl SeedableRng for Mixer32 {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            state: u32::from_ne_bytes(seed),
        }
    }
}

impl RngCore for Mixer32 {
    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(0xcc9e2d51)
            .rotate_left(15)
            .wrapping_mul(0x1b873593);

        self.state
    }

    fn next_u64(&mut self) -> u64 {
        rand_core::impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
