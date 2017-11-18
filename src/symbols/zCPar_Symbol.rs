#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[allow(unused_imports)]
use bitflags::*;
use symbols::zSymb_Table::zSymbol_Table;

use ast::*;
use symbols::*;
use itertools::*;

use std::clone::Clone;



bitflags! {
    pub struct zFLAG: u32 {
        const NONE         = 0;
        const CONST        = 0b00000001;
        const RETURN       = 0b00000010;
        const CLASSVAR     = 0b00000100;
        const EXTERNAL     = 0b00001000;
        const MERGED       = 0b00010000;
    }
}

impl zFLAG {
    pub fn to_string(&self) -> String {
        let mut result = Vec::new();

        if *self == zFLAG::NONE {
            result.push("NONE");
        }

        if self.contains(zFLAG::CONST) {
            result.push("CONST");
        }

        if self.contains(zFLAG::RETURN) {
            result.push("RETURN");
        }

        if self.contains(zFLAG::CLASSVAR) {
            result.push("CLASSVAR");
        }

        if self.contains(zFLAG::EXTERNAL) {
            result.push("EXTERNAL");
        }

        if self.contains(zFLAG::MERGED) {
            result.push("MERGED");
        }

        result.iter().join("|")
    }
}

pub enum zTYPE {
    Void,
    Float,
    Int,
    String,
    Class,
    Func,
    Prototype,
    Instance(String)
}

impl zTYPE {
    pub fn from_str(typ: &str) -> zTYPE {
        match typ {
            "VOID" => zTYPE::Void,
            "FLOAT" => zTYPE::Float,
            "INT" => zTYPE::Int,
            "STRING" => zTYPE::String,
            "CLASS" => zTYPE::Class,
            "FUNC" => zTYPE::Func,
            "PROTOTYPE" => zTYPE::Prototype,
            "INSTANCE" => zTYPE::Instance("".to_string()),
            _ => zTYPE::Instance(typ.to_string())
        }
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            zTYPE::Void => "VOID",
            zTYPE::Float => "FLOAT",
            zTYPE::Int => "INT",
            zTYPE::String => "STRING",
            zTYPE::Class => "CLASS",
            zTYPE::Func => "FUNC",
            zTYPE::Prototype => "PROTOTYPE",
            zTYPE::Instance(ref s) => s
        };

        str.to_string()
    }
}

pub enum zOFFSET {
    offset(Option<i32>),
    typ(zTYPE)
}

impl ToString for zOFFSET {
    fn to_string(&self) -> String {
        match *self {
            zOFFSET::offset(ref opt) => opt.unwrap_or(0).to_string(),
            zOFFSET::typ(ref typ) => typ.to_string()

        }
    }
}

pub enum zCONTENT {
    single(ConstantFoldedValue),
    array(Vec<ConstantFoldedValue>)
}

impl ToString for zCONTENT {
    fn to_string(&self) -> String {
        match *self {
            zCONTENT::single(ref i) => i.to_string(),
            zCONTENT::array(ref arr) => arr.iter().map(|item|item.to_string()).join(", ")
        }
    }
}

pub enum ConstInitialization {
    single(ConstantValue),
    array(ConstArrayInitializerList)
}

pub struct zSymbol {
    pub name: String,
    pub parent: Option<String>,

    pub content: Option<zCONTENT>,       //0x0018 void* oder int* oder float* oder zSTRING* oder int oder float. Bei Funktionen / Instanzen / Prototypen: Stackpointer. Sonst Daten oder Datenpointer.
    pub offset: zOFFSET,        //0x001C Offset bei Klassenvariablen // Adresse bei Instanzen // Rückgabewert bei Funktionen // Größe in Bytes bei Klassen

    pub typ: zTYPE,
    pub flags: zFLAG,

    pub element_count: ArrayIndex,

    pub initialization: Option<ConstInitialization>,

    pub func_body: Option<StatementList>
}

impl zSymbol {
    fn new(name: String, parent: Option<String>, content: Option<zCONTENT>, offset: zOFFSET,
           typ: zTYPE, flags: zFLAG, array_size: Option<ArrayIndex>,
           initialization: Option<ConstInitialization>, func_body: Option<StatementList> ) -> Self {

        zSymbol {name, parent, content, offset, typ, flags,
            element_count: array_size.unwrap_or(ArrayIndex::Number(1)),
            initialization, func_body: func_body }
    }

    pub fn from_func(func: &mut Function) -> Self {
        zSymbol::new(func.name.to_string(),
                     None,
                     None,
                     zOFFSET::typ(zTYPE::from_str(&func.typ)),
                     zTYPE::Func,
                     zFLAG::RETURN | zFLAG::CONST,
                     Some(ArrayIndex::Number(func.params.len() as i32)),
                     None,
                     func.body.take())
    }

    pub fn from_const_decl(con: &ConstantDeclaration, parent: Option<String>) -> Self {
        let name = format_name(&con.name, parent.as_ref());
        let flag = zFLAG::CONST;

        let typ = match con.typ.as_ref() {
            "INT" => zTYPE::Int,
            "STRING" => zTYPE::String,
            "FLOAT" => zTYPE::Float,
            "FUNC" => zTYPE::Func,
            _ => panic!("illegal constant type {}", con.typ)
        };


        zSymbol::new(name,
             parent,
             None,
             zOFFSET::offset(None),
             typ,
             flag,
             None,
             Some(ConstInitialization::single(con.value.clone())),
             None
        )
    }

    pub fn from_var_decl(var: &VariableDeclaration, parent: Option<String>, classvar: bool) -> Self {
        let name = format_name(&var.name, parent.as_ref());

        let flag = if classvar {
            zFLAG::CLASSVAR
        } else {
            zFLAG::NONE
        };


        zSymbol::new(name,
                    parent,
                    None,
                    zOFFSET::offset(None),
                    zTYPE::from_str(&var.typ),
                    flag,
                    var.array_size.clone(),
                    None,
                    None)
    }

    pub fn from_class(class: &Class) -> Self {
        let size = class.members
            .iter()
            .map(|var|var.get_size())
            .fold(0, |acc, tmp| acc + tmp);

        zSymbol::new(class.name.to_string(),
                     None,
                     None,
                     zOFFSET::offset(Some(size)),
                     zTYPE::Class,
                     zFLAG::NONE,
                     Some(ArrayIndex::Number(class.members.len() as i32)),
                     None,
                     None
                )
    }

    pub fn from_inst(inst: &Instance) -> Self {
        zSymbol::new   (inst.name.to_string(),
                        Some(inst.class.to_string()),
                        None,
                        zOFFSET::offset(None),
                        zTYPE::Instance(inst.class.to_string()),
                        zFLAG::NONE,
                        Some(ArrayIndex::Number(0)),
                        None,
                        None)
    }

    pub fn from_proto(proto: &Prototype) -> Self {
        zSymbol::new   (proto.name.to_string(),
                        Some(proto.class.to_string()),
                        None,
                        zOFFSET::offset(None),
                        zTYPE::Prototype,
                        zFLAG::NONE,
                        Some(ArrayIndex::Number(0)),
                        None,
                        None)
    }

    pub fn from_const_array_decl(const_array: &ConstantArrayDeclaration) -> Self {
        zSymbol::new (const_array.name.to_string(),
                None,
                None,
                zOFFSET::offset(None),
                zTYPE::from_str(&const_array.typ),
                zFLAG::CONST,
                Some(const_array.array_size.clone()),
                Some(ConstInitialization::array(const_array.values.clone())),
                None)
    }

    pub fn to_string(&self) -> String {
        format!("name: {}; parent: {}; content: {}; offset: {}; type: {}; flags: {}; elem: {};",
                self.name,
                self.parent.as_ref().unwrap_or(&"None".to_string()),
                self.content.as_ref().map_or("None".to_string(), |i|i.to_string()),
                self.offset.to_string(),
                self.typ.to_string(),
                self.flags.to_string(),
                self.element_count.to_string())

    }


    pub fn fold_const_value(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        println!("folding {}", &self.name);
        let val = self.initialization.as_ref().expect("Trying to fold constant without initialization");
        let unpacked = match *val {
            ConstInitialization::single(ref cv) => cv,
            ConstInitialization::array(_) => panic!("fold_const_value on array")
        };

        unpacked.fold(table)
    }

  pub fn fold_const_array(&self, table: &zSymbol_Table) -> Vec<ConstantFoldedValue> {
      let val = self.initialization.as_ref().expect("Trying to fold constant array without initialization");
      let unpacked = match *val {
          ConstInitialization::single(_) => panic!("fold_const_array on value"),
          ConstInitialization::array(ref vec) => vec
      };


      unpacked.fold(table)
    }
}