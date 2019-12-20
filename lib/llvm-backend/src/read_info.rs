use wasmer_runtime_core::module::ModuleInfo;
use wasmer_runtime_core::parse::wp_type_to_type;
use wasmer_runtime_core::structures::TypedIndex;
use wasmer_runtime_core::types::{SigIndex, Type};
use wasmparser::{Type as WpType, TypeOrFuncType as WpTypeOrFuncType};

pub fn blocktype_to_types(ty: WpTypeOrFuncType, info: &ModuleInfo) -> Vec<Type> {
    match ty {
        WpTypeOrFuncType::Type(WpType::EmptyBlockType) => vec![],
        WpTypeOrFuncType::Type(inner_ty) => vec![wp_type_to_type(inner_ty).unwrap()],
        WpTypeOrFuncType::FuncType(sig_index) => {
            let ty = &info.signatures[SigIndex::new(sig_index as usize)];
            ty.returns().to_vec()
        }
    }
}

pub fn blocktype_to_param_types(ty: WpTypeOrFuncType, info: &ModuleInfo) -> Vec<Type> {
    match ty {
        WpTypeOrFuncType::Type(_) => vec![],
        WpTypeOrFuncType::FuncType(sig_index) => {
            let ty = &info.signatures[SigIndex::new(sig_index as usize)];
            ty.params().to_vec()
        }
    }
}
