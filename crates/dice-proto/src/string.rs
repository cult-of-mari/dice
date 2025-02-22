use binrw::{BinRead, BinWrite};
use derive_more::{Display, Error};
use std::str::FromStr;

#[derive(Debug, Display, Error)]
#[display("invalid length")]
pub struct DiceStringError;

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(big)]
pub struct DiceString {
    #[br(assert(len >= 0, "len cannot be negative"))]
    len: i16,
    #[br(count = len as usize)]
    data: Vec<u16>,
}

impl DiceString {
    pub const MAX_LEN: usize = i16::MAX as usize;

    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(&self.data)
    }
}

impl FromStr for DiceString {
    type Err = DiceStringError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.len() < Self::MAX_LEN {
            let data: Vec<_> = string.encode_utf16().collect();

            assert!(data.len() <= string.len());

            Ok(Self {
                len: data.len() as i16,
                data,
            })
        } else {
            Err(DiceStringError)
        }
    }
}
