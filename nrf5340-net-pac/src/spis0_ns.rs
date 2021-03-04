#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Acquire SPI semaphore"]
    pub tasks_acquire: crate::Reg<tasks_acquire::TASKS_ACQUIRE_SPEC>,
    #[doc = "0x28 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    pub tasks_release: crate::Reg<tasks_release::TASKS_RELEASE_SPEC>,
    _reserved2: [u8; 120usize],
    #[doc = "0xa4 - Subscribe configuration for task ACQUIRE"]
    pub subscribe_acquire: crate::Reg<subscribe_acquire::SUBSCRIBE_ACQUIRE_SPEC>,
    #[doc = "0xa8 - Subscribe configuration for task RELEASE"]
    pub subscribe_release: crate::Reg<subscribe_release::SUBSCRIBE_RELEASE_SPEC>,
    _reserved4: [u8; 88usize],
    #[doc = "0x104 - Granted transaction completed"]
    pub events_end: crate::Reg<events_end::EVENTS_END_SPEC>,
    _reserved5: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>,
    _reserved6: [u8; 20usize],
    #[doc = "0x128 - Semaphore acquired"]
    pub events_acquired: crate::Reg<events_acquired::EVENTS_ACQUIRED_SPEC>,
    _reserved7: [u8; 88usize],
    #[doc = "0x184 - Publish configuration for event END"]
    pub publish_end: crate::Reg<publish_end::PUBLISH_END_SPEC>,
    _reserved8: [u8; 8usize],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: crate::Reg<publish_endrx::PUBLISH_ENDRX_SPEC>,
    _reserved9: [u8; 20usize],
    #[doc = "0x1a8 - Publish configuration for event ACQUIRED"]
    pub publish_acquired: crate::Reg<publish_acquired::PUBLISH_ACQUIRED_SPEC>,
    _reserved10: [u8; 84usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved11: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved13: [u8; 244usize],
    #[doc = "0x400 - Semaphore status register"]
    pub semstat: crate::Reg<semstat::SEMSTAT_SPEC>,
    _reserved14: [u8; 60usize],
    #[doc = "0x440 - Status from last transaction"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved15: [u8; 188usize],
    #[doc = "0x500 - Enable SPI slave"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved17: [u8; 28usize],
    #[doc = "0x534 - Unspecified"]
    pub rxd: RXD,
    #[doc = "0x544 - Unspecified"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved20: [u8; 4usize],
    #[doc = "0x55c - Default character. Character clocked out in case of an ignored transaction."]
    pub def: crate::Reg<def::DEF_SPEC>,
    _reserved21: [u8; 96usize],
    #[doc = "0x5c0 - Over-read character"]
    pub orc: crate::Reg<orc::ORC_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: crate::Reg<self::psel::sck::SCK_SPEC>,
    #[doc = "0x04 - Pin select for MISO signal"]
    pub miso: crate::Reg<self::psel::miso::MISO_SPEC>,
    #[doc = "0x08 - Pin select for MOSI signal"]
    pub mosi: crate::Reg<self::psel::mosi::MOSI_SPEC>,
    #[doc = "0x0c - Pin select for CSN signal"]
    pub csn: crate::Reg<self::psel::csn::CSN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD data pointer"]
    pub ptr: crate::Reg<self::rxd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: crate::Reg<self::rxd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes received in last granted transaction"]
    pub amount: crate::Reg<self::rxd::amount::AMOUNT_SPEC>,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: crate::Reg<self::rxd::list::LIST_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD data pointer"]
    pub ptr: crate::Reg<self::txd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: crate::Reg<self::txd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes transmitted in last granted transaction"]
    pub amount: crate::Reg<self::txd::amount::AMOUNT_SPEC>,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: crate::Reg<self::txd::list::LIST_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "TASKS_ACQUIRE register accessor: an alias for `Reg<TASKS_ACQUIRE_SPEC>`"]
pub type TASKS_ACQUIRE = crate::Reg<tasks_acquire::TASKS_ACQUIRE_SPEC>;
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "TASKS_RELEASE register accessor: an alias for `Reg<TASKS_RELEASE_SPEC>`"]
pub type TASKS_RELEASE = crate::Reg<tasks_release::TASKS_RELEASE_SPEC>;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "SUBSCRIBE_ACQUIRE register accessor: an alias for `Reg<SUBSCRIBE_ACQUIRE_SPEC>`"]
pub type SUBSCRIBE_ACQUIRE = crate::Reg<subscribe_acquire::SUBSCRIBE_ACQUIRE_SPEC>;
#[doc = "Subscribe configuration for task ACQUIRE"]
pub mod subscribe_acquire;
#[doc = "SUBSCRIBE_RELEASE register accessor: an alias for `Reg<SUBSCRIBE_RELEASE_SPEC>`"]
pub type SUBSCRIBE_RELEASE = crate::Reg<subscribe_release::SUBSCRIBE_RELEASE_SPEC>;
#[doc = "Subscribe configuration for task RELEASE"]
pub mod subscribe_release;
#[doc = "EVENTS_END register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Granted transaction completed"]
pub mod events_end;
#[doc = "EVENTS_ENDRX register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_ACQUIRED register accessor: an alias for `Reg<EVENTS_ACQUIRED_SPEC>`"]
pub type EVENTS_ACQUIRED = crate::Reg<events_acquired::EVENTS_ACQUIRED_SPEC>;
#[doc = "Semaphore acquired"]
pub mod events_acquired;
#[doc = "PUBLISH_END register accessor: an alias for `Reg<PUBLISH_END_SPEC>`"]
pub type PUBLISH_END = crate::Reg<publish_end::PUBLISH_END_SPEC>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_ENDRX register accessor: an alias for `Reg<PUBLISH_ENDRX_SPEC>`"]
pub type PUBLISH_ENDRX = crate::Reg<publish_endrx::PUBLISH_ENDRX_SPEC>;
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "PUBLISH_ACQUIRED register accessor: an alias for `Reg<PUBLISH_ACQUIRED_SPEC>`"]
pub type PUBLISH_ACQUIRED = crate::Reg<publish_acquired::PUBLISH_ACQUIRED_SPEC>;
#[doc = "Publish configuration for event ACQUIRED"]
pub mod publish_acquired;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "SEMSTAT register accessor: an alias for `Reg<SEMSTAT_SPEC>`"]
pub type SEMSTAT = crate::Reg<semstat::SEMSTAT_SPEC>;
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPI slave"]
pub mod enable;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "DEF register accessor: an alias for `Reg<DEF_SPEC>`"]
pub type DEF = crate::Reg<def::DEF_SPEC>;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "ORC register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Over-read character"]
pub mod orc;
