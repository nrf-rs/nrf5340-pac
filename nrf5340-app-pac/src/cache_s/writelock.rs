#[doc = "Register `WRITELOCK` reader"]
pub struct R(crate::R<WRITELOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITELOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WRITELOCK_SPEC>> for R {
    fn from(reader: crate::R<WRITELOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITELOCK` writer"]
pub struct W(crate::W<WRITELOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITELOCK_SPEC>;
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
impl core::convert::From<crate::W<WRITELOCK_SPEC>> for W {
    fn from(writer: crate::W<WRITELOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock cache updates\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITELOCK_A {
    #[doc = "0: Cache updates unlocked"]
    UNLOCKED = 0,
    #[doc = "1: Cache updates locked"]
    LOCKED = 1,
}
impl From<WRITELOCK_A> for bool {
    #[inline(always)]
    fn from(variant: WRITELOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITELOCK` reader - Lock cache updates"]
pub struct WRITELOCK_R(crate::FieldReader<bool, WRITELOCK_A>);
impl WRITELOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITELOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITELOCK_A {
        match self.bits {
            false => WRITELOCK_A::UNLOCKED,
            true => WRITELOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == WRITELOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == WRITELOCK_A::LOCKED
    }
}
impl core::ops::Deref for WRITELOCK_R {
    type Target = crate::FieldReader<bool, WRITELOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITELOCK` writer - Lock cache updates"]
pub struct WRITELOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITELOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITELOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cache updates unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(WRITELOCK_A::UNLOCKED)
    }
    #[doc = "Cache updates locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(WRITELOCK_A::LOCKED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Lock cache updates"]
    #[inline(always)]
    pub fn writelock(&self) -> WRITELOCK_R {
        WRITELOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock cache updates"]
    #[inline(always)]
    pub fn writelock(&mut self) -> WRITELOCK_W {
        WRITELOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writelock](index.html) module"]
pub struct WRITELOCK_SPEC;
impl crate::RegisterSpec for WRITELOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writelock::R](R) reader structure"]
impl crate::Readable for WRITELOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writelock::W](W) writer structure"]
impl crate::Writable for WRITELOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITELOCK to value 0"]
impl crate::Resettable for WRITELOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
