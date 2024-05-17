mod boot;

unsafe extern "C" fn rust_entry(magic: usize, mbi: usize) {
    // TODO: handle multiboot info
    if magic == self::boot::MULTIBOOT_BOOTLOADER_MAGIC {
        super::clear_bss();
        runtime_main(magic, mbi);
    }
    unreachable!();
}

extern "Rust" {
    fn runtime_main(magic: usize, mbi: usize);
}
