#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK source"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK source"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK source"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start RCOSC32k calibration"]
    pub tasks_cal: TASKS_CAL,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - Start HFCLKAUDIO source"]
    pub tasks_hfclkaudiostart: TASKS_HFCLKAUDIOSTART,
    #[doc = "0x1c - Stop HFCLKAUDIO source"]
    pub tasks_hfclkaudiostop: TASKS_HFCLKAUDIOSTOP,
    #[doc = "0x20 - Start HFCLK192M source"]
    pub tasks_hfclk192mstart: TASKS_HFCLK192MSTART,
    #[doc = "0x24 - Stop HFCLK192M source"]
    pub tasks_hfclk192mstop: TASKS_HFCLK192MSTOP,
    _reserved9: [u8; 88usize],
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    pub subscribe_hfclkstart: SUBSCRIBE_HFCLKSTART,
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    pub subscribe_hfclkstop: SUBSCRIBE_HFCLKSTOP,
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    pub subscribe_lfclkstart: SUBSCRIBE_LFCLKSTART,
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    pub subscribe_lfclkstop: SUBSCRIBE_LFCLKSTOP,
    #[doc = "0x90 - Subscribe configuration for task CAL"]
    pub subscribe_cal: SUBSCRIBE_CAL,
    _reserved14: [u8; 4usize],
    #[doc = "0x98 - Subscribe configuration for task HFCLKAUDIOSTART"]
    pub subscribe_hfclkaudiostart: SUBSCRIBE_HFCLKAUDIOSTART,
    #[doc = "0x9c - Subscribe configuration for task HFCLKAUDIOSTOP"]
    pub subscribe_hfclkaudiostop: SUBSCRIBE_HFCLKAUDIOSTOP,
    #[doc = "0xa0 - Subscribe configuration for task HFCLK192MSTART"]
    pub subscribe_hfclk192mstart: SUBSCRIBE_HFCLK192MSTART,
    #[doc = "0xa4 - Subscribe configuration for task HFCLK192MSTOP"]
    pub subscribe_hfclk192mstop: SUBSCRIBE_HFCLK192MSTOP,
    _reserved18: [u8; 88usize],
    #[doc = "0x100 - HFCLK source started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK source started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved20: [u8; 20usize],
    #[doc = "0x11c - Calibration of LFCLK RC oscillator complete event"]
    pub events_done: EVENTS_DONE,
    #[doc = "0x120 - HFCLKAUDIO source started"]
    pub events_hfclkaudiostarted: EVENTS_HFCLKAUDIOSTARTED,
    #[doc = "0x124 - HFCLK192M source started"]
    pub events_hfclk192mstarted: EVENTS_HFCLK192MSTARTED,
    _reserved23: [u8; 88usize],
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    pub publish_hfclkstarted: PUBLISH_HFCLKSTARTED,
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    pub publish_lfclkstarted: PUBLISH_LFCLKSTARTED,
    _reserved25: [u8; 20usize],
    #[doc = "0x19c - Publish configuration for event DONE"]
    pub publish_done: PUBLISH_DONE,
    #[doc = "0x1a0 - Publish configuration for event HFCLKAUDIOSTARTED"]
    pub publish_hfclkaudiostarted: PUBLISH_HFCLKAUDIOSTARTED,
    #[doc = "0x1a4 - Publish configuration for event HFCLK192MSTARTED"]
    pub publish_hfclk192mstarted: PUBLISH_HFCLK192MSTARTED,
    _reserved28: [u8; 344usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved32: [u8; 248usize],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - Status indicating which HFCLK source is running Note: Value of this register in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    pub hfclkstat: HFCLKSTAT,
    _reserved34: [u8; 4usize],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - Status indicating which LFCLK source is running Note: Value of this register in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved37: [u8; 48usize],
    #[doc = "0x450 - Status indicating that HFCLKAUDIOSTART task has been triggered"]
    pub hfclkaudiorun: HFCLKAUDIORUN,
    #[doc = "0x454 - Which HFCLKAUDIO source is running"]
    pub hfclkaudiostat: HFCLKAUDIOSTAT,
    #[doc = "0x458 - Status indicating that HFCLK192MSTART task has been triggered"]
    pub hfclk192mrun: HFCLK192MRUN,
    #[doc = "0x45c - Which HFCLK192M source is running"]
    pub hfclk192mstat: HFCLK192MSTAT,
    _reserved41: [u8; 180usize],
    #[doc = "0x514 - Clock source for HFCLK"]
    pub hfclksrc: HFCLKSRC,
    #[doc = "0x518 - Clock source for the LFCLK"]
    pub lfclksrc: LFCLKSRC,
    _reserved43: [u8; 60usize],
    #[doc = "0x558 - HFCLK frequency configuration"]
    pub hfclkctrl: HFCLKCTRL,
    #[doc = "0x55c - Unspecified"]
    pub hfclkaudio: HFCLKAUDIO,
    _reserved45: [u8; 16usize],
    #[doc = "0x570 - Automatic or manual control of HFCLK"]
    pub hfclkalwaysrun: HFCLKALWAYSRUN,
    #[doc = "0x574 - Automatic or manual control of LFCLK"]
    pub lfclkalwaysrun: LFCLKALWAYSRUN,
    _reserved47: [u8; 4usize],
    #[doc = "0x57c - Automatic or manual control of HFCLKAUDIO"]
    pub hfclkaudioalwaysrun: HFCLKAUDIOALWAYSRUN,
    #[doc = "0x580 - Clock source for the HFCLK192M oscillator"]
    pub hfclk192msrc: HFCLK192MSRC,
    #[doc = "0x584 - Automatic or manual control of HFCLK192M"]
    pub hfclk192malwaysrun: HFCLK192MALWAYSRUN,
    _reserved50: [u8; 48usize],
    #[doc = "0x5b8 - HFCLK192M frequency configuration"]
    pub hfclk192mctrl: HFCLK192MCTRL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct HFCLKAUDIO {
    #[doc = "0x00 - Audio PLL frequency in 11.176 MHz - 11.402 MHz or 12.165 MHz - 12.411 MHz frequency bands"]
    pub frequency: self::hfclkaudio::FREQUENCY,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod hfclkaudio;
#[doc = "Start HFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclkstart](tasks_hfclkstart) module"]
pub type TASKS_HFCLKSTART = crate::Reg<u32, _TASKS_HFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKSTART;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstart::W](tasks_hfclkstart::W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTART {}
#[doc = "Start HFCLK source"]
pub mod tasks_hfclkstart;
#[doc = "Stop HFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclkstop](tasks_hfclkstop) module"]
pub type TASKS_HFCLKSTOP = crate::Reg<u32, _TASKS_HFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstop::W](tasks_hfclkstop::W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTOP {}
#[doc = "Stop HFCLK source"]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_lfclkstart](tasks_lfclkstart) module"]
pub type TASKS_LFCLKSTART = crate::Reg<u32, _TASKS_LFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LFCLKSTART;
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstart::W](tasks_lfclkstart::W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTART {}
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_lfclkstop](tasks_lfclkstop) module"]
pub type TASKS_LFCLKSTOP = crate::Reg<u32, _TASKS_LFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LFCLKSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstop::W](tasks_lfclkstop::W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTOP {}
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "Start RCOSC32k calibration\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_cal](tasks_cal) module"]
pub type TASKS_CAL = crate::Reg<u32, _TASKS_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CAL;
#[doc = "`write(|w| ..)` method takes [tasks_cal::W](tasks_cal::W) writer structure"]
impl crate::Writable for TASKS_CAL {}
#[doc = "Start RCOSC32k calibration"]
pub mod tasks_cal;
#[doc = "Start HFCLKAUDIO source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclkaudiostart](tasks_hfclkaudiostart) module"]
pub type TASKS_HFCLKAUDIOSTART = crate::Reg<u32, _TASKS_HFCLKAUDIOSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKAUDIOSTART;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkaudiostart::W](tasks_hfclkaudiostart::W) writer structure"]
impl crate::Writable for TASKS_HFCLKAUDIOSTART {}
#[doc = "Start HFCLKAUDIO source"]
pub mod tasks_hfclkaudiostart;
#[doc = "Stop HFCLKAUDIO source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclkaudiostop](tasks_hfclkaudiostop) module"]
pub type TASKS_HFCLKAUDIOSTOP = crate::Reg<u32, _TASKS_HFCLKAUDIOSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKAUDIOSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkaudiostop::W](tasks_hfclkaudiostop::W) writer structure"]
impl crate::Writable for TASKS_HFCLKAUDIOSTOP {}
#[doc = "Stop HFCLKAUDIO source"]
pub mod tasks_hfclkaudiostop;
#[doc = "Start HFCLK192M source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclk192mstart](tasks_hfclk192mstart) module"]
pub type TASKS_HFCLK192MSTART = crate::Reg<u32, _TASKS_HFCLK192MSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLK192MSTART;
#[doc = "`write(|w| ..)` method takes [tasks_hfclk192mstart::W](tasks_hfclk192mstart::W) writer structure"]
impl crate::Writable for TASKS_HFCLK192MSTART {}
#[doc = "Start HFCLK192M source"]
pub mod tasks_hfclk192mstart;
#[doc = "Stop HFCLK192M source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclk192mstop](tasks_hfclk192mstop) module"]
pub type TASKS_HFCLK192MSTOP = crate::Reg<u32, _TASKS_HFCLK192MSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLK192MSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_hfclk192mstop::W](tasks_hfclk192mstop::W) writer structure"]
impl crate::Writable for TASKS_HFCLK192MSTOP {}
#[doc = "Stop HFCLK192M source"]
pub mod tasks_hfclk192mstop;
#[doc = "Subscribe configuration for task HFCLKSTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclkstart](subscribe_hfclkstart) module"]
pub type SUBSCRIBE_HFCLKSTART = crate::Reg<u32, _SUBSCRIBE_HFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLKSTART;
#[doc = "`read()` method returns [subscribe_hfclkstart::R](subscribe_hfclkstart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLKSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclkstart::W](subscribe_hfclkstart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLKSTART {}
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "Subscribe configuration for task HFCLKSTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclkstop](subscribe_hfclkstop) module"]
pub type SUBSCRIBE_HFCLKSTOP = crate::Reg<u32, _SUBSCRIBE_HFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLKSTOP;
#[doc = "`read()` method returns [subscribe_hfclkstop::R](subscribe_hfclkstop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLKSTOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclkstop::W](subscribe_hfclkstop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLKSTOP {}
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "Subscribe configuration for task LFCLKSTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_lfclkstart](subscribe_lfclkstart) module"]
pub type SUBSCRIBE_LFCLKSTART = crate::Reg<u32, _SUBSCRIBE_LFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_LFCLKSTART;
#[doc = "`read()` method returns [subscribe_lfclkstart::R](subscribe_lfclkstart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_LFCLKSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_lfclkstart::W](subscribe_lfclkstart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_LFCLKSTART {}
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "Subscribe configuration for task LFCLKSTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_lfclkstop](subscribe_lfclkstop) module"]
pub type SUBSCRIBE_LFCLKSTOP = crate::Reg<u32, _SUBSCRIBE_LFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_LFCLKSTOP;
#[doc = "`read()` method returns [subscribe_lfclkstop::R](subscribe_lfclkstop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_LFCLKSTOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_lfclkstop::W](subscribe_lfclkstop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_LFCLKSTOP {}
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "Subscribe configuration for task CAL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_cal](subscribe_cal) module"]
pub type SUBSCRIBE_CAL = crate::Reg<u32, _SUBSCRIBE_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CAL;
#[doc = "`read()` method returns [subscribe_cal::R](subscribe_cal::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CAL {}
#[doc = "`write(|w| ..)` method takes [subscribe_cal::W](subscribe_cal::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CAL {}
#[doc = "Subscribe configuration for task CAL"]
pub mod subscribe_cal;
#[doc = "Subscribe configuration for task HFCLKAUDIOSTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclkaudiostart](subscribe_hfclkaudiostart) module"]
pub type SUBSCRIBE_HFCLKAUDIOSTART = crate::Reg<u32, _SUBSCRIBE_HFCLKAUDIOSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLKAUDIOSTART;
#[doc = "`read()` method returns [subscribe_hfclkaudiostart::R](subscribe_hfclkaudiostart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLKAUDIOSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclkaudiostart::W](subscribe_hfclkaudiostart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLKAUDIOSTART {}
#[doc = "Subscribe configuration for task HFCLKAUDIOSTART"]
pub mod subscribe_hfclkaudiostart;
#[doc = "Subscribe configuration for task HFCLKAUDIOSTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclkaudiostop](subscribe_hfclkaudiostop) module"]
pub type SUBSCRIBE_HFCLKAUDIOSTOP = crate::Reg<u32, _SUBSCRIBE_HFCLKAUDIOSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLKAUDIOSTOP;
#[doc = "`read()` method returns [subscribe_hfclkaudiostop::R](subscribe_hfclkaudiostop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLKAUDIOSTOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclkaudiostop::W](subscribe_hfclkaudiostop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLKAUDIOSTOP {}
#[doc = "Subscribe configuration for task HFCLKAUDIOSTOP"]
pub mod subscribe_hfclkaudiostop;
#[doc = "Subscribe configuration for task HFCLK192MSTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclk192mstart](subscribe_hfclk192mstart) module"]
pub type SUBSCRIBE_HFCLK192MSTART = crate::Reg<u32, _SUBSCRIBE_HFCLK192MSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLK192MSTART;
#[doc = "`read()` method returns [subscribe_hfclk192mstart::R](subscribe_hfclk192mstart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLK192MSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclk192mstart::W](subscribe_hfclk192mstart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLK192MSTART {}
#[doc = "Subscribe configuration for task HFCLK192MSTART"]
pub mod subscribe_hfclk192mstart;
#[doc = "Subscribe configuration for task HFCLK192MSTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclk192mstop](subscribe_hfclk192mstop) module"]
pub type SUBSCRIBE_HFCLK192MSTOP = crate::Reg<u32, _SUBSCRIBE_HFCLK192MSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLK192MSTOP;
#[doc = "`read()` method returns [subscribe_hfclk192mstop::R](subscribe_hfclk192mstop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLK192MSTOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclk192mstop::W](subscribe_hfclk192mstop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLK192MSTOP {}
#[doc = "Subscribe configuration for task HFCLK192MSTOP"]
pub mod subscribe_hfclk192mstop;
#[doc = "HFCLK source started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_hfclkstarted](events_hfclkstarted) module"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<u32, _EVENTS_HFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_HFCLKSTARTED;
#[doc = "`read()` method returns [events_hfclkstarted::R](events_hfclkstarted::R) reader structure"]
impl crate::Readable for EVENTS_HFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_hfclkstarted::W](events_hfclkstarted::W) writer structure"]
impl crate::Writable for EVENTS_HFCLKSTARTED {}
#[doc = "HFCLK source started"]
pub mod events_hfclkstarted;
#[doc = "LFCLK source started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_lfclkstarted](events_lfclkstarted) module"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<u32, _EVENTS_LFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_LFCLKSTARTED;
#[doc = "`read()` method returns [events_lfclkstarted::R](events_lfclkstarted::R) reader structure"]
impl crate::Readable for EVENTS_LFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_lfclkstarted::W](events_lfclkstarted::W) writer structure"]
impl crate::Writable for EVENTS_LFCLKSTARTED {}
#[doc = "LFCLK source started"]
pub mod events_lfclkstarted;
#[doc = "Calibration of LFCLK RC oscillator complete event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_done](events_done) module"]
pub type EVENTS_DONE = crate::Reg<u32, _EVENTS_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DONE;
#[doc = "`read()` method returns [events_done::R](events_done::R) reader structure"]
impl crate::Readable for EVENTS_DONE {}
#[doc = "`write(|w| ..)` method takes [events_done::W](events_done::W) writer structure"]
impl crate::Writable for EVENTS_DONE {}
#[doc = "Calibration of LFCLK RC oscillator complete event"]
pub mod events_done;
#[doc = "HFCLKAUDIO source started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_hfclkaudiostarted](events_hfclkaudiostarted) module"]
pub type EVENTS_HFCLKAUDIOSTARTED = crate::Reg<u32, _EVENTS_HFCLKAUDIOSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_HFCLKAUDIOSTARTED;
#[doc = "`read()` method returns [events_hfclkaudiostarted::R](events_hfclkaudiostarted::R) reader structure"]
impl crate::Readable for EVENTS_HFCLKAUDIOSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_hfclkaudiostarted::W](events_hfclkaudiostarted::W) writer structure"]
impl crate::Writable for EVENTS_HFCLKAUDIOSTARTED {}
#[doc = "HFCLKAUDIO source started"]
pub mod events_hfclkaudiostarted;
#[doc = "HFCLK192M source started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_hfclk192mstarted](events_hfclk192mstarted) module"]
pub type EVENTS_HFCLK192MSTARTED = crate::Reg<u32, _EVENTS_HFCLK192MSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_HFCLK192MSTARTED;
#[doc = "`read()` method returns [events_hfclk192mstarted::R](events_hfclk192mstarted::R) reader structure"]
impl crate::Readable for EVENTS_HFCLK192MSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_hfclk192mstarted::W](events_hfclk192mstarted::W) writer structure"]
impl crate::Writable for EVENTS_HFCLK192MSTARTED {}
#[doc = "HFCLK192M source started"]
pub mod events_hfclk192mstarted;
#[doc = "Publish configuration for event HFCLKSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_hfclkstarted](publish_hfclkstarted) module"]
pub type PUBLISH_HFCLKSTARTED = crate::Reg<u32, _PUBLISH_HFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_HFCLKSTARTED;
#[doc = "`read()` method returns [publish_hfclkstarted::R](publish_hfclkstarted::R) reader structure"]
impl crate::Readable for PUBLISH_HFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_hfclkstarted::W](publish_hfclkstarted::W) writer structure"]
impl crate::Writable for PUBLISH_HFCLKSTARTED {}
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "Publish configuration for event LFCLKSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_lfclkstarted](publish_lfclkstarted) module"]
pub type PUBLISH_LFCLKSTARTED = crate::Reg<u32, _PUBLISH_LFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_LFCLKSTARTED;
#[doc = "`read()` method returns [publish_lfclkstarted::R](publish_lfclkstarted::R) reader structure"]
impl crate::Readable for PUBLISH_LFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_lfclkstarted::W](publish_lfclkstarted::W) writer structure"]
impl crate::Writable for PUBLISH_LFCLKSTARTED {}
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
#[doc = "Publish configuration for event DONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_done](publish_done) module"]
pub type PUBLISH_DONE = crate::Reg<u32, _PUBLISH_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_DONE;
#[doc = "`read()` method returns [publish_done::R](publish_done::R) reader structure"]
impl crate::Readable for PUBLISH_DONE {}
#[doc = "`write(|w| ..)` method takes [publish_done::W](publish_done::W) writer structure"]
impl crate::Writable for PUBLISH_DONE {}
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "Publish configuration for event HFCLKAUDIOSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_hfclkaudiostarted](publish_hfclkaudiostarted) module"]
pub type PUBLISH_HFCLKAUDIOSTARTED = crate::Reg<u32, _PUBLISH_HFCLKAUDIOSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_HFCLKAUDIOSTARTED;
#[doc = "`read()` method returns [publish_hfclkaudiostarted::R](publish_hfclkaudiostarted::R) reader structure"]
impl crate::Readable for PUBLISH_HFCLKAUDIOSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_hfclkaudiostarted::W](publish_hfclkaudiostarted::W) writer structure"]
impl crate::Writable for PUBLISH_HFCLKAUDIOSTARTED {}
#[doc = "Publish configuration for event HFCLKAUDIOSTARTED"]
pub mod publish_hfclkaudiostarted;
#[doc = "Publish configuration for event HFCLK192MSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_hfclk192mstarted](publish_hfclk192mstarted) module"]
pub type PUBLISH_HFCLK192MSTARTED = crate::Reg<u32, _PUBLISH_HFCLK192MSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_HFCLK192MSTARTED;
#[doc = "`read()` method returns [publish_hfclk192mstarted::R](publish_hfclk192mstarted::R) reader structure"]
impl crate::Readable for PUBLISH_HFCLK192MSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_hfclk192mstarted::W](publish_hfclk192mstarted::W) writer structure"]
impl crate::Writable for PUBLISH_HFCLK192MSTARTED {}
#[doc = "Publish configuration for event HFCLK192MSTARTED"]
pub mod publish_hfclk192mstarted;
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
#[doc = "Pending interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u32, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "Status indicating that HFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkrun](hfclkrun) module"]
pub type HFCLKRUN = crate::Reg<u32, _HFCLKRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKRUN;
#[doc = "`read()` method returns [hfclkrun::R](hfclkrun::R) reader structure"]
impl crate::Readable for HFCLKRUN {}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "Status indicating which HFCLK source is running Note: Value of this register in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkstat](hfclkstat) module"]
pub type HFCLKSTAT = crate::Reg<u32, _HFCLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKSTAT;
#[doc = "`read()` method returns [hfclkstat::R](hfclkstat::R) reader structure"]
impl crate::Readable for HFCLKSTAT {}
#[doc = "Status indicating which HFCLK source is running Note: Value of this register in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod hfclkstat;
#[doc = "Status indicating that LFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclkrun](lfclkrun) module"]
pub type LFCLKRUN = crate::Reg<u32, _LFCLKRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKRUN;
#[doc = "`read()` method returns [lfclkrun::R](lfclkrun::R) reader structure"]
impl crate::Readable for LFCLKRUN {}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "Status indicating which LFCLK source is running Note: Value of this register in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclkstat](lfclkstat) module"]
pub type LFCLKSTAT = crate::Reg<u32, _LFCLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSTAT;
#[doc = "`read()` method returns [lfclkstat::R](lfclkstat::R) reader structure"]
impl crate::Readable for LFCLKSTAT {}
#[doc = "Status indicating which LFCLK source is running Note: Value of this register in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
pub mod lfclkstat;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclksrccopy](lfclksrccopy) module"]
pub type LFCLKSRCCOPY = crate::Reg<u32, _LFCLKSRCCOPY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSRCCOPY;
#[doc = "`read()` method returns [lfclksrccopy::R](lfclksrccopy::R) reader structure"]
impl crate::Readable for LFCLKSRCCOPY {}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "Status indicating that HFCLKAUDIOSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkaudiorun](hfclkaudiorun) module"]
pub type HFCLKAUDIORUN = crate::Reg<u32, _HFCLKAUDIORUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKAUDIORUN;
#[doc = "`read()` method returns [hfclkaudiorun::R](hfclkaudiorun::R) reader structure"]
impl crate::Readable for HFCLKAUDIORUN {}
#[doc = "Status indicating that HFCLKAUDIOSTART task has been triggered"]
pub mod hfclkaudiorun;
#[doc = "Which HFCLKAUDIO source is running\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkaudiostat](hfclkaudiostat) module"]
pub type HFCLKAUDIOSTAT = crate::Reg<u32, _HFCLKAUDIOSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKAUDIOSTAT;
#[doc = "`read()` method returns [hfclkaudiostat::R](hfclkaudiostat::R) reader structure"]
impl crate::Readable for HFCLKAUDIOSTAT {}
#[doc = "Which HFCLKAUDIO source is running"]
pub mod hfclkaudiostat;
#[doc = "Status indicating that HFCLK192MSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclk192mrun](hfclk192mrun) module"]
pub type HFCLK192MRUN = crate::Reg<u32, _HFCLK192MRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLK192MRUN;
#[doc = "`read()` method returns [hfclk192mrun::R](hfclk192mrun::R) reader structure"]
impl crate::Readable for HFCLK192MRUN {}
#[doc = "Status indicating that HFCLK192MSTART task has been triggered"]
pub mod hfclk192mrun;
#[doc = "Which HFCLK192M source is running\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclk192mstat](hfclk192mstat) module"]
pub type HFCLK192MSTAT = crate::Reg<u32, _HFCLK192MSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLK192MSTAT;
#[doc = "`read()` method returns [hfclk192mstat::R](hfclk192mstat::R) reader structure"]
impl crate::Readable for HFCLK192MSTAT {}
#[doc = "Which HFCLK192M source is running"]
pub mod hfclk192mstat;
#[doc = "Clock source for HFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclksrc](hfclksrc) module"]
pub type HFCLKSRC = crate::Reg<u32, _HFCLKSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKSRC;
#[doc = "`read()` method returns [hfclksrc::R](hfclksrc::R) reader structure"]
impl crate::Readable for HFCLKSRC {}
#[doc = "`write(|w| ..)` method takes [hfclksrc::W](hfclksrc::W) writer structure"]
impl crate::Writable for HFCLKSRC {}
#[doc = "Clock source for HFCLK"]
pub mod hfclksrc;
#[doc = "Clock source for the LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclksrc](lfclksrc) module"]
pub type LFCLKSRC = crate::Reg<u32, _LFCLKSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSRC;
#[doc = "`read()` method returns [lfclksrc::R](lfclksrc::R) reader structure"]
impl crate::Readable for LFCLKSRC {}
#[doc = "`write(|w| ..)` method takes [lfclksrc::W](lfclksrc::W) writer structure"]
impl crate::Writable for LFCLKSRC {}
#[doc = "Clock source for the LFCLK"]
pub mod lfclksrc;
#[doc = "HFCLK frequency configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkctrl](hfclkctrl) module"]
pub type HFCLKCTRL = crate::Reg<u32, _HFCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKCTRL;
#[doc = "`read()` method returns [hfclkctrl::R](hfclkctrl::R) reader structure"]
impl crate::Readable for HFCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [hfclkctrl::W](hfclkctrl::W) writer structure"]
impl crate::Writable for HFCLKCTRL {}
#[doc = "HFCLK frequency configuration"]
pub mod hfclkctrl;
#[doc = "Automatic or manual control of HFCLK\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkalwaysrun](hfclkalwaysrun) module"]
pub type HFCLKALWAYSRUN = crate::Reg<u32, _HFCLKALWAYSRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKALWAYSRUN;
#[doc = "`read()` method returns [hfclkalwaysrun::R](hfclkalwaysrun::R) reader structure"]
impl crate::Readable for HFCLKALWAYSRUN {}
#[doc = "Automatic or manual control of HFCLK"]
pub mod hfclkalwaysrun;
#[doc = "Automatic or manual control of LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclkalwaysrun](lfclkalwaysrun) module"]
pub type LFCLKALWAYSRUN = crate::Reg<u32, _LFCLKALWAYSRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKALWAYSRUN;
#[doc = "`read()` method returns [lfclkalwaysrun::R](lfclkalwaysrun::R) reader structure"]
impl crate::Readable for LFCLKALWAYSRUN {}
#[doc = "Automatic or manual control of LFCLK"]
pub mod lfclkalwaysrun;
#[doc = "Automatic or manual control of HFCLKAUDIO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkaudioalwaysrun](hfclkaudioalwaysrun) module"]
pub type HFCLKAUDIOALWAYSRUN = crate::Reg<u32, _HFCLKAUDIOALWAYSRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKAUDIOALWAYSRUN;
#[doc = "`read()` method returns [hfclkaudioalwaysrun::R](hfclkaudioalwaysrun::R) reader structure"]
impl crate::Readable for HFCLKAUDIOALWAYSRUN {}
#[doc = "Automatic or manual control of HFCLKAUDIO"]
pub mod hfclkaudioalwaysrun;
#[doc = "Clock source for the HFCLK192M oscillator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclk192msrc](hfclk192msrc) module"]
pub type HFCLK192MSRC = crate::Reg<u32, _HFCLK192MSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLK192MSRC;
#[doc = "`read()` method returns [hfclk192msrc::R](hfclk192msrc::R) reader structure"]
impl crate::Readable for HFCLK192MSRC {}
#[doc = "`write(|w| ..)` method takes [hfclk192msrc::W](hfclk192msrc::W) writer structure"]
impl crate::Writable for HFCLK192MSRC {}
#[doc = "Clock source for the HFCLK192M oscillator"]
pub mod hfclk192msrc;
#[doc = "Automatic or manual control of HFCLK192M\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclk192malwaysrun](hfclk192malwaysrun) module"]
pub type HFCLK192MALWAYSRUN = crate::Reg<u32, _HFCLK192MALWAYSRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLK192MALWAYSRUN;
#[doc = "`read()` method returns [hfclk192malwaysrun::R](hfclk192malwaysrun::R) reader structure"]
impl crate::Readable for HFCLK192MALWAYSRUN {}
#[doc = "Automatic or manual control of HFCLK192M"]
pub mod hfclk192malwaysrun;
#[doc = "HFCLK192M frequency configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclk192mctrl](hfclk192mctrl) module"]
pub type HFCLK192MCTRL = crate::Reg<u32, _HFCLK192MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLK192MCTRL;
#[doc = "`read()` method returns [hfclk192mctrl::R](hfclk192mctrl::R) reader structure"]
impl crate::Readable for HFCLK192MCTRL {}
#[doc = "`write(|w| ..)` method takes [hfclk192mctrl::W](hfclk192mctrl::W) writer structure"]
impl crate::Writable for HFCLK192MCTRL {}
#[doc = "HFCLK192M frequency configuration"]
pub mod hfclk192mctrl;
