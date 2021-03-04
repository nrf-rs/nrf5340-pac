#[doc = "Register `OVERSAMPLE` reader"]
pub struct R(crate::R<OVERSAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERSAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OVERSAMPLE_SPEC>> for R {
    fn from(reader: crate::R<OVERSAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERSAMPLE` writer"]
pub struct W(crate::W<OVERSAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERSAMPLE_SPEC>;
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
impl core::convert::From<crate::W<OVERSAMPLE_SPEC>> for W {
    fn from(writer: crate::W<OVERSAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Oversample control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVERSAMPLE_A {
    #[doc = "0: Bypass oversampling"]
    BYPASS = 0,
    #[doc = "1: Oversample 2x"]
    OVER2X = 1,
    #[doc = "2: Oversample 4x"]
    OVER4X = 2,
    #[doc = "3: Oversample 8x"]
    OVER8X = 3,
    #[doc = "4: Oversample 16x"]
    OVER16X = 4,
    #[doc = "5: Oversample 32x"]
    OVER32X = 5,
    #[doc = "6: Oversample 64x"]
    OVER64X = 6,
    #[doc = "7: Oversample 128x"]
    OVER128X = 7,
    #[doc = "8: Oversample 256x"]
    OVER256X = 8,
}
impl From<OVERSAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERSAMPLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVERSAMPLE` reader - Oversample control"]
pub struct OVERSAMPLE_R(crate::FieldReader<u8, OVERSAMPLE_A>);
impl OVERSAMPLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVERSAMPLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVERSAMPLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVERSAMPLE_A::BYPASS),
            1 => Val(OVERSAMPLE_A::OVER2X),
            2 => Val(OVERSAMPLE_A::OVER4X),
            3 => Val(OVERSAMPLE_A::OVER8X),
            4 => Val(OVERSAMPLE_A::OVER16X),
            5 => Val(OVERSAMPLE_A::OVER32X),
            6 => Val(OVERSAMPLE_A::OVER64X),
            7 => Val(OVERSAMPLE_A::OVER128X),
            8 => Val(OVERSAMPLE_A::OVER256X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == OVERSAMPLE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `OVER2X`"]
    #[inline(always)]
    pub fn is_over2x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER2X
    }
    #[doc = "Checks if the value of the field is `OVER4X`"]
    #[inline(always)]
    pub fn is_over4x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER4X
    }
    #[doc = "Checks if the value of the field is `OVER8X`"]
    #[inline(always)]
    pub fn is_over8x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER8X
    }
    #[doc = "Checks if the value of the field is `OVER16X`"]
    #[inline(always)]
    pub fn is_over16x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER16X
    }
    #[doc = "Checks if the value of the field is `OVER32X`"]
    #[inline(always)]
    pub fn is_over32x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER32X
    }
    #[doc = "Checks if the value of the field is `OVER64X`"]
    #[inline(always)]
    pub fn is_over64x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER64X
    }
    #[doc = "Checks if the value of the field is `OVER128X`"]
    #[inline(always)]
    pub fn is_over128x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER128X
    }
    #[doc = "Checks if the value of the field is `OVER256X`"]
    #[inline(always)]
    pub fn is_over256x(&self) -> bool {
        **self == OVERSAMPLE_A::OVER256X
    }
}
impl core::ops::Deref for OVERSAMPLE_R {
    type Target = crate::FieldReader<u8, OVERSAMPLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERSAMPLE` writer - Oversample control"]
pub struct OVERSAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERSAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERSAMPLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bypass oversampling"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::BYPASS)
    }
    #[doc = "Oversample 2x"]
    #[inline(always)]
    pub fn over2x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER2X)
    }
    #[doc = "Oversample 4x"]
    #[inline(always)]
    pub fn over4x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER4X)
    }
    #[doc = "Oversample 8x"]
    #[inline(always)]
    pub fn over8x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER8X)
    }
    #[doc = "Oversample 16x"]
    #[inline(always)]
    pub fn over16x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER16X)
    }
    #[doc = "Oversample 32x"]
    #[inline(always)]
    pub fn over32x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER32X)
    }
    #[doc = "Oversample 64x"]
    #[inline(always)]
    pub fn over64x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER64X)
    }
    #[doc = "Oversample 128x"]
    #[inline(always)]
    pub fn over128x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER128X)
    }
    #[doc = "Oversample 256x"]
    #[inline(always)]
    pub fn over256x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER256X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&self) -> OVERSAMPLE_R {
        OVERSAMPLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&mut self) -> OVERSAMPLE_W {
        OVERSAMPLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oversample](index.html) module"]
pub struct OVERSAMPLE_SPEC;
impl crate::RegisterSpec for OVERSAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oversample::R](R) reader structure"]
impl crate::Readable for OVERSAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oversample::W](W) writer structure"]
impl crate::Writable for OVERSAMPLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVERSAMPLE to value 0"]
impl crate::Resettable for OVERSAMPLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
