//! Platform-specific operations.

cfg_if::cfg_if! {
if #[cfg(all(target_arch = "x86_64", platform_family = "x86-pc"))] {
    mod x86_pc;
} else if #[cfg(all(target_arch = "riscv64", platform_family = "riscv64-qemu-virt"))] {
    mod riscv64_qemu_virt;
} else if #[cfg(all(target_arch = "aarch64", platform_family = "aarch64-qemu-virt"))] {
    mod aarch64_qemu_virt;
} else {
    mod dummy;
    pub use self::dummy::*;
}
}

/// Fills the `.bss` section with zeros.
pub fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(_sbss as usize as *mut u8, _ebss as usize - _sbss as usize)
            .fill(0);
    }
}

extern "C" {
    fn _sbss();
    fn _ebss();
}
