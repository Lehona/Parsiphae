use crate::types::{parsed::zPAR_TYPE, Identifier};
use strum::EnumDiscriminants;
use strum::EnumIter;

pub type Result<O> = std::result::Result<O, TypecheckError>;
type Span = (usize, usize);
type SymbolKind = ();

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(TypecheckErrorVariant))]
pub enum TypecheckErrorKind {
    InternalFailure(String),
    UnknownIdentifier(Vec<u8>),
    UnknownReturnType(Identifier), // Return Type is an unknown symbol
    UnknownParameterType(Identifier), // Parameter type is an unknown symbol
    UnknownFunctionCall(Identifier), // Tries to call an unknown function
    UnknownVariableType(Identifier), // Variable type is an unknown symbol
    FunctionCallWrongType(Identifier, SymbolKind), // Tries to call symbol that is not a function
    UnknownIdentifierInExpression(Identifier), // Tries to use an unknown identifier in an expression
    IdentifierIsClassInExpression(Identifier), // The identifier in an expression is known but a class
    FunctionCallParameterWrongType(zPAR_TYPE, zPAR_TYPE), // Parameter has type 1 but Expression has type 2
    FunctionCallWrongAmountOfParameters(usize, usize),    // Expected parameters, actual parameters
    BinaryExpressionNotInt,
    UnaryExpressionNotInt,
    AssignmentWrongTypes(zPAR_TYPE, Span, zPAR_TYPE, Span), // Left type and left side span, right type and right side span
    WrongTypeInArrayInitialization(zPAR_TYPE, Span, zPAR_TYPE, Span), // Left type and left side span, right type and right side span
    CanOnlyAssignToString, // String does not support anything besides Assignment (no +=, *=, ...)
    CanOnlyAssignToFloat,  // Float does not support anything besides Assignment (no +=, *=, ...)
    CanOnlyAssignToInstance, // Instances do not support anything besides Assignment (no +=, *=, ...)
    ConditionNotInt(zPAR_TYPE), // The condition in an if clause is something else than Int
    ReturnExpressionDoesNotMatchReturnType(Identifier, zPAR_TYPE), // 1 is function return type, 2 is type of return expression
    ReturnWithoutExpression(Identifier), // 1 is the return type of the function
    ReturnExpressionInVoidFunction(Identifier), // 1 is the return type of the function
    UnknownIdentifierInArraySize(Identifier),
    NonConstantArraySize, // Array Size is an Identifier that does not point to a constant
    ArraySizeIsNotInteger(zPAR_TYPE, Span), // Array Size is an Identifier of wrong type, Identifier defined at 2
    InstanceHasUnknownParent(Identifier),
    InstanceParentNotClassOrProto(Identifier, SymbolKind), // The parent 1 is not a class or prototype, instead the symbol is kind 2
    IdentifierIsNotType(Identifier), // An identifier is used in type-position, but is not actually a type.
    IdentifierIsNotInstance(Identifier), // An identifier is used in instance-position (inst.member), but is not suitable
    TypeIsPrimitive(zPAR_TYPE), // A variable of the given type was accessed like an instance (inst.member)
    UnknownMember(Identifier, Identifier), // Something is trying to access member '2' of class '1'
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypecheckError {
    pub kind: TypecheckErrorKind,
    pub span: Span,
}

impl TypecheckError {
    pub fn not_a_type(identifier: Identifier) -> Self {
        Self {
            span: identifier.span,
            kind: TypecheckErrorKind::IdentifierIsNotType(identifier),
        }
    }
}
