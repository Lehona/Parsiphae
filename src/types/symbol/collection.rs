use crate::types::parsed;
use crate::types::{self, Identifier};

pub struct SymbolCollection {
    syms: Vec<(Vec<u8>, parsed::Symbol)>, // TODO Do we really need a tuple?
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

    pub fn get_by_name(&self, name: &[u8], scope: Option<&[u8]>) -> Option<&parsed::Symbol> {
        scope
            .and_then(|scope| self.get(&[scope, b".", name].concat()))
            .or_else(|| self.get(name))
    }

    pub fn lookup_symbol(
        &self,
        name: &Identifier,
        scope: Option<&Identifier>,
    ) -> Option<&parsed::Symbol> {
        self.get_by_name(name.as_bytes(), scope.map(|s| s.as_bytes()))
    }

    fn get(&self, name: &[u8]) -> Option<&parsed::Symbol> {
        for (ref fullname, ref symb) in self.syms.iter() {
            if fullname.eq_ignore_ascii_case(name) {
                return Some(symb);
            }
        }

        None
    }
}
