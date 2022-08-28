//! Module provide additional utilities to handle LLVM `BasicBlock`.

use inkwell::values::BasicBlock;

use rutil::string::StringUtil;

use super::{
    AnyTerminator, InstructionExt, PhiNode, PredecessorBlock, SuccessorBlock,
    TerminatorInst,
};

// use instructions::TerminatorUtil;

/// Trait providing utility functions to handle the `Vec<BasicBlock>` data
/// structure.
pub trait BasicBlockExt<'ctx> {
    /// Get name of the `BasicBlock` or return a default name.
    fn get_name_or_default(&self) -> String;

    /// Get all the Phi instructions of the current `BasicBlock`.
    ///
    /// By LLVM IR formatl, all Phi instructions must be located at the top of
    /// the `BasicBlock`.
    fn get_phi_instructions(&self) -> Vec<PhiNode<'ctx>>;

    /// Get predecessor blocks of the current `BasicBlock`.
    ///
    /// A predecessor block is the block that jumps to the current block.
    fn get_predecessors(&self) -> Vec<BasicBlock<'ctx>>;

    /// Get successor blocks of the current `BasicBlock`.
    ///
    /// A successor block is the block that the current block jumps to.
    fn get_successors(&self) -> Vec<BasicBlock<'ctx>>;

    /// Get predecessor blocks of the current `BasicBlock` and their path
    /// conditions.
    fn get_conditioned_predecessors(self) -> Vec<PredecessorBlock<'ctx>>;

    /// Get successor blocks of the current `BasicBlock` and their path
    /// conditions.
    fn get_conditioned_successors(self) -> Vec<SuccessorBlock<'ctx>>;
}

impl<'ctx> BasicBlockExt<'ctx> for BasicBlock<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-block-name>".to_string(),
        }
    }

    fn get_phi_instructions(&self) -> Vec<PhiNode<'ctx>> {
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

    fn get_predecessors(&self) -> Vec<BasicBlock<'ctx>> {
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

    fn get_successors(&self) -> Vec<BasicBlock<'ctx>> {
        if let Some(inst) = self.get_terminator() {
            let res: Result<TerminatorInst, _> = inst.try_into();
            if let Ok(term_inst) = res {
                return term_inst.get_successors();
            }
        }
        vec![]
    }

    fn get_conditioned_predecessors(self) -> Vec<PredecessorBlock<'ctx>> {
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

    fn get_conditioned_successors(self) -> Vec<SuccessorBlock<'ctx>> {
        if let Some(inst) = self.get_terminator() {
            let res: Result<TerminatorInst, _> = inst.try_into();
            if let Ok(term_inst) = res {
                return term_inst.get_conditioned_successors();
            }
        }
        vec![]
    }
}

/// Trait providing utility functions to handle the `Vec<BasicBlock>` data
/// structure.
pub trait Blocks<'a> {
    /// Print names of `BasicBlock` in the list.
    fn print_block_names(&self) -> String;
}

/// Implement the trait `Blocks` for a vector of `BasicBlock`.
impl<'a> Blocks<'a> for Vec<BasicBlock<'a>> {
    fn print_block_names(&self) -> String {
        self.iter()
            .map(|blk| blk.get_name_or_default())
            .collect::<Vec<String>>()
            .join(", ")
            .add_prefix_and_suffix("[", "]")
    }
}

/// Implement the trait `Blocks` for a vector of `PredecessorBlock`.
impl<'a> Blocks<'a> for Vec<PredecessorBlock<'a>> {
    fn print_block_names(&self) -> String {
        self.iter()
            .map(|pblk| pblk.block.get_name_or_default())
            .collect::<Vec<String>>()
            .join(", ")
            .add_prefix_and_suffix("[", "]")
    }
}
