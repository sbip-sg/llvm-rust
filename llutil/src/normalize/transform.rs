//! Transform LLVM bitcode using existing LLVM transformation passes

use inkwell::{module::Module, passes::PassManager, values::FunctionValue};

use crate::{file::CodeFile, ir::FunctionExt};

/// Transform a function using LLVM Function Passes. This function should call
/// only the function normalization passes of LLVM. Otherwise, the
/// transformation will crash!
// TODO: turn this transformation into trait
fn transform_function(func: &FunctionValue, module: &Module) -> bool {
    // Do not transform empty-body function
    if func.is_only_declared() {
        return false;
    }

    debug!("Transforming function: {}", func.get_name_or_default());

    // Run LLVM transformation passes
    let fpm: PassManager<FunctionValue> = PassManager::create(module);
    fpm.initialize();

    // normalize all functions' arguments
    // fpm.add_argument_promotion_pass();

    // merge Load/Store instrs related to the same instructions
    // fpm.add_merged_load_store_motion_pass();

    // // alias analysis
    // fpm.add_basic_alias_analysis_pass();
    // fpm.add_type_based_alias_analysis_pass();

    fpm.run_on(func);
    fpm.finalize()
}

/// Transform a module using LLVM Module Passes. This function should call only
/// the module normalization passes of LLVM. Otherwise, the transformation will
/// crash!
// TODO: turn this transformation into trait
pub fn transform_module(file: &CodeFile, module: &Module) -> bool {
    // debug!("Transforming module: {}", module.get_name_or("N/A"));
    let mut changed = false;

    // Transform functions first
    for func in module.get_functions() {
        if func.is_only_declared() || func.is_c_library_function(file) {
            continue;
        }

        let updated = transform_function(&func, module);
        changed |= updated;
    }

    // Then transform the module
    let mpm: PassManager<Module> = PassManager::create(());

    // alias analysis
    mpm.add_basic_alias_analysis_pass();
    mpm.add_type_based_alias_analysis_pass();

    // normalize all functions' arguments
    mpm.add_argument_promotion_pass();

    // merge duplicate global constants
    mpm.add_constant_merge_pass();

    // merge Load/Store instrs related to the same instructions
    mpm.add_merged_load_store_motion_pass();

    // Note: Disable the GVN pass since it might remove bugs from the bitcode
    // Perform global value numbering
    // mpm.add_gvn_pass();

    let updated = mpm.run_on(module);
    changed |= updated;

    // return
    changed
}
