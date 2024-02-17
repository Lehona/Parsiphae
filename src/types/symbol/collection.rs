use crate::types::Identifier;

use super::parsed::Symbol;

#[derive(Default)]
pub struct SymbolCollection {
    syms: Vec<(Vec<u8>, Symbol)>, // TODO Do we really need a tuple?
}

impl SymbolCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_symbols(syms: Vec<Symbol>) -> Self {
        let mut collection;
        collection = Self::default();
        collection.set_symbols(syms);
        collection
    }

    pub fn set_symbols(&mut self, syms: Vec<Symbol>) {
        self.syms = syms.into_iter().map(|symb| (symb.name(), symb)).collect();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Symbol> {
        self.syms.iter().map(|(_fullname, symb)| symb)
    }

    pub fn get_by_name(&self, name: &[u8], scope: Option<&[u8]>) -> Option<&Symbol> {
        scope
            .and_then(|scope| self.get(&[scope, b".", name].concat()))
            .or_else(|| self.get(name))
    }

    pub fn lookup_symbol(&self, name: &Identifier, scope: Option<&Identifier>) -> Option<&Symbol> {
        self.get_by_name(name.as_bytes(), scope.map(|s| s.as_bytes()))
    }

    fn get(&self, name: &[u8]) -> Option<&Symbol> {
        for (ref fullname, ref symb) in self.syms.iter() {
            if fullname.eq_ignore_ascii_case(name) {
                return Some(symb);
            }
        }

        None
    }
}
