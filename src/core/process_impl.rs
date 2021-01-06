use std::mem::size_of;
use std::mem::transmute;
use std::str::from_utf8;
use winapi::shared::minwindef::{DWORD, HMODULE};
use winapi::shared::ntdef::NULL;
use winapi::shared::{basetsd::SIZE_T, minwindef::LPCVOID};
use winapi::shared::{minwindef::FALSE, ntdef::HANDLE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleFileNameExW;
use winapi::um::tlhelp32::CreateToolhelp32Snapshot;
use winapi::um::tlhelp32::Process32Next;
use winapi::um::tlhelp32::PROCESSENTRY32;
use winapi::um::tlhelp32::TH32CS_SNAPPROCESS;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::{shared::minwindef::MAX_PATH, um::psapi::GetModuleBaseNameW};
use winapi::{
    shared::minwindef::{LPVOID, TRUE},
    um::psapi::{EnumProcessModulesEx, GetModuleInformation, LIST_MODULES_ALL, MODULEINFO},
};

pub struct Process {
    process: HANDLE,
    // old_protect: DWORD,
}

unsafe impl Send for Process {}

unsafe impl Sync for Process {}

impl Process {
    pub fn find(exe_file: &str) -> Option<Process> {
        unsafe {
            let process_id = find_process_id(exe_file);
            if process_id == 0 {
                None
            } else {
                Some(Process {
                    process: OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id),
                    // old_protect: 0,
                })
            }
        }
    }

    pub fn path(&self) -> String {
        let mut buf = [0u16; MAX_PATH];
        unsafe {
            GetModuleFileNameExW(
                self.process,
                NULL as HMODULE,
                buf.as_mut_ptr(),
                MAX_PATH as u32,
            ) == 0
        };
        String::from_utf16_lossy(&buf).into()
    }

    pub fn base_addr_of_module_name(&self, module_name: &str) -> Option<u32> {
        unsafe { module_infos(self.process) }
            .into_iter()
            .find(|(name, _)| name == module_name)
            .map(|(_, base_addr)| base_addr)
    }

    pub fn read_u32(&self, address: u32) -> u32 {
        const SIZE: usize = 4;
        let mut buf: [u8; SIZE] = Default::default();
        unsafe {
            read_process_memory(self.process, address, &mut buf);
            transmute::<[u8; SIZE], u32>(buf).to_le()
        }
    }

    pub fn read_i32(&self, address: u32) -> i32 {
        const SIZE: usize = 4;
        let mut buf: [u8; SIZE] = Default::default();
        unsafe {
            read_process_memory(self.process, address, &mut buf);
            transmute::<[u8; SIZE], i32>(buf).to_le()
        }
    }

    // pub fn read_u16(&self, address: u32) -> u16 {
    //     const SIZE: usize = 2;
    //     let mut buf: [u8; SIZE] = Default::default();
    //     unsafe {
    //         read_process_memory(self.process, address, &mut buf);
    //         transmute::<[u8; SIZE], u16>(buf).to_le()
    //     }
    // }

    pub fn read_u8(&self, address: u32) -> u8 {
        const SIZE: usize = 1;
        let mut buf: [u8; SIZE] = Default::default();
        unsafe {
            read_process_memory(self.process, address, &mut buf);
        }
        return buf[0];
    }

    pub fn read_f32(&self, address: u32) -> f32 {
        const SIZE: usize = 4;
        let mut buf: [u8; SIZE] = Default::default();
        unsafe {
            read_process_memory(self.process, address, &mut buf);
            transmute::<[u8; SIZE], f32>(buf)
        }
    }

    pub fn write_u8(&self, address: u32, value: u8) {
        let buf = value.to_le_bytes();
        unsafe {
            write_process_memory(self.process, address, &buf);
        }
    }

    // pub fn write_u32(&self, address: u32, value: u32) {
    //     let buf = value.to_le_bytes();
    //     unsafe {
    //         write_process_memory(self.process, address, &buf);
    //     }
    // }

    pub fn write_i32(&self, address: u32, value: i32) {
        let buf = value.to_le_bytes();
        unsafe {
            write_process_memory(self.process, address, &buf);
        }
    }

    pub fn write_f32(&self, address: u32, value: f32) {
        let buf = value.to_le_bytes();
        unsafe {
            write_process_memory(self.process, address, &buf);
        }
    }

    // pub fn write(&self, address: u32, buf: &[u8]) {
    //     unsafe {
    //         write_process_memory(self.process, address, buf);
    //     }
    // }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.process);
        }
    }
}

unsafe fn read_process_memory(process: HANDLE, address: u32, buf: &mut [u8]) {
    let number_of_bytes_read: DWORD = 0;
    ReadProcessMemory(
        process,
        address as LPCVOID,
        &mut buf[0] as *mut u8 as LPVOID,
        buf.len(),
        &mut (number_of_bytes_read as SIZE_T),
    );
}

unsafe fn write_process_memory(process: HANDLE, address: u32, buf: &[u8]) {
    let number_of_bytes_read: DWORD = 0;
    WriteProcessMemory(
        process,
        address as LPVOID,
        &buf[0] as *const u8 as LPVOID,
        buf.len(),
        &mut (number_of_bytes_read as SIZE_T),
    );
}

// unsafe fn virtual_protect(process: HANDLE, address: u32, size: usize, new_protect: DWORD) -> DWORD {
//     let mut old: DWORD = 0;
//     VirtualProtectEx(process, address as LPVOID, size, new_protect, &mut old);
//     old
// }

// http://peryaudo.hatenablog.com/entry/20100516/1273998518
// https://github.com/retep998/winapi-rs/issues/849
unsafe fn find_process_id(exe_file: &str) -> DWORD {
    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    if snapshot == INVALID_HANDLE_VALUE {
        panic!("INVALID_HANDLE_VALUE");
    }

    let mut pe = PROCESSENTRY32 {
        dwSize: size_of::<PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; 260],
    };
    let mut process_id: DWORD = 0;
    while Process32Next(snapshot, &mut pe) != 0 {
        let current = from_utf8(transmute::<&[i8], &[u8]>(&pe.szExeFile)).unwrap();
        if !current.contains(exe_file) {
            continue;
        }
        process_id = pe.th32ProcessID;
        break;
    }
    CloseHandle(snapshot);
    process_id
}

unsafe fn module_infos(process: HANDLE) -> Vec<(String, u32)> {
    let mut modules: [HMODULE; 1024] = [0 as HMODULE; 1024];
    let mut cb_needed: DWORD = 0;
    if EnumProcessModulesEx(
        process,
        modules.as_mut_ptr(),
        modules.len() as DWORD,
        &mut cb_needed,
        LIST_MODULES_ALL,
    ) != TRUE
    {
        panic!();
    }
    let module_num = cb_needed / size_of::<HMODULE>() as u32;
    (0..module_num as usize)
        .map(move |i| {
            let base_name = {
                let mut base_name = [0u16; 1024];
                let base_name_size = GetModuleBaseNameW(
                    process,
                    modules[i],
                    base_name.as_mut_ptr(),
                    base_name.len() as DWORD,
                ) as usize;
                String::from_utf16_lossy(&base_name[0..base_name_size])
            };
            let mut module_info: MODULEINFO = Default::default();
            if GetModuleInformation(
                process,
                modules[i],
                &mut module_info,
                size_of::<MODULEINFO> as DWORD,
            ) != TRUE
            {
                panic!();
            }
            (base_name, module_info.lpBaseOfDll as u32)
        })
        .collect()
}
