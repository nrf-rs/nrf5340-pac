#[doc = "Reader of register WAY[%s]"]
pub type R = crate::R<u32, super::WAY>;
#[doc = "Writer for register WAY[%s]"]
pub type W = crate::W<u32, super::WAY>;
#[doc = "Register WAY[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::WAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TAG`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Valid bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: Invalid cache line"]
    INVALID,
    #[doc = "1: Valid cache line"]
    VALID,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        match variant {
            V_A::INVALID => false,
            V_A::VALID => true,
        }
    }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::INVALID,
            true => V_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == V_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == V_A::VALID
    }
}
#[doc = "Most recently used way.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRU_A {
    #[doc = "0: Way0 was most recently used"]
    WAY0,
    #[doc = "1: Way1 was most recently used"]
    WAY1,
}
impl From<MRU_A> for bool {
    #[inline(always)]
    fn from(variant: MRU_A) -> Self {
        match variant {
            MRU_A::WAY0 => false,
            MRU_A::WAY1 => true,
        }
    }
}
#[doc = "Reader of field `MRU`"]
pub type MRU_R = crate::R<bool, MRU_A>;
impl MRU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRU_A {
        match self.bits {
            false => MRU_A::WAY0,
            true => MRU_A::WAY1,
        }
    }
    #[doc = "Checks if the value of the field is `WAY0`"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        *self == MRU_A::WAY0
    }
    #[doc = "Checks if the value of the field is `WAY1`"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        *self == MRU_A::WAY1
    }
}
impl R {
    #[doc = "Bits 0:16 - Cache tag."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 30 - Valid bit."]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Most recently used way."]
    #[inline(always)]
    pub fn mru(&self) -> MRU_R {
        MRU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Cache tag."]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
}
