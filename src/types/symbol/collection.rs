use types;
use types::parsed;

pub struct SymbolCollection {
    syms: Vec<(types::Identifier, parsed::Symbol)>,
}

impl SymbolCollection {
    pub fn new(syms: Vec<parsed::Symbol>) -> Self {
        SymbolCollection {
            syms: syms.into_iter().map(|symb| (symb.name(), symb)).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &parsed::Symbol> {
        self.syms.iter().map(|(_fullname, symb)| symb)
    }

    pub fn get_by_name(&self, name: &[u8]) -> Option<&parsed::Symbol> {
        for (ref fullname, ref symb) in self.syms.iter() {
            if fullname.as_bytes().eq_ignore_ascii_case(name) {
                return Some(symb);
            }
        }

        None
    }
}
