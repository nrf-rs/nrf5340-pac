#[doc = "Register `VREGHVOUT` reader"]
pub struct R(crate::R<VREGHVOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGHVOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VREGHVOUT_SPEC>> for R {
    fn from(reader: crate::R<VREGHVOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGHVOUT` writer"]
pub struct W(crate::W<VREGHVOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGHVOUT_SPEC>;
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
impl core::convert::From<crate::W<VREGHVOUT_SPEC>> for W {
    fn from(writer: crate::W<VREGHVOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VREGH regulator output voltage.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREGHVOUT_A {
    #[doc = "0: 1.8 V"]
    _1V8 = 0,
    #[doc = "1: 2.1 V"]
    _2V1 = 1,
    #[doc = "2: 2.4 V"]
    _2V4 = 2,
    #[doc = "3: 2.7 V"]
    _2V7 = 3,
    #[doc = "4: 3.0 V"]
    _3V0 = 4,
    #[doc = "5: 3.3 V"]
    _3V3 = 5,
    #[doc = "7: Default voltage: 1.8 V"]
    DEFAULT = 7,
}
impl From<VREGHVOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: VREGHVOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREGHVOUT` reader - VREGH regulator output voltage."]
pub struct VREGHVOUT_R(crate::FieldReader<u8, VREGHVOUT_A>);
impl VREGHVOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREGHVOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VREGHVOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VREGHVOUT_A::_1V8),
            1 => Val(VREGHVOUT_A::_2V1),
            2 => Val(VREGHVOUT_A::_2V4),
            3 => Val(VREGHVOUT_A::_2V7),
            4 => Val(VREGHVOUT_A::_3V0),
            5 => Val(VREGHVOUT_A::_3V3),
            7 => Val(VREGHVOUT_A::DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        **self == VREGHVOUT_A::_1V8
    }
    #[doc = "Checks if the value of the field is `_2V1`"]
    #[inline(always)]
    pub fn is_2v1(&self) -> bool {
        **self == VREGHVOUT_A::_2V1
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        **self == VREGHVOUT_A::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V7`"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        **self == VREGHVOUT_A::_2V7
    }
    #[doc = "Checks if the value of the field is `_3V0`"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        **self == VREGHVOUT_A::_3V0
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        **self == VREGHVOUT_A::_3V3
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == VREGHVOUT_A::DEFAULT
    }
}
impl core::ops::Deref for VREGHVOUT_R {
    type Target = crate::FieldReader<u8, VREGHVOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGHVOUT` writer - VREGH regulator output voltage."]
pub struct VREGHVOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGHVOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREGHVOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::_1V8)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn _2v1(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::_2V1)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::_2V4)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::_2V7)
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::_3V0)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::_3V3)
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(VREGHVOUT_A::DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - VREGH regulator output voltage."]
    #[inline(always)]
    pub fn vreghvout(&self) -> VREGHVOUT_R {
        VREGHVOUT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VREGH regulator output voltage."]
    #[inline(always)]
    pub fn vreghvout(&mut self) -> VREGHVOUT_W {
        VREGHVOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreghvout](index.html) module"]
pub struct VREGHVOUT_SPEC;
impl crate::RegisterSpec for VREGHVOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreghvout::R](R) reader structure"]
impl crate::Readable for VREGHVOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreghvout::W](W) writer structure"]
impl crate::Writable for VREGHVOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREGHVOUT to value 0xffff_ffff"]
impl crate::Resettable for VREGHVOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
