use crate::types::Expression;
use crate::types::Identifier;

#[derive(Clone, PartialEq)]
pub struct VarAccess {
    pub name: Identifier,
    pub instance: Option<Identifier>,
    pub index: Option<Expression>,
}

impl VarAccess {
    pub fn new(
        first_ident: Identifier,
        second_ident: Option<Identifier>,
        index: Option<Expression>,
    ) -> Self {
        // In case there is a second identifier it's an object access (instance.member), so we swap the parameters around.
        if second_ident.is_some() {
            VarAccess {
                name: second_ident.unwrap(),
                instance: Some(first_ident),
                index,
            }
        } else {
            VarAccess {
                name: first_ident,
                instance: None,
                index,
            }
        }
    }
}

impl ::std::fmt::Debug for VarAccess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let array_access = if let Some(ref access) = self.index {
            format!("[{:#?}]", access)
        } else {
            "".to_string()
        };

        let body = if let Some(ref instance) = self.instance {
            format!("{}.{}", instance, self.name)
        } else {
            format!("{}", self.name)
        };

        write!(f, "VarAccess:  {}{}", body, array_access)
    }
}

impl VarAccess {
    pub fn is_constant(&self) -> bool {
        /* TODO: I'm pretty sure this is bogus and only constants (or constant-arrays) are constant in this context */
        true
    }
}
