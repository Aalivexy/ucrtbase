#ifdef _M_IX86
__declspec(selectany) unsigned int __security_cookie = 0xBB40E64E;
__declspec(selectany) unsigned int __security_cookie_complement = 0x44BF19B1;
#else
__declspec(selectany) unsigned __int64 __security_cookie = 0xBB40E64EBB40E64E;
__declspec(selectany) unsigned __int64 __security_cookie_complement = 0x44BF19B144BF19B1;
#endif

void __cdecl __telemetry_main_invoke_trigger(void* instance) { (void)instance; }
void __cdecl __telemetry_main_return_trigger(void* instance) { (void)instance; }

#pragma comment(lib, "kernel32")

#ifndef _M_IX86
#pragma comment(linker, "/alternatename:__vcrt_GetModuleFileNameW=GetModuleFileNameW")
#pragma comment(linker, "/alternatename:__vcrt_GetModuleHandleW=GetModuleHandleW")
#pragma comment(linker, "/alternatename:__vcrt_LoadLibraryExW=LoadLibraryExW")
#ifdef _M_ARM64
#pragma comment(linker, "/alternatename:__vcrt_InitializeCriticalSectionEx=InitializeCriticalSectionEx")
#else
#pragma comment(linker, "/alternatename:__vcrt_InitializeCriticalSectionEx=InitializeCriticalSectionAndSpinCount")
#endif
#endif

#ifdef _M_IX86
__declspec(dllimport) unsigned long __stdcall GetModuleFileNameW(void*, unsigned short*, unsigned long);
__declspec(dllimport) void* __stdcall GetModuleHandleW(unsigned short*);
__declspec(dllimport) void* __stdcall LoadLibraryExW(void*, unsigned short*, unsigned long);
__declspec(dllimport) unsigned long __stdcall InitializeCriticalSectionAndSpinCount(void*, unsigned long);

unsigned long __cdecl __vcrt_GetModuleFileNameW(void* h, unsigned short* b, unsigned long s) { return GetModuleFileNameW(h, b, s); }
void* __cdecl __vcrt_GetModuleHandleW(unsigned short* n) { return GetModuleHandleW(n); }
void* __cdecl __vcrt_LoadLibraryExW(void* n, void* f, unsigned long g) { return LoadLibraryExW(n, f, g); }
unsigned long __cdecl __vcrt_InitializeCriticalSectionEx(void* cs, unsigned long sc, unsigned long f) { (void)f; return InitializeCriticalSectionAndSpinCount(cs, sc); }
#endif
