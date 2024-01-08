use crate::scu::ccu;
use crate::log::debug;

use super::infra::is_application_reset;

pub fn init_software() {
    if !is_application_reset() {
        debug!("power on reset");
        // TODO Handle error
        let _ = ccu::init(&ccu::DEFAULT_CLOCK_CONFIG);
    } else {
        debug!("application reset");
    }
}
