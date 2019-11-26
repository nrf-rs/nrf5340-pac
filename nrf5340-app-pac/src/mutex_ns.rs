#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Description collection: Mutex register"]
    pub mutex: [MUTEX; 16],
}
#[doc = "Description collection: Mutex register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mutex](mutex) module"]
pub type MUTEX = crate::Reg<u32, _MUTEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUTEX;
#[doc = "`read()` method returns [mutex::R](mutex::R) reader structure"]
impl crate::Readable for MUTEX {}
#[doc = "`write(|w| ..)` method takes [mutex::W](mutex::W) writer structure"]
impl crate::Writable for MUTEX {}
#[doc = "Description collection: Mutex register"]
pub mod mutex;
