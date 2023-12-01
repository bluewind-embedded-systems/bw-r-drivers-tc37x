use core::fmt;
use tc37x_pac::{self as pac};
use core::marker::PhantomData;
use tc37x_pac::RegisterValue;
mod can;

use crate::scu; 
pub use can::*;



//pub use embedded_hal::can::Can; 
pub struct CanNode; 
#[derive(Debug, Default)]
struct CanError;
struct Result; 