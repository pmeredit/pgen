use std::str::FromStr;
use ast::{Expr, Opcode, Let, Cond, Switch};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__TopExpr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, Opcode, Let, Cond, Switch};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3e_22(&'input str),
        Term_22default_22(&'input str),
        Term_22else_22(&'input str),
        Term_22false_22(&'input str),
        Term_22if_22(&'input str),
        Term_22in_22(&'input str),
        Term_22let_22(&'input str),
        Term_22switch_22(&'input str),
        Term_22then_22(&'input str),
        Term_22true_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ___5d_5c_5cw_2a_22_23(&'input str),
        Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(&'input str),
        Nt_28_3cAssignment_3e_20_22_2c_22_29((String, Box<Expr>)),
        Nt_28_3cAssignment_3e_20_22_2c_22_29_2a(::std::vec::Vec<(String, Box<Expr>)>),
        Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(::std::vec::Vec<(String, Box<Expr>)>),
        Nt_28_3cCase_3e_20_22_2c_22_29((Box<Expr>, Box<Expr>)),
        Nt_28_3cCase_3e_20_22_2c_22_29_2a(::std::vec::Vec<(Box<Expr>, Box<Expr>)>),
        Nt_28_3cCase_3e_20_22_2c_22_29_2b(::std::vec::Vec<(Box<Expr>, Box<Expr>)>),
        Nt_28_3cExpr_3e_20_22_2c_22_29(Box<Expr>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<Box<Expr>>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<Box<Expr>>),
        NtArgs(Vec<Box<Expr>>),
        NtAssignment((String, Box<Expr>)),
        NtAssignment_3f(::std::option::Option<(String, Box<Expr>)>),
        NtAssignments(Vec<(String, Box<Expr>)>),
        NtBool(bool),
        NtCase((Box<Expr>, Box<Expr>)),
        NtCase_3f(::std::option::Option<(Box<Expr>, Box<Expr>)>),
        NtCases(Vec<(Box<Expr>, Box<Expr>)>),
        NtCol(String),
        NtExpr(Box<Expr>),
        NtExpr_3f(::std::option::Option<Box<Expr>>),
        NtExprOp(Opcode),
        NtFactor(Box<Expr>),
        NtFactorOp(Opcode),
        NtID(String),
        NtList_3cAssignment_2c_20_22_2c_22_3e(Vec<(String, Box<Expr>)>),
        NtList_3cCase_2c_20_22_2c_22_3e(Vec<(Box<Expr>, Box<Expr>)>),
        NtList_3cExpr_2c_20_22_2c_22_3e(Vec<Box<Expr>>),
        NtNum(i32),
        NtTerm(Box<Expr>),
        NtTier_3cExprOp_2c_20Factor_3e(Box<Expr>),
        NtTier_3cFactorOp_2c_20Term_3e(Box<Expr>),
        NtTopExpr(Box<Expr>),
        Nt____TopExpr(Box<Expr>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 16, 0, 17, 0, 0, 18, 19, 20,
        // State 1
        -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50,
        // State 2
        -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53,
        // State 3
        -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60,
        // State 4
        -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57,
        // State 5
        21, -52, -52, -52, -52, -52, -52, 0, 0, -52, -52, -52, 0, 0, -52, 0, 0, -52, 0, 0, -52, 0, 0, 0,
        // State 6
        -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51,
        // State 7
        -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59,
        // State 8
        0, -28, 0, 23, -28, 24, 0, 0, 0, -28, -28, -28, 0, 0, -28, 0, 0, -28, 0, 0, -28, 0, 0, 0,
        // State 9
        0, -33, 26, -33, -33, -33, 27, 0, 0, -33, -33, -33, 0, 0, -33, 0, 0, -33, 0, 0, -33, 0, 0, 0,
        // State 10
        -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64,
        // State 11
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 16, 0, 17, 0, 0, 18, 19, 20,
        // State 12
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 13
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 19, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0,
        // State 16
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 17
        -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49, -49,
        // State 18
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 19
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 20
        12, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 21
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 22
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 23
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 24
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 25
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 26
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 27
        0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 19, 0,
        // State 30
        0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 34
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 35
        12, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 36
        0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -45, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 39
        -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56,
        // State 40
        -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58,
        // State 41
        -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55,
        // State 42
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 43
        0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 45
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 16, 0, 17, 0, 0, 18, 19, 20,
        // State 46
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 47
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 48
        0, 0, 0, 0, 61, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 52
        0, -47, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54,
        // State 54
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 57
        -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61,
        // State 58
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 59
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 63
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 64
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 65
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 66
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 17, 0, 0, 18, 19, 20,
        // State 67
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 68
        -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62, -62,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0,
        // State 70
        -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -50,
        -53,
        -60,
        -57,
        -52,
        -51,
        -59,
        -28,
        -33,
        -64,
        0,
        -22,
        0,
        0,
        0,
        -21,
        -49,
        -36,
        -27,
        0,
        0,
        -31,
        -32,
        0,
        -34,
        -35,
        0,
        0,
        0,
        0,
        0,
        0,
        -20,
        0,
        0,
        0,
        0,
        -16,
        -56,
        -58,
        -55,
        0,
        0,
        -4,
        0,
        0,
        0,
        0,
        0,
        0,
        -26,
        0,
        -54,
        -14,
        0,
        -5,
        -61,
        -17,
        0,
        -9,
        0,
        0,
        -15,
        0,
        -10,
        0,
        -23,
        -62,
        0,
        -63,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 4, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 11, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 4, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 28, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 29, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 14
        0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 31, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 0, 0, 0, 2, 0, 0, 0, 3, 38, 0, 0, 5, 0, 6, 0, 0, 39, 7, 8, 9, 10, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 40, 0, 6, 0, 0, 0, 7, 8, 0, 10, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 6, 0, 0, 0, 7, 41, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 2, 49, 0, 50, 3, 51, 0, 0, 5, 0, 6, 0, 52, 0, 7, 8, 9, 10, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 53, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 56, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 4, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 58, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 59, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 60, 0, 0, 3, 51, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 68, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 69, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 70, 0, 0, 5, 0, 6, 0, 0, 0, 7, 8, 9, 10, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###""=""###,
            r###""=>""###,
            r###""default""###,
            r###""else""###,
            r###""false""###,
            r###""if""###,
            r###""in""###,
            r###""let""###,
            r###""switch""###,
            r###""then""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z_]\\w*"#"###,
            r###"r#"\\$\\w+"#"###,
        ];
        __ACTION[(__state * 24)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_TopExpr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (6, _) if true => 3,
                (7, _) if true => 4,
                (8, _) if true => 5,
                (9, _) if true => 6,
                (10, _) if true => 7,
                (11, _) if true => 8,
                (12, _) if true => 9,
                (13, _) if true => 10,
                (14, _) if true => 11,
                (15, _) if true => 12,
                (16, _) if true => 13,
                (17, _) if true => 14,
                (18, _) if true => 15,
                (19, _) if true => 16,
                (20, _) if true => 17,
                (21, _) if true => 18,
                (22, _) if true => 19,
                (23, _) if true => 20,
                (0, _) if true => 21,
                (1, _) if true => 22,
                (2, _) if true => 23,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 24 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_3d_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22default_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22else_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22if_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22in_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22let_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22switch_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22then_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22_7b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22_7d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5c_5cw_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5c_24_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Assignment> ",") = Assignment, "," => ActionFn(38);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtAssignment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Assignment> ",")* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Assignment> ",")* = (<Assignment> ",")+ => ActionFn(37);
                let __sym0 = __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Assignment> ",")+ = Assignment, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtAssignment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Assignment> ",")+ = (<Assignment> ",")+, Assignment, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtAssignment(__symbols);
                let __sym0 = __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Case> ",") = Case, "," => ActionFn(48);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtCase(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Case> ",")* =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Case> ",")* = (<Case> ",")+ => ActionFn(47);
                let __sym0 = __pop_Nt_28_3cCase_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Case> ",")+ = Case, "," => ActionFn(59);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtCase(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action59::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Case> ",")+ = (<Case> ",")+, Case, "," => ActionFn(60);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtCase(__symbols);
                let __sym0 = __pop_Nt_28_3cCase_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action60::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                6
            }
            12 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            14 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(63);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(64);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action64::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            16 => {
                // Args = List<Expr, ","> => ActionFn(2);
                let __sym0 = __pop_NtList_3cExpr_2c_20_22_2c_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtArgs(__nt), __end));
                9
            }
            17 => {
                // Assignment = ID, "=", Expr => ActionFn(10);
                let __sym2 = __pop_NtExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAssignment(__nt), __end));
                10
            }
            18 => {
                // Assignment? = Assignment => ActionFn(34);
                let __sym0 = __pop_NtAssignment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssignment_3f(__nt), __end));
                11
            }
            19 => {
                // Assignment? =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssignment_3f(__nt), __end));
                11
            }
            20 => {
                // Assignments = List<Assignment, ","> => ActionFn(1);
                let __sym0 = __pop_NtList_3cAssignment_2c_20_22_2c_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssignments(__nt), __end));
                12
            }
            21 => {
                // Bool = "true" => ActionFn(22);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBool(__nt), __end));
                13
            }
            22 => {
                // Bool = "false" => ActionFn(23);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBool(__nt), __end));
                13
            }
            23 => {
                // Case = Expr, "=>", Expr => ActionFn(11);
                let __sym2 = __pop_NtExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_3e_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCase(__nt), __end));
                14
            }
            24 => {
                // Case? = Case => ActionFn(44);
                let __sym0 = __pop_NtCase(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCase_3f(__nt), __end));
                15
            }
            25 => {
                // Case? =  => ActionFn(45);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action45::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtCase_3f(__nt), __end));
                15
            }
            26 => {
                // Cases = List<Case, ","> => ActionFn(3);
                let __sym0 = __pop_NtList_3cCase_2c_20_22_2c_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCases(__nt), __end));
                16
            }
            27 => {
                // Col = r#"\\$\\w+"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCol(__nt), __end));
                17
            }
            28 => {
                // Expr = Tier<ExprOp, Factor> => ActionFn(8);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                18
            }
            29 => {
                // Expr? = Expr => ActionFn(39);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                19
            }
            30 => {
                // Expr? =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                19
            }
            31 => {
                // ExprOp = "+" => ActionFn(12);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                20
            }
            32 => {
                // ExprOp = "-" => ActionFn(13);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                20
            }
            33 => {
                // Factor = Tier<FactorOp, Term> => ActionFn(9);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                21
            }
            34 => {
                // FactorOp = "*" => ActionFn(14);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                22
            }
            35 => {
                // FactorOp = "/" => ActionFn(15);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                22
            }
            36 => {
                // ID = r#"[a-zA-Z_]\\w*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5c_5cw_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtID(__nt), __end));
                23
            }
            37 => {
                // List<Assignment, ","> = Assignment => ActionFn(67);
                let __sym0 = __pop_NtAssignment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action67::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList_3cAssignment_2c_20_22_2c_22_3e(__nt), __end));
                24
            }
            38 => {
                // List<Assignment, ","> =  => ActionFn(68);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action68::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtList_3cAssignment_2c_20_22_2c_22_3e(__nt), __end));
                24
            }
            39 => {
                // List<Assignment, ","> = (<Assignment> ",")+, Assignment => ActionFn(69);
                let __sym1 = __pop_NtAssignment(__symbols);
                let __sym0 = __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList_3cAssignment_2c_20_22_2c_22_3e(__nt), __end));
                24
            }
            40 => {
                // List<Assignment, ","> = (<Assignment> ",")+ => ActionFn(70);
                let __sym0 = __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList_3cAssignment_2c_20_22_2c_22_3e(__nt), __end));
                24
            }
            41 => {
                // List<Case, ","> = Case => ActionFn(71);
                let __sym0 = __pop_NtCase(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action71::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList_3cCase_2c_20_22_2c_22_3e(__nt), __end));
                25
            }
            42 => {
                // List<Case, ","> =  => ActionFn(72);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action72::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtList_3cCase_2c_20_22_2c_22_3e(__nt), __end));
                25
            }
            43 => {
                // List<Case, ","> = (<Case> ",")+, Case => ActionFn(73);
                let __sym1 = __pop_NtCase(__symbols);
                let __sym0 = __pop_Nt_28_3cCase_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action73::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList_3cCase_2c_20_22_2c_22_3e(__nt), __end));
                25
            }
            44 => {
                // List<Case, ","> = (<Case> ",")+ => ActionFn(74);
                let __sym0 = __pop_Nt_28_3cCase_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action74::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList_3cCase_2c_20_22_2c_22_3e(__nt), __end));
                25
            }
            45 => {
                // List<Expr, ","> = Expr => ActionFn(75);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList_3cExpr_2c_20_22_2c_22_3e(__nt), __end));
                26
            }
            46 => {
                // List<Expr, ","> =  => ActionFn(76);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action76::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtList_3cExpr_2c_20_22_2c_22_3e(__nt), __end));
                26
            }
            47 => {
                // List<Expr, ","> = (<Expr> ",")+, Expr => ActionFn(77);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action77::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList_3cExpr_2c_20_22_2c_22_3e(__nt), __end));
                26
            }
            48 => {
                // List<Expr, ","> = (<Expr> ",")+ => ActionFn(78);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action78::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList_3cExpr_2c_20_22_2c_22_3e(__nt), __end));
                26
            }
            49 => {
                // Num = r#"[0-9]+"# => ActionFn(24);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                27
            }
            50 => {
                // Term = Bool => ActionFn(16);
                let __sym0 = __pop_NtBool(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                28
            }
            51 => {
                // Term = Num => ActionFn(17);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                28
            }
            52 => {
                // Term = ID => ActionFn(18);
                let __sym0 = __pop_NtID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                28
            }
            53 => {
                // Term = Col => ActionFn(19);
                let __sym0 = __pop_NtCol(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                28
            }
            54 => {
                // Term = ID, "(", Args, ")" => ActionFn(20);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtArgs(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_NtID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                28
            }
            55 => {
                // Term = "(", TopExpr, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTopExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                28
            }
            56 => {
                // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(29);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_NtExprOp(__symbols);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                29
            }
            57 => {
                // Tier<ExprOp, Factor> = Factor => ActionFn(30);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                29
            }
            58 => {
                // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(27);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtFactorOp(__symbols);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                30
            }
            59 => {
                // Tier<FactorOp, Term> = Term => ActionFn(28);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                30
            }
            60 => {
                // TopExpr = Expr => ActionFn(4);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTopExpr(__nt), __end));
                31
            }
            61 => {
                // TopExpr = "let", Assignments, "in", TopExpr => ActionFn(5);
                let __sym3 = __pop_NtTopExpr(__symbols);
                let __sym2 = __pop_Term_22in_22(__symbols);
                let __sym1 = __pop_NtAssignments(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTopExpr(__nt), __end));
                31
            }
            62 => {
                // TopExpr = "if", Expr, "then", Expr, "else", Expr => ActionFn(6);
                let __sym5 = __pop_NtExpr(__symbols);
                let __sym4 = __pop_Term_22else_22(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22then_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTopExpr(__nt), __end));
                31
            }
            63 => {
                // TopExpr = "switch", "{", Cases, "default", ":", Expr, "}" => ActionFn(7);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtExpr(__symbols);
                let __sym4 = __pop_Term_22_3a_22(__symbols);
                let __sym3 = __pop_Term_22default_22(__symbols);
                let __sym2 = __pop_NtCases(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_Term_22switch_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtTopExpr(__nt), __end));
                31
            }
            64 => {
                // __TopExpr = TopExpr => ActionFn(0);
                let __sym0 = __pop_NtTopExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22default_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22default_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22else_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22else_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22false_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22false_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22if_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22if_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22in_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22in_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22let_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22let_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22switch_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22switch_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22then_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22then_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22true_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22true_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5c_5cw_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5c_5cw_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5c_24_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (String, Box<Expr>), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cAssignment_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cAssignment_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cCase_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Box<Expr>, Box<Expr>), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cCase_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cCase_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cCase_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArgs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArgs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssignment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (String, Box<Expr>), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssignment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssignment_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(String, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssignment_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssignments<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(String, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssignments(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBool<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBool(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCase<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Box<Expr>, Box<Expr>), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCase(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCase_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(Box<Expr>, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCase_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCases<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(Box<Expr>, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCases(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCol<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCol(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactorOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactorOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtList_3cAssignment_2c_20_22_2c_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(String, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList_3cAssignment_2c_20_22_2c_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtList_3cCase_2c_20_22_2c_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(Box<Expr>, Box<Expr>)>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList_3cCase_2c_20_22_2c_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtList_3cExpr_2c_20_22_2c_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList_3cExpr_2c_20_22_2c_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cExprOp_2c_20Factor_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cFactorOp_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTopExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTopExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____TopExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____TopExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__TopExpr::parse_TopExpr;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast::{Expr, Opcode, Let, Cond, Switch};
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[0-9])+",
                "^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])*",
                "^(?u:\\$)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:,)",
                "^(?u:\\-)",
                "^(?u:/)",
                "^(?u::)",
                "^(?u:=)",
                "^(?u:=>)",
                "^(?u:default)",
                "^(?u:else)",
                "^(?u:false)",
                "^(?u:if)",
                "^(?u:in)",
                "^(?u:let)",
                "^(?u:switch)",
                "^(?u:then)",
                "^(?u:true)",
                "^(?u:\\{)",
                "^(?u:\\})",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])*").unwrap(),
                __regex::Regex::new("^(?u:\\$)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:/)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:=>)").unwrap(),
                __regex::Regex::new("^(?u:default)").unwrap(),
                __regex::Regex::new("^(?u:else)").unwrap(),
                __regex::Regex::new("^(?u:false)").unwrap(),
                __regex::Regex::new("^(?u:if)").unwrap(),
                __regex::Regex::new("^(?u:in)").unwrap(),
                __regex::Regex::new("^(?u:let)").unwrap(),
                __regex::Regex::new("^(?u:switch)").unwrap(),
                __regex::Regex::new("^(?u:then)").unwrap(),
                __regex::Regex::new("^(?u:true)").unwrap(),
                __regex::Regex::new("^(?u:\\{)").unwrap(),
                __regex::Regex::new("^(?u:\\})").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 24 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<(String, Box<Expr>)>, usize),
) -> Vec<(String, Box<Expr>)>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<(Box<Expr>, Box<Expr>)>, usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<(String, Box<Expr>)>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Let(
	    Box::new(Let{assignments: a, expr: e})))
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Cond(
	    Box::new(Cond{cond:i, then:t, otherwise:e})))
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, Vec<(Box<Expr>, Box<Expr>)>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, d, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Switch(
	    Box::new(Switch{cases:c, default:d})))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, val, _): (usize, Box<Expr>, usize),
) -> (String, Box<Expr>)
{
    (id, val)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, c, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> (Box<Expr>, Box<Expr>)
{
    (c,e)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Add
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Sub
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Mul
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Div
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, bool, usize),
) -> Box<Expr>
{
    Box::new(Expr::Bool(__0))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Number(__0))
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Box<Expr>
{
    Box::new(Expr::ID(__0))
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Box<Expr>
{
    Box::new(Expr::Col(__0))
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<Box<Expr>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::App(id, args))
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    false
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
    (_, e, _): (usize, ::std::option::Option<(Box<Expr>, Box<Expr>)>, usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
    (_, e, _): (usize, ::std::option::Option<(String, Box<Expr>)>, usize),
) -> Vec<(String, Box<Expr>)>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (String, Box<Expr>), usize),
) -> ::std::option::Option<(String, Box<Expr>)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(String, Box<Expr>)>
{
    None
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(String, Box<Expr>)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
) -> ::std::vec::Vec<(String, Box<Expr>)>
{
    v
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (String, Box<Expr>), usize),
    (_, _, _): (usize, &'input str, usize),
) -> (String, Box<Expr>)
{
    (__0)
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::option::Option<Box<Expr>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Box<Expr>>
{
    None
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    v
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (Box<Expr>, Box<Expr>), usize),
) -> ::std::option::Option<(Box<Expr>, Box<Expr>)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(Box<Expr>, Box<Expr>)>
{
    None
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(Box<Expr>, Box<Expr>)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
) -> ::std::vec::Vec<(Box<Expr>, Box<Expr>)>
{
    v
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (Box<Expr>, Box<Expr>), usize),
    (_, _, _): (usize, &'input str, usize),
) -> (Box<Expr>, Box<Expr>)
{
    (__0)
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (Box<Expr>, Box<Expr>), usize),
) -> ::std::vec::Vec<(Box<Expr>, Box<Expr>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
    (_, e, _): (usize, (Box<Expr>, Box<Expr>), usize),
) -> ::std::vec::Vec<(Box<Expr>, Box<Expr>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (String, Box<Expr>), usize),
) -> ::std::vec::Vec<(String, Box<Expr>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
    (_, e, _): (usize, (String, Box<Expr>), usize),
) -> ::std::vec::Vec<(String, Box<Expr>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, (String, Box<Expr>), usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<(String, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
    __1: (usize, (String, Box<Expr>), usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<(String, Box<Expr>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action38(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<(String, Box<Expr>)>, usize),
) -> Vec<(String, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
    __1: (usize, ::std::option::Option<(String, Box<Expr>)>, usize),
) -> Vec<(String, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    input: &'input str,
    __0: (usize, (Box<Expr>, Box<Expr>), usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
    __1: (usize, (Box<Expr>, Box<Expr>), usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<(Box<Expr>, Box<Expr>)>, usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action46(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
    __1: (usize, ::std::option::Option<(Box<Expr>, Box<Expr>)>, usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action43(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action41(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, (String, Box<Expr>), usize),
) -> Vec<(String, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(String, Box<Expr>)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action35(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
    __1: (usize, (String, Box<Expr>), usize),
) -> Vec<(String, Box<Expr>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(String, Box<Expr>)>, usize),
) -> Vec<(String, Box<Expr>)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action71<
    'input,
>(
    input: &'input str,
    __0: (usize, (Box<Expr>, Box<Expr>), usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action72<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action45(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action73<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
    __1: (usize, (Box<Expr>, Box<Expr>), usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action44(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action74<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Box<Expr>, Box<Expr>)>, usize),
) -> Vec<(Box<Expr>, Box<Expr>)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action45(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action75<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expr>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action40(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
