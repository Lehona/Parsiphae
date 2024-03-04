use crate::diagnostics::diagnostics::{Diagnostic, DiagnosticBuilder, Label};
use crate::file::db::FileId;
use crate::types::parsed::{Symbol, SymbolKind};
use crate::types::{parsed::zPAR_TYPE, Identifier};
use strum::EnumDiscriminants;
use strum::EnumIter;

pub type Result<O> = std::result::Result<O, TypecheckError>;
type Span = (usize, usize);

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(TypecheckErrorVariant))]
pub enum TypecheckErrorKind {
    InternalFailure(String),
    UnknownIdentifier(Vec<u8>),
    /// Return Type is an unknown symbol
    UnknownReturnType(Identifier),
    /// Parameter type is an unknown symbol
    UnknownParameterType(Identifier),
    /// Tries to call an unknown function
    UnknownFunctionCall(Identifier),
    /// Variable type is an unknown symbol
    UnknownVariableType(Identifier),
    /// Tries to call symbol that is not a function, instead it's a <symbol>
    FunctionCallWrongType(Identifier, Symbol),
    /// Tries to use an unknown identifier in an expression
    UnknownIdentifierInExpression(Identifier),
    /// The identifier in an expression is known but a class (which is given by Symbol)
    IdentifierIsClassInExpression(Identifier, Symbol),
    /// Parameter has type 1 but Expression has type 2
    FunctionCallParameterWrongType(zPAR_TYPE, zPAR_TYPE),
    /// Expected parameters, actual parameters
    FunctionCallWrongAmountOfParameters(usize, usize, Symbol),
    BinaryExpressionNotInt(zPAR_TYPE),
    UnaryExpressionNotInt(zPAR_TYPE),
    /// Left type and left side span, right type and right side span. For non-consts the definition is also given in the last param.
    AssignmentWrongTypes(zPAR_TYPE, Span, zPAR_TYPE, Span, Option<Symbol>),
    /// Left type and left side span, right type and right side span
    WrongTypeInArrayInitialization(zPAR_TYPE, Span, zPAR_TYPE, Span),
    /// String does not support anything besides Assignment (no +=, *=, ...)
    CanOnlyAssignToString,
    /// Float does not support anything besides Assignment (no +=, *=, ...)
    CanOnlyAssignToFloat,
    /// Instances do not support anything besides Assignment (no +=, *=, ...)
    CanOnlyAssignToInstance,
    /// The condition in an if clause is something else than Int
    ConditionNotInt(zPAR_TYPE),
    /// 1 is return expression type, 2 is Function
    ReturnExpressionDoesNotMatchReturnType(zPAR_TYPE, Symbol),
    /// 1 is the return type of the function
    ReturnWithoutExpression(zPAR_TYPE, Symbol),
    /// The return expression has the given type, but the function is in Symbol.
    ReturnExpressionInVoidFunction(zPAR_TYPE, Symbol),
    UnknownIdentifierInArraySize(Identifier),
    /// Array Size is an Identifier that does not point to a constant
    // TODO: this could be improved by reporting what exactly was used instead.
    NonConstantArraySize,
    /// Array Size is an Identifier '1' of wrong type defined at '2'
    ArraySizeIsNotInteger(Identifier, Symbol),
    InstanceHasUnknownParent(Identifier),
    /// The parent 1 is not a class or prototype, instead the symbol is kind 2
    InstanceParentNotClassOrProto(Identifier, Symbol),
    /// An identifier is used in type-position, but is not actually a type. The actual symbol is defined in 'Symbol'
    IdentifierIsNotType(Identifier, Symbol),
    /// An identifier is used in instance-position (inst.member), but is not suitable. That identifier is defined in 'Symbol'
    IdentifierIsNotInstance(Identifier, Symbol),
    /// A variable of the given type was accessed like an instance (inst.member), the variable is defined in '1', and the accessed member is '2'
    TypeIsPrimitive(Symbol, Identifier, zPAR_TYPE),
    /// Something is trying to access member '2' of class '1', via instance '3'
    /// (Class, member, instance)
    UnknownMember(Symbol, Identifier, Symbol),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypecheckError {
    pub kind: TypecheckErrorKind,
    pub file_id: FileId,
    pub span: Span,
}

impl TypecheckError {
    pub fn not_a_type(identifier: Identifier, symbol: Symbol, file_id: FileId) -> Self {
        Self {
            span: identifier.span,
            kind: TypecheckErrorKind::IdentifierIsNotType(identifier, symbol),
            file_id,
        }
    }

    pub fn to_code(&self) -> String {
        match self.kind {
            TypecheckErrorKind::InternalFailure(_) => "TC000".into(),
            TypecheckErrorKind::UnknownIdentifier(_) => "TC001".into(),
            TypecheckErrorKind::UnknownReturnType(_) => "TC002".into(),
            TypecheckErrorKind::UnknownParameterType(_) => "TC003".into(),
            TypecheckErrorKind::UnknownFunctionCall(_) => "TC004".into(),
            TypecheckErrorKind::UnknownVariableType(_) => "TC005".into(),
            TypecheckErrorKind::FunctionCallWrongType(_, _) => "TC006".into(),
            TypecheckErrorKind::UnknownIdentifierInExpression(_) => "TC007".into(),
            TypecheckErrorKind::IdentifierIsClassInExpression(_, _) => "TC008".into(),
            TypecheckErrorKind::FunctionCallParameterWrongType(_, _) => "TC009".into(),
            TypecheckErrorKind::FunctionCallWrongAmountOfParameters(_, _, _) => "TC010".into(),
            TypecheckErrorKind::BinaryExpressionNotInt(_) => "TC011".into(),
            TypecheckErrorKind::UnaryExpressionNotInt(_) => "TC012".into(),
            TypecheckErrorKind::AssignmentWrongTypes(_, _, _, _, _) => "TC013".into(),
            TypecheckErrorKind::WrongTypeInArrayInitialization(_, _, _, _) => "TC014".into(),
            TypecheckErrorKind::CanOnlyAssignToString => "TC015".into(),
            TypecheckErrorKind::CanOnlyAssignToFloat => "TC016".into(),
            TypecheckErrorKind::CanOnlyAssignToInstance => "TC017".into(),
            TypecheckErrorKind::ConditionNotInt(_) => "TC018".into(),
            TypecheckErrorKind::ReturnExpressionDoesNotMatchReturnType(_, _) => "TC019".into(),
            TypecheckErrorKind::ReturnWithoutExpression(_, _) => "TC020".into(),
            TypecheckErrorKind::ReturnExpressionInVoidFunction(_, _) => "TC021".into(),
            TypecheckErrorKind::UnknownIdentifierInArraySize(_) => "TC022".into(),
            TypecheckErrorKind::NonConstantArraySize => "TC023".into(),
            TypecheckErrorKind::ArraySizeIsNotInteger(_, _) => "TC024".into(),
            TypecheckErrorKind::InstanceHasUnknownParent(_) => "TC025".into(),
            TypecheckErrorKind::InstanceParentNotClassOrProto(_, _) => "TC026".into(),
            TypecheckErrorKind::IdentifierIsNotType(_, _) => "TC027".into(),
            TypecheckErrorKind::IdentifierIsNotInstance(_, _) => "TC028".into(),
            TypecheckErrorKind::TypeIsPrimitive(_, _, _) => "TC029".into(),
            TypecheckErrorKind::UnknownMember(_, _, _) => "TC030".into(),
        }
    }

    pub fn to_diagnostic(&self) -> Diagnostic {
        let code = self.to_code();
        match &self.kind {
            TypecheckErrorKind::InternalFailure(msg) => {
                Diagnostic {
                    message: format!("Internal Failure: {msg}"),
                    code,
                    labels: vec![],
                }
            }
            TypecheckErrorKind::UnknownIdentifier(ident) => {
                Diagnostic {
                    message: format!("Unknown Identifier: {}", String::from_utf8_lossy(ident)),
                    code,
                    labels: vec![
                        Label {
                            message: "This identifier is not defined anywhere.".into(),
                            file_id: self.file_id,
                            span: self.span,
                            primary: true,
                        }
                    ]
                }
            },
            TypecheckErrorKind::UnknownReturnType(ident) => Diagnostic {
                message: format!("Unknown return type: {ident}"),
                code,
                labels: vec![
                    Label {
                        message: "This return type is not defined anywhere.".into(),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::UnknownParameterType(ident) => Diagnostic {
                message: format!("Unknown parameter type: '{ident}"),
                code,
                labels: vec![
                    Label {
                        message: format!("This parameter's type is not defined anywhere."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::UnknownFunctionCall(ident) => Diagnostic {
                message: format!("Trying to call an unknown identifier: '{ident}'"),
                code,
                labels: vec![
                    Label {
                        message: format!("You're trying to call '{ident}', but that identifier is not defined anywhere"),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::UnknownVariableType(ident) => Diagnostic {
                message: format!("Unknown variable type: '{ident}'"),
                code,
                labels: vec![
                    Label {
                        message: format!("This variable has an unknown type '{ident}'"),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::FunctionCallWrongType(call, target) => {
                let target_name = String::from_utf8_lossy(&target.kind.name_without_scope()).to_string();
                Diagnostic {
                    message: "Trying to call something that is not a function.".into(),
                    code,
                    labels: vec![
                        Label {
                            message: format!("Here is the function call to '{target_name}'."),
                            file_id: self.file_id,
                            span: call.span,
                            primary: true,
                        },
                        Label {
                            message: format!("But '{target_name}' is defined here and not a function."),
                            file_id: target.file_id,
                            span: target.kind.span(),
                            primary: false,
                        }
                    ]
                }
            }
            TypecheckErrorKind::UnknownIdentifierInExpression(ident) => Diagnostic {
                message: format!("Unknown identifier in expression: '{ident}'"),
                code,
                labels: vec![
                    Label {
                        message: format!("Unknown identifier here."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::IdentifierIsClassInExpression(ident, symbol) => Diagnostic {
                message: format!("Classes cannot be used in expressions."),
                code,
                labels: vec![
                    Label {
                        message: format!("Identifier '{ident}' is used here."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("But '{}' is defined here as a class", String::from_utf8_lossy(&symbol.kind.name_without_scope())),
                        file_id: symbol.file_id,
                        span: symbol.kind.span(),
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::FunctionCallParameterWrongType(expected, actual) => DiagnosticBuilder::default()
                .message("Function parameter has the wrong type.")
                .code(code)
                .label(Label {
                    message: format!("This expression has type '{actual}', but the function expects a parameter of type '{expected}'."),
                    file_id: self.file_id,
                    span: self.span,
                    primary: true,
                })
                .build().unwrap(),
            TypecheckErrorKind::FunctionCallWrongAmountOfParameters(expected, actual, func) => DiagnosticBuilder::default()
                .message(format!("This function call takes {expected} parameters, but {actual} parameters were supplied."))
                .code(code)
                .label(Label {
                    message: format!("This function call provides {actual} parameters, but the function '{func}' expects {expected} parameters.", func = String::from_utf8_lossy(&func.kind.name())),
                    file_id: self.file_id,
                    span: self.span,
                    primary: true,
                })
                .label(Label {
                    message: format!("The function '{func}' is defined here.", func = String::from_utf8_lossy(&func.kind.name())),
                    file_id: func.file_id,
                    span: func.kind.span(),
                    primary: false,
                })
                .build().unwrap(),
            TypecheckErrorKind::BinaryExpressionNotInt(typ) => DiagnosticBuilder::default()
                .message("Binary operators can only be applied to integers.")
                .code(code)
                .label(Label {
                    message: format!("This expression has type '{typ}', but only integers are allowed in binary expressions."),
                    file_id: self.file_id,
                    span: self.span,
                    primary: true,
                }).build().unwrap(),
            TypecheckErrorKind::UnaryExpressionNotInt(typ) => DiagnosticBuilder::default()
                .message("Unary operators can only be applied to integers.")
                .code(code)
                .label(Label {
                    message: format!("This expression has type '{typ}', but only integers are allowed in unary expressions."),
                    file_id: self.file_id,
                    span: self.span,
                    primary: true,
                }).build().unwrap(),
            TypecheckErrorKind::AssignmentWrongTypes(lhs, lhs_span, rhs, rhs_span, symb) => {
                let mut builder = DiagnosticBuilder::default()
                .message("Wrong type in assignment.")
                .code(code)
                .label(Label {
                    message: format!("This has type '{rhs}', but the left-hand-side is of type '{lhs}'."),
                    file_id: self.file_id,
                    span: *rhs_span,
                    primary: true,
                })
                .label(Label {
                        message: format!("This is type '{lhs}'."),
                        file_id: self.file_id,
                        span: *lhs_span,
                        primary: false,
                });
                if let Some(symb) = symb {
                    builder = builder.label(Label {
                        message: format!("The variable is defined here."),
                        file_id: symb.file_id,
                        span: symb.kind.span(),
                        primary: false,
                    });
                }
                builder.build().unwrap()
            }
            TypecheckErrorKind::WrongTypeInArrayInitialization(lhs, lhs_span, rhs, rhs_span) => Diagnostic {
                message: format!("Wrong type in array initialization."),
                code,
                labels: vec![
                    Label {
                        message: format!("This has type '{rhs}', but the array is of type '{lhs}'."),
                        file_id: self.file_id,
                        span: *rhs_span,
                        primary: true,
                    },
                    Label {
                        message: format!("The array type '{lhs}' is declared here."),
                        file_id: self.file_id,
                        span: *lhs_span,
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::CanOnlyAssignToString => Diagnostic {
                message: format!("Strings only support assignment (no +=, *=, ...)."),
                code,
                labels: vec![
                    Label {
                        message: format!("You can only use '=' here"),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::CanOnlyAssignToFloat => Diagnostic {
                message: format!("Floats only support assignment (no +=, *=, ...)."),
                code,
                labels: vec![
                    Label {
                        message: format!("You can only use '=' here"),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::CanOnlyAssignToInstance => Diagnostic {
                message: format!("Instances only support assignment (no +=, *=, ...)."),
                code,
                labels: vec![
                    Label {
                        message: format!("You can only use '=' here"),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::ConditionNotInt(typ) => Diagnostic {
                message: format!("Non-integer condition in an if-clause."),
                code,
                labels: vec![
                    Label {
                        message: format!("This expression has type '{typ}', but only integers are allowed here."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::ReturnExpressionDoesNotMatchReturnType(typ, func) => Diagnostic {
                message: format!("Return expression does not match function's return type."),
                code,
                labels: vec![
                    Label {
                        message: format!("This return statement has type '{typ}', but the function's return type is '{func_type}'.", func_type = func.kind.typ()),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("The return type is defined here, but it's '{func_type}'. Did you mean '{typ}'?", func_type = func.kind.typ()),
                        file_id: func.file_id,
                        span: func.kind.typ_ident().span,
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::ReturnWithoutExpression(typ, func) => Diagnostic {
                message: format!("A return is missing an expression."),
                code,
                labels: vec![
                    Label {
                        message: format!("This return statement has no expression, but the function's return type is '{typ}'."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("The return type is defined here, but it's not 'void'."),
                        file_id: func.file_id,
                        span: func.kind.typ_ident().span,
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::ReturnExpressionInVoidFunction(typ, func) => Diagnostic {
                message: format!("Return expression in void function."),
                code,
                labels: vec![
                    Label {
                        message: format!("This return statement has an expression of type '{typ}', but the function's return type is 'void'."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("The return type is defined here, did you mean '{typ}'?"),
                        file_id: func.file_id,
                        span: func.kind.typ_ident().span,
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::UnknownIdentifierInArraySize(ident) => {
                Diagnostic {
                    message: format!("Unknown Identifier in array size: {}", ident),
                    code,
                    labels: vec![
                        Label {
                            message: "This identifier is not defined anywhere.".into(),
                            file_id: self.file_id,
                            span: self.span,
                            primary: true,
                        }
                    ]
                }
            },
            TypecheckErrorKind::NonConstantArraySize => Diagnostic {
                message: format!("Only constants are allowed in array sizes."),
                code,
                labels: vec![
                    Label {
                        message: format!("You're using a non-constant array size here."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::ArraySizeIsNotInteger(ident, symbol) => Diagnostic {
                message: format!("Attempting to use a non-integer constant as array size."),
                code,
                labels: vec![
                    Label {
                        message: format!("You're using '{ident}' as the array size here, but it has type '{}'.", symbol.kind.typ()),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("'{ident}' is defined here but it has type '{}'", symbol.kind.typ()),
                        file_id: symbol.file_id,
                        span: symbol.kind.span(),
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::InstanceHasUnknownParent(ident) => Diagnostic {
                message: format!("An instance has an unknown parent class."),
                code,
                labels: vec![
                    Label {
                        message: format!("The parent class '{ident}' is unknown."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    }
                ]
            },
            TypecheckErrorKind::InstanceParentNotClassOrProto(ident, symbol) => Diagnostic {
                message: format!("The parent of an instance is neither a class nor a prototype."),
                code,
                labels: vec![
                    Label {
                        message: format!("You're using '{ident}' as the parent of an instance here."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("'{ident}' is defined here, but it's neither a class nor a prototype."),
                        file_id: symbol.file_id,
                        span: symbol.kind.span(),
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::IdentifierIsNotType(ident, symbol) => Diagnostic {
                message: format!("An identifier is used in type position, but it is not a type."),
                code,
                labels: vec![
                    Label {
                        message: format!("You're using '{ident}' here as a type."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("'{ident}' is defined here, but it's not a type."),
                        file_id: symbol.file_id,
                        span: symbol.kind.span(),
                        primary: false
                    }
                ]
            },
            TypecheckErrorKind::IdentifierIsNotInstance(ident, symbol) => Diagnostic {
                message: format!("Attempting to access a class member but target is not an instance."),
                code,
                labels: vec![
                    Label {
                        message: format!("You're attempting to access a member of '{ident}', but it is not an instance."),
                        file_id: self.file_id,
                        span: self.span,
                        primary: true,
                    },
                    Label {
                        message: format!("'{ident}' is defined here"),
                        file_id: symbol.file_id,
                        span: symbol.kind.span(),
                        primary: false,
                    }
                ]
            },
            TypecheckErrorKind::TypeIsPrimitive(instance, member, typ) => {
                Diagnostic {
                    message: format!("Attempting to access members of a primitive type."),
                    code,
                    labels: vec![
                        Label {
                            message: format!("You're attempting to access member '{member}' here, but '{}' has type '{typ}'.", String::from_utf8_lossy(&instance.kind.name_without_scope())),
                            file_id: self.file_id,
                            span: self.span,
                            primary: true,
                        },
                        Label {
                            message: format!("'{}' is defined here with type '{typ}'.", String::from_utf8_lossy(&instance.kind.name_without_scope())),
                            file_id: instance.file_id,
                            span: instance.kind.span(),
                            primary: false,
                        }
                    ]
                }
            },
            TypecheckErrorKind::UnknownMember(class, member, instance) => {
                let inst_or_var = match instance.kind {
                    SymbolKind::Var(_, _) => "variable",
                    SymbolKind::Inst(_) => "instance",
                    _ => "<error>",
                };

                Diagnostic {
                    message: format!("The accessed class member does not exist."),
                    code,
                    labels: vec![
                        Label {
                            message: format!("The member '{member}' is accessed here, but it does not exist."),
                            file_id: self.file_id,
                            span: self.span,
                            primary: true,
                        },
                        Label {
                            message: format!("The {inst_or_var} used to access class '{}' is defined here.", String::from_utf8_lossy(&class.kind.name_without_scope())),
                            file_id: instance.file_id,
                            span: instance.kind.span(),
                            primary: false,
                        },
                        Label {
                            message: format!("The class '{}' is defined here, but does not have member '{member}'.", String::from_utf8_lossy(&class.kind.name_without_scope())),
                            file_id: class.file_id,
                            span: class.kind.span(),
                            primary: false,
                        }
                    ]
               }
            }
        }
    }
}
