#[doc = "Register `EPIN[%s]` reader"]
pub struct R(crate::R<EPIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EPIN_SPEC>> for R {
    fn from(reader: crate::R<EPIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GETSTATUS_A {
    #[doc = "0: Endpoint is not halted"]
    NOTHALTED = 0,
    #[doc = "1: Endpoint is halted"]
    HALTED = 1,
}
impl From<GETSTATUS_A> for u16 {
    #[inline(always)]
    fn from(variant: GETSTATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GETSTATUS` reader - IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub struct GETSTATUS_R(crate::FieldReader<u16, GETSTATUS_A>);
impl GETSTATUS_R {
    pub(crate) fn new(bits: u16) -> Self {
        GETSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, GETSTATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GETSTATUS_A::NOTHALTED),
            1 => Val(GETSTATUS_A::HALTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALTED`"]
    #[inline(always)]
    pub fn is_not_halted(&self) -> bool {
        **self == GETSTATUS_A::NOTHALTED
    }
    #[doc = "Checks if the value of the field is `HALTED`"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        **self == GETSTATUS_A::HALTED
    }
}
impl core::ops::Deref for GETSTATUS_R {
    type Target = crate::FieldReader<u16, GETSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn getstatus(&self) -> GETSTATUS_R {
        GETSTATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epin](index.html) module"]
pub struct EPIN_SPEC;
impl crate::RegisterSpec for EPIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epin::R](R) reader structure"]
impl crate::Readable for EPIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPIN[%s]
to value 0"]
impl crate::Resettable for EPIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
