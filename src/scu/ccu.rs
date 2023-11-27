use super::wdt; 
use tc37x_pac::SCU; 
pub struct CpuConfig; 

const SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT: usize = 0x3000;
const OSCCON_PLLLV_OR_HV_TIMEOUT_COUNT: usize = 0x493E0;
const PLL_LOCK_TIMEOUT_COUNT: usize = 0x3000;

const CCUCON_LCK_BIT_TIMEOUT_COUNT: usize = 0x1000;
const PLL_KRDY_TIMEOUT_COUNT: usize = 0x6000;


pub fn init(config: &CpuConfig) -> Result<(), ()> {
    configure_ccu_initial_step(&config)?;

    modulation_init(&config);

    distribute_clock_inline(&config)?;

    throttle_sys_pll_clock_inline(config)?;

    Ok(())
}

pub fn configure_ccu_initial_step(config: &CpuConfig) -> Result<(), ()> 
{
    let endinit_sfty_pw = wdt::get_safety_watchdog_password();
    wdt::clear_safety_endinit_inline(endinit_sfty_pw);
   
    wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| {
        unsafe { SCU.ccucon0().read() }.lck().get()
    })?;

    Ok(())
}

pub fn modulation_init(config: &CpuConfig) -> Result<(), ()> 
{
    Ok(())
}

pub fn distribute_clock_inline(config: &CpuConfig) -> Result<(), ()> 
{
    Ok(())
}

pub fn throttle_sys_pll_clock_inline(config: &CpuConfig) -> Result<(), ()> 
{
    Ok(())
}

#[inline]
pub fn wait_cond<const C: usize>(cond: impl Fn() -> bool) -> Result<(),()> {
    let mut timeout_cycle_count = C;
    while cond() {
        timeout_cycle_count -= 1;
        if timeout_cycle_count <= 0 {
            return Err(());
        }
    }
    
    Ok(())
}
