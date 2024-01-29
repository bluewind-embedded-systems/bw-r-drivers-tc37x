use crate::can::can_node::{OutputIdx, PortNumber, RxSel, RxdIn, TxdOut};
use crate::can::{ModuleId, NodeId};

pub const PIN_RX_0_0_P02_1: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node0, PortNumber::_02, 1, RxSel::_A);
pub const PIN_RX_0_0_P20_7: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node0, PortNumber::_20, 7, RxSel::_B);
pub const PIN_RX_0_0_P12_0: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node0, PortNumber::_12, 0, RxSel::_C);
pub const PIN_RX_0_0_P33_12: RxdIn = RxdIn::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_33,
    12,
    RxSel::_D,
);
pub const PIN_RX_0_0_P33_7: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node0, PortNumber::_33, 7, RxSel::_E);
pub const PIN_RX_0_0_P34_2: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node0, PortNumber::_34, 2, RxSel::_G);
pub const PIN_RX_0_1_P15_3: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node1, PortNumber::_15, 3, RxSel::_A);
pub const PIN_RX_0_1_P14_1: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node1, PortNumber::_14, 1, RxSel::_B);
pub const PIN_RX_0_1_P01_4: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node1, PortNumber::_01, 4, RxSel::_C);
pub const PIN_RX_0_1_P33_10: RxdIn = RxdIn::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_33,
    10,
    RxSel::_D,
);
pub const PIN_RX_0_1_P02_10: RxdIn = RxdIn::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_02,
    10,
    RxSel::_E,
);
pub const PIN_RX_0_2_P15_1: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node2, PortNumber::_15, 1, RxSel::_A);
pub const PIN_RX_0_2_P02_3: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node2, PortNumber::_02, 3, RxSel::_B);
pub const PIN_RX_0_2_P32_6: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node2, PortNumber::_32, 6, RxSel::_C);
pub const PIN_RX_0_2_P14_8: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node2, PortNumber::_14, 8, RxSel::_D);
pub const PIN_RX_0_2_P10_2: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node2, PortNumber::_10, 2, RxSel::_E);
pub const PIN_RX_0_3_P00_3: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node3, PortNumber::_00, 3, RxSel::_A);
pub const PIN_RX_0_3_P32_2: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node3, PortNumber::_32, 2, RxSel::_B);
pub const PIN_RX_0_3_P20_0: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node3, PortNumber::_20, 0, RxSel::_C);
pub const PIN_RX_0_3_P11_10: RxdIn = RxdIn::new(
    ModuleId::Can0,
    NodeId::Node3,
    PortNumber::_11,
    10,
    RxSel::_D,
);
pub const PIN_RX_0_3_P20_9: RxdIn =
    RxdIn::new(ModuleId::Can0, NodeId::Node3, PortNumber::_20, 9, RxSel::_E);
pub const PIN_RX_1_0_P00_1: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node0, PortNumber::_00, 1, RxSel::_A);
pub const PIN_RX_1_0_P14_7: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node0, PortNumber::_14, 7, RxSel::_B);
pub const PIN_RX_1_0_P23_0: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node0, PortNumber::_23, 0, RxSel::_C);
pub const PIN_RX_1_0_P13_1: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node0, PortNumber::_13, 1, RxSel::_D);
pub const PIN_RX_1_1_P02_4: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node1, PortNumber::_02, 4, RxSel::_A);
pub const PIN_RX_1_1_P00_5: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node1, PortNumber::_00, 5, RxSel::_B);
pub const PIN_RX_1_1_P23_7: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node1, PortNumber::_23, 7, RxSel::_C);
pub const PIN_RX_1_1_P11_7: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node1, PortNumber::_11, 7, RxSel::_D);
pub const PIN_RX_1_2_P20_6: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node2, PortNumber::_20, 6, RxSel::_A);
pub const PIN_RX_1_2_P10_8: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node2, PortNumber::_10, 8, RxSel::_B);
pub const PIN_RX_1_2_P23_3: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node2, PortNumber::_23, 3, RxSel::_C);
pub const PIN_RX_1_2_P11_8: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node2, PortNumber::_11, 8, RxSel::_D);
pub const PIN_RX_1_3_P14_7: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node3, PortNumber::_14, 7, RxSel::_A);
pub const PIN_RX_1_3_P33_5: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node3, PortNumber::_33, 5, RxSel::_B);
pub const PIN_RX_1_3_P22_5: RxdIn =
    RxdIn::new(ModuleId::Can1, NodeId::Node3, PortNumber::_22, 5, RxSel::_C);
pub const PIN_RX_1_3_P11_13: RxdIn = RxdIn::new(
    ModuleId::Can1,
    NodeId::Node3,
    PortNumber::_11,
    13,
    RxSel::_D,
);

pub const PIN_TX_0_0_P02_0: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_02,
    0,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_0_P12_1: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_12,
    1,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_0_P20_8: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_20,
    8,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_0_P33_13: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_33,
    13,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_0_P33_8: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_33,
    8,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_0_P34_1: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node0,
    PortNumber::_34,
    1,
    OutputIdx::ALT4,
);
pub const PIN_TX_0_1_P01_3: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_01,
    3,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_1_P02_9: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_02,
    9,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_1_P14_0: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_14,
    0,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_1_P15_2: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_15,
    2,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_1_P33_9: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node1,
    PortNumber::_33,
    9,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_2_P02_2: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node2,
    PortNumber::_02,
    2,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_2_P10_3: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node2,
    PortNumber::_10,
    3,
    OutputIdx::ALT6,
);
pub const PIN_TX_0_2_P14_10: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node2,
    PortNumber::_14,
    10,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_2_P15_0: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node2,
    PortNumber::_15,
    0,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_2_P32_5: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node2,
    PortNumber::_32,
    5,
    OutputIdx::ALT6,
);
pub const PIN_TX_0_3_P00_2: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node3,
    PortNumber::_00,
    2,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_3_P11_12: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node3,
    PortNumber::_11,
    12,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_3_P20_10: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node3,
    PortNumber::_20,
    10,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_3_P20_3: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node3,
    PortNumber::_20,
    3,
    OutputIdx::ALT5,
);
pub const PIN_TX_0_3_P32_3: TxdOut = TxdOut::new(
    ModuleId::Can0,
    NodeId::Node3,
    PortNumber::_32,
    3,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_0_P00_0: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node0,
    PortNumber::_00,
    0,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_0_P13_0: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node0,
    PortNumber::_13,
    0,
    OutputIdx::ALT7,
);
pub const PIN_TX_1_0_P14_9: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node0,
    PortNumber::_14,
    9,
    OutputIdx::ALT4,
);
pub const PIN_TX_1_0_P23_1: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node0,
    PortNumber::_23,
    1,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_1_P00_4: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node1,
    PortNumber::_00,
    4,
    OutputIdx::ALT3,
);
pub const PIN_TX_1_1_P02_5: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node1,
    PortNumber::_02,
    5,
    OutputIdx::ALT2,
);
pub const PIN_TX_1_1_P11_0: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node1,
    PortNumber::_11,
    0,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_1_P23_6: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node1,
    PortNumber::_23,
    6,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_2_P10_7: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node2,
    PortNumber::_10,
    7,
    OutputIdx::ALT6,
);
pub const PIN_TX_1_2_P11_1: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node2,
    PortNumber::_11,
    1,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_2_P20_7: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node2,
    PortNumber::_20,
    7,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_2_P23_2: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node2,
    PortNumber::_23,
    2,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_3_P11_4: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node3,
    PortNumber::_11,
    4,
    OutputIdx::ALT5,
);
pub const PIN_TX_1_3_P14_6: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node3,
    PortNumber::_14,
    6,
    OutputIdx::ALT4,
);
pub const PIN_TX_1_3_P22_4: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node3,
    PortNumber::_22,
    4,
    OutputIdx::ALT6,
);
pub const PIN_TX_1_3_P33_4: TxdOut = TxdOut::new(
    ModuleId::Can1,
    NodeId::Node3,
    PortNumber::_33,
    4,
    OutputIdx::ALT7,
);
