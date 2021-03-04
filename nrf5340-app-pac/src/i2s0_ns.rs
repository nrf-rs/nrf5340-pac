#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous I2S transfer. Also starts MCK generator when this is enabled"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stops I2S transfer and MCK generator. Triggering this task will cause the event STOPPED to be generated."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    _reserved4: [u8; 124usize],
    #[doc = "0x104 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
    pub events_rxptrupd: crate::Reg<events_rxptrupd::EVENTS_RXPTRUPD_SPEC>,
    #[doc = "0x108 - I2S transfer stopped."]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved6: [u8; 8usize],
    #[doc = "0x114 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    pub events_txptrupd: crate::Reg<events_txptrupd::EVENTS_TXPTRUPD_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0x11c - Frame start event, generated on the active edge of LRCK"]
    pub events_framestart: crate::Reg<events_framestart::EVENTS_FRAMESTART_SPEC>,
    _reserved8: [u8; 100usize],
    #[doc = "0x184 - Publish configuration for event RXPTRUPD"]
    pub publish_rxptrupd: crate::Reg<publish_rxptrupd::PUBLISH_RXPTRUPD_SPEC>,
    #[doc = "0x188 - Publish configuration for event STOPPED"]
    pub publish_stopped: crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>,
    _reserved10: [u8; 8usize],
    #[doc = "0x194 - Publish configuration for event TXPTRUPD"]
    pub publish_txptrupd: crate::Reg<publish_txptrupd::PUBLISH_TXPTRUPD_SPEC>,
    _reserved11: [u8; 4usize],
    #[doc = "0x19c - Publish configuration for event FRAMESTART"]
    pub publish_framestart: crate::Reg<publish_framestart::PUBLISH_FRAMESTART_SPEC>,
    _reserved12: [u8; 352usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved15: [u8; 500usize],
    #[doc = "0x500 - Enable I2S module"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - Unspecified"]
    pub config: CONFIG,
    _reserved17: [u8; 8usize],
    #[doc = "0x538 - Unspecified"]
    pub rxd: RXD,
    _reserved18: [u8; 4usize],
    #[doc = "0x540 - Unspecified"]
    pub txd: TXD,
    _reserved19: [u8; 12usize],
    #[doc = "0x550 - Unspecified"]
    pub rxtxd: RXTXD,
    _reserved20: [u8; 12usize],
    #[doc = "0x560 - Unspecified"]
    pub psel: PSEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - I2S mode"]
    pub mode: crate::Reg<self::config::mode::MODE_SPEC>,
    #[doc = "0x04 - Reception (RX) enable"]
    pub rxen: crate::Reg<self::config::rxen::RXEN_SPEC>,
    #[doc = "0x08 - Transmission (TX) enable"]
    pub txen: crate::Reg<self::config::txen::TXEN_SPEC>,
    #[doc = "0x0c - Master clock generator enable"]
    pub mcken: crate::Reg<self::config::mcken::MCKEN_SPEC>,
    #[doc = "0x10 - I2S clock generator control"]
    pub mckfreq: crate::Reg<self::config::mckfreq::MCKFREQ_SPEC>,
    #[doc = "0x14 - MCK / LRCK ratio"]
    pub ratio: crate::Reg<self::config::ratio::RATIO_SPEC>,
    #[doc = "0x18 - Sample width"]
    pub swidth: crate::Reg<self::config::swidth::SWIDTH_SPEC>,
    #[doc = "0x1c - Alignment of sample within a frame"]
    pub align: crate::Reg<self::config::align::ALIGN_SPEC>,
    #[doc = "0x20 - Frame format"]
    pub format: crate::Reg<self::config::format::FORMAT_SPEC>,
    #[doc = "0x24 - Enable channels"]
    pub channels: crate::Reg<self::config::channels::CHANNELS_SPEC>,
    #[doc = "0x28 - Clock source selection for the I2S module"]
    pub clkconfig: crate::Reg<self::config::clkconfig::CLKCONFIG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod config;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Receive buffer RAM start address."]
    pub ptr: crate::Reg<self::rxd::ptr::PTR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Transmit buffer RAM start address"]
    pub ptr: crate::Reg<self::txd::ptr::PTR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXTXD {
    #[doc = "0x00 - Size of RXD and TXD buffers"]
    pub maxcnt: crate::Reg<self::rxtxd::maxcnt::MAXCNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for MCK signal"]
    pub mck: crate::Reg<self::psel::mck::MCK_SPEC>,
    #[doc = "0x04 - Pin select for SCK signal"]
    pub sck: crate::Reg<self::psel::sck::SCK_SPEC>,
    #[doc = "0x08 - Pin select for LRCK signal"]
    pub lrck: crate::Reg<self::psel::lrck::LRCK_SPEC>,
    #[doc = "0x0c - Pin select for SDIN signal"]
    pub sdin: crate::Reg<self::psel::sdin::SDIN_SPEC>,
    #[doc = "0x10 - Pin select for SDOUT signal"]
    pub sdout: crate::Reg<self::psel::sdout::SDOUT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops I2S transfer and MCK generator. Triggering this task will cause the event STOPPED to be generated."]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_RXPTRUPD register accessor: an alias for `Reg<EVENTS_RXPTRUPD_SPEC>`"]
pub type EVENTS_RXPTRUPD = crate::Reg<events_rxptrupd::EVENTS_RXPTRUPD_SPEC>;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "EVENTS_TXPTRUPD register accessor: an alias for `Reg<EVENTS_TXPTRUPD_SPEC>`"]
pub type EVENTS_TXPTRUPD = crate::Reg<events_txptrupd::EVENTS_TXPTRUPD_SPEC>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
#[doc = "EVENTS_FRAMESTART register accessor: an alias for `Reg<EVENTS_FRAMESTART_SPEC>`"]
pub type EVENTS_FRAMESTART = crate::Reg<events_framestart::EVENTS_FRAMESTART_SPEC>;
#[doc = "Frame start event, generated on the active edge of LRCK"]
pub mod events_framestart;
#[doc = "PUBLISH_RXPTRUPD register accessor: an alias for `Reg<PUBLISH_RXPTRUPD_SPEC>`"]
pub type PUBLISH_RXPTRUPD = crate::Reg<publish_rxptrupd::PUBLISH_RXPTRUPD_SPEC>;
#[doc = "Publish configuration for event RXPTRUPD"]
pub mod publish_rxptrupd;
#[doc = "PUBLISH_STOPPED register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_TXPTRUPD register accessor: an alias for `Reg<PUBLISH_TXPTRUPD_SPEC>`"]
pub type PUBLISH_TXPTRUPD = crate::Reg<publish_txptrupd::PUBLISH_TXPTRUPD_SPEC>;
#[doc = "Publish configuration for event TXPTRUPD"]
pub mod publish_txptrupd;
#[doc = "PUBLISH_FRAMESTART register accessor: an alias for `Reg<PUBLISH_FRAMESTART_SPEC>`"]
pub type PUBLISH_FRAMESTART = crate::Reg<publish_framestart::PUBLISH_FRAMESTART_SPEC>;
#[doc = "Publish configuration for event FRAMESTART"]
pub mod publish_framestart;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable I2S module"]
pub mod enable;
