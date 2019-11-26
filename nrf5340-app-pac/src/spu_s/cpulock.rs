#[doc = "Reader of register CPULOCK"]
pub type R = crate::R<u32, super::CPULOCK>;
#[doc = "Writer for register CPULOCK"]
pub type W = crate::W<u32, super::CPULOCK>;
#[doc = "Register CPULOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::CPULOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to prevent updating the secure interrupt configuration until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKSVTAIRCR_A {
    #[doc = "1: Disables writes to the VTOR_S, AIRCR.PRIS, and AIRCR.BFHFNMINS registers"]
    LOCKED,
    #[doc = "0: These registers can be updated"]
    UNLOCKED,
}
impl From<LOCKSVTAIRCR_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKSVTAIRCR_A) -> Self {
        match variant {
            LOCKSVTAIRCR_A::LOCKED => true,
            LOCKSVTAIRCR_A::UNLOCKED => false,
        }
    }
}
#[doc = "Reader of field `LOCKSVTAIRCR`"]
pub type LOCKSVTAIRCR_R = crate::R<bool, LOCKSVTAIRCR_A>;
impl LOCKSVTAIRCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKSVTAIRCR_A {
        match self.bits {
            true => LOCKSVTAIRCR_A::LOCKED,
            false => LOCKSVTAIRCR_A::UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKSVTAIRCR_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKSVTAIRCR_A::UNLOCKED
    }
}
#[doc = "Write proxy for field `LOCKSVTAIRCR`"]
pub struct LOCKSVTAIRCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSVTAIRCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKSVTAIRCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables writes to the VTOR_S, AIRCR.PRIS, and AIRCR.BFHFNMINS registers"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKSVTAIRCR_A::LOCKED)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKSVTAIRCR_A::UNLOCKED)
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
#[doc = "Write '1' to prevent updating the non-secure vector table base address until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKNSVTOR_A {
    #[doc = "1: The address of the non-secure vector table is locked"]
    LOCKED,
    #[doc = "0: The address of the non-secure vector table can be updated"]
    UNLOCKED,
}
impl From<LOCKNSVTOR_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSVTOR_A) -> Self {
        match variant {
            LOCKNSVTOR_A::LOCKED => true,
            LOCKNSVTOR_A::UNLOCKED => false,
        }
    }
}
#[doc = "Reader of field `LOCKNSVTOR`"]
pub type LOCKNSVTOR_R = crate::R<bool, LOCKNSVTOR_A>;
impl LOCKNSVTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKNSVTOR_A {
        match self.bits {
            true => LOCKNSVTOR_A::LOCKED,
            false => LOCKNSVTOR_A::UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKNSVTOR_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKNSVTOR_A::UNLOCKED
    }
}
#[doc = "Write proxy for field `LOCKNSVTOR`"]
pub struct LOCKNSVTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKNSVTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKNSVTOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The address of the non-secure vector table is locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKNSVTOR_A::LOCKED)
    }
    #[doc = "The address of the non-secure vector table can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKNSVTOR_A::UNLOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write '1' to prevent updating the secure MPU regions until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKSMPU_A {
    #[doc = "1: Disables writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
    LOCKED,
    #[doc = "0: These registers can be updated"]
    UNLOCKED,
}
impl From<LOCKSMPU_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKSMPU_A) -> Self {
        match variant {
            LOCKSMPU_A::LOCKED => true,
            LOCKSMPU_A::UNLOCKED => false,
        }
    }
}
#[doc = "Reader of field `LOCKSMPU`"]
pub type LOCKSMPU_R = crate::R<bool, LOCKSMPU_A>;
impl LOCKSMPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKSMPU_A {
        match self.bits {
            true => LOCKSMPU_A::LOCKED,
            false => LOCKSMPU_A::UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKSMPU_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKSMPU_A::UNLOCKED
    }
}
#[doc = "Write proxy for field `LOCKSMPU`"]
pub struct LOCKSMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSMPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKSMPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKSMPU_A::LOCKED)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKSMPU_A::UNLOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write '1' to prevent updating the Non-secure MPU regions until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKNSMPU_A {
    #[doc = "1: Disables writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
    LOCKED,
    #[doc = "0: These registers can be updated"]
    UNLOCKED,
}
impl From<LOCKNSMPU_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKNSMPU_A) -> Self {
        match variant {
            LOCKNSMPU_A::LOCKED => true,
            LOCKNSMPU_A::UNLOCKED => false,
        }
    }
}
#[doc = "Reader of field `LOCKNSMPU`"]
pub type LOCKNSMPU_R = crate::R<bool, LOCKNSMPU_A>;
impl LOCKNSMPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKNSMPU_A {
        match self.bits {
            true => LOCKNSMPU_A::LOCKED,
            false => LOCKNSMPU_A::UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKNSMPU_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKNSMPU_A::UNLOCKED
    }
}
#[doc = "Write proxy for field `LOCKNSMPU`"]
pub struct LOCKNSMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKNSMPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKNSMPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKNSMPU_A::LOCKED)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKNSMPU_A::UNLOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write '1' to prevent updating the secure SAU regions until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKSAU_A {
    #[doc = "1: Disables writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
    LOCKED,
    #[doc = "0: These registers can be updated"]
    UNLOCKED,
}
impl From<LOCKSAU_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKSAU_A) -> Self {
        match variant {
            LOCKSAU_A::LOCKED => true,
            LOCKSAU_A::UNLOCKED => false,
        }
    }
}
#[doc = "Reader of field `LOCKSAU`"]
pub type LOCKSAU_R = crate::R<bool, LOCKSAU_A>;
impl LOCKSAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKSAU_A {
        match self.bits {
            true => LOCKSAU_A::LOCKED,
            false => LOCKSAU_A::UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKSAU_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKSAU_A::UNLOCKED
    }
}
#[doc = "Write proxy for field `LOCKSAU`"]
pub struct LOCKSAU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKSAU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKSAU_A::LOCKED)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKSAU_A::UNLOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to prevent updating the secure interrupt configuration until the next reset"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to prevent updating the non-secure vector table base address until the next reset"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to prevent updating the secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to prevent updating the secure SAU regions until the next reset"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to prevent updating the secure interrupt configuration until the next reset"]
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W {
        LOCKSVTAIRCR_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to prevent updating the non-secure vector table base address until the next reset"]
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W {
        LOCKNSVTOR_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to prevent updating the secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W {
        LOCKSMPU_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W {
        LOCKNSMPU_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to prevent updating the secure SAU regions until the next reset"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W {
        LOCKSAU_W { w: self }
    }
}
