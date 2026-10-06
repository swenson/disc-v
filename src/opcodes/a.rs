//! "A" extension: atomic instructions.

use super::{fmt, Codec, Opcode};

pub(crate) static LR_W: Opcode = Opcode::new("lr.w", Codec::RL, fmt::RD_ADDR_RS1);
pub(crate) static SC_W: Opcode = Opcode::new("sc.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOSWAP_W: Opcode = Opcode::new("amoswap.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOADD_W: Opcode = Opcode::new("amoadd.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOXOR_W: Opcode = Opcode::new("amoxor.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOOR_W: Opcode = Opcode::new("amoor.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOAND_W: Opcode = Opcode::new("amoand.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOMIN_W: Opcode = Opcode::new("amomin.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOMAX_W: Opcode = Opcode::new("amomax.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOMINU_W: Opcode = Opcode::new("amominu.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static AMOMAXU_W: Opcode = Opcode::new("amomaxu.w", Codec::RA, fmt::RD_RS2_ADDR_RS1);
pub(crate) static LR_D: Opcode = Opcode::new("lr.d", Codec::RL, fmt::RD_ADDR_RS1).rv64();
pub(crate) static SC_D: Opcode = Opcode::new("sc.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOSWAP_D: Opcode =
    Opcode::new("amoswap.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOADD_D: Opcode =
    Opcode::new("amoadd.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOXOR_D: Opcode =
    Opcode::new("amoxor.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOOR_D: Opcode = Opcode::new("amoor.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOAND_D: Opcode =
    Opcode::new("amoand.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOMIN_D: Opcode =
    Opcode::new("amomin.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOMAX_D: Opcode =
    Opcode::new("amomax.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOMINU_D: Opcode =
    Opcode::new("amominu.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static AMOMAXU_D: Opcode =
    Opcode::new("amomaxu.d", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv64();
pub(crate) static LR_Q: Opcode = Opcode::new("lr.q", Codec::RL, fmt::RD_ADDR_RS1).rv128();
pub(crate) static SC_Q: Opcode = Opcode::new("sc.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOSWAP_Q: Opcode =
    Opcode::new("amoswap.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOADD_Q: Opcode =
    Opcode::new("amoadd.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOXOR_Q: Opcode =
    Opcode::new("amoxor.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOOR_Q: Opcode = Opcode::new("amoor.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOAND_Q: Opcode =
    Opcode::new("amoand.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOMIN_Q: Opcode =
    Opcode::new("amomin.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOMAX_Q: Opcode =
    Opcode::new("amomax.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOMINU_Q: Opcode =
    Opcode::new("amominu.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
pub(crate) static AMOMAXU_Q: Opcode =
    Opcode::new("amomaxu.q", Codec::RA, fmt::RD_RS2_ADDR_RS1).rv128();
