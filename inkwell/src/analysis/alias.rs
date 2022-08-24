//! This module contain bindings to functions of alias analyses.

use llvm_sys::analysis::{
    LLVMAliasResult, LLVMBasicAAQuery, LLVMTypeBasedAAQuery,
};

#[cfg(feature = "internal-getters")]
use crate::LLVMReference;
use crate::{
    module::Module,
    values::{AsValueRef, FunctionValue, PointerValue},
};

/// Data structure representing a basic alias analysis.
#[derive(Debug)]
pub struct BasicAliasAnalysis<'a> {
    /// Target module of the analysis.
    module: Module<'a>,
}

/// Data structure representing a type-based alias analysis.
#[derive(Debug)]
pub struct TypeBasedAliasAnalysis<'a> {
    /// Target module of the analysis.
    module: Module<'a>,
}

#[cfg(feature = "internal-getters")]
impl<'a> BasicAliasAnalysis<'a> {
    /// Constructor
    pub fn new(module: Module<'a>) -> BasicAliasAnalysis<'a> {
        BasicAliasAnalysis { module }
    }

    /// Check alias between two pointers using the basic alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn check_alias(
        &self,
        func: &FunctionValue,
        v1: PointerValue,
        v2: PointerValue,
    ) -> LLVMAliasResult {
        let vref1 = v1.as_value_ref();
        let vref2 = v2.as_value_ref();
        let func_name = func.get_name().to_str().unwrap_or_else(|msg| {
            panic!("Function name not found! Error: {}", msg)
        });
        unsafe {
            LLVMBasicAAQuery(
                self.module.get_ref(),
                func_name.as_ptr() as *const ::libc::c_char,
                func_name.len(),
                vref1,
                vref2,
            )
        }
    }

    /// Check must alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn is_must_alias(
        &self,
        func: &FunctionValue,
        v1: PointerValue,
        v2: PointerValue,
    ) -> bool {
        unsafe {
            self.check_alias(func, v1, v2) == LLVMAliasResult::LLVMMustAlias
        }
    }

    /// Check no alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn is_no_alias(
        &self,
        func: &FunctionValue,
        v1: PointerValue,
        v2: PointerValue,
    ) -> bool {
        unsafe {
            self.check_alias(func, v1, v2) == LLVMAliasResult::LLVMNoAlias
        }
    }

    /// Check may alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn is_may_alias(
        &self,
        func: &FunctionValue,
        v1: PointerValue,
        v2: PointerValue,
    ) -> bool {
        unsafe {
            self.check_alias(func, v1, v2) == LLVMAliasResult::LLVMMayAlias
        }
    }
}

#[cfg(feature = "internal-getters")]
impl<'a> TypeBasedAliasAnalysis<'a> {
    /// Constructor
    pub fn new(module: Module<'a>) -> TypeBasedAliasAnalysis<'a> {
        TypeBasedAliasAnalysis { module }
    }

    /// Check alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn check_alias(
        &self,
        v1: PointerValue,
        v2: PointerValue,
    ) -> LLVMAliasResult {
        let vref1 = v1.as_value_ref();
        let vref2 = v2.as_value_ref();
        unsafe { LLVMTypeBasedAAQuery(self.module.get_ref(), vref1, vref2) }
    }

    /// Check must alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn is_must_alias(&self, v1: PointerValue, v2: PointerValue) -> bool {
        unsafe { self.check_alias(v1, v2) == LLVMAliasResult::LLVMMustAlias }
    }

    /// Check no alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn is_no_alias(&self, v1: PointerValue, v2: PointerValue) -> bool {
        unsafe { self.check_alias(v1, v2) == LLVMAliasResult::LLVMNoAlias }
    }

    /// Check may alias between two pointers using the type-based alias analysis.
    #[llvm_versions(14.0..=latest)]
    pub fn is_may_alias(&self, v1: PointerValue, v2: PointerValue) -> bool {
        unsafe { self.check_alias(v1, v2) == LLVMAliasResult::LLVMMayAlias }
    }
}
