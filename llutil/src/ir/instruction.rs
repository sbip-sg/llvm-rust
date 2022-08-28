//! Module provide additional utilities to handle LLVM `InstructionValue`.

use inkwell::values::{
    BasicValue, BasicValueEnum, FloatValue, InstructionValue, IntValue,
    PointerValue,
};

use super::{
    AllocaInst, BinaryOperator, BranchInst, CallBase, CallBrInst, CallInst,
    CastInst, CmpInst, FCmpInst, ICmpInst, IndirectBrInst, InvokeInst,
    LoadInst, PhiNode, ReturnInst, SExtInst, StoreInst, SwitchInst,
    TerminatorInst, TruncInst, UnaryOperator, UnreachableInst, ZExtInst,
};

/// Trait providing additional functions to handle `InstructionValue`.
pub trait InstructionExt<'ctx> {
    /// Get name of the `InstructionValue` or return a default name.
    fn get_name_or_default(&self) -> String;

    /// Convert the current `InstructionValue` to `AllocaInst`.
    fn try_into_alloca_inst(self) -> Option<AllocaInst<'ctx>>;

    /// Convert the current `InstructionValue` to `BinaryOperator`.
    fn try_into_binary_operator(self) -> Option<BinaryOperator<'ctx>>;

    /// Convert the current `InstructionValue` to `BranchInst`.
    fn try_into_branch_inst(self) -> Option<BranchInst<'ctx>>;

    /// Convert the current `InstructionValue` to `CallBase`.
    fn try_into_call_base(self) -> Option<CallBase<'ctx>>;

    /// Convert the current `InstructionValue` to `CallInst`.
    fn try_into_call_inst(self) -> Option<CallInst<'ctx>>;

    /// Convert the current `InstructionValue` to `CallBrInst`.
    fn try_into_callbr_inst(self) -> Option<CallBrInst<'ctx>>;

    /// Convert the current `InstructionValue` to `CastInst`.
    fn try_into_cast_inst(self) -> Option<CastInst<'ctx>>;

    /// Convert the current `InstructionValue` to `CmpInst`.
    fn try_into_cmp_inst(self) -> Option<CmpInst<'ctx>>;

    /// Convert the current `InstructionValue` to `FCmpInst`.
    fn try_into_fcmp_inst(self) -> Option<FCmpInst<'ctx>>;

    /// Convert the current `InstructionValue` to `ICmpInst`.
    fn try_into_icmp_inst(self) -> Option<ICmpInst<'ctx>>;

    /// Convert the current `InstructionValue` to `IndirectBrInst`.
    fn try_into_indirectbr_inst(self) -> Option<IndirectBrInst<'ctx>>;

    /// Convert the current `InstructionValue` to `InvokeInst`.
    fn try_into_invoke_inst(self) -> Option<InvokeInst<'ctx>>;

    /// Convert the current `InstructionValue` to `LoadInst`.
    fn try_into_load_inst(self) -> Option<LoadInst<'ctx>>;

    /// Convert the current `InstructionValue` to `PhiNode`.
    fn try_into_phi_node(self) -> Option<PhiNode<'ctx>>;

    /// Convert the current `InstructionValue` to `ReturnInst`.
    fn try_into_return_inst(self) -> Option<ReturnInst<'ctx>>;

    /// Convert the current `InstructionValue` to `SExtInst`.
    fn try_into_sext_inst(self) -> Option<SExtInst<'ctx>>;

    /// Convert the current `InstructionValue` to `StoreInst`.
    fn try_into_store_inst(self) -> Option<StoreInst<'ctx>>;

    /// Convert the current `InstructionValue` to `SwitchInst`.
    fn try_into_switch_inst(self) -> Option<SwitchInst<'ctx>>;

    /// Convert the current `InstructionValue` to `TerminatorInst`.
    fn try_into_terminator_inst(self) -> Option<TerminatorInst<'ctx>>;

    /// Convert the current `InstructionValue` to `TruncInst`.
    fn try_into_trunc_inst(self) -> Option<TruncInst<'ctx>>;

    /// Convert the current `InstructionValue` to `UnaryOperator`.
    fn try_into_unary_operator(self) -> Option<UnaryOperator<'ctx>>;

    /// Convert the current `InstructionValue` to `SwitchInst`.
    fn try_into_unreachable_inst(self) -> Option<UnreachableInst<'ctx>>;

    /// Convert the current `InstructionValue` to `ZExtInst`.
    fn try_into_zext_inst(self) -> Option<ZExtInst<'ctx>>;

    /// Convert the current `InstructionValue` to an `IntValue`.
    fn try_into_int_value(self) -> Option<IntValue<'ctx>>;

    /// Convert the current `InstructionValue` to a `FloatValue`.
    fn try_into_float_value(self) -> Option<FloatValue<'ctx>>;

    /// Convert the current `InstructionValue` to a `PointerValue`.
    fn try_into_pointer_value(self) -> Option<PointerValue<'ctx>>;

    /// Convert the current `InstructionValue` to a `BasicValueEnum`.
    fn try_into_basic_value_enum(self) -> Option<BasicValueEnum<'ctx>>;
}

/// Implement the trait `InstructionExt` for `InstructionValue`.
impl<'ctx> InstructionExt<'ctx> for InstructionValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        if let Some(name) = self.get_name() {
            if let Ok(name) = name.to_str() {
                return name.to_string();
            }
        }

        String::from("<empty-instruction-name>")
    }

    fn try_into_alloca_inst(self) -> Option<AllocaInst<'ctx>> {
        let res: Result<AllocaInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_binary_operator(self) -> Option<BinaryOperator<'ctx>> {
        let res: Result<BinaryOperator, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_branch_inst(self) -> Option<BranchInst<'ctx>> {
        let res: Result<BranchInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_call_base(self) -> Option<CallBase<'ctx>> {
        let res: Result<CallBase, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_call_inst(self) -> Option<CallInst<'ctx>> {
        let res: Result<CallInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_callbr_inst(self) -> Option<CallBrInst<'ctx>> {
        let res: Result<CallBrInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_cast_inst(self) -> Option<CastInst<'ctx>> {
        let res: Result<CastInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_cmp_inst(self) -> Option<CmpInst<'ctx>> {
        let res: Result<CmpInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_fcmp_inst(self) -> Option<FCmpInst<'ctx>> {
        let res: Result<FCmpInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_icmp_inst(self) -> Option<ICmpInst<'ctx>> {
        let res: Result<ICmpInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_indirectbr_inst(self) -> Option<IndirectBrInst<'ctx>> {
        let res: Result<IndirectBrInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_invoke_inst(self) -> Option<InvokeInst<'ctx>> {
        let res: Result<InvokeInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_load_inst(self) -> Option<LoadInst<'ctx>> {
        let res: Result<LoadInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_phi_node(self) -> Option<PhiNode<'ctx>> {
        let res: Result<PhiNode, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_return_inst(self) -> Option<ReturnInst<'ctx>> {
        let res: Result<ReturnInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_sext_inst(self) -> Option<SExtInst<'ctx>> {
        let res: Result<SExtInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_store_inst(self) -> Option<StoreInst<'ctx>> {
        let res: Result<StoreInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_switch_inst(self) -> Option<SwitchInst<'ctx>> {
        let res: Result<SwitchInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_terminator_inst(self) -> Option<TerminatorInst<'ctx>> {
        let res: Result<TerminatorInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_trunc_inst(self) -> Option<TruncInst<'ctx>> {
        let res: Result<TruncInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_unary_operator(self) -> Option<UnaryOperator<'ctx>> {
        let res: Result<UnaryOperator, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_unreachable_inst(self) -> Option<UnreachableInst<'ctx>> {
        let res: Result<UnreachableInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_zext_inst(self) -> Option<ZExtInst<'ctx>> {
        let res: Result<ZExtInst, _> = self.try_into();
        match res {
            Ok(inst) => Some(inst),
            Err(_) => None,
        }
    }

    fn try_into_int_value(self) -> Option<IntValue<'ctx>> {
        let res: Result<IntValue, _> = self.try_into();
        match res {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    fn try_into_float_value(self) -> Option<FloatValue<'ctx>> {
        let res: Result<FloatValue, _> = self.try_into();
        match res {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    fn try_into_pointer_value(self) -> Option<PointerValue<'ctx>> {
        let res: Result<PointerValue, _> = self.try_into();
        match res {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    fn try_into_basic_value_enum(self) -> Option<BasicValueEnum<'ctx>> {
        if let Some(v) = self.try_into_int_value() {
            return Some(v.as_basic_value_enum());
        } else if let Some(v) = self.try_into_float_value() {
            return Some(v.as_basic_value_enum());
        } else if let Some(v) = self.try_into_pointer_value() {
            return Some(v.as_basic_value_enum());
        } else {
            None
        }
    }
}
