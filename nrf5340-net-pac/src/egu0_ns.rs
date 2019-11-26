#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\] event"]
    pub tasks_trigger: [TASKS_TRIGGER; 16],
    _reserved1: [u8; 64usize],
    #[doc = "0x80 - Description collection: Subscribe configuration for task TRIGGER\\[n\\]"]
    pub subscribe_trigger: [SUBSCRIBE_TRIGGER; 16],
    _reserved2: [u8; 64usize],
    #[doc = "0x100 - Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\] task"]
    pub events_triggered: [EVENTS_TRIGGERED; 16],
    _reserved3: [u8; 64usize],
    #[doc = "0x180 - Description collection: Publish configuration for event TRIGGERED\\[n\\]"]
    pub publish_triggered: [PUBLISH_TRIGGERED; 16],
    _reserved4: [u8; 320usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
}
#[doc = "Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\] event\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_trigger](tasks_trigger) module"]
pub type TASKS_TRIGGER = crate::Reg<u32, _TASKS_TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_TRIGGER;
#[doc = "`write(|w| ..)` method takes [tasks_trigger::W](tasks_trigger::W) writer structure"]
impl crate::Writable for TASKS_TRIGGER {}
#[doc = "Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\] event"]
pub mod tasks_trigger;
#[doc = "Description collection: Subscribe configuration for task TRIGGER\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_trigger](subscribe_trigger) module"]
pub type SUBSCRIBE_TRIGGER = crate::Reg<u32, _SUBSCRIBE_TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_TRIGGER;
#[doc = "`read()` method returns [subscribe_trigger::R](subscribe_trigger::R) reader structure"]
impl crate::Readable for SUBSCRIBE_TRIGGER {}
#[doc = "`write(|w| ..)` method takes [subscribe_trigger::W](subscribe_trigger::W) writer structure"]
impl crate::Writable for SUBSCRIBE_TRIGGER {}
#[doc = "Description collection: Subscribe configuration for task TRIGGER\\[n\\]"]
pub mod subscribe_trigger;
#[doc = "Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\] task\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_triggered](events_triggered) module"]
pub type EVENTS_TRIGGERED = crate::Reg<u32, _EVENTS_TRIGGERED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TRIGGERED;
#[doc = "`read()` method returns [events_triggered::R](events_triggered::R) reader structure"]
impl crate::Readable for EVENTS_TRIGGERED {}
#[doc = "`write(|w| ..)` method takes [events_triggered::W](events_triggered::W) writer structure"]
impl crate::Writable for EVENTS_TRIGGERED {}
#[doc = "Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\] task"]
pub mod events_triggered;
#[doc = "Description collection: Publish configuration for event TRIGGERED\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_triggered](publish_triggered) module"]
pub type PUBLISH_TRIGGERED = crate::Reg<u32, _PUBLISH_TRIGGERED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TRIGGERED;
#[doc = "`read()` method returns [publish_triggered::R](publish_triggered::R) reader structure"]
impl crate::Readable for PUBLISH_TRIGGERED {}
#[doc = "`write(|w| ..)` method takes [publish_triggered::W](publish_triggered::W) writer structure"]
impl crate::Writable for PUBLISH_TRIGGERED {}
#[doc = "Description collection: Publish configuration for event TRIGGERED\\[n\\]"]
pub mod publish_triggered;
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