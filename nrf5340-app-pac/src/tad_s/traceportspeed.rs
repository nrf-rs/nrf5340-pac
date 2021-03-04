#[doc = "Register `TRACEPORTSPEED` reader"]
pub struct R(crate::R<TRACEPORTSPEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEPORTSPEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRACEPORTSPEED_SPEC>> for R {
    fn from(reader: crate::R<TRACEPORTSPEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEPORTSPEED` writer"]
pub struct W(crate::W<TRACEPORTSPEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEPORTSPEED_SPEC>;
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
impl core::convert::From<crate::W<TRACEPORTSPEED_SPEC>> for W {
    fn from(writer: crate::W<TRACEPORTSPEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEPORTSPEED_A {
    #[doc = "0: Trace Port clock is: 64MHz"]
    _64MHZ = 0,
    #[doc = "1: Trace Port clock is: 32MHz"]
    _32MHZ = 1,
    #[doc = "2: Trace Port clock is: 16MHz"]
    _16MHZ = 2,
    #[doc = "3: Trace Port clock is: 8MHz"]
    _8MHZ = 3,
}
impl From<TRACEPORTSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEPORTSPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRACEPORTSPEED` reader - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
pub struct TRACEPORTSPEED_R(crate::FieldReader<u8, TRACEPORTSPEED_A>);
impl TRACEPORTSPEED_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRACEPORTSPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEPORTSPEED_A {
        match self.bits {
            0 => TRACEPORTSPEED_A::_64MHZ,
            1 => TRACEPORTSPEED_A::_32MHZ,
            2 => TRACEPORTSPEED_A::_16MHZ,
            3 => TRACEPORTSPEED_A::_8MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64MHZ`"]
    #[inline(always)]
    pub fn is_64mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_64MHZ
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_8MHZ
    }
}
impl core::ops::Deref for TRACEPORTSPEED_R {
    type Target = crate::FieldReader<u8, TRACEPORTSPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
pub struct TRACEPORTSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEPORTSPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEPORTSPEED_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trace Port clock is: 64MHz"]
    #[inline(always)]
    pub fn _64mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_64MHZ)
    }
    #[doc = "Trace Port clock is: 32MHz"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_32MHZ)
    }
    #[doc = "Trace Port clock is: 16MHz"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_16MHZ)
    }
    #[doc = "Trace Port clock is: 8MHz"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_8MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TRACEPORTSPEED_R {
        TRACEPORTSPEED_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub fn traceportspeed(&mut self) -> TRACEPORTSPEED_W {
        TRACEPORTSPEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceportspeed](index.html) module"]
pub struct TRACEPORTSPEED_SPEC;
impl crate::RegisterSpec for TRACEPORTSPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceportspeed::R](R) reader structure"]
impl crate::Readable for TRACEPORTSPEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceportspeed::W](W) writer structure"]
impl crate::Writable for TRACEPORTSPEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACEPORTSPEED to value 0"]
impl crate::Resettable for TRACEPORTSPEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
