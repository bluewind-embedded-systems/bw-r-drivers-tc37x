use crate::scu::ccu;

use super::is_application_reset;

pub fn init_software() {
    if !is_application_reset() {
        #[cfg(feature = "log")]
        defmt::debug!("power on reset");
        //TODO (annabo)
        //ccu::init(&ccu::DEFAULT_CLOCK_CONFIG).unwrap();
    } else {
        #[cfg(feature = "log")]
        defmt::debug!("application reset")
    }
}
