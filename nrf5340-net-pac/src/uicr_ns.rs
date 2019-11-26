#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    #[doc = "0x04 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    _reserved2: [u8; 504usize],
    #[doc = "0x200 - Description collection: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 32],
    _reserved3: [u8; 128usize],
    #[doc = "0x300 - Description collection: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
}
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [approtect](approtect) module"]
pub type APPROTECT = crate::Reg<u32, _APPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPROTECT;
#[doc = "`read()` method returns [approtect::R](approtect::R) reader structure"]
impl crate::Readable for APPROTECT {}
#[doc = "`write(|w| ..)` method takes [approtect::W](approtect::W) writer structure"]
impl crate::Writable for APPROTECT {}
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "Erase protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eraseprotect](eraseprotect) module"]
pub type ERASEPROTECT = crate::Reg<u32, _ERASEPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPROTECT;
#[doc = "`read()` method returns [eraseprotect::R](eraseprotect::R) reader structure"]
impl crate::Readable for ERASEPROTECT {}
#[doc = "`write(|w| ..)` method takes [eraseprotect::W](eraseprotect::W) writer structure"]
impl crate::Writable for ERASEPROTECT {}
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "Description collection: Reserved for Nordic firmware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nrffw](nrffw) module"]
pub type NRFFW = crate::Reg<u32, _NRFFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFFW;
#[doc = "`read()` method returns [nrffw::R](nrffw::R) reader structure"]
impl crate::Readable for NRFFW {}
#[doc = "`write(|w| ..)` method takes [nrffw::W](nrffw::W) writer structure"]
impl crate::Writable for NRFFW {}
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "Description collection: Reserved for customer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [customer](customer) module"]
pub type CUSTOMER = crate::Reg<u32, _CUSTOMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUSTOMER;
#[doc = "`read()` method returns [customer::R](customer::R) reader structure"]
impl crate::Readable for CUSTOMER {}
#[doc = "`write(|w| ..)` method takes [customer::W](customer::W) writer structure"]
impl crate::Writable for CUSTOMER {}
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
