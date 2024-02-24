use deku::ctx::{BitSize, Order};
use deku::prelude::*;

use crate::types::parsed::zPAR_TYPE;
use crate::types::Identifier;

#[derive(DekuRead, DekuWrite, Debug)]
pub struct GothicDat {
    version: u8, // 0x32
    #[deku(update = "self.symbols.len()")]
    pub num_symbols: u32,
    #[deku(count = "num_symbols")]
    symbols_sorted: Vec<u32>,
    #[deku(count = "num_symbols")]
    pub symbols: Vec<DatSymbol>,
    // Stack - this really means bytecode
    #[deku(update = "self.bytecode.len()")]
    bytecode_size: u32,
    #[deku(count = "bytecode_size")]
    pub bytecode: Vec<u8>,
}

#[derive(DekuRead, DekuWrite, Debug)]
#[deku(bit_order = "lsb")]
pub struct DatSymbol {
    named: u32,
    #[deku(until = "|c| *c == 0x0A")]
    pub name: Vec<u8>,
    pub off_cls_ret: u32, // Offset (ClassVar) | Size (Class) | ReturnType (Func)
    #[deku(bits = 12)]
    pub count: u16,
    #[deku(bits = 4)]
    pub typ: SymbolType,
    #[deku(map = "|f: SymbolFlags| -> Result<_, DekuError> { Ok(f & SymbolFlags::all()) }")]
    pub flags: SymbolFlags,
    file_index: u32,
    line_start: u32,
    line_count: u32,
    char_start: u32,
    char_count: u32,
    #[deku(
        ctx = "*count, *typ as u8",
        cond = "*flags & SymbolFlags::ClassVar == SymbolFlags::empty()"
    )]
    pub symbol_data: Option<SymbolSpecific>,
    parent: u32, // 0xFFFFFFFF == None
}

#[derive(DekuRead, DekuWrite, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[deku(type = "u8")]
#[deku(bits = 4, bit_order = "lsb", ctx = "_bits: BitSize, _ctx_lsb: Order")]
pub enum SymbolType {
    Void = 0,
    Float = 1,
    Int = 2,
    String = 3,
    Class = 4,
    Func = 5,
    Prototype = 6,
    Instance = 7,
}

impl SymbolType {
    #[allow(non_snake_case)]
    pub fn to_zPAR_TYPE(self) -> Option<zPAR_TYPE> {
        match self {
            SymbolType::Void => Some(zPAR_TYPE::Void),
            SymbolType::Float => Some(zPAR_TYPE::Float),
            SymbolType::Int => Some(zPAR_TYPE::Int),
            SymbolType::String => Some(zPAR_TYPE::String),
            SymbolType::Func => Some(zPAR_TYPE::Func),
            SymbolType::Instance => Some(zPAR_TYPE::Instance(Identifier::new(b"<inst>", (0, 0)))),
            _ => None,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            val if val == SymbolType::Void as u32 => Some(SymbolType::Void),
            val if val == SymbolType::Float as u32 => Some(SymbolType::Float),
            val if val == SymbolType::Int as u32 => Some(SymbolType::Int),
            val if val == SymbolType::String as u32 => Some(SymbolType::String),
            val if val == SymbolType::Class as u32 => Some(SymbolType::Class),
            val if val == SymbolType::Func as u32 => Some(SymbolType::Func),
            val if val == SymbolType::Prototype as u32 => Some(SymbolType::Prototype),
            val if val == SymbolType::Instance as u32 => Some(SymbolType::Instance),
            _ => None,
        }
    }
}

#[derive(DekuRead, DekuWrite, Debug)]
#[deku(
    bit_order = "ctx_lsb",
    ctx = "ctx_lsb: Order, count: u16, symbol_type: u8",
    id = "symbol_type"
)]
pub enum SymbolSpecific {
    #[deku(id = "0")]
    Empty,
    #[deku(id = "1")]
    FloatData(#[deku(count = "count")] Vec<f32>),
    #[deku(id = "2")]
    IntData(#[deku(count = "count")] Vec<u32>),
    #[deku(id = "3")]
    StringData(#[deku(count = "count")] Vec<DatString>),
    #[deku(id = "4")]
    ClassVar(u32),
    #[deku(id_pat = "5..=7")]
    FuncLike(u32),
}

impl SymbolSpecific {
    pub fn get_funclike(&self) -> Option<u32> {
        if let SymbolSpecific::FuncLike(addr) = self {
            Some(*addr)
        } else {
            None
        }
    }
}

#[derive(DekuRead, DekuWrite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
#[deku(ctx = "_ctx_lsb: Order")]
pub struct SymbolFlags(u16);
bitflags::bitflags! {
    impl SymbolFlags: u16 {
        const Const = 1;
        const Return = 2;
        const ClassVar = 4;
        const External = 8;
        const Merged = 16;
    }
}

impl std::fmt::Debug for SymbolFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}

#[derive(DekuRead, DekuWrite, Debug)]
#[deku(bit_order = "ctx_lsb", ctx = "ctx_lsb: Order")]
pub struct DatString {
    #[deku(until = "|c| *c == 0x0A")]
    data: Vec<u8>,
}

impl GothicDat {
    pub fn parse(data: &[u8]) -> Result<Self, DekuError> {
        let (rest, dat) = GothicDat::from_bytes((data.as_ref(), 0))?;
        if !rest.0.is_empty() {
            panic!("Trailing data");
        }
        Ok(dat)
    }
}

#[cfg(test)]
mod tests {
    use deku::prelude::*;

    use super::{DatSymbol, GothicDat};

    fn data() -> &'static [u8] {
        let dat_file = include_bytes!("C:/Users/Leon/Downloads/GOTHIC.DAT");
        dat_file
    }
    #[test]
    fn foo() -> Result<(), DekuError> {
        let dat_file = data();

        let dat = GothicDat::parse(dat_file)?;

        for (idx, symbol) in dat.symbols.iter().enumerate() {
            let name = String::from_utf8_lossy(&symbol.name);
            println!("({idx}): {name}");
        }

        let to_bytes = dat.to_bytes()?;

        Ok(())
    }

    #[test]
    fn instance_help() -> Result<(), DekuError> {
        let dat_file = data();

        let (_rest, dat) = DatSymbol::from_bytes((dat_file[0x48e71..].as_ref(), 0))?;
        let to_bytes = dat.to_bytes()?;

        // assert!((&dat_file[0x48e71..].starts_with(&to_bytes)));
        println!("{:?}", &dat_file[0x48e71..0x48ea2]);
        println!("{:?}", to_bytes);
        println!("{dat:?}");
        Ok(())
    }
    #[test]
    fn inttostring() -> Result<(), DekuError> {
        let dat_file = data();

        let (_rest, dat) = DatSymbol::from_bytes((dat_file[0x48EA8..].as_ref(), 0))?;
        let to_bytes = dat.to_bytes()?;

        // assert!((&dat_file[0x48e71..].starts_with(&to_bytes)));
        println!("{:?}", &dat_file[0x48EA8..0x48EE8]);
        println!("{:?}", to_bytes);
        println!("{dat:?}");
        Ok(())
    }

    #[test]
    fn inttostring_par0() -> Result<(), DekuError> {
        let dat_file = data();

        let (_rest, dat) = DatSymbol::from_bytes((dat_file[0x48EDC..].as_ref(), 0))?;
        let to_bytes = dat.to_bytes()?;

        // assert!((&dat_file[0x48e71..].starts_with(&to_bytes)));
        println!("{:?}", &dat_file[0x48EDC..0x48EFC]);
        println!("{:?}", to_bytes);
        println!("{dat:?}");
        Ok(())
    }

    #[derive(DekuRead, DekuWrite, Debug)]
    #[deku(bit_order = "lsb")]
    struct DekuTest {
        pad: u8,
        #[deku(bits = 6, pad_bits_after = "10")]
        flag: u16,
        sent: u8,
    }

    #[test]
    fn dekutest() -> Result<(), DekuError> {
        let data = vec![0x13, 0x75, 0x0, 0xFF];
        let (_, dt) = DekuTest::from_bytes((&data, 0))?;
        let to_bytes = dt.to_bytes()?;
        println!("{:?}", data);
        println!("{:?}", to_bytes);
        Ok(())
    }
}

// 0x48e71
