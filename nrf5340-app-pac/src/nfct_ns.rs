#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    pub tasks_activate: TASKS_ACTIVATE,
    #[doc = "0x04 - Disable NFCT peripheral"]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x08 - Enable NFC sense field mode, change state to sense mode"]
    pub tasks_sense: TASKS_SENSE,
    #[doc = "0x0c - Start transmission of an outgoing frame, change state to transmit"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - Initializes the EasyDMA for receive."]
    pub tasks_enablerxdata: TASKS_ENABLERXDATA,
    _reserved5: [u8; 4usize],
    #[doc = "0x24 - Force state machine to IDLE state"]
    pub tasks_goidle: TASKS_GOIDLE,
    #[doc = "0x28 - Force state machine to SLEEP_A state"]
    pub tasks_gosleep: TASKS_GOSLEEP,
    _reserved7: [u8; 84usize],
    #[doc = "0x80 - Subscribe configuration for task ACTIVATE"]
    pub subscribe_activate: SUBSCRIBE_ACTIVATE,
    #[doc = "0x84 - Subscribe configuration for task DISABLE"]
    pub subscribe_disable: SUBSCRIBE_DISABLE,
    #[doc = "0x88 - Subscribe configuration for task SENSE"]
    pub subscribe_sense: SUBSCRIBE_SENSE,
    #[doc = "0x8c - Subscribe configuration for task STARTTX"]
    pub subscribe_starttx: SUBSCRIBE_STARTTX,
    _reserved11: [u8; 12usize],
    #[doc = "0x9c - Subscribe configuration for task ENABLERXDATA"]
    pub subscribe_enablerxdata: SUBSCRIBE_ENABLERXDATA,
    _reserved12: [u8; 4usize],
    #[doc = "0xa4 - Subscribe configuration for task GOIDLE"]
    pub subscribe_goidle: SUBSCRIBE_GOIDLE,
    #[doc = "0xa8 - Subscribe configuration for task GOSLEEP"]
    pub subscribe_gosleep: SUBSCRIBE_GOSLEEP,
    _reserved14: [u8; 84usize],
    #[doc = "0x100 - The NFCT peripheral is ready to receive and send frames"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Remote NFC field detected"]
    pub events_fielddetected: EVENTS_FIELDDETECTED,
    #[doc = "0x108 - Remote NFC field lost"]
    pub events_fieldlost: EVENTS_FIELDLOST,
    #[doc = "0x10c - Marks the start of the first symbol of a transmitted frame"]
    pub events_txframestart: EVENTS_TXFRAMESTART,
    #[doc = "0x110 - Marks the end of the last transmitted on-air symbol of a frame"]
    pub events_txframeend: EVENTS_TXFRAMEEND,
    #[doc = "0x114 - Marks the end of the first symbol of a received frame"]
    pub events_rxframestart: EVENTS_RXFRAMESTART,
    #[doc = "0x118 - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    pub events_rxframeend: EVENTS_RXFRAMEEND,
    #[doc = "0x11c - NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
    pub events_error: EVENTS_ERROR,
    _reserved22: [u8; 8usize],
    #[doc = "0x128 - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    pub events_rxerror: EVENTS_RXERROR,
    #[doc = "0x12c - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    pub events_endrx: EVENTS_ENDRX,
    #[doc = "0x130 - Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved25: [u8; 4usize],
    #[doc = "0x138 - Auto collision resolution process has started"]
    pub events_autocolresstarted: EVENTS_AUTOCOLRESSTARTED,
    _reserved26: [u8; 12usize],
    #[doc = "0x148 - NFC auto collision resolution error reported."]
    pub events_collision: EVENTS_COLLISION,
    #[doc = "0x14c - NFC auto collision resolution successfully completed"]
    pub events_selected: EVENTS_SELECTED,
    #[doc = "0x150 - EasyDMA is ready to receive or send frames."]
    pub events_started: EVENTS_STARTED,
    _reserved29: [u8; 44usize],
    #[doc = "0x180 - Publish configuration for event READY"]
    pub publish_ready: PUBLISH_READY,
    #[doc = "0x184 - Publish configuration for event FIELDDETECTED"]
    pub publish_fielddetected: PUBLISH_FIELDDETECTED,
    #[doc = "0x188 - Publish configuration for event FIELDLOST"]
    pub publish_fieldlost: PUBLISH_FIELDLOST,
    #[doc = "0x18c - Publish configuration for event TXFRAMESTART"]
    pub publish_txframestart: PUBLISH_TXFRAMESTART,
    #[doc = "0x190 - Publish configuration for event TXFRAMEEND"]
    pub publish_txframeend: PUBLISH_TXFRAMEEND,
    #[doc = "0x194 - Publish configuration for event RXFRAMESTART"]
    pub publish_rxframestart: PUBLISH_RXFRAMESTART,
    #[doc = "0x198 - Publish configuration for event RXFRAMEEND"]
    pub publish_rxframeend: PUBLISH_RXFRAMEEND,
    #[doc = "0x19c - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved37: [u8; 8usize],
    #[doc = "0x1a8 - Publish configuration for event RXERROR"]
    pub publish_rxerror: PUBLISH_RXERROR,
    #[doc = "0x1ac - Publish configuration for event ENDRX"]
    pub publish_endrx: PUBLISH_ENDRX,
    #[doc = "0x1b0 - Publish configuration for event ENDTX"]
    pub publish_endtx: PUBLISH_ENDTX,
    _reserved40: [u8; 4usize],
    #[doc = "0x1b8 - Publish configuration for event AUTOCOLRESSTARTED"]
    pub publish_autocolresstarted: PUBLISH_AUTOCOLRESSTARTED,
    _reserved41: [u8; 12usize],
    #[doc = "0x1c8 - Publish configuration for event COLLISION"]
    pub publish_collision: PUBLISH_COLLISION,
    #[doc = "0x1cc - Publish configuration for event SELECTED"]
    pub publish_selected: PUBLISH_SELECTED,
    #[doc = "0x1d0 - Publish configuration for event STARTED"]
    pub publish_started: PUBLISH_STARTED,
    _reserved44: [u8; 44usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved45: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved48: [u8; 248usize],
    #[doc = "0x404 - NFC Error Status register"]
    pub errorstatus: ERRORSTATUS,
    _reserved49: [u8; 4usize],
    #[doc = "0x40c - Unspecified"]
    pub framestatus: FRAMESTATUS,
    #[doc = "0x410 - NfcTag state register"]
    pub nfctagstate: NFCTAGSTATE,
    _reserved51: [u8; 12usize],
    #[doc = "0x420 - Sleep state during automatic collision resolution"]
    pub sleepstate: SLEEPSTATE,
    _reserved52: [u8; 24usize],
    #[doc = "0x43c - Indicates the presence or not of a valid field"]
    pub fieldpresent: FIELDPRESENT,
    _reserved53: [u8; 196usize],
    #[doc = "0x504 - Minimum frame delay"]
    pub framedelaymin: FRAMEDELAYMIN,
    #[doc = "0x508 - Maximum frame delay"]
    pub framedelaymax: FRAMEDELAYMAX,
    #[doc = "0x50c - Configuration register for the Frame Delay Timer"]
    pub framedelaymode: FRAMEDELAYMODE,
    #[doc = "0x510 - Packet pointer for TXD and RXD data storage in Data RAM"]
    pub packetptr: PACKETPTR,
    #[doc = "0x514 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    pub maxlen: MAXLEN,
    #[doc = "0x518 - Unspecified"]
    pub txd: TXD,
    #[doc = "0x520 - Unspecified"]
    pub rxd: RXD,
    _reserved60: [u8; 4usize],
    #[doc = "0x52c - Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
    pub modulationctrl: MODULATIONCTRL,
    _reserved61: [u8; 8usize],
    #[doc = "0x538 - Pin select for Modulation control."]
    pub modulationpsel: MODULATIONPSEL,
    _reserved62: [u8; 84usize],
    #[doc = "0x590 - Last NFCID1 part (4, 7 or 10 bytes ID)"]
    pub nfcid1_last: NFCID1_LAST,
    #[doc = "0x594 - Second last NFCID1 part (7 or 10 bytes ID)"]
    pub nfcid1_2nd_last: NFCID1_2ND_LAST,
    #[doc = "0x598 - Third last NFCID1 part (10 bytes ID)"]
    pub nfcid1_3rd_last: NFCID1_3RD_LAST,
    #[doc = "0x59c - Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
    pub autocolresconfig: AUTOCOLRESCONFIG,
    #[doc = "0x5a0 - NFC-A SENS_RES auto-response settings"]
    pub sensres: SENSRES,
    #[doc = "0x5a4 - NFC-A SEL_RES auto-response settings"]
    pub selres: SELRES,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMESTATUS {
    #[doc = "0x00 - Result of last incoming frame"]
    pub rx: self::framestatus::RX,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod framestatus;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Configuration of outgoing frames"]
    pub frameconfig: self::txd::FRAMECONFIG,
    #[doc = "0x04 - Size of outgoing frame"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Configuration of incoming frames"]
    pub frameconfig: self::rxd::FRAMECONFIG,
    #[doc = "0x04 - Size of last incoming frame"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_activate](tasks_activate) module"]
pub type TASKS_ACTIVATE = crate::Reg<u32, _TASKS_ACTIVATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ACTIVATE;
#[doc = "`write(|w| ..)` method takes [tasks_activate::W](tasks_activate::W) writer structure"]
impl crate::Writable for TASKS_ACTIVATE {}
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub mod tasks_activate;
#[doc = "Disable NFCT peripheral\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_disable](tasks_disable) module"]
pub type TASKS_DISABLE = crate::Reg<u32, _TASKS_DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_DISABLE;
#[doc = "`write(|w| ..)` method takes [tasks_disable::W](tasks_disable::W) writer structure"]
impl crate::Writable for TASKS_DISABLE {}
#[doc = "Disable NFCT peripheral"]
pub mod tasks_disable;
#[doc = "Enable NFC sense field mode, change state to sense mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_sense](tasks_sense) module"]
pub type TASKS_SENSE = crate::Reg<u32, _TASKS_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SENSE;
#[doc = "`write(|w| ..)` method takes [tasks_sense::W](tasks_sense::W) writer structure"]
impl crate::Writable for TASKS_SENSE {}
#[doc = "Enable NFC sense field mode, change state to sense mode"]
pub mod tasks_sense;
#[doc = "Start transmission of an outgoing frame, change state to transmit\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_starttx](tasks_starttx) module"]
pub type TASKS_STARTTX = crate::Reg<u32, _TASKS_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTTX;
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](tasks_starttx::W) writer structure"]
impl crate::Writable for TASKS_STARTTX {}
#[doc = "Start transmission of an outgoing frame, change state to transmit"]
pub mod tasks_starttx;
#[doc = "Initializes the EasyDMA for receive.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_enablerxdata](tasks_enablerxdata) module"]
pub type TASKS_ENABLERXDATA = crate::Reg<u32, _TASKS_ENABLERXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ENABLERXDATA;
#[doc = "`write(|w| ..)` method takes [tasks_enablerxdata::W](tasks_enablerxdata::W) writer structure"]
impl crate::Writable for TASKS_ENABLERXDATA {}
#[doc = "Initializes the EasyDMA for receive."]
pub mod tasks_enablerxdata;
#[doc = "Force state machine to IDLE state\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_goidle](tasks_goidle) module"]
pub type TASKS_GOIDLE = crate::Reg<u32, _TASKS_GOIDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_GOIDLE;
#[doc = "`write(|w| ..)` method takes [tasks_goidle::W](tasks_goidle::W) writer structure"]
impl crate::Writable for TASKS_GOIDLE {}
#[doc = "Force state machine to IDLE state"]
pub mod tasks_goidle;
#[doc = "Force state machine to SLEEP_A state\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_gosleep](tasks_gosleep) module"]
pub type TASKS_GOSLEEP = crate::Reg<u32, _TASKS_GOSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_GOSLEEP;
#[doc = "`write(|w| ..)` method takes [tasks_gosleep::W](tasks_gosleep::W) writer structure"]
impl crate::Writable for TASKS_GOSLEEP {}
#[doc = "Force state machine to SLEEP_A state"]
pub mod tasks_gosleep;
#[doc = "Subscribe configuration for task ACTIVATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_activate](subscribe_activate) module"]
pub type SUBSCRIBE_ACTIVATE = crate::Reg<u32, _SUBSCRIBE_ACTIVATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_ACTIVATE;
#[doc = "`read()` method returns [subscribe_activate::R](subscribe_activate::R) reader structure"]
impl crate::Readable for SUBSCRIBE_ACTIVATE {}
#[doc = "`write(|w| ..)` method takes [subscribe_activate::W](subscribe_activate::W) writer structure"]
impl crate::Writable for SUBSCRIBE_ACTIVATE {}
#[doc = "Subscribe configuration for task ACTIVATE"]
pub mod subscribe_activate;
#[doc = "Subscribe configuration for task DISABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_disable](subscribe_disable) module"]
pub type SUBSCRIBE_DISABLE = crate::Reg<u32, _SUBSCRIBE_DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_DISABLE;
#[doc = "`read()` method returns [subscribe_disable::R](subscribe_disable::R) reader structure"]
impl crate::Readable for SUBSCRIBE_DISABLE {}
#[doc = "`write(|w| ..)` method takes [subscribe_disable::W](subscribe_disable::W) writer structure"]
impl crate::Writable for SUBSCRIBE_DISABLE {}
#[doc = "Subscribe configuration for task DISABLE"]
pub mod subscribe_disable;
#[doc = "Subscribe configuration for task SENSE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_sense](subscribe_sense) module"]
pub type SUBSCRIBE_SENSE = crate::Reg<u32, _SUBSCRIBE_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SENSE;
#[doc = "`read()` method returns [subscribe_sense::R](subscribe_sense::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SENSE {}
#[doc = "`write(|w| ..)` method takes [subscribe_sense::W](subscribe_sense::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SENSE {}
#[doc = "Subscribe configuration for task SENSE"]
pub mod subscribe_sense;
#[doc = "Subscribe configuration for task STARTTX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_starttx](subscribe_starttx) module"]
pub type SUBSCRIBE_STARTTX = crate::Reg<u32, _SUBSCRIBE_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STARTTX;
#[doc = "`read()` method returns [subscribe_starttx::R](subscribe_starttx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STARTTX {}
#[doc = "`write(|w| ..)` method takes [subscribe_starttx::W](subscribe_starttx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STARTTX {}
#[doc = "Subscribe configuration for task STARTTX"]
pub mod subscribe_starttx;
#[doc = "Subscribe configuration for task ENABLERXDATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_enablerxdata](subscribe_enablerxdata) module"]
pub type SUBSCRIBE_ENABLERXDATA = crate::Reg<u32, _SUBSCRIBE_ENABLERXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_ENABLERXDATA;
#[doc = "`read()` method returns [subscribe_enablerxdata::R](subscribe_enablerxdata::R) reader structure"]
impl crate::Readable for SUBSCRIBE_ENABLERXDATA {}
#[doc = "`write(|w| ..)` method takes [subscribe_enablerxdata::W](subscribe_enablerxdata::W) writer structure"]
impl crate::Writable for SUBSCRIBE_ENABLERXDATA {}
#[doc = "Subscribe configuration for task ENABLERXDATA"]
pub mod subscribe_enablerxdata;
#[doc = "Subscribe configuration for task GOIDLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_goidle](subscribe_goidle) module"]
pub type SUBSCRIBE_GOIDLE = crate::Reg<u32, _SUBSCRIBE_GOIDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_GOIDLE;
#[doc = "`read()` method returns [subscribe_goidle::R](subscribe_goidle::R) reader structure"]
impl crate::Readable for SUBSCRIBE_GOIDLE {}
#[doc = "`write(|w| ..)` method takes [subscribe_goidle::W](subscribe_goidle::W) writer structure"]
impl crate::Writable for SUBSCRIBE_GOIDLE {}
#[doc = "Subscribe configuration for task GOIDLE"]
pub mod subscribe_goidle;
#[doc = "Subscribe configuration for task GOSLEEP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_gosleep](subscribe_gosleep) module"]
pub type SUBSCRIBE_GOSLEEP = crate::Reg<u32, _SUBSCRIBE_GOSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_GOSLEEP;
#[doc = "`read()` method returns [subscribe_gosleep::R](subscribe_gosleep::R) reader structure"]
impl crate::Readable for SUBSCRIBE_GOSLEEP {}
#[doc = "`write(|w| ..)` method takes [subscribe_gosleep::W](subscribe_gosleep::W) writer structure"]
impl crate::Writable for SUBSCRIBE_GOSLEEP {}
#[doc = "Subscribe configuration for task GOSLEEP"]
pub mod subscribe_gosleep;
#[doc = "The NFCT peripheral is ready to receive and send frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "The NFCT peripheral is ready to receive and send frames"]
pub mod events_ready;
#[doc = "Remote NFC field detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_fielddetected](events_fielddetected) module"]
pub type EVENTS_FIELDDETECTED = crate::Reg<u32, _EVENTS_FIELDDETECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_FIELDDETECTED;
#[doc = "`read()` method returns [events_fielddetected::R](events_fielddetected::R) reader structure"]
impl crate::Readable for EVENTS_FIELDDETECTED {}
#[doc = "`write(|w| ..)` method takes [events_fielddetected::W](events_fielddetected::W) writer structure"]
impl crate::Writable for EVENTS_FIELDDETECTED {}
#[doc = "Remote NFC field detected"]
pub mod events_fielddetected;
#[doc = "Remote NFC field lost\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_fieldlost](events_fieldlost) module"]
pub type EVENTS_FIELDLOST = crate::Reg<u32, _EVENTS_FIELDLOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_FIELDLOST;
#[doc = "`read()` method returns [events_fieldlost::R](events_fieldlost::R) reader structure"]
impl crate::Readable for EVENTS_FIELDLOST {}
#[doc = "`write(|w| ..)` method takes [events_fieldlost::W](events_fieldlost::W) writer structure"]
impl crate::Writable for EVENTS_FIELDLOST {}
#[doc = "Remote NFC field lost"]
pub mod events_fieldlost;
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_txframestart](events_txframestart) module"]
pub type EVENTS_TXFRAMESTART = crate::Reg<u32, _EVENTS_TXFRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXFRAMESTART;
#[doc = "`read()` method returns [events_txframestart::R](events_txframestart::R) reader structure"]
impl crate::Readable for EVENTS_TXFRAMESTART {}
#[doc = "`write(|w| ..)` method takes [events_txframestart::W](events_txframestart::W) writer structure"]
impl crate::Writable for EVENTS_TXFRAMESTART {}
#[doc = "Marks the start of the first symbol of a transmitted frame"]
pub mod events_txframestart;
#[doc = "Marks the end of the last transmitted on-air symbol of a frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_txframeend](events_txframeend) module"]
pub type EVENTS_TXFRAMEEND = crate::Reg<u32, _EVENTS_TXFRAMEEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXFRAMEEND;
#[doc = "`read()` method returns [events_txframeend::R](events_txframeend::R) reader structure"]
impl crate::Readable for EVENTS_TXFRAMEEND {}
#[doc = "`write(|w| ..)` method takes [events_txframeend::W](events_txframeend::W) writer structure"]
impl crate::Writable for EVENTS_TXFRAMEEND {}
#[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
pub mod events_txframeend;
#[doc = "Marks the end of the first symbol of a received frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_rxframestart](events_rxframestart) module"]
pub type EVENTS_RXFRAMESTART = crate::Reg<u32, _EVENTS_RXFRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXFRAMESTART;
#[doc = "`read()` method returns [events_rxframestart::R](events_rxframestart::R) reader structure"]
impl crate::Readable for EVENTS_RXFRAMESTART {}
#[doc = "`write(|w| ..)` method takes [events_rxframestart::W](events_rxframestart::W) writer structure"]
impl crate::Writable for EVENTS_RXFRAMESTART {}
#[doc = "Marks the end of the first symbol of a received frame"]
pub mod events_rxframestart;
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_rxframeend](events_rxframeend) module"]
pub type EVENTS_RXFRAMEEND = crate::Reg<u32, _EVENTS_RXFRAMEEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXFRAMEEND;
#[doc = "`read()` method returns [events_rxframeend::R](events_rxframeend::R) reader structure"]
impl crate::Readable for EVENTS_RXFRAMEEND {}
#[doc = "`write(|w| ..)` method takes [events_rxframeend::W](events_rxframeend::W) writer structure"]
impl crate::Writable for EVENTS_RXFRAMEEND {}
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub mod events_rxframeend;
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
pub mod events_error;
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_rxerror](events_rxerror) module"]
pub type EVENTS_RXERROR = crate::Reg<u32, _EVENTS_RXERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXERROR;
#[doc = "`read()` method returns [events_rxerror::R](events_rxerror::R) reader structure"]
impl crate::Readable for EVENTS_RXERROR {}
#[doc = "`write(|w| ..)` method takes [events_rxerror::W](events_rxerror::W) writer structure"]
impl crate::Writable for EVENTS_RXERROR {}
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub mod events_rxerror;
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endrx](events_endrx) module"]
pub type EVENTS_ENDRX = crate::Reg<u32, _EVENTS_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDRX;
#[doc = "`read()` method returns [events_endrx::R](events_endrx::R) reader structure"]
impl crate::Readable for EVENTS_ENDRX {}
#[doc = "`write(|w| ..)` method takes [events_endrx::W](events_endrx::W) writer structure"]
impl crate::Writable for EVENTS_ENDRX {}
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub mod events_endrx;
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endtx](events_endtx) module"]
pub type EVENTS_ENDTX = crate::Reg<u32, _EVENTS_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDTX;
#[doc = "`read()` method returns [events_endtx::R](events_endtx::R) reader structure"]
impl crate::Readable for EVENTS_ENDTX {}
#[doc = "`write(|w| ..)` method takes [events_endtx::W](events_endtx::W) writer structure"]
impl crate::Writable for EVENTS_ENDTX {}
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
pub mod events_endtx;
#[doc = "Auto collision resolution process has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_autocolresstarted](events_autocolresstarted) module"]
pub type EVENTS_AUTOCOLRESSTARTED = crate::Reg<u32, _EVENTS_AUTOCOLRESSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_AUTOCOLRESSTARTED;
#[doc = "`read()` method returns [events_autocolresstarted::R](events_autocolresstarted::R) reader structure"]
impl crate::Readable for EVENTS_AUTOCOLRESSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_autocolresstarted::W](events_autocolresstarted::W) writer structure"]
impl crate::Writable for EVENTS_AUTOCOLRESSTARTED {}
#[doc = "Auto collision resolution process has started"]
pub mod events_autocolresstarted;
#[doc = "NFC auto collision resolution error reported.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_collision](events_collision) module"]
pub type EVENTS_COLLISION = crate::Reg<u32, _EVENTS_COLLISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_COLLISION;
#[doc = "`read()` method returns [events_collision::R](events_collision::R) reader structure"]
impl crate::Readable for EVENTS_COLLISION {}
#[doc = "`write(|w| ..)` method takes [events_collision::W](events_collision::W) writer structure"]
impl crate::Writable for EVENTS_COLLISION {}
#[doc = "NFC auto collision resolution error reported."]
pub mod events_collision;
#[doc = "NFC auto collision resolution successfully completed\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_selected](events_selected) module"]
pub type EVENTS_SELECTED = crate::Reg<u32, _EVENTS_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SELECTED;
#[doc = "`read()` method returns [events_selected::R](events_selected::R) reader structure"]
impl crate::Readable for EVENTS_SELECTED {}
#[doc = "`write(|w| ..)` method takes [events_selected::W](events_selected::W) writer structure"]
impl crate::Writable for EVENTS_SELECTED {}
#[doc = "NFC auto collision resolution successfully completed"]
pub mod events_selected;
#[doc = "EasyDMA is ready to receive or send frames.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::Writable for EVENTS_STARTED {}
#[doc = "EasyDMA is ready to receive or send frames."]
pub mod events_started;
#[doc = "Publish configuration for event READY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_ready](publish_ready) module"]
pub type PUBLISH_READY = crate::Reg<u32, _PUBLISH_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_READY;
#[doc = "`read()` method returns [publish_ready::R](publish_ready::R) reader structure"]
impl crate::Readable for PUBLISH_READY {}
#[doc = "`write(|w| ..)` method takes [publish_ready::W](publish_ready::W) writer structure"]
impl crate::Writable for PUBLISH_READY {}
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "Publish configuration for event FIELDDETECTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_fielddetected](publish_fielddetected) module"]
pub type PUBLISH_FIELDDETECTED = crate::Reg<u32, _PUBLISH_FIELDDETECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_FIELDDETECTED;
#[doc = "`read()` method returns [publish_fielddetected::R](publish_fielddetected::R) reader structure"]
impl crate::Readable for PUBLISH_FIELDDETECTED {}
#[doc = "`write(|w| ..)` method takes [publish_fielddetected::W](publish_fielddetected::W) writer structure"]
impl crate::Writable for PUBLISH_FIELDDETECTED {}
#[doc = "Publish configuration for event FIELDDETECTED"]
pub mod publish_fielddetected;
#[doc = "Publish configuration for event FIELDLOST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_fieldlost](publish_fieldlost) module"]
pub type PUBLISH_FIELDLOST = crate::Reg<u32, _PUBLISH_FIELDLOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_FIELDLOST;
#[doc = "`read()` method returns [publish_fieldlost::R](publish_fieldlost::R) reader structure"]
impl crate::Readable for PUBLISH_FIELDLOST {}
#[doc = "`write(|w| ..)` method takes [publish_fieldlost::W](publish_fieldlost::W) writer structure"]
impl crate::Writable for PUBLISH_FIELDLOST {}
#[doc = "Publish configuration for event FIELDLOST"]
pub mod publish_fieldlost;
#[doc = "Publish configuration for event TXFRAMESTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_txframestart](publish_txframestart) module"]
pub type PUBLISH_TXFRAMESTART = crate::Reg<u32, _PUBLISH_TXFRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TXFRAMESTART;
#[doc = "`read()` method returns [publish_txframestart::R](publish_txframestart::R) reader structure"]
impl crate::Readable for PUBLISH_TXFRAMESTART {}
#[doc = "`write(|w| ..)` method takes [publish_txframestart::W](publish_txframestart::W) writer structure"]
impl crate::Writable for PUBLISH_TXFRAMESTART {}
#[doc = "Publish configuration for event TXFRAMESTART"]
pub mod publish_txframestart;
#[doc = "Publish configuration for event TXFRAMEEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_txframeend](publish_txframeend) module"]
pub type PUBLISH_TXFRAMEEND = crate::Reg<u32, _PUBLISH_TXFRAMEEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TXFRAMEEND;
#[doc = "`read()` method returns [publish_txframeend::R](publish_txframeend::R) reader structure"]
impl crate::Readable for PUBLISH_TXFRAMEEND {}
#[doc = "`write(|w| ..)` method takes [publish_txframeend::W](publish_txframeend::W) writer structure"]
impl crate::Writable for PUBLISH_TXFRAMEEND {}
#[doc = "Publish configuration for event TXFRAMEEND"]
pub mod publish_txframeend;
#[doc = "Publish configuration for event RXFRAMESTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_rxframestart](publish_rxframestart) module"]
pub type PUBLISH_RXFRAMESTART = crate::Reg<u32, _PUBLISH_RXFRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXFRAMESTART;
#[doc = "`read()` method returns [publish_rxframestart::R](publish_rxframestart::R) reader structure"]
impl crate::Readable for PUBLISH_RXFRAMESTART {}
#[doc = "`write(|w| ..)` method takes [publish_rxframestart::W](publish_rxframestart::W) writer structure"]
impl crate::Writable for PUBLISH_RXFRAMESTART {}
#[doc = "Publish configuration for event RXFRAMESTART"]
pub mod publish_rxframestart;
#[doc = "Publish configuration for event RXFRAMEEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_rxframeend](publish_rxframeend) module"]
pub type PUBLISH_RXFRAMEEND = crate::Reg<u32, _PUBLISH_RXFRAMEEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXFRAMEEND;
#[doc = "`read()` method returns [publish_rxframeend::R](publish_rxframeend::R) reader structure"]
impl crate::Readable for PUBLISH_RXFRAMEEND {}
#[doc = "`write(|w| ..)` method takes [publish_rxframeend::W](publish_rxframeend::W) writer structure"]
impl crate::Writable for PUBLISH_RXFRAMEEND {}
#[doc = "Publish configuration for event RXFRAMEEND"]
pub mod publish_rxframeend;
#[doc = "Publish configuration for event ERROR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_error](publish_error) module"]
pub type PUBLISH_ERROR = crate::Reg<u32, _PUBLISH_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ERROR;
#[doc = "`read()` method returns [publish_error::R](publish_error::R) reader structure"]
impl crate::Readable for PUBLISH_ERROR {}
#[doc = "`write(|w| ..)` method takes [publish_error::W](publish_error::W) writer structure"]
impl crate::Writable for PUBLISH_ERROR {}
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "Publish configuration for event RXERROR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_rxerror](publish_rxerror) module"]
pub type PUBLISH_RXERROR = crate::Reg<u32, _PUBLISH_RXERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXERROR;
#[doc = "`read()` method returns [publish_rxerror::R](publish_rxerror::R) reader structure"]
impl crate::Readable for PUBLISH_RXERROR {}
#[doc = "`write(|w| ..)` method takes [publish_rxerror::W](publish_rxerror::W) writer structure"]
impl crate::Writable for PUBLISH_RXERROR {}
#[doc = "Publish configuration for event RXERROR"]
pub mod publish_rxerror;
#[doc = "Publish configuration for event ENDRX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endrx](publish_endrx) module"]
pub type PUBLISH_ENDRX = crate::Reg<u32, _PUBLISH_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDRX;
#[doc = "`read()` method returns [publish_endrx::R](publish_endrx::R) reader structure"]
impl crate::Readable for PUBLISH_ENDRX {}
#[doc = "`write(|w| ..)` method takes [publish_endrx::W](publish_endrx::W) writer structure"]
impl crate::Writable for PUBLISH_ENDRX {}
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "Publish configuration for event ENDTX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endtx](publish_endtx) module"]
pub type PUBLISH_ENDTX = crate::Reg<u32, _PUBLISH_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDTX;
#[doc = "`read()` method returns [publish_endtx::R](publish_endtx::R) reader structure"]
impl crate::Readable for PUBLISH_ENDTX {}
#[doc = "`write(|w| ..)` method takes [publish_endtx::W](publish_endtx::W) writer structure"]
impl crate::Writable for PUBLISH_ENDTX {}
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "Publish configuration for event AUTOCOLRESSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_autocolresstarted](publish_autocolresstarted) module"]
pub type PUBLISH_AUTOCOLRESSTARTED = crate::Reg<u32, _PUBLISH_AUTOCOLRESSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_AUTOCOLRESSTARTED;
#[doc = "`read()` method returns [publish_autocolresstarted::R](publish_autocolresstarted::R) reader structure"]
impl crate::Readable for PUBLISH_AUTOCOLRESSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_autocolresstarted::W](publish_autocolresstarted::W) writer structure"]
impl crate::Writable for PUBLISH_AUTOCOLRESSTARTED {}
#[doc = "Publish configuration for event AUTOCOLRESSTARTED"]
pub mod publish_autocolresstarted;
#[doc = "Publish configuration for event COLLISION\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_collision](publish_collision) module"]
pub type PUBLISH_COLLISION = crate::Reg<u32, _PUBLISH_COLLISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_COLLISION;
#[doc = "`read()` method returns [publish_collision::R](publish_collision::R) reader structure"]
impl crate::Readable for PUBLISH_COLLISION {}
#[doc = "`write(|w| ..)` method takes [publish_collision::W](publish_collision::W) writer structure"]
impl crate::Writable for PUBLISH_COLLISION {}
#[doc = "Publish configuration for event COLLISION"]
pub mod publish_collision;
#[doc = "Publish configuration for event SELECTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_selected](publish_selected) module"]
pub type PUBLISH_SELECTED = crate::Reg<u32, _PUBLISH_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_SELECTED;
#[doc = "`read()` method returns [publish_selected::R](publish_selected::R) reader structure"]
impl crate::Readable for PUBLISH_SELECTED {}
#[doc = "`write(|w| ..)` method takes [publish_selected::W](publish_selected::W) writer structure"]
impl crate::Writable for PUBLISH_SELECTED {}
#[doc = "Publish configuration for event SELECTED"]
pub mod publish_selected;
#[doc = "Publish configuration for event STARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_started](publish_started) module"]
pub type PUBLISH_STARTED = crate::Reg<u32, _PUBLISH_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_STARTED;
#[doc = "`read()` method returns [publish_started::R](publish_started::R) reader structure"]
impl crate::Readable for PUBLISH_STARTED {}
#[doc = "`write(|w| ..)` method takes [publish_started::W](publish_started::W) writer structure"]
impl crate::Writable for PUBLISH_STARTED {}
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "NFC Error Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [errorstatus](errorstatus) module"]
pub type ERRORSTATUS = crate::Reg<u32, _ERRORSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORSTATUS;
#[doc = "`read()` method returns [errorstatus::R](errorstatus::R) reader structure"]
impl crate::Readable for ERRORSTATUS {}
#[doc = "`write(|w| ..)` method takes [errorstatus::W](errorstatus::W) writer structure"]
impl crate::Writable for ERRORSTATUS {}
#[doc = "NFC Error Status register"]
pub mod errorstatus;
#[doc = "NfcTag state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nfctagstate](nfctagstate) module"]
pub type NFCTAGSTATE = crate::Reg<u32, _NFCTAGSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCTAGSTATE;
#[doc = "`read()` method returns [nfctagstate::R](nfctagstate::R) reader structure"]
impl crate::Readable for NFCTAGSTATE {}
#[doc = "NfcTag state register"]
pub mod nfctagstate;
#[doc = "Sleep state during automatic collision resolution\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sleepstate](sleepstate) module"]
pub type SLEEPSTATE = crate::Reg<u32, _SLEEPSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPSTATE;
#[doc = "`read()` method returns [sleepstate::R](sleepstate::R) reader structure"]
impl crate::Readable for SLEEPSTATE {}
#[doc = "Sleep state during automatic collision resolution"]
pub mod sleepstate;
#[doc = "Indicates the presence or not of a valid field\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fieldpresent](fieldpresent) module"]
pub type FIELDPRESENT = crate::Reg<u32, _FIELDPRESENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIELDPRESENT;
#[doc = "`read()` method returns [fieldpresent::R](fieldpresent::R) reader structure"]
impl crate::Readable for FIELDPRESENT {}
#[doc = "Indicates the presence or not of a valid field"]
pub mod fieldpresent;
#[doc = "Minimum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framedelaymin](framedelaymin) module"]
pub type FRAMEDELAYMIN = crate::Reg<u32, _FRAMEDELAYMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEDELAYMIN;
#[doc = "`read()` method returns [framedelaymin::R](framedelaymin::R) reader structure"]
impl crate::Readable for FRAMEDELAYMIN {}
#[doc = "`write(|w| ..)` method takes [framedelaymin::W](framedelaymin::W) writer structure"]
impl crate::Writable for FRAMEDELAYMIN {}
#[doc = "Minimum frame delay"]
pub mod framedelaymin;
#[doc = "Maximum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framedelaymax](framedelaymax) module"]
pub type FRAMEDELAYMAX = crate::Reg<u32, _FRAMEDELAYMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEDELAYMAX;
#[doc = "`read()` method returns [framedelaymax::R](framedelaymax::R) reader structure"]
impl crate::Readable for FRAMEDELAYMAX {}
#[doc = "`write(|w| ..)` method takes [framedelaymax::W](framedelaymax::W) writer structure"]
impl crate::Writable for FRAMEDELAYMAX {}
#[doc = "Maximum frame delay"]
pub mod framedelaymax;
#[doc = "Configuration register for the Frame Delay Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framedelaymode](framedelaymode) module"]
pub type FRAMEDELAYMODE = crate::Reg<u32, _FRAMEDELAYMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEDELAYMODE;
#[doc = "`read()` method returns [framedelaymode::R](framedelaymode::R) reader structure"]
impl crate::Readable for FRAMEDELAYMODE {}
#[doc = "`write(|w| ..)` method takes [framedelaymode::W](framedelaymode::W) writer structure"]
impl crate::Writable for FRAMEDELAYMODE {}
#[doc = "Configuration register for the Frame Delay Timer"]
pub mod framedelaymode;
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [packetptr](packetptr) module"]
pub type PACKETPTR = crate::Reg<u32, _PACKETPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKETPTR;
#[doc = "`read()` method returns [packetptr::R](packetptr::R) reader structure"]
impl crate::Readable for PACKETPTR {}
#[doc = "`write(|w| ..)` method takes [packetptr::W](packetptr::W) writer structure"]
impl crate::Writable for PACKETPTR {}
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
pub mod packetptr;
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxlen](maxlen) module"]
pub type MAXLEN = crate::Reg<u32, _MAXLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXLEN;
#[doc = "`read()` method returns [maxlen::R](maxlen::R) reader structure"]
impl crate::Readable for MAXLEN {}
#[doc = "`write(|w| ..)` method takes [maxlen::W](maxlen::W) writer structure"]
impl crate::Writable for MAXLEN {}
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub mod maxlen;
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modulationctrl](modulationctrl) module"]
pub type MODULATIONCTRL = crate::Reg<u32, _MODULATIONCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODULATIONCTRL;
#[doc = "`read()` method returns [modulationctrl::R](modulationctrl::R) reader structure"]
impl crate::Readable for MODULATIONCTRL {}
#[doc = "`write(|w| ..)` method takes [modulationctrl::W](modulationctrl::W) writer structure"]
impl crate::Writable for MODULATIONCTRL {}
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
pub mod modulationctrl;
#[doc = "Pin select for Modulation control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modulationpsel](modulationpsel) module"]
pub type MODULATIONPSEL = crate::Reg<u32, _MODULATIONPSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODULATIONPSEL;
#[doc = "`read()` method returns [modulationpsel::R](modulationpsel::R) reader structure"]
impl crate::Readable for MODULATIONPSEL {}
#[doc = "`write(|w| ..)` method takes [modulationpsel::W](modulationpsel::W) writer structure"]
impl crate::Writable for MODULATIONPSEL {}
#[doc = "Pin select for Modulation control."]
pub mod modulationpsel;
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nfcid1_last](nfcid1_last) module"]
pub type NFCID1_LAST = crate::Reg<u32, _NFCID1_LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCID1_LAST;
#[doc = "`read()` method returns [nfcid1_last::R](nfcid1_last::R) reader structure"]
impl crate::Readable for NFCID1_LAST {}
#[doc = "`write(|w| ..)` method takes [nfcid1_last::W](nfcid1_last::W) writer structure"]
impl crate::Writable for NFCID1_LAST {}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub mod nfcid1_last;
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nfcid1_2nd_last](nfcid1_2nd_last) module"]
pub type NFCID1_2ND_LAST = crate::Reg<u32, _NFCID1_2ND_LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCID1_2ND_LAST;
#[doc = "`read()` method returns [nfcid1_2nd_last::R](nfcid1_2nd_last::R) reader structure"]
impl crate::Readable for NFCID1_2ND_LAST {}
#[doc = "`write(|w| ..)` method takes [nfcid1_2nd_last::W](nfcid1_2nd_last::W) writer structure"]
impl crate::Writable for NFCID1_2ND_LAST {}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub mod nfcid1_2nd_last;
#[doc = "Third last NFCID1 part (10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nfcid1_3rd_last](nfcid1_3rd_last) module"]
pub type NFCID1_3RD_LAST = crate::Reg<u32, _NFCID1_3RD_LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCID1_3RD_LAST;
#[doc = "`read()` method returns [nfcid1_3rd_last::R](nfcid1_3rd_last::R) reader structure"]
impl crate::Readable for NFCID1_3RD_LAST {}
#[doc = "`write(|w| ..)` method takes [nfcid1_3rd_last::W](nfcid1_3rd_last::W) writer structure"]
impl crate::Writable for NFCID1_3RD_LAST {}
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub mod nfcid1_3rd_last;
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [autocolresconfig](autocolresconfig) module"]
pub type AUTOCOLRESCONFIG = crate::Reg<u32, _AUTOCOLRESCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOCOLRESCONFIG;
#[doc = "`read()` method returns [autocolresconfig::R](autocolresconfig::R) reader structure"]
impl crate::Readable for AUTOCOLRESCONFIG {}
#[doc = "`write(|w| ..)` method takes [autocolresconfig::W](autocolresconfig::W) writer structure"]
impl crate::Writable for AUTOCOLRESCONFIG {}
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
pub mod autocolresconfig;
#[doc = "NFC-A SENS_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sensres](sensres) module"]
pub type SENSRES = crate::Reg<u32, _SENSRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSRES;
#[doc = "`read()` method returns [sensres::R](sensres::R) reader structure"]
impl crate::Readable for SENSRES {}
#[doc = "`write(|w| ..)` method takes [sensres::W](sensres::W) writer structure"]
impl crate::Writable for SENSRES {}
#[doc = "NFC-A SENS_RES auto-response settings"]
pub mod sensres;
#[doc = "NFC-A SEL_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [selres](selres) module"]
pub type SELRES = crate::Reg<u32, _SELRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELRES;
#[doc = "`read()` method returns [selres::R](selres::R) reader structure"]
impl crate::Readable for SELRES {}
#[doc = "`write(|w| ..)` method takes [selres::W](selres::W) writer structure"]
impl crate::Writable for SELRES {}
#[doc = "NFC-A SEL_RES auto-response settings"]
pub mod selres;
