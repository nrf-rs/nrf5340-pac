#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous PDM transfer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops PDM transfer"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved4: [u8; 120usize],
    #[doc = "0x100 - PDM transfer has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - PDM transfer has finished"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 116usize],
    #[doc = "0x180 - Publish configuration for event STARTED"]
    pub publish_started: PUBLISH_STARTED,
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    #[doc = "0x188 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    _reserved10: [u8; 372usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 500usize],
    #[doc = "0x500 - PDM module enable register"]
    pub enable: ENABLE,
    #[doc = "0x504 - PDM clock generator control"]
    pub pdmclkctrl: PDMCLKCTRL,
    #[doc = "0x508 - Defines the routing of the connected PDM microphones' signals"]
    pub mode: MODE,
    _reserved16: [u8; 12usize],
    #[doc = "0x518 - Left output gain adjustment"]
    pub gainl: GAINL,
    #[doc = "0x51c - Right output gain adjustment"]
    pub gainr: GAINR,
    #[doc = "0x520 - Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
    pub ratio: RATIO,
    _reserved19: [u8; 28usize],
    #[doc = "0x540 - Unspecified"]
    pub psel: PSEL,
    _reserved20: [u8; 4usize],
    #[doc = "0x54c - Master clock generator configuration"]
    pub mclkconfig: MCLKCONFIG,
    _reserved21: [u8; 16usize],
    #[doc = "0x560 - Unspecified"]
    pub sample: SAMPLE,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin number configuration for PDM CLK signal"]
    pub clk: self::psel::CLK,
    #[doc = "0x04 - Pin number configuration for PDM DIN signal"]
    pub din: self::psel::DIN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct SAMPLE {
    #[doc = "0x00 - RAM address pointer to write samples to with EasyDMA"]
    pub ptr: self::sample::PTR,
    #[doc = "0x04 - Number of samples to allocate memory for in EasyDMA mode"]
    pub maxcnt: self::sample::MAXCNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod sample;
#[doc = "Starts continuous PDM transfer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Starts continuous PDM transfer"]
pub mod tasks_start;
#[doc = "Stops PDM transfer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stops PDM transfer"]
pub mod tasks_stop;
#[doc = "Subscribe configuration for task START\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_start](subscribe_start) module"]
pub type SUBSCRIBE_START = crate::Reg<u32, _SUBSCRIBE_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_START;
#[doc = "`read()` method returns [subscribe_start::R](subscribe_start::R) reader structure"]
impl crate::Readable for SUBSCRIBE_START {}
#[doc = "`write(|w| ..)` method takes [subscribe_start::W](subscribe_start::W) writer structure"]
impl crate::Writable for SUBSCRIBE_START {}
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "Subscribe configuration for task STOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stop](subscribe_stop) module"]
pub type SUBSCRIBE_STOP = crate::Reg<u32, _SUBSCRIBE_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOP;
#[doc = "`read()` method returns [subscribe_stop::R](subscribe_stop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_stop::W](subscribe_stop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOP {}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "PDM transfer has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::Writable for EVENTS_STARTED {}
#[doc = "PDM transfer has started"]
pub mod events_started;
#[doc = "PDM transfer has finished\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "PDM transfer has finished"]
pub mod events_stopped;
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
pub mod events_end;
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
#[doc = "Publish configuration for event STOPPED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_stopped](publish_stopped) module"]
pub type PUBLISH_STOPPED = crate::Reg<u32, _PUBLISH_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_STOPPED;
#[doc = "`read()` method returns [publish_stopped::R](publish_stopped::R) reader structure"]
impl crate::Readable for PUBLISH_STOPPED {}
#[doc = "`write(|w| ..)` method takes [publish_stopped::W](publish_stopped::W) writer structure"]
impl crate::Writable for PUBLISH_STOPPED {}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "Publish configuration for event END\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_end](publish_end) module"]
pub type PUBLISH_END = crate::Reg<u32, _PUBLISH_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_END;
#[doc = "`read()` method returns [publish_end::R](publish_end::R) reader structure"]
impl crate::Readable for PUBLISH_END {}
#[doc = "`write(|w| ..)` method takes [publish_end::W](publish_end::W) writer structure"]
impl crate::Writable for PUBLISH_END {}
#[doc = "Publish configuration for event END"]
pub mod publish_end;
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
#[doc = "PDM module enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "PDM module enable register"]
pub mod enable;
#[doc = "PDM clock generator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdmclkctrl](pdmclkctrl) module"]
pub type PDMCLKCTRL = crate::Reg<u32, _PDMCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMCLKCTRL;
#[doc = "`read()` method returns [pdmclkctrl::R](pdmclkctrl::R) reader structure"]
impl crate::Readable for PDMCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [pdmclkctrl::W](pdmclkctrl::W) writer structure"]
impl crate::Writable for PDMCLKCTRL {}
#[doc = "PDM clock generator control"]
pub mod pdmclkctrl;
#[doc = "Defines the routing of the connected PDM microphones' signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Defines the routing of the connected PDM microphones' signals"]
pub mod mode;
#[doc = "Left output gain adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gainl](gainl) module"]
pub type GAINL = crate::Reg<u32, _GAINL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAINL;
#[doc = "`read()` method returns [gainl::R](gainl::R) reader structure"]
impl crate::Readable for GAINL {}
#[doc = "`write(|w| ..)` method takes [gainl::W](gainl::W) writer structure"]
impl crate::Writable for GAINL {}
#[doc = "Left output gain adjustment"]
pub mod gainl;
#[doc = "Right output gain adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gainr](gainr) module"]
pub type GAINR = crate::Reg<u32, _GAINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAINR;
#[doc = "`read()` method returns [gainr::R](gainr::R) reader structure"]
impl crate::Readable for GAINR {}
#[doc = "`write(|w| ..)` method takes [gainr::W](gainr::W) writer structure"]
impl crate::Writable for GAINR {}
#[doc = "Right output gain adjustment"]
pub mod gainr;
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ratio](ratio) module"]
pub type RATIO = crate::Reg<u32, _RATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATIO;
#[doc = "`read()` method returns [ratio::R](ratio::R) reader structure"]
impl crate::Readable for RATIO {}
#[doc = "`write(|w| ..)` method takes [ratio::W](ratio::W) writer structure"]
impl crate::Writable for RATIO {}
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
pub mod ratio;
#[doc = "Master clock generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mclkconfig](mclkconfig) module"]
pub type MCLKCONFIG = crate::Reg<u32, _MCLKCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKCONFIG;
#[doc = "`read()` method returns [mclkconfig::R](mclkconfig::R) reader structure"]
impl crate::Readable for MCLKCONFIG {}
#[doc = "`write(|w| ..)` method takes [mclkconfig::W](mclkconfig::W) writer structure"]
impl crate::Writable for MCLKCONFIG {}
#[doc = "Master clock generator configuration"]
pub mod mclkconfig;
