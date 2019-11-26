#[doc = "Reader of register INTCAP"]
pub type R = crate::R<u32, super::INTCAP>;
#[doc = "Writer for register INTCAP"]
pub type W = crate::W<u32, super::INTCAP>;
#[doc = "Register INTCAP `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control usage of internal load capacitors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCAP_A {
    #[doc = "0: Use external load capacitors"]
    EXTERNAL,
    #[doc = "1: 6 pF internal load capacitance"]
    C6PF,
    #[doc = "2: 7 pF internal load capacitance"]
    C7PF,
    #[doc = "3: 11 pF internal load capacitance"]
    C11PF,
}
impl From<INTCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: INTCAP_A) -> Self {
        match variant {
            INTCAP_A::EXTERNAL => 0,
            INTCAP_A::C6PF => 1,
            INTCAP_A::C7PF => 2,
            INTCAP_A::C11PF => 3,
        }
    }
}
#[doc = "Reader of field `INTCAP`"]
pub type INTCAP_R = crate::R<u8, INTCAP_A>;
impl INTCAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCAP_A {
        match self.bits {
            0 => INTCAP_A::EXTERNAL,
            1 => INTCAP_A::C6PF,
            2 => INTCAP_A::C7PF,
            3 => INTCAP_A::C11PF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == INTCAP_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `C6PF`"]
    #[inline(always)]
    pub fn is_c6pf(&self) -> bool {
        *self == INTCAP_A::C6PF
    }
    #[doc = "Checks if the value of the field is `C7PF`"]
    #[inline(always)]
    pub fn is_c7pf(&self) -> bool {
        *self == INTCAP_A::C7PF
    }
    #[doc = "Checks if the value of the field is `C11PF`"]
    #[inline(always)]
    pub fn is_c11pf(&self) -> bool {
        *self == INTCAP_A::C11PF
    }
}
#[doc = "Write proxy for field `INTCAP`"]
pub struct INTCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use external load capacitors"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(INTCAP_A::EXTERNAL)
    }
    #[doc = "6 pF internal load capacitance"]
    #[inline(always)]
    pub fn c6pf(self) -> &'a mut W {
        self.variant(INTCAP_A::C6PF)
    }
    #[doc = "7 pF internal load capacitance"]
    #[inline(always)]
    pub fn c7pf(self) -> &'a mut W {
        self.variant(INTCAP_A::C7PF)
    }
    #[doc = "11 pF internal load capacitance"]
    #[inline(always)]
    pub fn c11pf(self) -> &'a mut W {
        self.variant(INTCAP_A::C11PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn intcap(&self) -> INTCAP_R {
        INTCAP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn intcap(&mut self) -> INTCAP_W {
        INTCAP_W { w: self }
    }
}
