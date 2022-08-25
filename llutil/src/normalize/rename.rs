//! Normalizing modules

// TODO: rename this module to a better name, not `rename.rs`.

use inkwell::module::Module;

use crate::ir::FunctionExt;

/// Rename basic blocks and values like  globals, variables, parameters.
/// Output: [`true`] if a renaming is performed, [`false`] if otherwise.
// TODO: turn this renaming into trait
pub fn rename_basic_blocks_and_values(module: &Module) -> bool {
    // index counter for globals, parameters, instructions, ...
    let mut block_index = 0;
    let mut value_index = 0;
    let mut global_index = 0;
    let mut updated = false;

    for global in module.get_globals() {
        global.set_name(format!("g{}", global_index).as_str());
        global_index += 1;
    }
    updated &= global_index > 0;

    for func in module.get_functions() {
        block_index = 0;
        value_index = 0;

        if func.is_only_declared() {
            continue;
        };

        debug!("Rename function: {}", func.get_name_or_default());
        for param in func.get_params() {
            param.set_name(format!("arg{}", value_index).as_str());
            value_index += 1;
        }

        for block in func.get_basic_blocks() {
            block.set_name(format!("bb{}", block_index).as_str());
            block_index += 1;

            for inst in block.get_instructions() {
                if !inst.get_type().is_void_type() {
                    inst.set_name(format!("v{}", value_index).as_str());
                    value_index += 1;
                }
            }
        }
    }

    updated &= value_index > 0 || block_index > 0;

    updated
}
