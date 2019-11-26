#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - Unspecified"]
    pub acl0: ACL,
    _reserved1: [u8; 4usize],
    #[doc = "0x810 - Unspecified"]
    pub acl1: ACL,
    _reserved2: [u8; 4usize],
    #[doc = "0x820 - Unspecified"]
    pub acl2: ACL,
    _reserved3: [u8; 4usize],
    #[doc = "0x830 - Unspecified"]
    pub acl3: ACL,
    _reserved4: [u8; 4usize],
    #[doc = "0x840 - Unspecified"]
    pub acl4: ACL,
    _reserved5: [u8; 4usize],
    #[doc = "0x850 - Unspecified"]
    pub acl5: ACL,
    _reserved6: [u8; 4usize],
    #[doc = "0x860 - Unspecified"]
    pub acl6: ACL,
    _reserved7: [u8; 4usize],
    #[doc = "0x870 - Unspecified"]
    pub acl7: ACL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct ACL {
    #[doc = "0x00 - Description cluster: Configure the word-aligned start address of region n to protect"]
    pub addr: self::acl::ADDR,
    #[doc = "0x04 - Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect."]
    pub size: self::acl::SIZE,
    #[doc = "0x08 - Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    pub perm: self::acl::PERM,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod acl;
