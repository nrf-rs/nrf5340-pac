#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CONFIG_SPEC>> for R {
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl core::convert::From<crate::W<CONFIG_SPEC>> for W {
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WEN_A {
    #[doc = "0: Read only access"]
    REN = 0,
    #[doc = "1: Write enabled"]
    WEN = 1,
    #[doc = "2: Erase enabled"]
    EEN = 2,
    #[doc = "4: Partial erase enabled"]
    PEEN = 4,
}
impl From<WEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WEN` reader - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
pub struct WEN_R(crate::FieldReader<u8, WEN_A>);
impl WEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WEN_A::REN),
            1 => Val(WEN_A::WEN),
            2 => Val(WEN_A::EEN),
            4 => Val(WEN_A::PEEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REN`"]
    #[inline(always)]
    pub fn is_ren(&self) -> bool {
        **self == WEN_A::REN
    }
    #[doc = "Checks if the value of the field is `WEN`"]
    #[inline(always)]
    pub fn is_wen(&self) -> bool {
        **self == WEN_A::WEN
    }
    #[doc = "Checks if the value of the field is `EEN`"]
    #[inline(always)]
    pub fn is_een(&self) -> bool {
        **self == WEN_A::EEN
    }
    #[doc = "Checks if the value of the field is `PEEN`"]
    #[inline(always)]
    pub fn is_peen(&self) -> bool {
        **self == WEN_A::PEEN
    }
}
impl core::ops::Deref for WEN_R {
    type Target = crate::FieldReader<u8, WEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEN` writer - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
pub struct WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read only access"]
    #[inline(always)]
    pub fn ren(self) -> &'a mut W {
        self.variant(WEN_A::REN)
    }
    #[doc = "Write enabled"]
    #[inline(always)]
    pub fn wen(self) -> &'a mut W {
        self.variant(WEN_A::WEN)
    }
    #[doc = "Erase enabled"]
    #[inline(always)]
    pub fn een(self) -> &'a mut W {
        self.variant(WEN_A::EEN)
    }
    #[doc = "Partial erase enabled"]
    #[inline(always)]
    pub fn peen(self) -> &'a mut W {
        self.variant(WEN_A::PEEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W {
        WEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
