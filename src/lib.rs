#[cfg(all(
    target_env = "msvc",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
mod inner {
    use core::ffi::c_void;

    #[cfg(target_arch = "x86")]
    #[unsafe(no_mangle)]
    pub static __security_cookie: u32 = 0xBB40E64E;

    #[cfg(target_arch = "x86")]
    #[unsafe(no_mangle)]
    pub static __security_cookie_complement: u32 = !0xBB40E64E;

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

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleFileNameW(h: *mut c_void, b: *mut u16, s: u32) -> u32;
        fn GetModuleHandleW(n: *const u16) -> *mut c_void;
        fn LoadLibraryExW(n: *const u16, f: *mut c_void, g: u32) -> *mut c_void;
        fn InitializeCriticalSectionEx(cs: *mut c_void, sc: u32, flags: u32) -> u32;
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __vcrt_GetModuleFileNameW(h: *mut c_void, b: *mut u16, s: u32) -> u32 {
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
