//! Module provide additional utilities to handle LLVM `BasicBlock`.

use inkwell::cfg::PredecessorBlock;
use inkwell::values::BasicBlock;

use rutil::string::StringUtil;

// use instructions::TerminatorUtil;

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
