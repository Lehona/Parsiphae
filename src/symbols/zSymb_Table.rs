#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[allow(unused_imports)]
use std::rc::Rc;
use symbols::zCPar_Symbol::{zSymbol, zCONTENT, zFLAG};
use ast::*;
use itertools::Itertools;


pub struct zSymbol_Table {
    pub symbols: Vec<zSymbol>
}

impl zSymbol_Table {
    pub fn new(symbols: Vec<zSymbol>) -> Self {
        zSymbol_Table {symbols}
    }

    pub fn symbID_by_name(&self, name: &str) -> usize {
        let (id, _) = self.symbols.iter()
            .enumerate()
            .find(|&(_, sym)|sym.name == name).expect("symbol not found");
        id
    }

    pub fn symb_by_ID(&self, id: usize) -> &zSymbol {
        &self.symbols[id]
    }

    pub fn symb_by_name(&self, name: &str) -> &zSymbol {
        let index = self.symbols.iter().position(|sym|sym.name == name).unwrap();

        &self.symbols[index]
    }

    pub fn symb_by_name_mut(&mut self, name: &str) -> &mut zSymbol {
        let index = self.symbols.iter().position(|sym| sym.name == name).unwrap();

        &mut self.symbols[index]
    }


    pub fn fold_constants(&mut self) {
        self.fold_const_values();
        self.fold_const_arrays();
    }

    pub fn fold_const_arrays(&mut self) {
        let foldeds =
            self.symbols
            .iter()
            .enumerate()
            .filter(|&(_, sym)|
                sym.flags == zFLAG::CONST
                    && sym.content.is_none()
                    && sym.element_count.is_array()
                    && sym.initialization.is_some())
            .map(|(i, sym)| (i, sym.fold_const_array(&self)))
            .collect::<Vec<(usize, Vec<ConstantFoldedValue>)>>();

        for (index, folded) in foldeds {
            let array_index = {
                let ref symbol = self.symbols[index];
                symbol.element_count.make_number(&self)
            };

            if array_index.as_number() != folded.len() as i32 {
                panic!("constant array with wrong amount of initializers");
            }

            let ref mut symbol_mut = self.symbols[index];
            symbol_mut.element_count = array_index;
            symbol_mut.content = Some(zCONTENT::array(folded));
        }
    }

    pub fn fold_const_values(&mut self) {
        let foldeds =
            self.symbols
                .iter()
                .enumerate()
                .filter(|&(_, sym)|sym.flags == zFLAG::CONST
                                                  && sym.content.is_none()
                                                  && sym.element_count.single_value()
                                                  && sym.initialization.is_some()
                )
                .map(|(i, sym)| (i, sym.fold_const_value(&self)))
                .collect::<Vec<(usize, ConstantFoldedValue)>>();

        for (index, folded) in foldeds {
            self.symbols[index].content = Some(zCONTENT::single(folded));
        }
    }

}