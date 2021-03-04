#[doc = "Register `APPROTECT` reader"]
pub struct R(crate::R<APPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<APPROTECT_SPEC>> for R {
    fn from(reader: crate::R<APPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPROTECT` writer"]
pub struct W(crate::W<APPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<APPROTECT_SPEC>> for W {
    fn from(writer: crate::W<APPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PALL_A {
    #[doc = "1358582010: Unprotected"]
    UNPROTECTED = 1358582010,
    #[doc = "0: Protected"]
    PROTECTED = 0,
}
impl From<PALL_A> for u32 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PALL` reader - Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
pub struct PALL_R(crate::FieldReader<u32, PALL_A>);
impl PALL_R {
    pub(crate) fn new(bits: u32) -> Self {
        PALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PALL_A> {
        use crate::Variant::*;
        match self.bits {
            1358582010 => Val(PALL_A::UNPROTECTED),
            0 => Val(PALL_A::PROTECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNPROTECTED`"]
    #[inline(always)]
    pub fn is_unprotected(&self) -> bool {
        **self == PALL_A::UNPROTECTED
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        **self == PALL_A::PROTECTED
    }
}
impl core::ops::Deref for PALL_R {
    type Target = crate::FieldReader<u32, PALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PALL` writer - Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
pub struct PALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn unprotected(self) -> &'a mut W {
        self.variant(PALL_A::UNPROTECTED)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(PALL_A::PROTECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
    #[inline(always)]
    pub fn pall(&mut self) -> PALL_W {
        PALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [approtect](index.html) module"]
pub struct APPROTECT_SPEC;
impl crate::RegisterSpec for APPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [approtect::R](R) reader structure"]
impl crate::Readable for APPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [approtect::W](W) writer structure"]
impl crate::Writable for APPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APPROTECT to value 0"]
impl crate::Resettable for APPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
