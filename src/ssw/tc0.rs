use crate::scu::ccu;

use super::infra::is_application_reset;

pub fn init_software() {
    if !is_application_reset() {
        #[cfg(feature = "log")]
        defmt::debug!("power on reset");
        // TODO Handle error
        let _ = ccu::init(&ccu::DEFAULT_CLOCK_CONFIG);
    } else {
        #[cfg(feature = "log")]
        defmt::debug!("application reset")
    }
}
