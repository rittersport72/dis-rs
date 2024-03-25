pub const ONE_BIT: usize = 1;
pub const TWO_BITS: usize = 2;
pub const THREE_BITS: usize = 3;
pub const FOUR_BITS: usize = 4;
pub const FIVE_BITS: usize = 5;
pub const SIX_BITS: usize = 6;
pub const SEVEN_BITS: usize = 7;
pub const EIGHT_BITS: usize = 8;
pub const NINE_BITS: usize = 9;
pub const THIRTEEN_BITS: usize = 14;
pub const FOURTEEN_BITS: usize = 14;
pub const TWENTY_SIX_BITS: usize = 26;
pub const LEAST_SIGNIFICANT_BIT : u32 = 0x001;
pub const NANOSECONDS_PER_HOUR: u32 = 3600 * 1e6 as u32;
pub const CDIS_TIME_UNITS_PER_HOUR: u32 = (2^25) - 1;
pub const CDIS_NANOSECONDS_PER_TIME_UNIT: f32 = NANOSECONDS_PER_HOUR as f32 / CDIS_TIME_UNITS_PER_HOUR as f32;
pub const MTU_BYTES : usize = 1500;
pub const MTU_BITS : usize = MTU_BYTES * EIGHT_BITS;