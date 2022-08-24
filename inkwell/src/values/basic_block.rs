//! A `BasicBlock` is a container of instructions.

#[llvm_versions(3.9..=latest)]
use llvm_sys::core::LLVMGetBasicBlockName;
use llvm_sys::core::{
    LLVMBasicBlockAsValue, LLVMBlockAddress, LLVMDeleteBasicBlock,
    LLVMGetBasicBlockParent, LLVMGetBasicBlockTerminator,
    LLVMGetFirstInstruction, LLVMGetFirstUse, LLVMGetLastInstruction,
    LLVMGetNextBasicBlock, LLVMGetPreviousBasicBlock, LLVMGetTypeContext,
    LLVMIsABasicBlock, LLVMIsConstant, LLVMMoveBasicBlockAfter,
    LLVMMoveBasicBlockBefore, LLVMPrintTypeToString, LLVMPrintValueToString,
    LLVMRemoveBasicBlockFromParent, LLVMReplaceAllUsesWith, LLVMSetValueName,
    LLVMTypeOf,
};
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMValueRef};

use crate::cfg::{PredecessorBlock, SuccessorBlock};
use crate::context::ContextRef;
use crate::support::{to_c_str, LLVMString};
use crate::values::{
    AnyValueEnum, AsValueRef, BasicValueUse, FunctionValue, InstructionValue,
    PointerValue,
};
#[cfg(feature = "internal-getters")]
use crate::LLVMReference;

use std::convert::TryInto;
use std::ffi::CStr;
use std::fmt::{self, Display};
use std::marker::PhantomData;

use super::instructions::{AnyTerminator, PhiNode, TerminatorInst};
use super::Value;

/// A `BasicBlock` is a container of instructions.
///
/// `BasicBlock`s are values because they can be referenced by instructions (ie branching and switches).
///
/// A well formed `BasicBlock` is a list of non terminating instructions followed by a single terminating
/// instruction. `BasicBlock`s are allowed to be malformed prior to running validation because it may be useful
/// when constructing or modifying a program.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct BasicBlock<'ctx> {
    pub(crate) basic_block: LLVMBasicBlockRef,
    _marker: PhantomData<&'ctx ()>,
}

impl<'ctx> BasicBlock<'ctx> {
    pub(crate) unsafe fn new(basic_block: LLVMBasicBlockRef) -> Option<Self> {
        if basic_block.is_null() {
            return None;
        }

        // NOTE: There is a LLVMBasicBlockAsValue but it might be the same as casting
        assert!(!LLVMIsABasicBlock(basic_block as LLVMValueRef).is_null());

        Some(BasicBlock {
            basic_block,
            _marker: PhantomData,
        })
    }

    /// Obtains the `FunctionValue` that this `BasicBlock` belongs to, if any.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// assert_eq!(basic_block.get_parent().unwrap(), function);
    ///
    /// basic_block.remove_from_function();
    ///
    /// assert!(basic_block.get_parent().is_none());
    /// ```
    pub fn get_parent(self) -> Option<FunctionValue<'ctx>> {
        unsafe { FunctionValue::new(LLVMGetBasicBlockParent(self.basic_block)) }
    }

    /// Gets the `BasicBlock` preceeding the current one, in its own scope, if any.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function1 = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block1 = context.append_basic_block(function1, "entry");
    ///
    /// assert!(basic_block1.get_previous_basic_block().is_none());
    ///
    /// let function2 = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block2 = context.append_basic_block(function2, "entry");
    /// let basic_block3 = context.append_basic_block(function2, "next");
    ///
    /// assert!(basic_block2.get_previous_basic_block().is_none());
    /// assert_eq!(basic_block3.get_previous_basic_block().unwrap(), basic_block2);
    /// ```
    pub fn get_previous_basic_block(self) -> Option<BasicBlock<'ctx>> {
        self.get_parent()?;

        unsafe { BasicBlock::new(LLVMGetPreviousBasicBlock(self.basic_block)) }
    }

    /// Gets the `BasicBlock` succeeding the current one, in its own scope, if any.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function1 = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block1 = context.append_basic_block(function1, "entry");
    ///
    /// assert!(basic_block1.get_next_basic_block().is_none());
    ///
    /// let function2 = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block2 = context.append_basic_block(function2, "entry");
    /// let basic_block3 = context.append_basic_block(function2, "next");
    ///
    /// assert!(basic_block1.get_next_basic_block().is_none());
    /// assert_eq!(basic_block2.get_next_basic_block().unwrap(), basic_block3);
    /// assert!(basic_block3.get_next_basic_block().is_none());
    /// ```
    pub fn get_next_basic_block(self) -> Option<BasicBlock<'ctx>> {
        self.get_parent()?;

        unsafe { BasicBlock::new(LLVMGetNextBasicBlock(self.basic_block)) }
    }

    /// Prepends one `BasicBlock` before another.
    /// It returns `Err(())` when either `BasicBlock` has no parent, as LLVM assumes they both have parents.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block1 = context.append_basic_block(function, "entry");
    /// let basic_block2 = context.append_basic_block(function, "next");
    ///
    /// basic_block2.move_before(basic_block1);
    ///
    /// assert!(basic_block1.get_next_basic_block().is_none());
    /// assert_eq!(basic_block2.get_next_basic_block().unwrap(), basic_block1);
    /// ```
    // REVIEW: What happens if blocks are from different scopes?
    pub fn move_before(self, basic_block: BasicBlock<'ctx>) -> Result<(), ()> {
        // This method is UB if the parent no longer exists, so we must check for parent (or encode into type system)
        if self.get_parent().is_none() || basic_block.get_parent().is_none() {
            return Err(());
        }

        unsafe {
            LLVMMoveBasicBlockBefore(self.basic_block, basic_block.basic_block)
        }

        Ok(())
    }

    /// Appends one `BasicBlock` after another.
    /// It returns `Err(())` when either `BasicBlock` has no parent, as LLVM assumes they both have parents.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    ///
    /// let basic_block1 = context.append_basic_block(function, "entry");
    /// let basic_block2 = context.append_basic_block(function, "next");
    ///
    /// basic_block1.move_after(basic_block2);
    ///
    /// assert!(basic_block1.get_next_basic_block().is_none());
    /// assert_eq!(basic_block2.get_next_basic_block().unwrap(), basic_block1);
    /// ```
    // REVIEW: What happens if blocks are from different scopes?
    pub fn move_after(self, basic_block: BasicBlock<'ctx>) -> Result<(), ()> {
        // This method is UB if the parent no longer exists, so we must check for parent (or encode into type system)
        if self.get_parent().is_none() || basic_block.get_parent().is_none() {
            return Err(());
        }

        unsafe {
            LLVMMoveBasicBlockAfter(self.basic_block, basic_block.basic_block)
        }

        Ok(())
    }

    /// Obtains the first `InstructionValue` in this `BasicBlock`, if any.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    /// use inkwell::values::InstructionOpcode;
    ///
    /// let context = Context::create();
    /// let builder = context.create_builder();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// builder.position_at_end(basic_block);
    /// builder.build_return(None);
    ///
    /// assert_eq!(basic_block.get_first_instruction().unwrap().get_opcode(), InstructionOpcode::Return);
    /// ```
    pub fn get_first_instruction(self) -> Option<InstructionValue<'ctx>> {
        let value = unsafe { LLVMGetFirstInstruction(self.basic_block) };

        if value.is_null() {
            return None;
        }

        unsafe { Some(InstructionValue::new(value)) }
    }

    /// Obtains the last `InstructionValue` in this `BasicBlock`, if any. A `BasicBlock` must have a last instruction to be valid.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    /// use inkwell::values::InstructionOpcode;
    ///
    /// let context = Context::create();
    /// let builder = context.create_builder();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// builder.position_at_end(basic_block);
    /// builder.build_return(None);
    ///
    /// assert_eq!(basic_block.get_last_instruction().unwrap().get_opcode(), InstructionOpcode::Return);
    /// ```
    pub fn get_last_instruction(self) -> Option<InstructionValue<'ctx>> {
        let value = unsafe { LLVMGetLastInstruction(self.basic_block) };

        if value.is_null() {
            return None;
        }

        unsafe { Some(InstructionValue::new(value)) }
    }

    /// Obtains the terminating `InstructionValue` in this `BasicBlock`, if any. A `BasicBlock` must have a terminating instruction to be valid.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    /// use inkwell::values::InstructionOpcode;
    ///
    /// let context = Context::create();
    /// let builder = context.create_builder();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// builder.position_at_end(basic_block);
    /// builder.build_return(None);
    ///
    /// assert_eq!(basic_block.get_terminator().unwrap().get_opcode(), InstructionOpcode::Return);
    /// ```
    // REVIEW: If we wanted the return type could be Option<Either<BasicValueEnum, InstructionValue>>
    // if getting a value over an instruction is preferable
    // TODOC: Every BB must have a terminating instruction or else it is invalid
    // REVIEW: Unclear how this differs from get_last_instruction
    pub fn get_terminator(self) -> Option<InstructionValue<'ctx>> {
        let value = unsafe { LLVMGetBasicBlockTerminator(self.basic_block) };

        if value.is_null() {
            return None;
        }

        unsafe { Some(InstructionValue::new(value)) }
    }

    /// Get all the Phi instructions of the current `BasicBlock`.
    ///
    /// By LLVM IR formatl, all Phi instructions must be located at the top of
    /// the `BasicBlock`.
    pub fn get_phi_instructions(&self) -> Vec<PhiNode<'ctx>> {
        let mut phi_insts = vec![];
        let mut inst_opt = self.get_first_instruction();

        while inst_opt.is_some() {
            let inst = inst_opt.unwrap();
            match inst.try_into_phi_node() {
                Some(phi) => {
                    phi_insts.push(phi);
                    inst_opt = inst.get_next_instruction()
                }
                None => break,
            }
        }

        phi_insts
    }

    /// Get all instructions of the current `BasicBlock`.
    pub fn get_instructions(&self) -> Vec<InstructionValue<'ctx>> {
        let mut insts = vec![];
        let mut inst_opt = self.get_first_instruction();

        while inst_opt.is_some() {
            let inst = inst_opt.unwrap();
            insts.push(inst);
            inst_opt = inst.get_next_instruction();
        }

        insts
    }

    /// Get predecessor blocks of the current `BasicBlock`.
    ///
    /// A predecessor block is the block that jumps to the current block.
    pub fn get_predecessors(&self) -> Vec<BasicBlock<'ctx>> {
        let mut predecessors = vec![];

        let mut use_ = self.get_first_use();

        while let Some(value_use) = use_ {
            let user = value_use.get_user();
            if user.is_instruction_value() {
                let inst = user.into_instruction_value();
                if let Some(blk) = inst.get_parent() {
                    predecessors.push(blk)
                }
            }
            use_ = value_use.get_next_use()
        }

        predecessors
    }

    /// Get successor blocks of the current `BasicBlock`.
    ///
    /// A successor block is the block that the current block jumps to.
    pub fn get_successors(&self) -> Vec<BasicBlock<'ctx>> {
        if let Some(inst) = self.get_terminator() {
            let res: Result<TerminatorInst, _> = inst.try_into();
            if let Ok(term_inst) = res {
                return term_inst.get_successors();
            }
        }
        vec![]
    }

    /// Get predecessor blocks of the current `BasicBlock` and their path
    /// conditions.
    pub fn get_conditioned_predecessors(self) -> Vec<PredecessorBlock<'ctx>> {
        let mut predecessors = vec![];
        let mut self_use = self.get_first_use();

        // Loop to get predecessor blocks from all instructions that use
        // the current block.
        while let Some(v) = self_use {
            // Get instruction that uses the current block
            let self_user = v.get_user();
            if self_user.is_instruction_value() {
                let inst = self_user.into_instruction_value();
                if let Some(term_inst) = inst.try_into_terminator_inst() {
                    // Find among all successors of the found instruction
                    // the path condition that jump to the current block.
                    for sblk in term_inst.get_conditioned_successors() {
                        if sblk.block == self {
                            let pred_blk = PredecessorBlock::new(
                                sblk.condition,
                                inst.get_parent().unwrap(),
                            );
                            predecessors.push(pred_blk);
                        }
                    }
                }
            }

            self_use = v.get_next_use()
        }

        predecessors
    }

    /// Get successor blocks of the current `BasicBlock` and their path
    /// conditions.
    pub fn get_conditioned_successors(self) -> Vec<SuccessorBlock<'ctx>> {
        if let Some(inst) = self.get_terminator() {
            let res: Result<TerminatorInst, _> = inst.try_into();
            if let Ok(term_inst) = res {
                return term_inst.get_conditioned_successors();
            }
        }
        vec![]
    }

    /// Removes this `BasicBlock` from its parent `FunctionValue`.
    /// It returns `Err(())` when it has no parent to remove from.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// assert_eq!(basic_block.get_parent().unwrap(), function);
    ///
    /// basic_block.remove_from_function();
    ///
    /// assert!(basic_block.get_parent().is_none());
    /// ```
    // SubTypes: Don't need to call get_parent for a BasicBlock<HasParent> and would return BasicBlock<Orphan>
    // by taking ownership of self (though BasicBlock's are not uniquely obtained...)
    // might have to make some methods do something like -> Result<..., BasicBlock<Orphan>> for BasicBlock<HasParent>
    // and would move_before/after make it no longer orphaned? etc..
    pub fn remove_from_function(self) -> Result<(), ()> {
        // This method is UB if the parent no longer exists, so we must check for parent (or encode into type system)
        if self.get_parent().is_none() {
            return Err(());
        }

        unsafe { LLVMRemoveBasicBlockFromParent(self.basic_block) }

        Ok(())
    }

    /// Removes this `BasicBlock` completely from memory. This is unsafe because you could easily have other references to the same `BasicBlock`.
    /// It returns `Err(())` when it has no parent to delete from, as LLVM assumes it has a parent.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// unsafe {
    ///     basic_block.delete();
    /// }
    /// assert!(function.get_basic_blocks().is_empty());
    /// ```
    pub unsafe fn delete(self) -> Result<(), ()> {
        // This method is UB if the parent no longer exists, so we must check for parent (or encode into type system)
        if self.get_parent().is_none() {
            return Err(());
        }

        LLVMDeleteBasicBlock(self.basic_block);

        Ok(())
    }

    /// Obtains the `ContextRef` this `BasicBlock` belongs to.
    ///
    /// # Example
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::module::Module;
    /// use inkwell::builder::Builder;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("my_module");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let function = module.add_function("do_nothing", fn_type, None);
    /// let basic_block = context.append_basic_block(function, "entry");
    ///
    /// assert_eq!(context, *basic_block.get_context());
    /// ```
    pub fn get_context(self) -> ContextRef<'ctx> {
        unsafe {
            ContextRef::new(LLVMGetTypeContext(LLVMTypeOf(
                LLVMBasicBlockAsValue(self.basic_block),
            )))
        }
    }

    /// Gets the name of a `BasicBlock`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let builder = context.create_builder();
    /// let module = context.create_module("my_mod");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let fn_val = module.add_function("my_fn", fn_type, None);
    /// let bb = context.append_basic_block(fn_val, "entry");
    ///
    /// assert_eq!(bb.get_name().to_str(), Ok("entry"));
    /// ```
    #[llvm_versions(3.9..=latest)]
    pub fn get_name(&self) -> &CStr {
        let ptr = unsafe { LLVMGetBasicBlockName(self.basic_block) };

        unsafe { CStr::from_ptr(ptr) }
    }

    /// Get name of the `BasicBlock` or return a default name.
    pub fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-block-name>".to_string(),
        }
    }

    /// Set name of the `BasicBlock`.
    pub fn set_name(&self, name: &str) {
        let c_string = to_c_str(name);
        unsafe {
            LLVMSetValueName(
                LLVMBasicBlockAsValue(self.basic_block),
                c_string.as_ptr(),
            )
        };
    }

    /// Replaces all uses of this basic block with another.
    ///
    /// # Example
    ///
    /// ```
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let builder = context.create_builder();
    /// let module = context.create_module("my_mod");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let fn_val = module.add_function("my_fn", fn_type, None);
    /// let entry = context.append_basic_block(fn_val, "entry");
    /// let bb1 = context.append_basic_block(fn_val, "bb1");
    /// let bb2 = context.append_basic_block(fn_val, "bb2");
    /// builder.position_at_end(entry);
    /// let branch_inst = builder.build_unconditional_branch(bb1);
    ///
    /// bb1.replace_all_uses_with(&bb2);
    ///
    /// assert_eq!(branch_inst.get_operand(0).unwrap().right().unwrap(), bb2);
    /// ```
    pub fn replace_all_uses_with(self, other: &BasicBlock<'ctx>) {
        let value = unsafe { LLVMBasicBlockAsValue(self.basic_block) };
        let other = unsafe { LLVMBasicBlockAsValue(other.basic_block) };

        // LLVM may infinite-loop when they aren't distinct, which is UB in C++.
        if value != other {
            unsafe {
                LLVMReplaceAllUsesWith(value, other);
            }
        }
    }

    /// Gets the first use of this `BasicBlock` if any.
    ///
    /// The following example,
    ///
    /// ```no_run
    /// use inkwell::AddressSpace;
    /// use inkwell::context::Context;
    /// use inkwell::values::BasicValue;
    ///
    /// let context = Context::create();
    /// let module = context.create_module("ivs");
    /// let builder = context.create_builder();
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let fn_val = module.add_function("my_fn", fn_type, None);
    /// let entry = context.append_basic_block(fn_val, "entry");
    /// let bb1 = context.append_basic_block(fn_val, "bb1");
    /// let bb2 = context.append_basic_block(fn_val, "bb2");
    /// builder.position_at_end(entry);
    /// let branch_inst = builder.build_unconditional_branch(bb1);
    ///
    /// assert!(bb2.get_first_use().is_none());
    /// assert!(bb1.get_first_use().is_some());
    /// ```
    pub fn get_first_use(self) -> Option<BasicValueUse<'ctx>> {
        let use_ =
            unsafe { LLVMGetFirstUse(LLVMBasicBlockAsValue(self.basic_block)) };

        if use_.is_null() {
            return None;
        }

        unsafe { Some(BasicValueUse::new(use_)) }
    }

    /// Get all users of the current `BasicBlock`.
    pub fn get_all_users(self) -> Vec<AnyValueEnum<'ctx>> {
        self.as_value().get_all_users()
    }

    /// Gets the address of this `BasicBlock` if possible. Returns `None` if `self` is the entry block to a function.
    ///
    /// # Safety
    ///
    /// The returned PointerValue may only be used for `call` and `indirect_branch` instructions
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    /// let context = Context::create();
    /// let module = context.create_module("my_mod");
    /// let void_type = context.void_type();
    /// let fn_type = void_type.fn_type(&[], false);
    /// let fn_val = module.add_function("my_fn", fn_type, None);
    /// let entry_bb = context.append_basic_block(fn_val, "entry");
    /// let next_bb = context.append_basic_block(fn_val, "next");
    ///
    /// assert!(unsafe { entry_bb.get_address() }.is_none());
    /// assert!(unsafe { next_bb.get_address() }.is_some());
    /// ```
    pub unsafe fn get_address(self) -> Option<PointerValue<'ctx>> {
        let parent = self.get_parent()?;

        // Taking the address of the entry block is illegal.
        self.get_previous_basic_block()?;

        let value = PointerValue::new(LLVMBlockAddress(
            parent.as_value_ref(),
            self.basic_block,
        ));

        if value.is_null() {
            return None;
        }

        Some(value)
    }

    /// Convert a `BasicBlock` to `Value`.
    fn as_value(self) -> Value<'ctx> {
        unsafe { Value::new(self.basic_block as LLVMValueRef) }
    }

    /// Print the `BasicBlock` to `LLVMString`.
    pub fn print_to_llvm_string(self) -> LLVMString {
        unsafe {
            let block_value = LLVMBasicBlockAsValue(self.basic_block);
            LLVMString::new(LLVMPrintValueToString(block_value))
        }
    }

    /// Print the `BasicBlock` to `String`.
    pub fn print_to_string(self) -> String {
        self.print_to_llvm_string().to_string()
    }
}

impl<'ctx> AsValueRef for BasicBlock<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.basic_block as LLVMValueRef
    }
}

impl<'ctx> Display for BasicBlock<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

impl<'ctx> fmt::Debug for BasicBlock<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let llvm_value = unsafe {
            CStr::from_ptr(LLVMPrintValueToString(
                self.basic_block as LLVMValueRef,
            ))
        };
        let llvm_type = unsafe {
            CStr::from_ptr(LLVMPrintTypeToString(LLVMTypeOf(
                self.basic_block as LLVMValueRef,
            )))
        };
        let is_const =
            unsafe { LLVMIsConstant(self.basic_block as LLVMValueRef) == 1 };

        f.debug_struct("BasicBlock")
            .field("address", &self.basic_block)
            .field("is_const", &is_const)
            .field("llvm_value", &llvm_value)
            .field("llvm_type", &llvm_type)
            .finish()
    }
}

#[cfg(feature = "internal-getters")]
impl<'ctx> LLVMReference<LLVMBasicBlockRef> for BasicBlock<'ctx> {
    unsafe fn get_ref(&self) -> LLVMBasicBlockRef {
        self.basic_block
    }
}
