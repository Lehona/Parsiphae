use crate::inner_errors::ParserError;
use crate::parsers::{
    class, const_array_decl, const_decl, func, instance, prototype, var_decl_list,
};
use crate::types::{Declaration, Input};
use nom::ErrorKind;

named!(pub  declaration<Input, Declaration, ParserError>, terminated!(
    add_return_error!(ErrorKind::Custom(ParserError::Declaration), alt!(
         map!(var_decl_list, Declaration::Var)
        |map!(const_array_decl, Declaration::ConstArray)
        |map!(const_decl, Declaration::Const)
        |map!(func, Declaration::Func)
        |map!(instance, Declaration::Inst)
        |map!(prototype, Declaration::Proto)
        |map!(class, Declaration::Class)
    )),
    return_error!(ErrorKind::Custom(ParserError::MissingSemi),  char_e!(';'))
));
