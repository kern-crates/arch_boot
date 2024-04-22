use top_early_console::init;

mod boot;

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    init(cpu_id, dtb);
}
