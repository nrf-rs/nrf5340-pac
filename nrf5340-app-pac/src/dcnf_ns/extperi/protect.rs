#[doc = "Reader of register PROTECT"]
pub type R = crate::R<u32, super::PROTECT>;
#[doc = "Writer for register PROTECT"]
pub type W = crate::W<u32, super::PROTECT>;
#[doc = "Register PROTECT `reset()`'s with value 0"]
impl crate::ResetValue for super::PROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control access to slave 0 of master EXTPERI\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE0_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE0_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE0_A) -> Self {
        match variant {
            SLAVE0_A::ALLOWED => false,
            SLAVE0_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE0`"]
pub type SLAVE0_R = crate::R<bool, SLAVE0_A>;
impl SLAVE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE0_A {
        match self.bits {
            false => SLAVE0_A::ALLOWED,
            true => SLAVE0_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE0_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE0_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE0`"]
pub struct SLAVE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE0_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE0_A::BLOCKED)
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
    #[doc = "Bit 0 - Control access to slave 0 of master EXTPERI\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> SLAVE0_R {
        SLAVE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTPERI\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> SLAVE0_W {
        SLAVE0_W { w: self }
    }
}
