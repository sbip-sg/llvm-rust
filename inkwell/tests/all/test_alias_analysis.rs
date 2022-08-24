#[test]
#[cfg(feature = "internal-getters")]
fn test_basic_alias_analysis() {
    use inkwell::analysis::alias::BasicAliasAnalysis;
    use inkwell::context::Context;
    use inkwell::values::AnyValue;
    use inkwell::AddressSpace;

    // Builds a function which takes an i32 pointer and stores a 7 in it.
    let context = Context::create();
    let module = context.create_module("alias");
    let builder = context.create_builder();
    let void_type = context.void_type();
    let i32_type = context.i32_type();
    let i8_type = context.i8_type();
    let i32_ptr_type = i32_type.ptr_type(AddressSpace::Generic);
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::Generic);
    let fn_type = void_type.fn_type(&[i32_ptr_type.into()], false);

    let fname = "fff";
    let fn_value = module.add_function(fname, fn_type, None);

    let entry = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry);

    let alloca_b = builder.build_alloca(i32_type, "b");
    let alloca_d = builder.build_alloca(i32_ptr_type, "d");
    let alloca_e = builder.build_alloca(i32_ptr_type, "e");

    builder.build_store(alloca_d, alloca_b);
    builder.build_store(alloca_e, alloca_b);

    let inst_g = builder.build_load(alloca_d, "g");
    let inst_m = builder
        .build_bitcast(inst_g, i8_ptr_type, "m")
        .as_any_value_enum()
        .into_pointer_value();

    let inst_h = builder.build_load(alloca_e, "h");
    let inst_n = builder
        .build_bitcast(inst_h, i8_ptr_type, "n")
        .as_any_value_enum()
        .into_pointer_value();

    let fn_name = "check_alias";
    let fn_type = void_type.fn_type(&[i8_type.into(), i8_type.into()], false);
    let alias_fun = module.add_function(fn_name, fn_type, None);
    let _ = builder
        .build_call(alias_fun, &[inst_m.into(), inst_n.into()], "")
        .try_as_basic_value()
        .right()
        .unwrap();
    builder.position_at_end(entry);
    builder.build_return(None);

    let baa = BasicAliasAnalysis::new(module);
    assert!(!baa.is_must_alias(&fn_value, inst_m, inst_n));
    assert!(baa.is_may_alias(&fn_value, inst_m, inst_n));
    assert!(!baa.is_no_alias(&fn_value, inst_m, inst_n));
}

#[test]
#[cfg(feature = "internal-getters")]
fn test_type_based_alias_analysis() {
    use inkwell::analysis::alias::TypeBasedAliasAnalysis;
    use inkwell::context::Context;
    use inkwell::values::AnyValue;
    use inkwell::AddressSpace;

    // Builds a function which takes an i32 pointer and stores a 7 in it.
    let context = Context::create();
    let module = context.create_module("alias");
    let builder = context.create_builder();
    let void_type = context.void_type();
    let i32_type = context.i32_type();
    let i8_type = context.i8_type();
    let i32_ptr_type = i32_type.ptr_type(AddressSpace::Generic);
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::Generic);
    let fn_type = void_type.fn_type(&[i32_ptr_type.into()], false);

    let fname = "fff";
    let fn_value = module.add_function(fname, fn_type, None);

    let entry = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry);

    let alloca_b = builder.build_alloca(i32_type, "b");
    let alloca_d = builder.build_alloca(i32_ptr_type, "d");
    let alloca_e = builder.build_alloca(i32_ptr_type, "e");

    builder.build_store(alloca_d, alloca_b);
    builder.build_store(alloca_e, alloca_b);

    let inst_g = builder.build_load(alloca_d, "g");
    let inst_m = builder
        .build_bitcast(inst_g, i8_ptr_type, "m")
        .as_any_value_enum()
        .into_pointer_value();

    let inst_h = builder.build_load(alloca_e, "h");
    let inst_n = builder
        .build_bitcast(inst_h, i8_ptr_type, "n")
        .as_any_value_enum()
        .into_pointer_value();

    let fn_name = "check_alias";
    let fn_type = void_type.fn_type(&[i8_type.into(), i8_type.into()], false);
    let alias_fun = module.add_function(fn_name, fn_type, None);
    let _ = builder
        .build_call(alias_fun, &[inst_m.into(), inst_n.into()], "")
        .try_as_basic_value()
        .right()
        .unwrap();
    builder.position_at_end(entry);
    builder.build_return(None);

    let tbaa = TypeBasedAliasAnalysis::new(module);
    assert!(!tbaa.is_must_alias(inst_m, inst_n));
    assert!(tbaa.is_may_alias(inst_m, inst_n));
    assert!(!tbaa.is_no_alias(inst_m, inst_n));
}
