![libmem-logo](LOGO.png)  

#  

# Discord Server
https://discord.com/invite/Qw8jsPD99X

# Note
The master branch often receives great changes. For stability, use a tag/release.

## Usage (C/C++)
Clone the repository:
```
git clone https://github.com/rdbo/libmem
```
Initialize and update the submodules:
```
git submodule init
git submodule update
```
Compile libmem:
```
mkdir build
cd build
cmake ..
make -j 4
```
Add the folder `libmem/include` to your include directories
Add `#include <libmem.h>` (C) or `#include <libmem.hpp>` (C++) to your source code
Link the generated libmem library against your binary (`libmem.a` for Unix-like or `libmem.lib` for Windows).

## Usage (Python)
Make sure to have Python >= 3.6 active  
Either install the `libmem` package from PyPi by running the following command:  
```
pip install --upgrade libmem
```
Or build and install it yourself by running the following commands:
```
cd libmem-py
python configure.py
python setup.py install
```
Now to import libmem, just do the following in your Python code:
```py
from libmem import *
```

## Dependencies
All:
- capstone (included in root project)
- keystone (included in root project)
- LIEF (included in root project)
- libstdc++ (used in keystone and LIEF)
- libmath (used in keystone)

Windows:  
- Windows SDK (-luser32, -lpsapi)  
  
Linux/Android:  
- libdl (-ldl)  
  
BSD:  
- libdl (-ldl)  
- libkvm (-lkvm)
- libprocstat (-lprocstat)    
- libelf (-lelf)

## License
Read `LICENSE`  
  
## Overview
```
LM_EnumProcesses
LM_GetProcessId
LM_GetProcessIdEx
LM_GetParentId
LM_GetParentIdEx
LM_OpenProcess
LM_OpenProcessEx
LM_CloseProcess
LM_GetProcessPath
LM_GetProcessPathEx
LM_GetProcessName
LM_GetProcessNameEx
LM_GetSystemBits
LM_GetProcessBits
LM_GetProcessBitsEx

LM_EnumThreads
LM_EnumThreadsEx
LM_GetThreadId
LM_GetThreadIdEx

LM_EnumModules
LM_EnumModulesEx
LM_GetModule
LM_GetModuleEx
LM_GetModulePath
LM_GetModulePathEx
LM_GetModuleName
LM_GetModuleNameEx
LM_LoadModule
LM_LoadModuleEx
LM_UnloadModule
LM_UnloadModuleEx

LM_EnumSymbols
LM_EnumSymbolsEx
LM_GetSymbol
LM_GetSymbolEx

LM_EnumPages
LM_EnumPagesEx
LM_GetPage
LM_GetPageEx

LM_ReadMemory
LM_ReadMemoryEx
LM_WriteMemory
LM_WriteMemoryEx
LM_SetMemory
LM_SetMemoryEx
LM_ProtMemory
LM_ProtMemoryEx
LM_AllocMemory
LM_AllocMemoryEx
LM_FreeMemory
LM_FreeMemoryEx
LM_DataScan
LM_DataScanEx
LM_PatternScan
LM_PatternScanEx
LM_SigScan
LM_SigScanEx

LM_SystemCall
LM_SystemCallEx
LM_FunctionCall
LM_FunctionCallEx
LM_DetourCode
LM_DetourCodeEx
LM_MakeTrampoline
LM_MakeTrampolineEx
LM_DestroyTrampoline
LM_DestroyTrampolineEx
LM_Assemble
LM_Disassemble

LM_DebugAttach
LM_DebugDetach
LM_DebugCheck
LM_DebugRead
LM_DebugWrite
LM_DebugGetRegs
LM_DebugSetRegs
LM_DebugPickReg
LM_DebugReadReg
LM_DebugWriteReg
LM_DebugContinue
LM_DebugStep
LM_DebugWait
LM_DebugWaitProcess
LM_DebugInject
LM_DebugInjectSingle
```

## Projects
Made with libmem:  
- ![AssaultCube Multihack](https://github.com/rdbo/AssaultCube-Multihack)  
- ![X-Inject](https://github.com/rdbo/x-inject)  
- ![DirectX9 BaseHook](https://github.com/rdbo/DX9-BaseHook)  
- ![DirectX11 BaseHook](https://github.com/rdbo/DX11-BaseHook)  
- ![OpenGL BaseHook](https://github.com/rdbo/GL-BaseHook)  
- ![Counter-Strike 1.6 BaseHook](https://github.com/rdbo/cstrike-basehook)  
- ![Crazymem - NodeJS Memory Library](https://github.com/karliky/Crazymem)  
  
