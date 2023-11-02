use tc37x_rt::{
    asm_calls::read_cpu_core_id,
    isr::load_interrupt_table,
    post_init, pre_init,
    wdtcon::{disable_cpu_watchdog, disable_safety_watchdog},
};

pre_init!(pre_init_fn);
fn pre_init_fn() {
    #[cfg(feature = "disable_watchdogs")]
    disable_watchdogs();
}

post_init!(post_init_fn);
fn post_init_fn() {
    load_interrupt_table();
}

fn disable_watchdogs() {
    if read_cpu_core_id() == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}
