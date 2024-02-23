use crate::log::debug;
use crate::scu::ccu;
use crate::scu::ccu::InitError;

use super::infra::is_application_reset;

// TODO Are we sure we want to publish this function?
pub fn init_clock() -> Result<(), InitError> {
    if is_application_reset() {
        debug!("application reset");
        Ok(())
    } else {
        debug!("power on reset");
        ccu::init(&ccu::DEFAULT_CLOCK_CONFIG)
    }
}
