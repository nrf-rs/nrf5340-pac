#[doc = "Reader of register MODULATIONCTRL"]
pub type R = crate::R<u32, super::MODULATIONCTRL>;
#[doc = "Writer for register MODULATIONCTRL"]
pub type W = crate::W<u32, super::MODULATIONCTRL>;
#[doc = "Register MODULATIONCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MODULATIONCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Configuration of modulation control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODULATIONCTRL_A {
    #[doc = "0: Invalid, defaults to same behaviour as for Internal"]
    INVALID,
    #[doc = "1: Use internal modulator only"]
    INTERNAL,
    #[doc = "2: Output digital modulation signal to a GPIO pin."]
    MODTOGPIO,
    #[doc = "3: Use internal modulator and output digital modulation signal to a GPIO pin."]
    INTERNALANDMODTOGPIO,
}
impl From<MODULATIONCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: MODULATIONCTRL_A) -> Self {
        match variant {
            MODULATIONCTRL_A::INVALID => 0,
            MODULATIONCTRL_A::INTERNAL => 1,
            MODULATIONCTRL_A::MODTOGPIO => 2,
            MODULATIONCTRL_A::INTERNALANDMODTOGPIO => 3,
        }
    }
}
#[doc = "Reader of field `MODULATIONCTRL`"]
pub type MODULATIONCTRL_R = crate::R<u8, MODULATIONCTRL_A>;
impl MODULATIONCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODULATIONCTRL_A {
        match self.bits {
            0 => MODULATIONCTRL_A::INVALID,
            1 => MODULATIONCTRL_A::INTERNAL,
            2 => MODULATIONCTRL_A::MODTOGPIO,
            3 => MODULATIONCTRL_A::INTERNALANDMODTOGPIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == MODULATIONCTRL_A::INVALID
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == MODULATIONCTRL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `MODTOGPIO`"]
    #[inline(always)]
    pub fn is_mod_to_gpio(&self) -> bool {
        *self == MODULATIONCTRL_A::MODTOGPIO
    }
    #[doc = "Checks if the value of the field is `INTERNALANDMODTOGPIO`"]
    #[inline(always)]
    pub fn is_internal_and_mod_to_gpio(&self) -> bool {
        *self == MODULATIONCTRL_A::INTERNALANDMODTOGPIO
    }
}
#[doc = "Write proxy for field `MODULATIONCTRL`"]
pub struct MODULATIONCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULATIONCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODULATIONCTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Invalid, defaults to same behaviour as for Internal"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INVALID)
    }
    #[doc = "Use internal modulator only"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INTERNAL)
    }
    #[doc = "Output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn mod_to_gpio(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::MODTOGPIO)
    }
    #[doc = "Use internal modulator and output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn internal_and_mod_to_gpio(self) -> &'a mut W {
        self.variant(MODULATIONCTRL_A::INTERNALANDMODTOGPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&self) -> MODULATIONCTRL_R {
        MODULATIONCTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&mut self) -> MODULATIONCTRL_W {
        MODULATIONCTRL_W { w: self }
    }
}
