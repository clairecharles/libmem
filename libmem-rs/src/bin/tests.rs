use libmem::*;
use std::fmt;
use std::mem;

fn separator() {
    println!("====================");
}

#[no_mangle]
pub extern "C" fn some_function() {

}

fn print_n<T>(vec : Vec<T>, n : usize)
where T : fmt::Display {
    for i in 0..n {
        if i > vec.len() {
            break;
        }

        println!("{}", vec[i]);
    }
}

fn main() {
    println!("[*] libmem-rs tests");

    separator();

    println!("[*] Process Enumeration");
    print_n(LM_EnumProcesses(), 5);

    separator();

    let cur_proc = LM_GetProcess().unwrap();
    println!("[*] Current Process: {}", cur_proc);

    separator();

    let proc = LM_FindProcess("firefox-esr").unwrap();
    println!("[*] Remote Process: {}", proc);

    separator();

    let is_alive = LM_IsProcessAlive(&proc);
    println!("[*] Is the Remote Process Alive? {}", if is_alive { "yes" } else { "no" });

    separator();

    let sysbits = LM_GetSystemBits();
    println!("[*] System Bits: {}", sysbits);

    separator();

    println!("[*] Current Process Threads: {:?}", LM_EnumThreadIds());

    separator();

    println!("[*] Remote Process Threads: {:?}", LM_EnumThreadIdsEx(&proc));

    separator();

    println!("[*] Current Thread ID: {}", LM_GetThreadId());
    println!("[*] Remote Process Thread ID: {}", LM_GetThreadIdEx(&proc));

    separator();

    println!("[*] Current Process - Module Enumeration");
    print_n(LM_EnumModules(), 5);

    separator();

    println!("[*] Remote Process - Module Enumeration");
    print_n(LM_EnumModulesEx(&proc), 5);

    separator();

    let cur_module = LM_FindModule(&cur_proc.get_name()).unwrap();
    println!("[*] Current Process Module: {}", cur_module);

    separator();

    let module = LM_FindModuleEx(&proc, "firefox-esr").unwrap();
    println!("[*] Remote Process Module: {}", module);

    separator();

    let libmodule = LM_LoadModule("/usr/local/lib/libtest.so").unwrap();
    println!("[*] Module Loaded into Current Process: {}", libmodule);

    separator();

    // TODO: Test LM_LoadModuleEx
    
    // separator();

    // Needs internal fixing
    // LM_UnloadModule(&module).unwrap();
    // println!("[*] Unloaded Module from Current Process: {}", module);

    // separator();

    // TODO: Test LM_UnloadModuleEx

    // separator();

    println!("[*] Current Process - Symbol Enumeration");
    println!("[*] Module: {}", cur_module.get_name());
    print_n(LM_EnumSymbols(&cur_module), 5);

    separator();

    let some_function_addr = LM_FindSymbolAddress(&cur_module, "some_function").unwrap();
    println!("[*] Address of 'some_function': {:p}", some_function as *const ());
    println!("[*] Symbol Address Lookup:      {:#x}", some_function_addr);

    separator();

    println!("[*] Current Process - Page Enumeration");
    print_n(LM_EnumPages(), 5);
   
    separator();

    println!("[*] Remote Process - Page Enumeration");
    print_n(LM_EnumPagesEx(&proc), 5);

    separator();

    println!("[*] Current Process - Page at: {:#x}", cur_module.get_base());
    println!("{}", LM_GetPage(cur_module.get_base()).unwrap());

    separator();

    println!("[*] Remote Process - Page at: {:#x}", module.get_base());
    println!("{}", LM_GetPageEx(&proc, module.get_base()).unwrap());

    separator();

    let number : i32 = 1337;
    let number_addr = &number as *const i32 as usize;
    let read_number : i32 = LM_ReadMemory(number_addr).unwrap();
    println!("[*] Number Value: {}", number);
    println!("[*] Read Number Value: {}", read_number);

    separator();

    // TODO: Add tests for LM_ReadMemoryEx

    // separator();

    let value : i32 = 69;
    LM_WriteMemory(number_addr, &value).unwrap();
    println!("[*] Value to write: {}", value);
    println!("[*] Number Value: {}", number);

    separator();

    // TODO: Add tests for LM_WriteMemoryEx

    // separator();

    let buffer : [u8;10] = [0;10];
    println!("[*] Buffer Original: {:?}", buffer);
    LM_SetMemory(buffer.as_ptr() as usize, 255, buffer.len() * mem::size_of::<u8>()).unwrap();
    println!("[*] Buffer After LM_SetMemory: {:?}", buffer);

    separator();

    // TODO: Add tests for LM_SetMemoryEx

    // separator();

    let old_prot = LM_ProtMemory(some_function_addr, 0x1000, LM_PROT_XRW).unwrap();
    println!("[*] Original Protection of '{:#x}': {}", some_function_addr, old_prot);
    let prot = LM_ProtMemory(some_function_addr, 0x1000, old_prot).unwrap();
    println!("[*] Reverted Protection (from '{}' back to '{}')", prot, old_prot);

    separator();

    let old_prot = LM_ProtMemoryEx(&proc, module.get_base(), 0x1000, LM_PROT_XRW).unwrap();
    println!("[*] Remote - Original Protection of '{:#x}': {}", module.get_base(), old_prot);
    let prot = LM_ProtMemoryEx(&proc, module.get_base(), 0x1000, old_prot).unwrap();
    println!("[*] Remote - Reverted Protection (from '{}' back to '{}')", prot, old_prot);
}

