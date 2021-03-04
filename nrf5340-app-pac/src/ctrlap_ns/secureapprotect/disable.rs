#[doc = "Register `DISABLE` reader"]
pub struct R(crate::R<DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DISABLE_SPEC>> for R {
    fn from(reader: crate::R<DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLE` writer"]
pub struct W(crate::W<DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLE_SPEC>;
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
impl core::convert::From<crate::W<DISABLE_SPEC>> for W {
    fn from(writer: crate::W<DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable SECUREAPPROTECT and enable debug access to secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.SECUREAPPROTECT protection needs to be disabled."]
pub struct KEY_R(crate::FieldReader<u32, u32>);
impl KEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable SECUREAPPROTECT and enable debug access to secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.SECUREAPPROTECT protection needs to be disabled."]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable SECUREAPPROTECT and enable debug access to secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.SECUREAPPROTECT protection needs to be disabled."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - If the value of the KEY field is non-zero, and the KEY fields match on both the CPU and debugger sides, disable SECUREAPPROTECT and enable debug access to secure mode until the next pin reset, brown-out reset, power-on reset, or watchog timer reset. After reset the debugger side register has a fixed KEY value. To enable debug access, both CTRL-AP and UICR.SECUREAPPROTECT protection needs to be disabled."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register disables the SECUREAPPROTECT register and enables debug access to secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable](index.html) module"]
pub struct DISABLE_SPEC;
impl crate::RegisterSpec for DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disable::R](R) reader structure"]
impl crate::Readable for DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disable::W](W) writer structure"]
impl crate::Writable for DISABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DISABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
