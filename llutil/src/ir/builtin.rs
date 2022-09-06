//! Module implementing a list of library functions

/// Module containing built-in names of LLVM functions.
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
pub mod llvm_lib {
    pub const LLVM_DBG_ADDR: &str = "llvm.dbg.addr";
    pub const LLVM_DBG_DECLARE: &str = "llvm.dbg.declare";
    pub const LLVM_DBG_VALUE: &str = "llvm.dbg.value";
}

/// Module containing built-in library function names of the C language.
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
pub mod c_lib {
    pub const ISOC99_FSCANF: &str = "__isoc99_fscanf";
    pub const ISOC99_SSCANF: &str = "__isoc99_sscanf";
    pub const ISOC99_SWSCANF: &str = "__isoc99_swscanf";
    pub const ISWXDIGIT: &str = "iswxdigit";
    pub const PRINTF: &str = "printf";
    pub const PUTS: &str = "puts";
    pub const RAND: &str = "rand";
    pub const SRAND: &str = "srand";
    pub const TIME: &str = "time";
    pub const WPRINTF: &str = "wprintf";
    pub const CTYPE_B_LOC: &str = "__ctype_b_loc";
}

/// Module containing the main function names of the C language.
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
pub mod cmain {
    pub const MAIN: &str = "main";
}

/// List of all considered C library functions.
pub const C_LIB_FUNCS: &[&str] = &[
    c_lib::ISOC99_FSCANF,
    c_lib::ISOC99_SSCANF,
    c_lib::ISWXDIGIT,
    c_lib::ISOC99_SWSCANF,
    c_lib::PRINTF,
    c_lib::PUTS,
    c_lib::RAND,
    c_lib::SRAND,
    c_lib::TIME,
    c_lib::WPRINTF,
    c_lib::CTYPE_B_LOC,
];

/// Module containing built-in names of assertion functions.
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
pub mod assertion_lib {
    // Range analysis
    pub const ASSERT_RANGE: &str = "__assert_range";
    pub const REFUTE_RANGE: &str = "__refute_range";
    pub const ASSERT_LOWER_BOUND: &str = "__assert_lower_bound";
    pub const REFUTE_LOWER_BOUND: &str = "__refute_lower_bound";
    pub const REFUTE_UPPER_BOUND: &str = "__refute_upper_bound";
    pub const ASSERT_UPPER_BOUND: &str = "__assert_upper_bound";

    // Alias analysis
    pub const ASSERT_NO_ALIAS: &str = "__assert_no_alias";
    pub const ASSERT_MUST_ALIAS: &str = "__assert_must_alias";
    pub const REFUTE_MUST_ALIAS: &str = "__refute_must_alias";
    pub const ASSERT_MAY_ALIAS: &str = "__assert_may_alias";
    pub const REFUTE_MAY_ALIAS: &str = "__refute_may_alias";
    pub const REFUTE_NO_ALIAS: &str = "__refute_no_alias";

    // General prefixes
    pub const PREFIX_ASSERT: &str = "__assert_";
    pub const PREFIX_REFUTE: &str = "__refute_";
    pub const PREFIX_ASSUME: &str = "__assume_";
}

/// Module containing built-in names of Solidity library functions generated by
/// the Solang compiler to EWASM target.
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
pub mod solang_ewasm_lib {
    pub const ACCOUNT_DATA_ALLOC: &str = "account_data_alloc";
    pub const ACCOUNT_DATA_FREE: &str = "account_data_free";
    pub const ACCOUNT_DATA_LEN: &str = "account_data_len";
    pub const ACCOUNT_DATA_REALLOC: &str = "account_data_realloc";
    pub const ADDRESS_EQUAL: &str = "address_equal";
    pub const ADDRESS_HASH: &str = "address_hash";
    pub const ASHLTI3: &str = "__ashlti3";
    pub const ASHRTI3: &str = "__ashrti3";
    pub const BE32TOLEN: &str = "__be32toleN";
    pub const BENTOLEN: &str = "__beNtoleN";
    pub const BITS128: &str = "bits128";
    pub const BITS256: &str = "bits256";
    pub const BITS512: &str = "bits512";
    pub const BITS: &str = "bits";
    pub const BZERO8: &str = "__bzero8";
    pub const CALL: &str = "call";
    pub const CALLDATACOPY: &str = "callDataCopy";
    pub const CALLDELEGATE: &str = "callDelegate";
    pub const CALLSTATIC: &str = "callStatic";
    pub const CODECOPY: &str = "codeCopy";
    pub const CONCAT: &str = "concat";
    pub const CREATE: &str = "create";
    pub const CREATE_CONTRACT: &str = "create_contract";
    pub const ENTRYPOINT: &str = "entrypoint";
    pub const EXTERNAL_CALL: &str = "external_call";
    pub const FINISH: &str = "finish";
    pub const FREE: &str = "__free";
    pub const GETADDRESS: &str = "getAddress";
    pub const GETBLOCKCOINBASE: &str = "getBlockCoinbase";
    pub const GETBLOCKDIFFICULTY: &str = "getBlockDifficulty";
    pub const GETBLOCKGASLIMIT: &str = "getBlockGasLimit";
    pub const GETBLOCKHASH: &str = "getBlockHash";
    pub const GETBLOCKNUMBER: &str = "getBlockNumber";
    pub const GETBLOCKTIMESTAMP: &str = "getBlockTimestamp";
    pub const GETCALLDATASIZE: &str = "getCallDataSize";
    pub const GETCALLER: &str = "getCaller";
    pub const GETCALLVALUE: &str = "getCallValue";
    pub const GETCODESIZE: &str = "getCodeSize";
    pub const GETEXTERNALBALANCE: &str = "getExternalBalance";
    pub const GETEXTERNALCODESIZE: &str = "getExternalCodeSize";
    pub const GETGASLEFT: &str = "getGasLeft";
    pub const GETRETURNDATASIZE: &str = "getReturnDataSize";
    pub const GETTXGASPRICE: &str = "getTxGasPrice";
    pub const GETTXORIGIN: &str = "getTxOrigin";
    pub const HEX_ENCODE: &str = "hex_encode";
    pub const HEX_ENCODE_REV: &str = "hex_encode_rev";
    pub const INIT_HEAP: &str = "__init_heap";
    pub const LENTOBE32: &str = "__leNtobe32";
    pub const LENTOBEN: &str = "__leNtobeN";
    pub const LLVM_ASSUME: &str = "llvm.assume";
    pub const LLVM_UADD_WITH_OVERFLOW_I64: &str = "llvm.uadd.with.overflow.i64";
    pub const LLVM_WASM_MEMORY_SIZE_I32: &str = "llvm.wasm.memory.size.i32";
    pub const LOG: &str = "log";
    pub const LSHRTI3: &str = "__lshrti3";
    pub const MALLOC: &str = "__malloc";
    pub const MEMCMP: &str = "__memcmp";
    pub const MEMCMP_ORD: &str = "__memcmp_ord";
    pub const MEMCPY8: &str = "__memcpy8";
    pub const MEMCPY: &str = "__memcpy";
    pub const MEMSET8: &str = "__memset8";
    pub const MEMSET: &str = "__memset";
    pub const MUL32: &str = "__mul32";
    pub const PRINTMEM: &str = "printMem";
    pub const REALLOC: &str = "__realloc";
    pub const RETURNDATACOPY: &str = "returnDataCopy";
    pub const REVERT: &str = "revert";
    pub const RIPEMD160: &str = "ripemd160";
    pub const RIPEMD160_COMPRESS: &str = "ripemd160_compress";
    pub const SDIVMOD128: &str = "sdivmod128";
    pub const SDIVMOD256: &str = "sdivmod256";
    pub const SDIVMOD512: &str = "sdivmod512";
    pub const SELFDESTRUC: &str = "selfDestruc";
    pub const SHL128: &str = "shl128";
    pub const SHR128: &str = "shr128";
    pub const SIGNATURE_VERIFY: &str = "signature_verify";
    pub const SOLANG_DISPATCH: &str = "solang_dispatch";
    pub const SOLPUBKEY_SAME: &str = "SolPubkey_same";
    pub const SOL_ACCOUNT_LAMPORT: &str = "sol_account_lamport";
    pub const SOL_CLOCK: &str = "sol_clock";
    pub const SOL_TRANSFER: &str = "sol_transfer";
    pub const SOL_TRY_TRANSFER: &str = "sol_try_transfer";
    pub const STORAGELOAD: &str = "storageLoad";
    pub const STORAGESTORE: &str = "storageStore";
    pub const UDIVMOD128: &str = "udivmod128";
    pub const UDIVMOD256: &str = "udivmod256";
    pub const UDIVMOD512: &str = "udivmod512";
    pub const UINT128DEC: &str = "uint128dec";
    pub const UINT256DEC: &str = "uint256dec";
    pub const UINT2BIN: &str = "uint2bin";
    pub const UINT2DEC: &str = "uint2dec";
    pub const UINT2HEX: &str = "uint2hex";
    pub const VECTOR_HASH: &str = "vector_hash";
    pub const VECTOR_NEW: &str = "vector_new";
}

/// List of all considered Solidity library functions generated by the Solang
/// compiler to EWASM target.
pub const SOLANG_WASM_LIB_FUNCS: &[&str] = &[
    solang_ewasm_lib::ACCOUNT_DATA_ALLOC,
    solang_ewasm_lib::ACCOUNT_DATA_FREE,
    solang_ewasm_lib::ACCOUNT_DATA_LEN,
    solang_ewasm_lib::ACCOUNT_DATA_REALLOC,
    solang_ewasm_lib::ADDRESS_EQUAL,
    solang_ewasm_lib::ADDRESS_HASH,
    solang_ewasm_lib::ASHLTI3,
    solang_ewasm_lib::ASHRTI3,
    solang_ewasm_lib::BE32TOLEN,
    solang_ewasm_lib::BENTOLEN,
    solang_ewasm_lib::BITS,
    solang_ewasm_lib::BITS128,
    solang_ewasm_lib::BITS256,
    solang_ewasm_lib::BITS512,
    solang_ewasm_lib::BZERO8,
    solang_ewasm_lib::CALL,
    solang_ewasm_lib::CALLDATACOPY,
    solang_ewasm_lib::CALLDELEGATE,
    solang_ewasm_lib::CALLSTATIC,
    solang_ewasm_lib::CODECOPY,
    solang_ewasm_lib::CONCAT,
    solang_ewasm_lib::CREATE,
    solang_ewasm_lib::CREATE_CONTRACT,
    solang_ewasm_lib::ENTRYPOINT,
    solang_ewasm_lib::EXTERNAL_CALL,
    solang_ewasm_lib::FINISH,
    solang_ewasm_lib::FREE,
    solang_ewasm_lib::GETADDRESS,
    solang_ewasm_lib::GETBLOCKCOINBASE,
    solang_ewasm_lib::GETBLOCKDIFFICULTY,
    solang_ewasm_lib::GETBLOCKGASLIMIT,
    solang_ewasm_lib::GETBLOCKHASH,
    solang_ewasm_lib::GETBLOCKNUMBER,
    solang_ewasm_lib::GETBLOCKTIMESTAMP,
    solang_ewasm_lib::GETCALLDATASIZE,
    solang_ewasm_lib::GETCALLER,
    solang_ewasm_lib::GETCALLVALUE,
    solang_ewasm_lib::GETCODESIZE,
    solang_ewasm_lib::GETEXTERNALBALANCE,
    solang_ewasm_lib::GETEXTERNALCODESIZE,
    solang_ewasm_lib::GETGASLEFT,
    solang_ewasm_lib::GETRETURNDATASIZE,
    solang_ewasm_lib::GETTXGASPRICE,
    solang_ewasm_lib::GETTXORIGIN,
    solang_ewasm_lib::HEX_ENCODE,
    solang_ewasm_lib::HEX_ENCODE_REV,
    solang_ewasm_lib::INIT_HEAP,
    solang_ewasm_lib::LENTOBE32,
    solang_ewasm_lib::LENTOBEN,
    solang_ewasm_lib::LLVM_ASSUME,
    solang_ewasm_lib::LLVM_UADD_WITH_OVERFLOW_I64,
    solang_ewasm_lib::LLVM_WASM_MEMORY_SIZE_I32,
    solang_ewasm_lib::LOG,
    solang_ewasm_lib::LSHRTI3,
    solang_ewasm_lib::MALLOC,
    solang_ewasm_lib::MEMCMP,
    solang_ewasm_lib::MEMCMP_ORD,
    solang_ewasm_lib::MEMCPY,
    solang_ewasm_lib::MEMCPY8,
    solang_ewasm_lib::MEMSET,
    solang_ewasm_lib::MEMSET8,
    solang_ewasm_lib::MUL32,
    solang_ewasm_lib::PRINTMEM,
    solang_ewasm_lib::REALLOC,
    solang_ewasm_lib::RETURNDATACOPY,
    solang_ewasm_lib::REVERT,
    solang_ewasm_lib::RIPEMD160,
    solang_ewasm_lib::RIPEMD160_COMPRESS,
    solang_ewasm_lib::SDIVMOD128,
    solang_ewasm_lib::SDIVMOD256,
    solang_ewasm_lib::SDIVMOD512,
    solang_ewasm_lib::SELFDESTRUC,
    solang_ewasm_lib::SHL128,
    solang_ewasm_lib::SHR128,
    solang_ewasm_lib::SIGNATURE_VERIFY,
    solang_ewasm_lib::SOLANG_DISPATCH,
    solang_ewasm_lib::SOLPUBKEY_SAME,
    solang_ewasm_lib::SOL_ACCOUNT_LAMPORT,
    solang_ewasm_lib::SOL_CLOCK,
    solang_ewasm_lib::SOL_TRANSFER,
    solang_ewasm_lib::SOL_TRY_TRANSFER,
    solang_ewasm_lib::STORAGELOAD,
    solang_ewasm_lib::STORAGESTORE,
    solang_ewasm_lib::UDIVMOD128,
    solang_ewasm_lib::UDIVMOD256,
    solang_ewasm_lib::UDIVMOD512,
    solang_ewasm_lib::UINT128DEC,
    solang_ewasm_lib::UINT256DEC,
    solang_ewasm_lib::UINT2BIN,
    solang_ewasm_lib::UINT2DEC,
    solang_ewasm_lib::UINT2HEX,
    solang_ewasm_lib::VECTOR_HASH,
    solang_ewasm_lib::VECTOR_NEW,
];

/// Check whether a function is a library function of C code.
pub fn is_c_library_function(func_name: &str) -> bool {
    C_LIB_FUNCS.contains(&func_name)
}

/// Check whether a function is a main function of C code.
pub fn is_c_main_function(func_name: &str) -> bool {
    func_name.eq(cmain::MAIN)
}

/// Check whether a function is a Solang-generated library function of Solidity.
pub fn is_solidity_library_function(func_name: &str) -> bool {
    SOLANG_WASM_LIB_FUNCS.contains(&func_name)
}

/// Check whether a function is a entry function of Solidity.
pub fn is_solang_main_function(func_name: &str) -> bool {
    func_name.eq(cmain::MAIN)
}

/// Check whether a function is an LLVM intrinsic function.
pub fn is_llvm_intrinsic_function(func_name: &str) -> bool {
    func_name.eq(llvm_lib::LLVM_DBG_ADDR)
        || func_name.eq(llvm_lib::LLVM_DBG_DECLARE)
        || func_name.eq(llvm_lib::LLVM_DBG_VALUE)
}

/// Check whether a function is a built-in function of Verazt.
pub fn is_assertio_checking_function(func_name: &str) -> bool {
    func_name.starts_with(assertion_lib::PREFIX_ASSERT)
        || func_name.starts_with(assertion_lib::PREFIX_REFUTE)
        || func_name.starts_with(assertion_lib::PREFIX_ASSUME)
}

/// Check whether a function is `__assert_range`
pub fn check_assert_range_full(func_name: &str) -> bool {
    if func_name.eq(assertion_lib::ASSERT_RANGE) {
        return true;
    }
    false
}

/// Check whether a function is `__assert_lower_bound`
pub fn check_assert_range_lower_bound(input_func: &str) -> bool {
    if input_func.eq(assertion_lib::ASSERT_LOWER_BOUND) {
        return true;
    }
    false
}

/// Check whether a function is `__assert_upper_bound`
pub fn check_assert_range_upper_bound(input_func: &str) -> bool {
    if input_func.eq(assertion_lib::ASSERT_UPPER_BOUND) {
        return true;
    }
    false
}
