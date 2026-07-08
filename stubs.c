void __cdecl __telemetry_main_invoke_trigger(void* instance) { (void)instance; }
void __cdecl __telemetry_main_return_trigger(void* instance) { (void)instance; }

#pragma comment(lib, "kernel32")

__declspec(dllimport) unsigned long __stdcall GetModuleFileNameW(void*, unsigned short*, unsigned long);
__declspec(dllimport) void* __stdcall GetModuleHandleW(unsigned short*);
__declspec(dllimport) void* __stdcall LoadLibraryExW(void*, unsigned short*, unsigned long);
__declspec(dllimport) unsigned long __stdcall InitializeCriticalSectionEx(void*, unsigned long, unsigned long);

unsigned long __cdecl __vcrt_GetModuleFileNameW(void* h, unsigned short* b, unsigned long s) { return GetModuleFileNameW(h, b, s); }
void* __cdecl __vcrt_GetModuleHandleW(unsigned short* n) { return GetModuleHandleW(n); }
void* __cdecl __vcrt_LoadLibraryExW(void* n, void* f, unsigned long g) { return LoadLibraryExW(n, f, g); }
unsigned long __cdecl __vcrt_InitializeCriticalSectionEx(void* cs, unsigned long sc, unsigned long f) { return InitializeCriticalSectionEx(cs, sc, f); }

#if defined(_M_AMD64)
void __cdecl _NLG_Notify(void* p1, void* p2, int p3) {
    __declspec(dllimport) void __cdecl __NLG_Dispatch2(void*, void*, int, int);
    __NLG_Dispatch2(p1, p2, p3, 0x19930520);
}
#endif
