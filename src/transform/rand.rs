use rand::{Rng, SeedableRng};

/// The RNG type used by this crate.
///
/// Exists as a type alias to make changing RNG implementation easier.
pub type CrateRng = rand_pcg::Pcg32;

#[derive(Clone)]
pub struct Seed([u8; 16]);

impl Seed {
    /// Generates a 128-bit seed from a 32-bit seed
    ///
    /// This is done via two steps:
    ///
    /// 1. Create a low quality 128-bit seed (LQS)
    ///
    ///    This is done with simple bit manipulation of the 32-bit seed.
    ///
    /// 2. Create a higher quality 128-bit seed (HQS)
    ///
    ///    This is done by seeding an Rng with the LQS then using the Rng to generate the HQS.
    pub fn from_u32(seed: u32) -> Seed {
        let mut rng = CrateRng::from_seed(
            Seed::from_u32_array([
                seed ^ 0xa5a5_a5a5,
                seed ^ 0x5a5a_5a5a,
                seed ^ 0x5555_5555,
                seed ^ 0xaaaa_aaaa,
            ]).0,
        );

        Seed::from_u32_array([rng.gen(), rng.gen(), rng.gen(), rng.gen()])
    }

    pub fn from_u32_array(x: [u32; 4]) -> Seed {
        Seed([
             (x[0] >>  0) as u8,
             (x[0] >>  8) as u8,
             (x[0] >> 16) as u8,
             (x[0] >> 24) as u8,
             (x[1] >>  0) as u8,
             (x[1] >>  8) as u8,
             (x[1] >> 16) as u8,
             (x[1] >> 24) as u8,
             (x[2] >>  0) as u8,
             (x[2] >>  8) as u8,
             (x[2] >> 16) as u8,
             (x[2] >> 24) as u8,
             (x[3] >>  0) as u8,
             (x[3] >>  8) as u8,
             (x[3] >> 16) as u8,
             (x[3] >> 24) as u8,
        ])
    }

    pub fn to_rng(&self) -> CrateRng {
        CrateRng::from_seed(self.0)
    }
}

impl Default for Seed {
    fn default() -> Seed {
        Seed::from_u32(0)
    }
}
