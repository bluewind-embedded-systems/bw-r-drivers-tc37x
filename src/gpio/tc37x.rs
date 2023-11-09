use super::*;
pub use super::Input as DefaultMode;

gpio!(gpio00, crate::pac::port_00::Port00, 0, P00n, [
    P00_5: (p00_5, 5, [1]),
    P00_6: (p00_6, 6, [1]),
]);

gpio!(gpio01, crate::pac::port_01::Port01, 1, P01n, [
    P01_9: (p01_9, 9, [1]),
    P01_10: (p01_10, 10, [1]),
]);
