#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Unspecified"]
    pub mailbox: MAILBOX,
    _reserved1: [u8; 120usize],
    #[doc = "0x500 - Unspecified"]
    pub eraseprotect: ERASEPROTECT,
    _reserved2: [u8; 56usize],
    #[doc = "0x540 - Unspecified"]
    pub approtect: APPROTECT,
    #[doc = "0x548 - Unspecified"]
    pub secureapprotect: SECUREAPPROTECT,
    _reserved4: [u8; 176usize],
    #[doc = "0x600 - Status bits for CTRL-AP peripheral"]
    pub status: STATUS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MAILBOX {
    #[doc = "0x00 - Data sent from the debugger to the CPU"]
    pub rxdata: self::mailbox::RXDATA,
    #[doc = "0x04 - Status to indicate if data sent from the debugger to the CPU has been read"]
    pub rxstatus: self::mailbox::RXSTATUS,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Data sent from the CPU to the debugger"]
    pub txdata: self::mailbox::TXDATA,
    #[doc = "0x84 - Status to indicate if data sent from the CPU to the debugger has been read"]
    pub txstatus: self::mailbox::TXSTATUS,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = r"Register block"]
#[repr(C)]
pub struct ERASEPROTECT {
    #[doc = "0x00 - Lock register ERASEPROTECT.DISABLE from being written until next reset"]
    pub lock: self::eraseprotect::LOCK,
    #[doc = "0x04 - Disable ERASEPROTECT and perform ERASEALL"]
    pub disable: self::eraseprotect::DISABLE,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod eraseprotect;
#[doc = r"Register block"]
#[repr(C)]
pub struct APPROTECT {
    #[doc = "0x00 - Lock register APPROTECT.DISABLE from being written to until next reset"]
    pub lock: self::approtect::LOCK,
    #[doc = "0x04 - Disable APPROTECT and enable debug access to non-secure mode"]
    pub disable: self::approtect::DISABLE,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod approtect;
#[doc = r"Register block"]
#[repr(C)]
pub struct SECUREAPPROTECT {
    #[doc = "0x00 - Lock register SECUREAPPROTECT.DISABLE from being written until next reset"]
    pub lock: self::secureapprotect::LOCK,
    #[doc = "0x04 - Disable SECUREAPPROTECT and enable debug access to secure mode"]
    pub disable: self::secureapprotect::DISABLE,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod secureapprotect;
#[doc = "Status bits for CTRL-AP peripheral\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status bits for CTRL-AP peripheral"]
pub mod status;
