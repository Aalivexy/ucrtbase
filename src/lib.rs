//! Link Rust programs to the system `ucrtbase.dll` and eliminate the MSVC redist dependency.
//!
//! This crate automatically configures the linker to target the `ucrtbase.dll` shipped with
//! Windows 10+, removing the need for MSVC redistributable packages.
//!
//! # Supported targets
//!
//! - `x86_64-pc-windows-msvc`
//! - `i686-pc-windows-msvc`
//! - `aarch64-pc-windows-msvc`

#[cfg(all(
    target_env = "msvc",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
mod inner {
    use core::ffi::c_void;

    #[cfg(not(target_arch = "x86"))]
    #[unsafe(no_mangle)]
    pub static __security_cookie: u64 = 0xBB40E64EBB40E64E;

    #[cfg(not(target_arch = "x86"))]
    #[unsafe(no_mangle)]
    pub static __security_cookie_complement: u64 = !0xBB40E64EBB40E64E;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __telemetry_main_invoke_trigger(_: *mut c_void) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __telemetry_main_return_trigger(_: *mut c_void) {}

    #[cfg(target_arch = "x86_64")]
    mod inner_x86_64 {
        use core::ffi::c_void;

        unsafe extern "system" {
            fn __NLG_Dispatch2(p1: *mut c_void, p2: *mut c_void, p3: i32, p4: i32);
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn _NLG_Notify(p1: *mut c_void, p2: *mut c_void, p3: i32) {
            unsafe {
                __NLG_Dispatch2(p1, p2, p3, 0x19930520);
            }
        }
    }

    #[cfg(target_arch = "x86")]
    mod inner_x86 {
        use core::ffi::c_void;

        #[unsafe(no_mangle)]
        pub static __security_cookie: u32 = 0xBB40E64E;

        #[unsafe(no_mangle)]
        pub static __security_cookie_complement: u32 = !0xBB40E64E;

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetModuleFileNameW(h: *mut c_void, b: *mut u16, s: u32) -> u32;
            fn GetModuleHandleW(n: *const u16) -> *mut c_void;
            fn LoadLibraryExW(n: *const u16, f: *mut c_void, g: u32) -> *mut c_void;
            fn InitializeCriticalSectionEx(cs: *mut c_void, sc: u32, flags: u32) -> u32;
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __vcrt_GetModuleFileNameW(
            h: *mut c_void,
            b: *mut u16,
            s: u32,
        ) -> u32 {
            unsafe { GetModuleFileNameW(h, b, s) }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __vcrt_GetModuleHandleW(n: *const u16) -> *mut c_void {
            unsafe { GetModuleHandleW(n) }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __vcrt_LoadLibraryExW(
            n: *const u16,
            f: *mut c_void,
            g: u32,
        ) -> *mut c_void {
            unsafe { LoadLibraryExW(n, f, g) }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __vcrt_InitializeCriticalSectionEx(
            cs: *mut c_void,
            sc: u32,
            flags: u32,
        ) -> u32 {
            unsafe { InitializeCriticalSectionEx(cs, sc, flags) }
        }
    }
}
