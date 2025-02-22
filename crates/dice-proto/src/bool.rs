use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(repr = i8)]
pub enum DiceBool {
    False = 0,
    True = 1,
}
