//! Module to instrument bug annotations into LLVM IR.

use inkwell::context::Context;
use inkwell::debug_info::DILocation;
use inkwell::module::Module;
use inkwell::types::{AnyTypeEnum, IntType};
use inkwell::values::BasicBlock;
use inkwell::values::{FunctionValue, InstructionValue, IntValue};
use std::convert::TryInto;
use std::fs;

pub mod annot_lexer;
pub mod annot_parser;
pub mod annot_token;

/// Prefix of function names that check integer overflow.
const ASSERT_BUG_INTEGER_OVERFLOW: &str = "__assert_bug_integer_overflow";

/// Parse the `source_file` to get a list of annotations.
pub fn parse_file(source_file: &str) -> Vec<(annot_token::BugType, u32, u32)> {
    let input =
        fs::read_to_string(source_file).expect("Unable to read the input file");
    let tokens = annot_lexer::nom_lexing_filtered(&input);
    annot_parser::parsing(tokens)
}

/// Check the location `location_opt` of the current instruction whether the
/// `location_opt` contains one location the same as location of one annotation
/// in `annotations`.
///
/// Moreover, check the type of the instruction `typ` whether it is supported.
/// Currently, we support `IntType` only.
///
/// Return the `IntType` variable to support adding a instrumentation
/// instruction.
pub fn get_annot_typ<'a>(
    typ: AnyTypeEnum<'a>,
    location_opt: Option<DILocation>,
    annotations: &[(annot_token::BugType, u32, u32)],
) -> Option<IntType<'a>> {
    if let Some(loc) = location_opt {
        if let AnyTypeEnum::IntType(int_type) = typ {
            for annot in annotations {
                let (_, line, col) = annot;
                let line_number = loc.get_line();
                let col_number = loc.get_column();
                if *line == line_number && *col == col_number {
                    return Some(int_type);
                }
            }
        }
    }
    None
}

/// Check if the function declaration with the name `fn_name` is added to
/// `module` or not.
pub fn get_assert_func<'a>(
    module: &Module<'a>,
    fn_name: &str,
) -> Option<FunctionValue<'a>> {
    let functions = module.get_functions();
    let mut assert_func = None;
    for func in functions {
        let name = func.get_name().to_str().unwrap();
        if name == fn_name {
            assert_func = Some(func);
        }
    }
    assert_func
}

/// Instrument a `module` using the builder after satisfying the
/// conditions of annotation instrumentation.
///
/// An instruction is created using a builder in `context`.
///
/// The two parameters `block` and `instr` are needed to specify the
/// annotation location that is after the location of `instr`.
///
/// The parameter `int_type` is used to create the instrumentation instruction
/// as a function call.
fn create_instrumented_instruction<'a>(
    module: &Module<'a>,
    context: &'a Context,
    block: &BasicBlock,
    inst: &InstructionValue<'a>,
    int_type: IntType<'a>,
) -> bool {
    let builder = context.create_builder();

    builder.position_at(*block, inst);

    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[int_type.into()], false);
    let fn_arg: IntValue = TryInto::try_into(*inst).unwrap();

    let mut fn_name = ASSERT_BUG_INTEGER_OVERFLOW.to_owned();
    fn_name.push('_');
    fn_name.push_str(&int_type.print_to_llvm_string().to_string());

    let fn_value = match get_assert_func(module, &fn_name) {
        None => module.add_function(&fn_name, fn_type, None),
        Some(assert_fun) => assert_fun,
    };

    let next_inst = match inst.get_next_instruction() {
        Some(next_inst) => next_inst,
        None => {
            // The overflow instruction needs to be in the rhs of an
            // assignment or a function argument. Therefore, it
            // cannot be the last instruction
            panic!("Error instruction: {:?}", inst)
        }
    };

    // builder needs to be after the current instruction to insert
    // the newly-created instruction
    builder.position_at(*block, &next_inst);
    let call_instruction = builder
        .build_call(fn_value, &[fn_arg.into()], "")
        .try_as_basic_value()
        .right()
        .unwrap();
    builder.position_at(*block, &call_instruction);
    false
}

/// Instrument LLVM IR related to bug annotation from `source_file` into LLVM
/// bitcode `module`.
pub fn instrument_bug_annotations<'a>(
    source_file: &str,
    module: &Module<'a>,
    ctx: &'a Context,
) -> bool {
    debug!(
        "Instrument bug annotations from source file {} to module: {}",
        source_file,
        module.get_name_or_default()
    );

    let annotations = parse_file(source_file);
    let functions = module.get_functions();
    let builder = ctx.create_builder();

    for func in functions {
        for block in func.get_basic_blocks() {
            for inst in block.get_instructions() {
                builder.position_at(block, &inst);
                let location_opt = builder.get_current_debug_location();
                let typ = inst.get_type();
                let instr_type = get_annot_typ(typ, location_opt, &annotations);
                if let Some(int_type) = instr_type {
                    create_instrumented_instruction(
                        module, ctx, &block, &inst, int_type,
                    );
                }
            }
        }
    }

    // verify the module whether newly added instructions can cause any error
    // exit when an error happens
    let res = module.verify();
    if let Err(msg) = res {
        panic!("Added instruction leads to error: {}", msg);
    }

    true
}
