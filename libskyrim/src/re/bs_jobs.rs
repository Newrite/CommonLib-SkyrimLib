use core::ffi::c_void;

/// C++ `RE::BSJobs::JobList`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSJobsJobList {
    pub job_list: *mut c_void, // 00
}

const _: () = assert!(core::mem::size_of::<BSJobsJobList>() == 0x8);
