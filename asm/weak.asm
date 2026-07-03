EXTERN GetModuleFileNameW:PROC
EXTERN GetModuleHandleW:PROC
EXTERN LoadLibraryExW:PROC
EXTERN InitializeCriticalSectionEx:PROC
ALIAS <__vcrt_GetModuleFileNameW> = <GetModuleFileNameW>
ALIAS <__vcrt_GetModuleHandleW> = <GetModuleHandleW>
ALIAS <__vcrt_LoadLibraryExW> = <LoadLibraryExW>
ALIAS <__vcrt_InitializeCriticalSectionEx> = <InitializeCriticalSectionEx>
END
