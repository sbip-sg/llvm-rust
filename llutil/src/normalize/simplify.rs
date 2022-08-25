//! This module simplify an LLVM module by eliminating unwanted instructions,
//! global vars, functions, etc. These eliminations are not semantic-preserving.
//!
//! Important note: for transforming bitcode, should try to use existing LLVM
//!                 transformation. Do not re-invent the wheel.

use inkwell::module::Module;
use inkwell::values::FunctionValue;

use crate::{file::CodeFile, ir::FunctionExt};

/// Module containing all functions that simplify an LLVM function.
mod simplify_func {
    use std::collections::HashSet;

    use inkwell::values::{
        instructions::{AnyCall, CallBase},
        FunctionValue, InstructionOpcode,
    };

    /// Eliminate intrinsic lifetime instructions
    ///
    /// Output: a list `intrinsic_lifetime_funcs` of intrinsic lifetime
    /// instructions.
    pub fn eliminate_intrinsic_lifetime_instructions<'a>(
        func: &FunctionValue<'a>,
    ) -> HashSet<FunctionValue<'a>> {
        // hashset to collect the called intrinsic lifetime function
        let mut intrinsic_lifetime_funcs = HashSet::new();

        // Delete instruction calls to intrinsic life time functions
        for block in func.get_basic_blocks() {
            for inst in block.get_instructions() {
                let callbase_opt: Result<CallBase, _> = inst.try_into();
                if let Ok(callbase) = callbase_opt {
                    if let Some(callee) = callbase.get_called_function() {
                        let callee_name = callee
                            .get_name()
                            .to_str()
                            .unwrap_or_default()
                            .to_string();

                        // Delete function calls starting with "llvm.lifetime"
                        if callee_name.starts_with("llvm.lifetime") {
                            debug!("elim_intrinsic_lifetime: {}", inst);
                            intrinsic_lifetime_funcs.insert(callee);
                            inst.erase_from_basic_block();
                        }
                    }
                }
            }
        }

        intrinsic_lifetime_funcs
    }

    /// Eliminate unused Load instructions in a function.
    /// Return `true` if a change is made, `false` otherwise.
    pub fn eliminate_unused_load(func: &FunctionValue) -> bool {
        let mut changed = false;
        let mut deleted = true;

        // run a fix-point to eliminate all possible unused load
        while deleted {
            deleted = false;

            let mut unused_instrs = vec![];

            // Collect and capture a list of unused instructions.
            // Don't delete on-the-fly since it might break the loop iteration
            for block in func.get_basic_blocks() {
                for inst in block.get_instructions() {
                    if inst.get_opcode() == InstructionOpcode::Load
                        && inst.get_first_use() == None
                    {
                        unused_instrs.push(inst);
                    }
                }
            }

            // Now delete unused instructions
            for inst in unused_instrs {
                inst.erase_from_basic_block();
                deleted = true;
                changed = true;
            }
        }

        changed
    }
}

/// Module containing all functions that simplify an LLVM module.
mod simplify_module {
    use crate::{
        file::{CodeFile, EntryPoint},
        ir::FunctionExt,
    };
    use inkwell::module::Module;
    use std::collections::HashSet;

    use super::simplify_func;

    /// Eliminate inline asm instructions.
    ///
    /// Return `true` if a change is made, `false` otherwise.
    pub fn remove_inline_asm(_module: &Module) -> bool {
        fixme!("Implement eliminate_inline_asm");
        false
    }

    /// Remove unused functions in a module.
    ///
    /// Return `true` if a change is made, `false` otherwise.
    pub fn remove_unused_functions(file: &CodeFile, module: &Module) -> bool {
        debug!("Removing unused functions");
        let mut unused_funcs = HashSet::new();
        for func in module.get_functions() {
            if func.get_first_use().is_some() {
                continue;
            }

            match file.entry_point {
                EntryPoint::MainFunctions => {
                    if !func.is_c_main_function(file)
                        && !func.is_solidity_entry_function(file)
                    {
                        unused_funcs.insert(func);
                    }
                }

                EntryPoint::UserFunctions => {
                    if func.is_library_function(file) {
                        unused_funcs.insert(func);
                    }
                }

                _ => {}
            }
        }

        if unused_funcs.is_empty() {
            return false;
        }

        for func in unused_funcs {
            ddebug!("Remove unused function: {}", func.get_name_or_default());

            unsafe {
                func.delete();
            }
        }

        true
    }

    /// Remove LLVM intrinsic functions in a module.
    ///
    /// Return `true` if a change is made, `false` otherwise.
    pub fn remove_llvm_instrinsic_lifetime(module: &Module) -> bool {
        // Simplify the remaining functions
        let mut intrinsic_lifetime_funcs = HashSet::new();

        for func in module.get_functions() {
            let il_funcs =
                simplify_func::eliminate_intrinsic_lifetime_instructions(&func);

            for func in il_funcs {
                intrinsic_lifetime_funcs.insert(func);
            }
        }

        if intrinsic_lifetime_funcs.is_empty() {
            return false;
        }

        // Delete all intrinsic lifetime functions
        for f in intrinsic_lifetime_funcs {
            unsafe { f.delete() }
        }

        true
    }
}

/// Simplify an LLVM bitcode function.
///
/// Output: Return a pair of `(changed, intrinsic_funcs)`.
/// `changed` is `true` if a change is made, `false` otherwise.
/// `intrinsic_funcs` is a list of intrinsic lifetime intructions
fn simplify_function(func: &FunctionValue) -> bool {
    debug!("Simplifying function: {}", func.get_name_or_default());
    let mut changed = false;

    changed &= simplify_func::eliminate_unused_load(func);

    changed
}

/// Simplify an LLVM bitcode module.
/// Return `true` if a change is made, `false` otherwise.
pub fn simplify_module(file: &CodeFile, module: &Module) -> bool {
    debug!("Simplifying module: {}", module.get_name_or_default());

    let mut changed = false;

    // First, remove all unused function.
    changed &= simplify_module::remove_unused_functions(file, module);

    // Remove intrinsic lifetime instructions.
    changed &= simplify_module::remove_llvm_instrinsic_lifetime(module);

    // Remove inline assembly.
    changed &= simplify_module::remove_inline_asm(module);

    // Simplify the remaining functions.
    for func in module.get_functions() {
        if func.is_only_declared() || func.is_c_library_function(file) {
            continue;
        }

        changed &= simplify_function(&func);
    }

    changed
}
