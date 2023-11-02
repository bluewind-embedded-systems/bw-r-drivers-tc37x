use tc37x_rt::{
    asm_calls::read_cpu_core_id,
    isr::load_interrupt_table,
    wdtcon::{disable_cpu_watchdog, disable_safety_watchdog},
    *,
};

pre_init!(pre_init_fn);
#[allow(unused)]
fn pre_init_fn() {
    if read_cpu_core_id() == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}

post_init!(post_init_fn);
#[allow(unused)]
fn post_init_fn() {
    load_interrupt_table();
}
