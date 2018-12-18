pub mod virtdisk;
pub mod virtdiskdefs;
pub mod virtdisk_bindings;

pub mod windefs {
    use libc;

    pub type Bool = libc::c_int;
    pub type Boolean = libc::c_uchar;
    pub type Byte = libc::c_uchar;
    pub type DWord = libc::c_ulong;
    pub type Handle = *mut libc::c_void;
    pub type HResult = libc::c_long;
    pub type PCWStr = *const libc::wchar_t;
    pub type PWStr = *mut libc::wchar_t;
    pub type PSId = *mut Void;
    pub type PVoid = *mut Void;
    pub type SecurityDescriptorControl = libc::c_ushort;
    pub type Word = libc::c_ushort;
    pub type Void = libc::c_void;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Guid {
        pub data1: libc::c_ulong,
        pub data2: libc::c_ushort,
        pub data3: libc::c_ushort,
        pub data4: [libc::c_uchar; 8],
    }
}
