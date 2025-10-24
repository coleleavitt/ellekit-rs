//! Mach kernel API bindings
//!
//! Low-level bindings to the Mach kernel APIs from <mach/mach.h>,
//! <mach/task.h>, <mach/thread_act.h>, and <mach/vm_map.h>.

use core::ffi::{c_int, c_uint};

// Basic types
pub type natural_t = c_uint;
pub type integer_t = c_int;
pub type boolean_t = c_int;
pub type kern_return_t = c_int;

// Mach port types
pub type mach_port_t = c_uint;
pub type mach_port_name_t = natural_t;
pub type mach_port_right_t = natural_t;

// Task and thread types
pub type task_t = mach_port_t;
pub type task_name_t = mach_port_t;
pub type thread_t = mach_port_t;
pub type thread_act_t = mach_port_t;
pub type thread_state_t = *mut natural_t;

// VM types
pub type vm_map_t = mach_port_t;
pub type vm_task_entry_t = mach_port_t;
pub type vm_address_t = usize;
pub type vm_offset_t = usize;
pub type vm_size_t = usize;
pub type mach_vm_address_t = u64;
pub type mach_vm_offset_t = u64;
pub type mach_vm_size_t = u64;

pub type vm_prot_t = c_int;
pub type vm_inherit_t = c_uint;
pub type vm_behavior_t = c_int;
pub type vm_sync_t = c_uint;
pub type vm_machine_attribute_t = c_uint;
pub type vm_machine_attribute_val_t = c_int;

// Memory protection constants
pub const VM_PROT_NONE: vm_prot_t = 0x00;
pub const VM_PROT_READ: vm_prot_t = 0x01;
pub const VM_PROT_WRITE: vm_prot_t = 0x02;
pub const VM_PROT_EXECUTE: vm_prot_t = 0x04;
pub const VM_PROT_DEFAULT: vm_prot_t = VM_PROT_READ | VM_PROT_WRITE;
pub const VM_PROT_ALL: vm_prot_t = VM_PROT_READ | VM_PROT_WRITE | VM_PROT_EXECUTE;

// VM inheritance
pub const VM_INHERIT_SHARE: vm_inherit_t = 0;
pub const VM_INHERIT_COPY: vm_inherit_t = 1;
pub const VM_INHERIT_NONE: vm_inherit_t = 2;

// Kern return values
pub const KERN_SUCCESS: kern_return_t = 0;
pub const KERN_INVALID_ADDRESS: kern_return_t = 1;
pub const KERN_PROTECTION_FAILURE: kern_return_t = 2;
pub const KERN_NO_SPACE: kern_return_t = 3;
pub const KERN_INVALID_ARGUMENT: kern_return_t = 4;
pub const KERN_FAILURE: kern_return_t = 5;
pub const KERN_RESOURCE_SHORTAGE: kern_return_t = 6;
pub const KERN_NOT_RECEIVER: kern_return_t = 7;
pub const KERN_NO_ACCESS: kern_return_t = 8;
pub const KERN_MEMORY_FAILURE: kern_return_t = 9;
pub const KERN_MEMORY_ERROR: kern_return_t = 10;

// VM flags
pub const VM_FLAGS_FIXED: c_int = 0x0000;
pub const VM_FLAGS_ANYWHERE: c_int = 0x0001;
pub const VM_FLAGS_PURGABLE: c_int = 0x0002;
pub const VM_FLAGS_RANDOM_ADDR: c_int = 0x0008;
pub const VM_FLAGS_OVERWRITE: c_int = 0x4000;

// Mach-O header types
#[repr(C)]
pub struct mach_header {
    pub magic: u32,
    pub cputype: c_int,
    pub cpusubtype: c_int,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct mach_header_64 {
    pub magic: u32,
    pub cputype: c_int,
    pub cpusubtype: c_int,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
    pub reserved: u32,
}

// Mach-O constants
pub const MH_MAGIC: u32 = 0xfeedface;
pub const MH_MAGIC_64: u32 = 0xfeedfacf;
pub const MH_CIGAM: u32 = 0xcefaedfe;
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

// CPU types
pub const CPU_TYPE_ARM: c_int = 12;
pub const CPU_TYPE_ARM64: c_int = 0x0100000c;
pub const CPU_TYPE_ARM64_32: c_int = 0x0200000c;

// Thread state flavor
pub type thread_state_flavor_t = c_int;

#[repr(C)]
pub struct vm_region_basic_info_64 {
    pub protection: vm_prot_t,
    pub max_protection: vm_prot_t,
    pub inheritance: vm_inherit_t,
    pub shared: boolean_t,
    pub reserved: boolean_t,
    pub offset: mach_vm_offset_t,
    pub behavior: vm_behavior_t,
    pub user_wired_count: c_uint,
}

pub type vm_region_info_t = *mut c_int;
pub type vm_region_flavor_t = c_int;
pub type mach_msg_type_number_t = natural_t;

extern "C" {
    // Task operations
    pub fn task_for_pid(
        target_tport: mach_port_t,
        pid: c_int,
        task: *mut task_t,
    ) -> kern_return_t;

    pub fn mach_task_self() -> task_t;

    pub fn task_threads(
        target_task: task_t,
        act_list: *mut *mut thread_act_t,
        act_listCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn task_suspend(target_task: task_t) -> kern_return_t;
    pub fn task_resume(target_task: task_t) -> kern_return_t;

    // Thread operations
    pub fn thread_suspend(target_act: thread_act_t) -> kern_return_t;
    pub fn thread_resume(target_act: thread_act_t) -> kern_return_t;
    pub fn thread_abort(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_get_state(
        target_act: thread_act_t,
        flavor: thread_state_flavor_t,
        old_state: thread_state_t,
        old_stateCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn thread_set_state(
        target_act: thread_act_t,
        flavor: thread_state_flavor_t,
        new_state: thread_state_t,
        new_stateCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    // VM operations
    pub fn mach_vm_allocate(
        target: vm_map_t,
        address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        flags: c_int,
    ) -> kern_return_t;

    pub fn mach_vm_deallocate(
        target: vm_map_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
    ) -> kern_return_t;

    pub fn mach_vm_protect(
        target_task: vm_map_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        set_maximum: boolean_t,
        new_protection: vm_prot_t,
    ) -> kern_return_t;

    pub fn mach_vm_read(
        target_task: vm_map_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        data: *mut vm_offset_t,
        dataCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn mach_vm_write(
        target_task: vm_map_t,
        address: mach_vm_address_t,
        data: vm_offset_t,
        dataCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn mach_vm_region(
        target_task: vm_map_t,
        address: *mut mach_vm_address_t,
        size: *mut mach_vm_size_t,
        flavor: vm_region_flavor_t,
        info: vm_region_info_t,
        infoCnt: *mut mach_msg_type_number_t,
        object_name: *mut mach_port_t,
    ) -> kern_return_t;

    pub fn mach_vm_remap(
        target_task: vm_map_t,
        target_address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        mask: mach_vm_offset_t,
        flags: c_int,
        src_task: vm_map_t,
        src_address: mach_vm_address_t,
        copy: boolean_t,
        cur_protection: *mut vm_prot_t,
        max_protection: *mut vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> kern_return_t;

    pub fn vm_allocate(
        target_task: vm_map_t,
        address: *mut vm_address_t,
        size: vm_size_t,
        flags: c_int,
    ) -> kern_return_t;

    pub fn vm_deallocate(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
    ) -> kern_return_t;

    pub fn vm_protect(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
        set_maximum: boolean_t,
        new_protection: vm_prot_t,
    ) -> kern_return_t;

    pub fn vm_read(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
        data: *mut vm_offset_t,
        dataCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn vm_write(
        target_task: vm_map_t,
        address: vm_address_t,
        data: vm_offset_t,
        dataCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    // Port operations
    pub fn mach_port_allocate(
        task: task_t,
        right: mach_port_right_t,
        name: *mut mach_port_name_t,
    ) -> kern_return_t;

    pub fn mach_port_deallocate(task: task_t, name: mach_port_name_t) -> kern_return_t;
}

// ARM64 thread state
#[repr(C)]
pub struct arm_thread_state64_t {
    pub x: [u64; 29],
    pub fp: u64,
    pub lr: u64,
    pub sp: u64,
    pub pc: u64,
    pub cpsr: u32,
    pub pad: u32,
}

pub const ARM_THREAD_STATE64: thread_state_flavor_t = 6;
pub const ARM_THREAD_STATE64_COUNT: mach_msg_type_number_t = 68;
