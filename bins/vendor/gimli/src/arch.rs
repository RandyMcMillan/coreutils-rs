use crate::common::Register;

macro_rules! registers {
    ($struct_name:ident, { $($name:ident = ($val:expr, $disp:expr)),+ $(,)? }
        $(, aliases { $($alias_name:ident = ($alias_val:expr, $alias_disp:expr)),+ $(,)? })?) => {
        #[allow(missing_docs)]
        impl $struct_name {
            $(
                pub const $name: Register = Register($val);
            )+
            $(
                $(pub const $alias_name: Register = Register($alias_val);)+
            )*
        }

        impl $struct_name {
            /// The name of a register, or `None` if the register number is unknown.
            ///
            /// Only returns the primary name for registers that alias with others.
            pub fn register_name(register: Register) -> Option<&'static str> {
                match register {
                    $(
                        Self::$name => Some($disp),
                    )+
                    _ => return None,
                }
            }

	    /// Converts a register name into a register number.
	    pub fn name_to_register(value: &str) -> Option<Register> {
		match value {
                    $(
                        $disp => Some(Self::$name),
                    )+
                    $(
                        $($alias_disp => Some(Self::$alias_name),)+
                    )*
                    _ => return None,
		}
	    }
        }
    };
}

/// ARM architecture specific definitions.
///
/// See [DWARF for the ARM Architecture](
/// https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).
#[derive(Debug, Clone, Copy)]
pub struct Arm;

registers!(Arm, {
    R0 = (0, "R0"),
    R1 = (1, "R1"),
    R2 = (2, "R2"),
    R3 = (3, "R3"),
    R4 = (4, "R4"),
    R5 = (5, "R5"),
    R6 = (6, "R6"),
    R7 = (7, "R7"),
    R8 = (8, "R8"),
    R9 = (9, "R9"),
    R10 = (10, "R10"),
    R11 = (11, "R11"),
    R12 = (12, "R12"),
    R13 = (13, "R13"),
    R14 = (14, "R14"),
    R15 = (15, "R15"),

    WCGR0 = (104, "wCGR0"),
    WCGR1 = (105, "wCGR1"),
    WCGR2 = (106, "wCGR2"),
    WCGR3 = (107, "wCGR3"),
    WCGR4 = (108, "wCGR4"),
    WCGR5 = (109, "wCGR5"),
    WCGR6 = (110, "wCGR6"),
    WCGR7 = (111, "wCGR7"),

    WR0 = (112, "wR0"),
    WR1 = (113, "wR1"),
    WR2 = (114, "wR2"),
    WR3 = (115, "wR3"),
    WR4 = (116, "wR4"),
    WR5 = (117, "wR5"),
    WR6 = (118, "wR6"),
    WR7 = (119, "wR7"),
    WR8 = (120, "wR8"),
    WR9 = (121, "wR9"),
    WR10 = (122, "wR10"),
    WR11 = (123, "wR11"),
    WR12 = (124, "wR12"),
    WR13 = (125, "wR13"),
    WR14 = (126, "wR14"),
    WR15 = (127, "wR15"),

    SPSR = (128, "SPSR"),
    SPSR_FIQ = (129, "SPSR_FIQ"),
    SPSR_IRQ = (130, "SPSR_IRQ"),
    SPSR_ABT = (131, "SPSR_ABT"),
    SPSR_UND = (132, "SPSR_UND"),
    SPSR_SVC = (133, "SPSR_SVC"),

    RA_AUTH_CODE = (143, "RA_AUTH_CODE"),

    R8_USR = (144, "R8_USR"),
    R9_USR = (145, "R9_USR"),
    R10_USR = (146, "R10_USR"),
    R11_USR = (147, "R11_USR"),
    R12_USR = (148, "R12_USR"),
    R13_USR = (149, "R13_USR"),
    R14_USR = (150, "R14_USR"),

    R8_FIQ = (151, "R8_FIQ"),
    R9_FIQ = (152, "R9_FIQ"),
    R10_FIQ = (153, "R10_FIQ"),
    R11_FIQ = (154, "R11_FIQ"),
    R12_FIQ = (155, "R12_FIQ"),
    R13_FIQ = (156, "R13_FIQ"),
    R14_FIQ = (157, "R14_FIQ"),

    R13_IRQ = (158, "R13_IRQ"),
    R14_IRQ = (159, "R14_IRQ"),

    R13_ABT = (160, "R13_ABT"),
    R14_ABT = (161, "R14_ABT"),

    R13_UND = (162, "R13_UND"),
    R14_UND = (163, "R14_UND"),

    R13_SVC = (164, "R13_SVC"),
    R14_SVC = (165, "R14_SVC"),

    WC0 = (192, "wC0"),
    WC1 = (193, "wC1"),
    WC2 = (194, "wC2"),
    WC3 = (195, "wC3"),
    WC4 = (196, "wC4"),
    WC5 = (197, "wC5"),
    WC6 = (198, "wC6"),
    WC7 = (199, "wC7"),

    D0 = (256, "D0"),
    D1 = (257, "D1"),
    D2 = (258, "D2"),
    D3 = (259, "D3"),
    D4 = (260, "D4"),
    D5 = (261, "D5"),
    D6 = (262, "D6"),
    D7 = (263, "D7"),
    D8 = (264, "D8"),
    D9 = (265, "D9"),
    D10 = (266, "D10"),
    D11 = (267, "D11"),
    D12 = (268, "D12"),
    D13 = (269, "D13"),
    D14 = (270, "D14"),
    D15 = (271, "D15"),
    D16 = (272, "D16"),
    D17 = (273, "D17"),
    D18 = (274, "D18"),
    D19 = (275, "D19"),
    D20 = (276, "D20"),
    D21 = (277, "D21"),
    D22 = (278, "D22"),
    D23 = (279, "D23"),
    D24 = (280, "D24"),
    D25 = (281, "D25"),
    D26 = (282, "D26"),
    D27 = (283, "D27"),
    D28 = (284, "D28"),
    D29 = (285, "D29"),
    D30 = (286, "D30"),
    D31 = (287, "D31"),

    TPIDRURO = (320, "TPIDRURO"),
    TPIDRURW = (321, "TPIDRURW"),
    TPIDPR = (322, "TPIDPR"),
    HTPIDPR = (323, "HTPIDPR"),
},
aliases {
    SP = (13, "SP"),
    LR = (14, "LR"),
    PC = (15, "PC"),

    ACC0 = (104, "ACC0"),
    ACC1 = (105, "ACC1"),
    ACC2 = (106, "ACC2"),
    ACC3 = (107, "ACC3"),
    ACC4 = (108, "ACC4"),
    ACC5 = (109, "ACC5"),
    ACC6 = (110, "ACC6"),
    ACC7 = (111, "ACC7"),

    S0 = (256, "S0"),
    S1 = (256, "S1"),
    S2 = (257, "S2"),
    S3 = (257, "S3"),
    S4 = (258, "S4"),
    S5 = (258, "S5"),
    S6 = (259, "S6"),
    S7 = (259, "S7"),
    S8 = (260, "S8"),
    S9 = (260, "S9"),
    S10 = (261, "S10"),
    S11 = (261, "S11"),
    S12 = (262, "S12"),
    S13 = (262, "S13"),
    S14 = (263, "S14"),
    S15 = (263, "S15"),
    S16 = (264, "S16"),
    S17 = (264, "S17"),
    S18 = (265, "S18"),
    S19 = (265, "S19"),
    S20 = (266, "S20"),
    S21 = (266, "S21"),
    S22 = (267, "S22"),
    S23 = (267, "S23"),
    S24 = (268, "S24"),
    S25 = (268, "S25"),
    S26 = (269, "S26"),
    S27 = (269, "S27"),
    S28 = (270, "S28"),
    S29 = (270, "S29"),
    S30 = (271, "S30"),
    S31 = (271, "S31"),
});

/// ARM 64-bit (AArch64) architecture specific definitions.
///
/// See [DWARF for the ARM 64-bit Architecture](
/// https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).
#[derive(Debug, Clone, Copy)]
pub struct AArch64;

registers!(AArch64, {
    X0 = (0, "X0"),
    X1 = (1, "X1"),
    X2 = (2, "X2"),
    X3 = (3, "X3"),
    X4 = (4, "X4"),
    X5 = (5, "X5"),
    X6 = (6, "X6"),
    X7 = (7, "X7"),
    X8 = (8, "X8"),
    X9 = (9, "X9"),
    X10 = (10, "X10"),
    X11 = (11, "X11"),
    X12 = (12, "X12"),
    X13 = (13, "X13"),
    X14 = (14, "X14"),
    X15 = (15, "X15"),
    X16 = (16, "X16"),
    X17 = (17, "X17"),
    X18 = (18, "X18"),
    X19 = (19, "X19"),
    X20 = (20, "X20"),
    X21 = (21, "X21"),
    X22 = (22, "X22"),
    X23 = (23, "X23"),
    X24 = (24, "X24"),
    X25 = (25, "X25"),
    X26 = (26, "X26"),
    X27 = (27, "X27"),
    X28 = (28, "X28"),
    X29 = (29, "X29"),
    X30 = (30, "X30"),
    SP = (31, "SP"),
    PC = (32, "PC"),
    ELR_MODE = (33, "ELR_mode"),
    RA_SIGN_STATE = (34, "RA_SIGN_STATE"),
    TPIDRRO_EL0 = (35, "TPIDRRO_EL0"),
    TPIDR_EL0 = (36, "TPIDR_EL0"),
    TPIDR_EL1 = (37, "TPIDR_EL1"),
    TPIDR_EL2 = (38, "TPIDR_EL2"),
    TPIDR_EL3 = (39, "TPIDR_EL3"),

    VG = (46, "VG"),
    FFR = (47, "FFR"),

    P0 = (48, "P0"),
    P1 = (49, "P1"),
    P2 = (50, "P2"),
    P3 = (51, "P3"),
    P4 = (52, "P4"),
    P5 = (53, "P5"),
    P6 = (54, "P6"),
    P7 = (55, "P7"),
    P8 = (56, "P8"),
    P9 = (57, "P9"),
    P10 = (58, "P10"),
    P11 = (59, "P11"),
    P12 = (60, "P12"),
    P13 = (61, "P13"),
    P14 = (62, "P14"),
    P15 = (63, "P15"),

    V0 = (64, "V0"),
    V1 = (65, "V1"),
    V2 = (66, "V2"),
    V3 = (67, "V3"),
    V4 = (68, "V4"),
    V5 = (69, "V5"),
    V6 = (70, "V6"),
    V7 = (71, "V7"),
    V8 = (72, "V8"),
    V9 = (73, "V9"),
    V10 = (74, "V10"),
    V11 = (75, "V11"),
    V12 = (76, "V12"),
    V13 = (77, "V13"),
    V14 = (78, "V14"),
    V15 = (79, "V15"),
    V16 = (80, "V16"),
    V17 = (81, "V17"),
    V18 = (82, "V18"),
    V19 = (83, "V19"),
    V20 = (84, "V20"),
    V21 = (85, "V21"),
    V22 = (86, "V22"),
    V23 = (87, "V23"),
    V24 = (88, "V24"),
    V25 = (89, "V25"),
    V26 = (90, "V26"),
    V27 = (91, "V27"),
    V28 = (92, "V28"),
    V29 = (93, "V29"),
    V30 = (94, "V30"),
    V31 = (95, "V31"),

    Z0 = (96, "Z0"),
    Z1 = (97, "Z1"),
    Z2 = (98, "Z2"),
    Z3 = (99, "Z3"),
    Z4 = (100, "Z4"),
    Z5 = (101, "Z5"),
    Z6 = (102, "Z6"),
    Z7 = (103, "Z7"),
    Z8 = (104, "Z8"),
    Z9 = (105, "Z9"),
    Z10 = (106, "Z10"),
    Z11 = (107, "Z11"),
    Z12 = (108, "Z12"),
    Z13 = (109, "Z13"),
    Z14 = (110, "Z14"),
    Z15 = (111, "Z15"),
    Z16 = (112, "Z16"),
    Z17 = (113, "Z17"),
    Z18 = (114, "Z18"),
    Z19 = (115, "Z19"),
    Z20 = (116, "Z20"),
    Z21 = (117, "Z21"),
    Z22 = (118, "Z22"),
    Z23 = (119, "Z23"),
    Z24 = (120, "Z24"),
    Z25 = (121, "Z25"),
    Z26 = (122, "Z26"),
    Z27 = (123, "Z27"),
    Z28 = (124, "Z28"),
    Z29 = (125, "Z29"),
    Z30 = (126, "Z30"),
    Z31 = (127, "Z31"),
});

/// LoongArch architecture specific definitions.
///
/// See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).
#[derive(Debug, Clone, Copy)]
pub struct LoongArch;

registers!(LoongArch, {
    R0 = (0, "$r0"),
    R1 = (1, "$r1"),
    R2 = (2, "$r2"),
    R3 = (3, "$r3"),
    R4 = (4, "$r4"),
    R5 = (5, "$r5"),
    R6 = (6, "$r6"),
    R7 = (7, "$r7"),
    R8 = (8, "$r8"),
    R9 = (9, "$r9"),
    R10 = (10, "$r10"),
    R11 = (11, "$r11"),
    R12 = (12, "$r12"),
    R13 = (13, "$r13"),
    R14 = (14, "$r14"),
    R15 = (15, "$r15"),
    R16 = (16, "$r16"),
    R17 = (17, "$r17"),
    R18 = (18, "$r18"),
    R19 = (19, "$r19"),
    R20 = (20, "$r20"),
    R21 = (21, "$r21"),
    R22 = (22, "$r22"),
    R23 = (23, "$r23"),
    R24 = (24, "$r24"),
    R25 = (25, "$r25"),
    R26 = (26, "$r26"),
    R27 = (27, "$r27"),
    R28 = (28, "$r28"),
    R29 = (29, "$r29"),
    R30 = (30, "$r30"),
    R31 = (31, "$r31"),

    F0 = (32, "$f0"),
    F1 = (33, "$f1"),
    F2 = (34, "$f2"),
    F3 = (35, "$f3"),
    F4 = (36, "$f4"),
    F5 = (37, "$f5"),
    F6 = (38, "$f6"),
    F7 = (39, "$f7"),
    F8 = (40, "$f8"),
    F9 = (41, "$f9"),
    F10 = (42, "$f10"),
    F11 = (43, "$f11"),
    F12 = (44, "$f12"),
    F13 = (45, "$f13"),
    F14 = (46, "$f14"),
    F15 = (47, "$f15"),
    F16 = (48, "$f16"),
    F17 = (49, "$f17"),
    F18 = (50, "$f18"),
    F19 = (51, "$f19"),
    F20 = (52, "$f20"),
    F21 = (53, "$f21"),
    F22 = (54, "$f22"),
    F23 = (55, "$f23"),
    F24 = (56, "$f24"),
    F25 = (57, "$f25"),
    F26 = (58, "$f26"),
    F27 = (59, "$f27"),
    F28 = (60, "$f28"),
    F29 = (61, "$f29"),
    F30 = (62, "$f30"),
    F31 = (63, "$f31"),
    FCC0 = (64, "$fcc0"),
    FCC1 = (65, "$fcc1"),
    FCC2 = (66, "$fcc2"),
    FCC3 = (67, "$fcc3"),
    FCC4 = (68, "$fcc4"),
    FCC5 = (69, "$fcc5"),
    FCC6 = (70, "$fcc6"),
    FCC7 = (71, "$fcc7"),
},
aliases {
    ZERO = (0, "$zero"),
    RA = (1, "$ra"),
    TP = (2, "$tp"),
    SP = (3, "$sp"),
    A0 = (4, "$a0"),
    A1 = (5, "$a1"),
    A2 = (6, "$a2"),
    A3 = (7, "$a3"),
    A4 = (8, "$a4"),
    A5 = (9, "$a5"),
    A6 = (10, "$a6"),
    A7 = (11, "$a7"),
    T0 = (12, "$t0"),
    T1 = (13, "$t1"),
    T2 = (14, "$t2"),
    T3 = (15, "$t3"),
    T4 = (16, "$t4"),
    T5 = (17, "$t5"),
    T6 = (18, "$t6"),
    T7 = (19, "$t7"),
    T8 = (20, "$t8"),
    FP = (22, "$fp"),
    S0 = (23, "$s0"),
    S1 = (24, "$s1"),
    S2 = (25, "$s2"),
    S3 = (26, "$s3"),
    S4 = (27, "$s4"),
    S5 = (28, "$s5"),
    S6 = (29, "$s6"),
    S7 = (30, "$s7"),
    S8 = (31, "$s8"),

    FA0 = (32, "$fa0"),
    FA1 = (33, "$fa1"),
    FA2 = (34, "$fa2"),
    FA3 = (35, "$fa3"),
    FA4 = (36, "$fa4"),
    FA5 = (37, "$fa5"),
    FA6 = (38, "$fa6"),
    FA7 = (39, "$fa7"),
    FT0 = (40, "$ft0"),
    FT1 = (41, "$ft1"),
    FT2 = (42, "$ft2"),
    FT3 = (43, "$ft3"),
    FT4 = (44, "$ft4"),
    FT5 = (45, "$ft5"),
    FT6 = (46, "$ft6"),
    FT7 = (47, "$ft7"),
    FT8 = (48, "$ft8"),
    FT9 = (49, "$ft9"),
    FT10 = (50, "$ft10"),
    FT11 = (51, "$ft11"),
    FT12 = (52, "$ft12"),
    FT13 = (53, "$ft13"),
    FT14 = (54, "$ft14"),
    FT15 = (55, "$ft15"),
    FS0 = (56, "$fs0"),
    FS1 = (57, "$fs1"),
    FS2 = (58, "$fs2"),
    FS3 = (59, "$fs3"),
    FS4 = (60, "$fs4"),
    FS5 = (61, "$fs5"),
    FS6 = (62, "$fs6"),
    FS7 = (63, "$fs7"),
});

/// RISC-V architecture specific definitions.
///
/// See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).
#[derive(Debug, Clone, Copy)]
pub struct RiscV;

registers!(RiscV, {
    X0 = (0, "x0"),
    X1 = (1, "x1"),
    X2 = (2, "x2"),
    X3 = (3, "x3"),
    X4 = (4, "x4"),
    X5 = (5, "x5"),
    X6 = (6, "x6"),
    X7 = (7, "x7"),
    X8 = (8, "x8"),
    X9 = (9, "x9"),
    X10 = (10, "x10"),
    X11 = (11, "x11"),
    X12 = (12, "x12"),
    X13 = (13, "x13"),
    X14 = (14, "x14"),
    X15 = (15, "x15"),
    X16 = (16, "x16"),
    X17 = (17, "x17"),
    X18 = (18, "x18"),
    X19 = (19, "x19"),
    X20 = (20, "x20"),
    X21 = (21, "x21"),
    X22 = (22, "x22"),
    X23 = (23, "x23"),
    X24 = (24, "x24"),
    X25 = (25, "x25"),
    X26 = (26, "x26"),
    X27 = (27, "x27"),
    X28 = (28, "x28"),
    X29 = (29, "x29"),
    X30 = (30, "x30"),
    X31 = (31, "x31"),

    F0 = (32, "f0"),
    F1 = (33, "f1"),
    F2 = (34, "f2"),
    F3 = (35, "f3"),
    F4 = (36, "f4"),
    F5 = (37, "f5"),
    F6 = (38, "f6"),
    F7 = (39, "f7"),
    F8 = (40, "f8"),
    F9 = (41, "f9"),
    F10 = (42, "f10"),
    F11 = (43, "f11"),
    F12 = (44, "f12"),
    F13 = (45, "f13"),
    F14 = (46, "f14"),
    F15 = (47, "f15"),
    F16 = (48, "f16"),
    F17 = (49, "f17"),
    F18 = (50, "f18"),
    F19 = (51, "f19"),
    F20 = (52, "f20"),
    F21 = (53, "f21"),
    F22 = (54, "f22"),
    F23 = (55, "f23"),
    F24 = (56, "f24"),
    F25 = (57, "f25"),
    F26 = (58, "f26"),
    F27 = (59, "f27"),
    F28 = (60, "f28"),
    F29 = (61, "f29"),
    F30 = (62, "f30"),
    F31 = (63, "f31"),
},
aliases {
    ZERO = (0, "zero"),
    RA = (1, "ra"),
    SP = (2, "sp"),
    GP = (3, "gp"),
    TP = (4, "tp"),
    T0 = (5, "t0"),
    T1 = (6, "t1"),
    T2 = (7, "t2"),
    S0 = (8, "s0"),
    S1 = (9, "s1"),
    A0 = (10, "a0"),
    A1 = (11, "a1"),
    A2 = (12, "a2"),
    A3 = (13, "a3"),
    A4 = (14, "a4"),
    A5 = (15, "a5"),
    A6 = (16, "a6"),
    A7 = (17, "a7"),
    S2 = (18, "s2"),
    S3 = (19, "s3"),
    S4 = (20, "s4"),
    S5 = (21, "s5"),
    S6 = (22, "s6"),
    S7 = (23, "s7"),
    S8 = (24, "s8"),
    S9 = (25, "s9"),
    S10 = (26, "s10"),
    S11 = (27, "s11"),
    T3 = (28, "t3"),
    T4 = (29, "t4"),
    T5 = (30, "t5"),
    T6 = (31, "t6"),

    FT0 = (32, "ft0"),
    FT1 = (33, "ft1"),
    FT2 = (34, "ft2"),
    FT3 = (35, "ft3"),
    FT4 = (36, "ft4"),
    FT5 = (37, "ft5"),
    FT6 = (38, "ft6"),
    FT7 = (39, "ft7"),
    FS0 = (40, "fs0"),
    FS1 = (41, "fs1"),
    FA0 = (42, "fa0"),
    FA1 = (43, "fa1"),
    FA2 = (44, "fa2"),
    FA3 = (45, "fa3"),
    FA4 = (46, "fa4"),
    FA5 = (47, "fa5"),
    FA6 = (48, "fa6"),
    FA7 = (49, "fa7"),
    FS2 = (50, "fs2"),
    FS3 = (51, "fs3"),
    FS4 = (52, "fs4"),
    FS5 = (53, "fs5"),
    FS6 = (54, "fs6"),
    FS7 = (55, "fs7"),
    FS8 = (56, "fs8"),
    FS9 = (57, "fs9"),
    FS10 = (58, "fs10"),
    FS11 = (59, "fs11"),
    FT8 = (60, "ft8"),
    FT9 = (61, "ft9"),
    FT10 = (62, "ft10"),
    FT11 = (63, "ft11"),
});

/// Intel i386 architecture specific definitions.
///
/// See Intel386 psABi version 1.1 at the [X86 psABI wiki](https://github.com/hjl-tools/x86-psABI/wiki/X86-psABI).
#[derive(Debug, Clone, Copy)]
pub struct X86;

registers!(X86, {
    EAX = (0, "eax"),
    ECX = (1, "ecx"),
    EDX = (2, "edx"),
    EBX = (3, "ebx"),
    ESP = (4, "esp"),
    EBP = (5, "ebp"),
    ESI = (6, "esi"),
    EDI = (7, "edi"),

    // Return Address register. This is stored in `0(%esp, "")` and is not a physical register.
    RA = (8, "RA"),

    ST0 = (11, "st0"),
    ST1 = (12, "st1"),
    ST2 = (13, "st2"),
    ST3 = (14, "st3"),
    ST4 = (15, "st4"),
    ST5 = (16, "st5"),
    ST6 = (17, "st6"),
    ST7 = (18, "st7"),

    XMM0 = (21, "xmm0"),
    XMM1 = (22, "xmm1"),
    XMM2 = (23, "xmm2"),
    XMM3 = (24, "xmm3"),
    XMM4 = (25, "xmm4"),
    XMM5 = (26, "xmm5"),
    XMM6 = (27, "xmm6"),
    XMM7 = (28, "xmm7"),

    MM0 = (29, "mm0"),
    MM1 = (30, "mm1"),
    MM2 = (31, "mm2"),
    MM3 = (32, "mm3"),
    MM4 = (33, "mm4"),
    MM5 = (34, "mm5"),
    MM6 = (35, "mm6"),
    MM7 = (36, "mm7"),

    MXCSR = (39, "mxcsr"),

    ES = (40, "es"),
    CS = (41, "cs"),
    SS = (42, "ss"),
    DS = (43, "ds"),
    FS = (44, "fs"),
    GS = (45, "gs"),

    TR = (48, "tr"),
    LDTR = (49, "ldtr"),

    FS_BASE = (93, "fs.base"),
    GS_BASE = (94, "gs.base"),
});

/// AMD64 architecture specific definitions.
///
/// See x86-64 psABI version 1.0 at the [X86 psABI wiki](https://github.com/hjl-tools/x86-psABI/wiki/X86-psABI).
#[derive(Debug, Clone, Copy)]
pub struct X86_64;

registers!(X86_64, {
    RAX = (0, "rax"),
    RDX = (1, "rdx"),
    RCX = (2, "rcx"),
    RBX = (3, "rbx"),
    RSI = (4, "rsi"),
    RDI = (5, "rdi"),
    RBP = (6, "rbp"),
    RSP = (7, "rsp"),

    R8 = (8, "r8"),
    R9 = (9, "r9"),
    R10 = (10, "r10"),
    R11 = (11, "r11"),
    R12 = (12, "r12"),
    R13 = (13, "r13"),
    R14 = (14, "r14"),
    R15 = (15, "r15"),

    // Return Address register. This is stored in `0(%rsp, "")` and is not a physical register.
    RA = (16, "RA"),

    XMM0 = (17, "xmm0"),
    XMM1 = (18, "xmm1"),
    XMM2 = (19, "xmm2"),
    XMM3 = (20, "xmm3"),
    XMM4 = (21, "xmm4"),
    XMM5 = (22, "xmm5"),
    XMM6 = (23, "xmm6"),
    XMM7 = (24, "xmm7"),

    XMM8 = (25, "xmm8"),
    XMM9 = (26, "xmm9"),
    XMM10 = (27, "xmm10"),
    XMM11 = (28, "xmm11"),
    XMM12 = (29, "xmm12"),
    XMM13 = (30, "xmm13"),
    XMM14 = (31, "xmm14"),
    XMM15 = (32, "xmm15"),

    ST0 = (33, "st0"),
    ST1 = (34, "st1"),
    ST2 = (35, "st2"),
    ST3 = (36, "st3"),
    ST4 = (37, "st4"),
    ST5 = (38, "st5"),
    ST6 = (39, "st6"),
    ST7 = (40, "st7"),

    MM0 = (41, "mm0"),
    MM1 = (42, "mm1"),
    MM2 = (43, "mm2"),
    MM3 = (44, "mm3"),
    MM4 = (45, "mm4"),
    MM5 = (46, "mm5"),
    MM6 = (47, "mm6"),
    MM7 = (48, "mm7"),

    RFLAGS = (49, "rFLAGS"),
    ES = (50, "es"),
    CS = (51, "cs"),
    SS = (52, "ss"),
    DS = (53, "ds"),
    FS = (54, "fs"),
    GS = (55, "gs"),

    FS_BASE = (58, "fs.base"),
    GS_BASE = (59, "gs.base"),

    TR = (62, "tr"),
    LDTR = (63, "ldtr"),
    MXCSR = (64, "mxcsr"),
    FCW = (65, "fcw"),
    FSW = (66, "fsw"),

    XMM16 = (67, "xmm16"),
    XMM17 = (68, "xmm17"),
    XMM18 = (69, "xmm18"),
    XMM19 = (70, "xmm19"),
    XMM20 = (71, "xmm20"),
    XMM21 = (72, "xmm21"),
    XMM22 = (73, "xmm22"),
    XMM23 = (74, "xmm23"),
    XMM24 = (75, "xmm24"),
    XMM25 = (76, "xmm25"),
    XMM26 = (77, "xmm26"),
    XMM27 = (78, "xmm27"),
    XMM28 = (79, "xmm28"),
    XMM29 = (80, "xmm29"),
    XMM30 = (81, "xmm30"),
    XMM31 = (82, "xmm31"),

    K0 = (118, "k0"),
    K1 = (119, "k1"),
    K2 = (120, "k2"),
    K3 = (121, "k3"),
    K4 = (122, "k4"),
    K5 = (123, "k5"),
    K6 = (124, "k6"),
    K7 = (125, "k7"),
});

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "std")]
    fn test_aarch64_registers() {
        use super::*;
        use std::collections::HashSet;

        let mut names = HashSet::new();
        for n in (0..=39).chain(46..=127) {
            let name = AArch64::register_name(Register(n))
                .unwrap_or_else(|| panic!("Register {} should have a name.", n));
            assert!(names.insert(name));
        }
    }
}