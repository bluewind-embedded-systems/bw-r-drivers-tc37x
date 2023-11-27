use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod can0 {
    use super::*;

    pin! {
        <Rx, PushPull> for no:NoPin, [
            // FIXME Alternate function here?
            P20_7<1>, // CAN00:RXDB = P20.7:IN
        ],

        <Tx, PushPull> for no:NoPin, [
            P20_8<5>, // CAN00:TXD = P20.8:ALT(5)
        ],
    }

    impl CanCommon for crate::pac::can0::Can0 {
        type Rx = Rx;
        type Tx = Tx;
    }
}
