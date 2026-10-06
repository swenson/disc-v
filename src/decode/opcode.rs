// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Maps instruction encodings to opcode table entries.

use crate::Isa;
use crate::opcodes::{Opcode, a, b, c, d, f, i, m, q, system, zawrs, zicond};

fn compressed_0(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    match (inst >> 13) & 7 {
        0 if inst == 0 => Some(&c::C_UNIMP),
        0 => Some(&c::C_ADDI4SPN),
        1 => Some(if isa == Isa::Rv128 {
            &c::C_LQ
        } else {
            &c::C_FLD
        }),
        2 => Some(&c::C_LW),
        3 => Some(if isa == Isa::Rv32 {
            &c::C_FLW
        } else {
            &c::C_LD
        }),
        4 => None,
        5 => Some(if isa == Isa::Rv128 {
            &c::C_SQ
        } else {
            &c::C_FSD
        }),
        6 => Some(&c::C_SW),
        7 => Some(if isa == Isa::Rv32 {
            &c::C_FSW
        } else {
            &c::C_SD
        }),
        _ => unreachable!(),
    }
}

fn compressed_1(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    match (inst >> 13) & 7 {
        0 => Some(match ((inst >> 7) & 0x1f, inst & 0x107c) {
            (0, 0) => &c::C_NOP,
            (0, _) => &c::C_NOP_HINT,
            _ => &c::C_ADDI,
        }),
        1 => Some(if isa == Isa::Rv32 {
            &c::C_JAL
        } else {
            &c::C_ADDIW
        }),
        2 => Some(&c::C_LI),
        3 => Some(match (inst >> 7) & 0x1f {
            2 => &c::C_ADDI16SP,
            _ => &c::C_LUI,
        }),
        4 => match (inst >> 10) & 3 {
            0 => Some(&c::C_SRLI),
            1 => Some(&c::C_SRAI),
            2 => Some(&c::C_ANDI),
            3 => match (inst >> 10) & 4 | (inst >> 5) & 3 {
                0 => Some(&c::C_SUB),
                1 => Some(&c::C_XOR),
                2 => Some(&c::C_OR),
                3 => Some(&c::C_AND),
                4 => Some(&c::C_SUBW),
                5 => Some(&c::C_ADDW),
                _ => None,
            },
            _ => unreachable!(),
        },
        5 => Some(&c::C_J),
        6 => Some(&c::C_BEQZ),
        7 => Some(&c::C_BNEZ),
        _ => unreachable!(),
    }
}

fn compressed_2(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    Some(match (inst >> 13) & 7 {
        0 => &c::C_SLLI,
        1 => {
            if isa == Isa::Rv128 {
                &c::C_LQSP
            } else {
                &c::C_FLDSP
            }
        }
        2 => &c::C_LWSP,
        3 => {
            if isa == Isa::Rv32 {
                &c::C_FLWSP
            } else {
                &c::C_LDSP
            }
        }
        4 => match (inst >> 12) & 1 {
            0 => match (inst >> 2) & 0x1f {
                0 => &c::C_JR,
                _ => &c::C_MV,
            },
            1 => match (inst >> 2) & 0x1f {
                0 => match (inst >> 7) & 0x1f {
                    0 => &c::C_EBREAK,
                    _ => &c::C_JALR,
                },
                _ => &c::C_ADD,
            },
            _ => unreachable!(),
        },
        5 => {
            if isa == Isa::Rv128 {
                &c::C_SQSP
            } else {
                &c::C_FSDSP
            }
        }
        6 => &c::C_SWSP,
        7 => {
            if isa == Isa::Rv32 {
                &c::C_FSWSP
            } else {
                &c::C_SDSP
            }
        }
        _ => unreachable!(),
    })
}

fn load(inst: u64) -> Option<&'static Opcode> {
    Some(match (inst >> 12) & 7 {
        0 => &i::LB,
        1 => &i::LH,
        2 => &i::LW,
        3 => &i::LD,
        4 => &i::LBU,
        5 => &i::LHU,
        6 => &i::LWU,
        7 => &i::LDU,
        _ => unreachable!(),
    })
}

fn load_fp(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        2 => Some(&f::FLW),
        3 => Some(&d::FLD),
        4 => Some(&q::FLQ),
        _ => None,
    }
}

fn misc_mem(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        // fence.tso is fm=1000 with pred=succ=rw.
        0 if inst >> 20 == 0x833 => Some(&i::FENCE_TSO),
        0 => Some(&i::FENCE),
        1 => Some(&i::FENCE_I),
        2 => Some(&i::LQ),
        _ => None,
    }
}

fn op_imm(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        0 => Some(&i::ADDI),
        1 => {
            if (inst >> 27) & 0x1f == 0 {
                Some(&i::SLLI)
            } else if inst >> 20 == 0x600 {
                Some(&b::CLZ)
            } else if inst >> 20 == 0x601 {
                Some(&b::CTZ)
            } else if inst >> 20 == 0x602 {
                Some(&b::CPOP)
            } else if inst >> 20 == 0x604 {
                Some(&b::SEXT_B)
            } else if inst >> 20 == 0x605 {
                Some(&b::SEXT_H)
            } else {
                match isa {
                    Isa::Rv32 => match (inst >> 25) & 0x7f {
                        0x14 => Some(&b::BSETI),
                        0x24 => Some(&b::BCLRI),
                        0x34 => Some(&b::BINVI),
                        _ => None,
                    },
                    Isa::Rv64 => match (inst >> 26) & 0x3f {
                        0xa => Some(&b::BSET_64),
                        0x12 => Some(&b::BCLRI_64),
                        0x1a => Some(&b::BINVI_64),
                        _ => None,
                    },
                    _ => None,
                }
            }
        }
        2 => Some(&i::SLTI),
        3 => Some(&i::SLTIU),
        4 => Some(&i::XORI),
        5 => {
            if inst >> 20 == 0x287 {
                Some(&b::ORC_B)
            } else if (isa == Isa::Rv32 && inst >> 20 == 0x698)
                || (isa == Isa::Rv64 && inst >> 20 == 0x6b8)
            {
                Some(&b::REV8)
            } else if isa == Isa::Rv32 && inst >> 25 == 0x30 {
                Some(&b::RORI)
            } else if isa == Isa::Rv64 && inst >> 26 == 0x18 {
                Some(&b::RORI_64)
            } else {
                match (inst >> 27) & 0x1f {
                    0 => Some(&i::SRLI),
                    8 => Some(&i::SRAI),
                    9 => match isa {
                        Isa::Rv32 if (inst >> 25) & 0x7f == 0x24 => Some(&b::BEXTI),
                        Isa::Rv64 if (inst >> 26) & 0x3f == 0x12 => Some(&b::BEXTI_64),
                        _ => None,
                    },
                    _ => None,
                }
            }
        }
        6 => Some(&i::ORI),
        7 => Some(&i::ANDI),
        _ => None,
    }
}

fn op_imm_32(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        0 => Some(&i::ADDIW),
        1 => {
            if (inst >> 26) & 0x3f == 2 {
                Some(&b::SLLI_UW)
            } else if (inst >> 25) & 0x7f == 0 {
                Some(&i::SLLIW)
            } else if inst >> 20 == 0x600 {
                Some(&b::CLZW)
            } else if inst >> 20 == 0x601 {
                Some(&b::CTZW)
            } else if inst >> 20 == 0x602 {
                Some(&b::CPOPW)
            } else {
                None
            }
        }
        5 => match (inst >> 25) & 0x7f {
            0 => Some(&i::SRLIW),
            32 => Some(&i::SRAIW),
            48 => Some(&b::RORIW),
            _ => None,
        },
        _ => None,
    }
}

fn store(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        0 => Some(&i::SB),
        1 => Some(&i::SH),
        2 => Some(&i::SW),
        3 => Some(&i::SD),
        4 => Some(&i::SQ),
        _ => None,
    }
}

fn store_fp(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        2 => Some(&f::FSW),
        3 => Some(&d::FSD),
        4 => Some(&q::FSQ),
        _ => None,
    }
}

fn amo(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 24) & 0xf8 | (inst >> 12) & 7 {
        2 => Some(&a::AMOADD_W),
        3 => Some(&a::AMOADD_D),
        4 => Some(&a::AMOADD_Q),
        10 => Some(&a::AMOSWAP_W),
        11 => Some(&a::AMOSWAP_D),
        12 => Some(&a::AMOSWAP_Q),
        18 => {
            if (inst >> 20) & 0x1f == 0 {
                Some(&a::LR_W)
            } else {
                None
            }
        }
        19 => {
            if (inst >> 20) & 0x1f == 0 {
                Some(&a::LR_D)
            } else {
                None
            }
        }
        20 => {
            if (inst >> 20) & 0x1f == 0 {
                Some(&a::LR_Q)
            } else {
                None
            }
        }
        26 => Some(&a::SC_W),
        27 => Some(&a::SC_D),
        28 => Some(&a::SC_Q),
        34 => Some(&a::AMOXOR_W),
        35 => Some(&a::AMOXOR_D),
        36 => Some(&a::AMOXOR_Q),
        66 => Some(&a::AMOOR_W),
        67 => Some(&a::AMOOR_D),
        68 => Some(&a::AMOOR_Q),
        98 => Some(&a::AMOAND_W),
        99 => Some(&a::AMOAND_D),
        100 => Some(&a::AMOAND_Q),
        130 => Some(&a::AMOMIN_W),
        131 => Some(&a::AMOMIN_D),
        132 => Some(&a::AMOMIN_Q),
        162 => Some(&a::AMOMAX_W),
        163 => Some(&a::AMOMAX_D),
        164 => Some(&a::AMOMAX_Q),
        194 => Some(&a::AMOMINU_W),
        195 => Some(&a::AMOMINU_D),
        196 => Some(&a::AMOMINU_Q),
        226 => Some(&a::AMOMAXU_W),
        227 => Some(&a::AMOMAXU_D),
        228 => Some(&a::AMOMAXU_Q),
        _ => None,
    }
}

fn op(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    match (inst >> 22) & 0x3f8 | (inst >> 12) & 7 {
        0 => Some(&i::ADD),
        1 => Some(&i::SLL),
        2 => Some(&i::SLT),
        3 => Some(&i::SLTU),
        4 => Some(&i::XOR),
        5 => Some(&i::SRL),
        6 => Some(&i::OR),
        7 => Some(&i::AND),
        8 => Some(&m::MUL),
        9 => Some(&m::MULH),
        10 => Some(&m::MULHSU),
        11 => Some(&m::MULHU),
        12 => Some(&m::DIV),
        13 => Some(&m::DIVU),
        14 => Some(&m::REM),
        15 => Some(&m::REMU),
        36 if isa == Isa::Rv32 && inst >> 20 == 0x080 => Some(&b::ZEXT_H),
        41 => Some(&b::CLMUL),
        42 => Some(&b::CLMULR),
        43 => Some(&b::CLMULH),
        44 => Some(&b::MIN),
        45 => Some(&b::MINU),
        46 => Some(&b::MAX),
        47 => Some(&b::MAXU),
        61 => Some(&zicond::CZERO_EQZ),
        63 => Some(&zicond::CZERO_NEZ),
        130 => Some(&b::SH1ADD),
        132 => Some(&b::SH2ADD),
        134 => Some(&b::SH3ADD),
        161 => Some(&b::BSET),
        256 => Some(&i::SUB),
        260 => Some(&b::XNOR),
        261 => Some(&i::SRA),
        262 => Some(&b::ORN),
        263 => Some(&b::ANDN),
        289 => Some(&b::BCLR),
        293 => Some(&b::BEXT),
        385 => Some(&b::ROL),
        389 => Some(&b::ROR),
        417 => Some(&b::BINV),
        _ => None,
    }
}

fn op_32(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 22) & 0x3f8 | (inst >> 12) & 7 {
        0 => Some(&i::ADDW),
        1 => Some(&i::SLLW),
        5 => Some(&i::SRLW),
        8 => Some(&m::MULW),
        12 => Some(&m::DIVW),
        13 => Some(&m::DIVUW),
        14 => Some(&m::REMW),
        15 => Some(&m::REMUW),
        32 => Some(&b::ADD_UW),
        36 if inst >> 20 == 0x080 => Some(&b::ZEXT_H),
        130 => Some(&b::SH1ADD_UW),
        132 => Some(&b::SH2ADD_UW),
        134 => Some(&b::SH3ADD_UW),
        256 => Some(&i::SUBW),
        261 => Some(&i::SRAW),
        385 => Some(&b::ROLW),
        389 => Some(&b::RORW),
        _ => None,
    }
}

fn madd(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 25) & 3 {
        0 => Some(&f::FMADD_S),
        1 => Some(&d::FMADD_D),
        3 => Some(&q::FMADD_Q),
        _ => None,
    }
}

fn msub(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 25) & 3 {
        0 => Some(&f::FMSUB_S),
        1 => Some(&d::FMSUB_D),
        3 => Some(&q::FMSUB_Q),
        _ => None,
    }
}

fn nmsub(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 25) & 3 {
        0 => Some(&f::FNMSUB_S),
        1 => Some(&d::FNMSUB_D),
        3 => Some(&q::FNMSUB_Q),
        _ => None,
    }
}

fn nmadd(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 25) & 3 {
        0 => Some(&f::FNMADD_S),
        1 => Some(&d::FNMADD_D),
        3 => Some(&q::FNMADD_Q),
        _ => None,
    }
}

fn op_fp(inst: u64) -> Option<&'static Opcode> {
    match ((inst >> 25) & 0x7f, (inst >> 20) & 0x1f, (inst >> 12) & 7) {
        (0, _, _) => Some(&f::FADD_S),
        (1, _, _) => Some(&d::FADD_D),
        (3, _, _) => Some(&q::FADD_Q),
        (4, _, _) => Some(&f::FSUB_S),
        (5, _, _) => Some(&d::FSUB_D),
        (7, _, _) => Some(&q::FSUB_Q),
        (8, _, _) => Some(&f::FMUL_S),
        (9, _, _) => Some(&d::FMUL_D),
        (11, _, _) => Some(&q::FMUL_Q),
        (12, _, _) => Some(&f::FDIV_S),
        (13, _, _) => Some(&d::FDIV_D),
        (15, _, _) => Some(&q::FDIV_Q),
        (16, _, 0) => Some(&f::FSGNJ_S),
        (16, _, 1) => Some(&f::FSGNJN_S),
        (16, _, 2) => Some(&f::FSGNJX_S),
        (17, _, 0) => Some(&d::FSGNJ_D),
        (17, _, 1) => Some(&d::FSGNJN_D),
        (17, _, 2) => Some(&d::FSGNJX_D),
        (19, _, 0) => Some(&q::FSGNJ_Q),
        (19, _, 1) => Some(&q::FSGNJN_Q),
        (19, _, 2) => Some(&q::FSGNJX_Q),
        (20, _, 0) => Some(&f::FMIN_S),
        (20, _, 1) => Some(&f::FMAX_S),
        (21, _, 0) => Some(&d::FMIN_D),
        (21, _, 1) => Some(&d::FMAX_D),
        (23, _, 0) => Some(&q::FMIN_Q),
        (23, _, 1) => Some(&q::FMAX_Q),
        (32, 1, _) => Some(&d::FCVT_S_D),
        (32, 3, _) => Some(&q::FCVT_S_Q),
        (33, 0, _) => Some(&d::FCVT_D_S),
        (33, 3, _) => Some(&q::FCVT_D_Q),
        (35, 0, _) => Some(&q::FCVT_Q_S),
        (35, 1, _) => Some(&q::FCVT_Q_D),
        (44, 0, _) => Some(&f::FSQRT_S),
        (45, 0, _) => Some(&d::FSQRT_D),
        (47, 0, _) => Some(&q::FSQRT_Q),
        (80, _, 0) => Some(&f::FLE_S),
        (80, _, 1) => Some(&f::FLT_S),
        (80, _, 2) => Some(&f::FEQ_S),
        (81, _, 0) => Some(&d::FLE_D),
        (81, _, 1) => Some(&d::FLT_D),
        (81, _, 2) => Some(&d::FEQ_D),
        (83, _, 0) => Some(&q::FLE_Q),
        (83, _, 1) => Some(&q::FLT_Q),
        (83, _, 2) => Some(&q::FEQ_Q),
        (96, 0, _) => Some(&f::FCVT_W_S),
        (96, 1, _) => Some(&f::FCVT_WU_S),
        (96, 2, _) => Some(&f::FCVT_L_S),
        (96, 3, _) => Some(&f::FCVT_LU_S),
        (97, 0, _) => Some(&d::FCVT_W_D),
        (97, 1, _) => Some(&d::FCVT_WU_D),
        (97, 2, _) => Some(&d::FCVT_L_D),
        (97, 3, _) => Some(&d::FCVT_LU_D),
        (99, 0, _) => Some(&q::FCVT_W_Q),
        (99, 1, _) => Some(&q::FCVT_WU_Q),
        (99, 2, _) => Some(&q::FCVT_L_Q),
        (99, 3, _) => Some(&q::FCVT_LU_Q),
        (104, 0, _) => Some(&f::FCVT_S_W),
        (104, 1, _) => Some(&f::FCVT_S_WU),
        (104, 2, _) => Some(&f::FCVT_S_L),
        (104, 3, _) => Some(&f::FCVT_S_LU),
        (105, 0, _) => Some(&d::FCVT_D_W),
        (105, 1, _) => Some(&d::FCVT_D_WU),
        (105, 2, _) => Some(&d::FCVT_D_L),
        (105, 3, _) => Some(&d::FCVT_D_LU),
        (107, 0, _) => Some(&q::FCVT_Q_W),
        (107, 1, _) => Some(&q::FCVT_Q_WU),
        (107, 2, _) => Some(&q::FCVT_Q_L),
        (107, 3, _) => Some(&q::FCVT_Q_LU),
        (112, _, _) => match (inst >> 17) & 0xf8 | (inst >> 12) & 7 {
            0 => Some(&f::FMV_X_W),
            1 => Some(&f::FCLASS_S),
            _ => None,
        },
        (113, _, _) => match (inst >> 17) & 0xf8 | (inst >> 12) & 7 {
            0 => Some(&d::FMV_X_D),
            1 => Some(&d::FCLASS_D),
            _ => None,
        },
        (115, _, _) => match (inst >> 17) & 0xf8 | (inst >> 12) & 7 {
            0 => Some(&q::FMV_X_Q),
            1 => Some(&q::FCLASS_Q),
            _ => None,
        },
        (120, _, _) => {
            if (inst >> 17) & 0xf8 | (inst >> 12) & 7 == 0 {
                Some(&f::FMV_W_X)
            } else {
                None
            }
        }
        (121, _, _) => {
            if (inst >> 17) & 0xf8 | (inst >> 12) & 7 == 0 {
                Some(&d::FMV_D_X)
            } else {
                None
            }
        }
        (123, _, _) => {
            if (inst >> 17) & 0xf8 | (inst >> 12) & 7 == 0 {
                Some(&q::FMV_Q_X)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn custom2_rv128(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        0 => Some(&i::ADDID),
        1 => {
            if (inst >> 26) & 0x3f == 0 {
                Some(&i::SLLID)
            } else {
                None
            }
        }
        5 => match (inst >> 26) & 0x3f {
            0 => Some(&i::SRLID),
            16 => Some(&i::SRAID),
            _ => None,
        },
        _ => None,
    }
}

fn branch(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        0 => Some(&i::BEQ),
        1 => Some(&i::BNE),
        4 => Some(&i::BLT),
        5 => Some(&i::BGE),
        6 => Some(&i::BLTU),
        7 => Some(&i::BGEU),
        _ => None,
    }
}

fn jalr(inst: u64) -> Option<&'static Opcode> {
    if (inst >> 12) & 7 == 0 {
        Some(&i::JALR)
    } else {
        None
    }
}

fn system_inst(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 12) & 7 {
        0 => match inst {
            0x0000_0073 => Some(&system::ECALL),
            0x0010_0073 => Some(&system::EBREAK),
            0x00d0_0073 => Some(&zawrs::WRS_NTO),
            0x01d0_0073 => Some(&zawrs::WRS_STO),
            0x1020_0073 => Some(&system::SRET),
            0x1050_0073 => Some(&system::WFI),
            0x3020_0073 => Some(&system::MRET),
            0x7b20_0073 => Some(&system::DRET),
            _ if inst & 0xfe00_7fff == 0x1200_0073 => Some(&system::SFENCE_VMA),
            _ => None,
        },
        1 => Some(&system::CSRRW),
        2 => Some(&system::CSRRS),
        3 => Some(&system::CSRRC),
        5 => Some(&system::CSRRWI),
        6 => Some(&system::CSRRSI),
        7 => Some(&system::CSRRCI),
        _ => None,
    }
}

fn custom3_rv128(inst: u64) -> Option<&'static Opcode> {
    match (inst >> 22) & 0x3f8 | (inst >> 12) & 7 {
        0 => Some(&i::ADDD),
        1 => Some(&i::SLLD),
        5 => Some(&i::SRLD),
        8 => Some(&m::MULD),
        12 => Some(&m::DIVD),
        13 => Some(&m::DIVUD),
        14 => Some(&m::REMD),
        15 => Some(&m::REMUD),
        256 => Some(&i::SUBD),
        261 => Some(&i::SRAD),
        _ => None,
    }
}

fn uncompressed(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    match (inst >> 2) & 0x1f {
        0 => load(inst),
        1 => load_fp(inst),
        3 => misc_mem(inst),
        4 => op_imm(isa, inst),
        5 => Some(&i::AUIPC),
        6 => {
            if isa == Isa::Rv64 {
                op_imm_32(inst)
            } else {
                None
            }
        }
        8 => store(inst),
        9 => store_fp(inst),
        11 => amo(inst),
        12 => op(isa, inst),
        13 => Some(&i::LUI),
        14 => {
            if isa == Isa::Rv64 {
                op_32(inst)
            } else {
                None
            }
        }
        16 => madd(inst),
        17 => msub(inst),
        18 => nmsub(inst),
        19 => nmadd(inst),
        20 => op_fp(inst),
        22 => custom2_rv128(inst),
        24 => branch(inst),
        25 => jalr(inst),
        27 => Some(&i::JAL),
        28 => system_inst(inst),
        30 => custom3_rv128(inst),
        _ => None,
    }
}

/// Looks up the opcode for `inst`, or `None` if the encoding is not recognized.
pub(super) fn lookup(isa: Isa, inst: u64) -> Option<&'static Opcode> {
    match inst & 3 {
        0 => compressed_0(isa, inst),
        1 => compressed_1(isa, inst),
        2 => compressed_2(isa, inst),
        _ => uncompressed(isa, inst),
    }
}
