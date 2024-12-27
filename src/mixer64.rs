use rand_core::{RngCore, SeedableRng};

#[derive(Clone, Copy, Debug, Default)]
pub struct Seed64(pub [u8; 8]);

#[derive(Clone, Copy, Debug, Default)]
pub struct Mixer64 {
    state: u64,
}

impl AsRef<[u8]> for Seed64 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl AsMut<[u8]> for Seed64 {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
}

impl SeedableRng for Mixer64 {
    type Seed = Seed64;

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            state: u64::from_ne_bytes(seed.0),
        }
    }
}

impl RngCore for Mixer64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut k = self.state;

        k ^= k >> 33;
        k = k.wrapping_mul(0xff51_afd7_ed55_8ccd);
        k ^= k >> 33;
        k = k.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        k ^= k >> 33;

        self.state = k;

        k
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
