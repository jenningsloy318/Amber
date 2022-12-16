use crate::modules::{types::Type, block::Block};
use super::{context::FunctionDecl, function_cache::FunctionInstance};



#[derive(Clone, Debug)]
pub struct FunctionInterface {
    pub id: Option<usize>,
    pub name: String,
    pub arg_names: Vec<String>,
    pub arg_types: Vec<Type>,
    pub arg_refs: Vec<bool>,
    pub returns: Type,
    pub is_public: bool
}

impl FunctionInterface {
    pub fn to_fun_declaration(self, id: usize) -> FunctionDecl {
        let is_args_typed = self.arg_types.iter().all(|t| t != &Type::Generic);
        FunctionDecl {
            name: self.name,
            arg_names: self.arg_names,
            arg_types: self.arg_types,
            arg_refs: self.arg_refs,
            returns: self.returns,
            is_args_typed,
            is_public: self.is_public,
            id
        }
    }

    pub fn to_fun_instance(self, block: Block) -> FunctionInstance {
        FunctionInstance {
            variant_id: 0,
            args: self.arg_types,
            returns: self.returns,
            block
        }
    }
}