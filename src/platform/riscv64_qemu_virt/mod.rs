mod boot;

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    runtime_main(cpu_id, dtb);
}

extern "Rust" {
    fn runtime_main(cpu_id: usize, dtb: usize);
}
