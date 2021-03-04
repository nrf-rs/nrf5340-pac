#[doc = "Register `NFCID1_LAST` reader"]
pub struct R(crate::R<NFCID1_LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCID1_LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NFCID1_LAST_SPEC>> for R {
    fn from(reader: crate::R<NFCID1_LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCID1_LAST` writer"]
pub struct W(crate::W<NFCID1_LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCID1_LAST_SPEC>;
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
impl core::convert::From<crate::W<NFCID1_LAST_SPEC>> for W {
    fn from(writer: crate::W<NFCID1_LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NFCID1_Z` reader - NFCID1 byte Z (very last byte sent)"]
pub struct NFCID1_Z_R(crate::FieldReader<u8, u8>);
impl NFCID1_Z_R {
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_Z_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_Z_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_Z` writer - NFCID1 byte Z (very last byte sent)"]
pub struct NFCID1_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_Z_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Field `NFCID1_Y` reader - NFCID1 byte Y"]
pub struct NFCID1_Y_R(crate::FieldReader<u8, u8>);
impl NFCID1_Y_R {
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_Y_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_Y_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_Y` writer - NFCID1 byte Y"]
pub struct NFCID1_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `NFCID1_X` reader - NFCID1 byte X"]
pub struct NFCID1_X_R(crate::FieldReader<u8, u8>);
impl NFCID1_X_R {
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_X_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_X` writer - NFCID1 byte X"]
pub struct NFCID1_X_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `NFCID1_W` reader - NFCID1 byte W"]
pub struct NFCID1_W_R(crate::FieldReader<u8, u8>);
impl NFCID1_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_W` writer - NFCID1 byte W"]
pub struct NFCID1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&self) -> NFCID1_Z_R {
        NFCID1_Z_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&self) -> NFCID1_Y_R {
        NFCID1_Y_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&self) -> NFCID1_X_R {
        NFCID1_X_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&self) -> NFCID1_W_R {
        NFCID1_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&mut self) -> NFCID1_Z_W {
        NFCID1_Z_W { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&mut self) -> NFCID1_Y_W {
        NFCID1_Y_W { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&mut self) -> NFCID1_X_W {
        NFCID1_X_W { w: self }
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&mut self) -> NFCID1_W_W {
        NFCID1_W_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_last](index.html) module"]
pub struct NFCID1_LAST_SPEC;
impl crate::RegisterSpec for NFCID1_LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcid1_last::R](R) reader structure"]
impl crate::Readable for NFCID1_LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcid1_last::W](W) writer structure"]
impl crate::Writable for NFCID1_LAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NFCID1_LAST to value 0x6363"]
impl crate::Resettable for NFCID1_LAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6363
    }
}
