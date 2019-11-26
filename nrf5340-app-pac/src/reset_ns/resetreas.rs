#[doc = "Reader of register RESETREAS"]
pub type R = crate::R<u32, super::RESETREAS>;
#[doc = "Writer for register RESETREAS"]
pub type W = crate::W<u32, super::RESETREAS>;
#[doc = "Register RESETREAS `reset()`'s with value 0"]
impl crate::ResetValue for super::RESETREAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset from pin reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETPIN_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<RESETPIN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETPIN_A) -> Self {
        match variant {
            RESETPIN_A::NOTDETECTED => false,
            RESETPIN_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `RESETPIN`"]
pub type RESETPIN_R = crate::R<bool, RESETPIN_A>;
impl RESETPIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETPIN_A {
        match self.bits {
            false => RESETPIN_A::NOTDETECTED,
            true => RESETPIN_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == RESETPIN_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == RESETPIN_A::DETECTED
    }
}
#[doc = "Write proxy for field `RESETPIN`"]
pub struct RESETPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETPIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::DETECTED)
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
#[doc = "Reset from watchdog timer 0 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG0_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<DOG0_A> for bool {
    #[inline(always)]
    fn from(variant: DOG0_A) -> Self {
        match variant {
            DOG0_A::NOTDETECTED => false,
            DOG0_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `DOG0`"]
pub type DOG0_R = crate::R<bool, DOG0_A>;
impl DOG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG0_A {
        match self.bits {
            false => DOG0_A::NOTDETECTED,
            true => DOG0_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DOG0_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DOG0_A::DETECTED
    }
}
#[doc = "Write proxy for field `DOG0`"]
pub struct DOG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG0_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG0_A::DETECTED)
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
#[doc = "Reset from application CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLAP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<CTRLAP_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLAP_A) -> Self {
        match variant {
            CTRLAP_A::NOTDETECTED => false,
            CTRLAP_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `CTRLAP`"]
pub type CTRLAP_R = crate::R<bool, CTRLAP_A>;
impl CTRLAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLAP_A {
        match self.bits {
            false => CTRLAP_A::NOTDETECTED,
            true => CTRLAP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == CTRLAP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == CTRLAP_A::DETECTED
    }
}
#[doc = "Write proxy for field `CTRLAP`"]
pub struct CTRLAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRLAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(CTRLAP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(CTRLAP_A::DETECTED)
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
#[doc = "Reset from AIRCR.SYSRESETREQ detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREQ_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<SREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SREQ_A) -> Self {
        match variant {
            SREQ_A::NOTDETECTED => false,
            SREQ_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `SREQ`"]
pub type SREQ_R = crate::R<bool, SREQ_A>;
impl SREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREQ_A {
        match self.bits {
            false => SREQ_A::NOTDETECTED,
            true => SREQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == SREQ_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SREQ_A::DETECTED
    }
}
#[doc = "Write proxy for field `SREQ`"]
pub struct SREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SREQ_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SREQ_A::DETECTED)
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
#[doc = "Reset from CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        match variant {
            LOCKUP_A::NOTDETECTED => false,
            LOCKUP_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `LOCKUP`"]
pub type LOCKUP_R = crate::R<bool, LOCKUP_A>;
impl LOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::NOTDETECTED,
            true => LOCKUP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LOCKUP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LOCKUP_A::DETECTED
    }
}
#[doc = "Write proxy for field `LOCKUP`"]
pub struct LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::DETECTED)
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
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<OFF_A> for bool {
    #[inline(always)]
    fn from(variant: OFF_A) -> Self {
        match variant {
            OFF_A::NOTDETECTED => false,
            OFF_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `OFF`"]
pub type OFF_R = crate::R<bool, OFF_A>;
impl OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFF_A {
        match self.bits {
            false => OFF_A::NOTDETECTED,
            true => OFF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OFF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OFF_A::DETECTED
    }
}
#[doc = "Write proxy for field `OFF`"]
pub struct OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OFF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OFF_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCOMP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<LPCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        match variant {
            LPCOMP_A::NOTDETECTED => false,
            LPCOMP_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `LPCOMP`"]
pub type LPCOMP_R = crate::R<bool, LPCOMP_A>;
impl LPCOMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            false => LPCOMP_A::NOTDETECTED,
            true => LPCOMP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LPCOMP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LPCOMP_A::DETECTED
    }
}
#[doc = "Write proxy for field `LPCOMP`"]
pub struct LPCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCOMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCOMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the debug interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<DIF_A> for bool {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        match variant {
            DIF_A::NOTDETECTED => false,
            DIF_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `DIF`"]
pub type DIF_R = crate::R<bool, DIF_A>;
impl DIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIF_A {
        match self.bits {
            false => DIF_A::NOTDETECTED,
            true => DIF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DIF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DIF_A::DETECTED
    }
}
#[doc = "Write proxy for field `DIF`"]
pub struct DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DIF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DIF_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reset from local soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSREQ_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<LSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: LSREQ_A) -> Self {
        match variant {
            LSREQ_A::NOTDETECTED => false,
            LSREQ_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `LSREQ`"]
pub type LSREQ_R = crate::R<bool, LSREQ_A>;
impl LSREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSREQ_A {
        match self.bits {
            false => LSREQ_A::NOTDETECTED,
            true => LSREQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LSREQ_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LSREQ_A::DETECTED
    }
}
#[doc = "Write proxy for field `LSREQ`"]
pub struct LSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LSREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LSREQ_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LSREQ_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reset from local CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLOCKUP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<LLOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LLOCKUP_A) -> Self {
        match variant {
            LLOCKUP_A::NOTDETECTED => false,
            LLOCKUP_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `LLOCKUP`"]
pub type LLOCKUP_R = crate::R<bool, LLOCKUP_A>;
impl LLOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLOCKUP_A {
        match self.bits {
            false => LLOCKUP_A::NOTDETECTED,
            true => LLOCKUP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LLOCKUP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LLOCKUP_A::DETECTED
    }
}
#[doc = "Write proxy for field `LLOCKUP`"]
pub struct LLOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LLOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLOCKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LLOCKUP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LLOCKUP_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reset from local watchdog timer detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOG_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<LDOG_A> for bool {
    #[inline(always)]
    fn from(variant: LDOG_A) -> Self {
        match variant {
            LDOG_A::NOTDETECTED => false,
            LDOG_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `LDOG`"]
pub type LDOG_R = crate::R<bool, LDOG_A>;
impl LDOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOG_A {
        match self.bits {
            false => LDOG_A::NOTDETECTED,
            true => LDOG_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LDOG_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LDOG_A::DETECTED
    }
}
#[doc = "Write proxy for field `LDOG`"]
pub struct LDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDOG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LDOG_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LDOG_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Force off reset from Application core detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MFORCEOFF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<MFORCEOFF_A> for bool {
    #[inline(always)]
    fn from(variant: MFORCEOFF_A) -> Self {
        match variant {
            MFORCEOFF_A::NOTDETECTED => false,
            MFORCEOFF_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `MFORCEOFF`"]
pub type MFORCEOFF_R = crate::R<bool, MFORCEOFF_A>;
impl MFORCEOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFORCEOFF_A {
        match self.bits {
            false => MFORCEOFF_A::NOTDETECTED,
            true => MFORCEOFF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == MFORCEOFF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == MFORCEOFF_A::DETECTED
    }
}
#[doc = "Write proxy for field `MFORCEOFF`"]
pub struct MFORCEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MFORCEOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MFORCEOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(MFORCEOFF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(MFORCEOFF_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reset after wakeup from System OFF mode due to NFC field being detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFC_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<NFC_A> for bool {
    #[inline(always)]
    fn from(variant: NFC_A) -> Self {
        match variant {
            NFC_A::NOTDETECTED => false,
            NFC_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `NFC`"]
pub type NFC_R = crate::R<bool, NFC_A>;
impl NFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFC_A {
        match self.bits {
            false => NFC_A::NOTDETECTED,
            true => NFC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == NFC_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == NFC_A::DETECTED
    }
}
#[doc = "Write proxy for field `NFC`"]
pub struct NFC_W<'a> {
    w: &'a mut W,
}
impl<'a> NFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(NFC_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(NFC_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reset from watchdog timer 1 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG1_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<DOG1_A> for bool {
    #[inline(always)]
    fn from(variant: DOG1_A) -> Self {
        match variant {
            DOG1_A::NOTDETECTED => false,
            DOG1_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `DOG1`"]
pub type DOG1_R = crate::R<bool, DOG1_A>;
impl DOG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG1_A {
        match self.bits {
            false => DOG1_A::NOTDETECTED,
            true => DOG1_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DOG1_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DOG1_A::DETECTED
    }
}
#[doc = "Write proxy for field `DOG1`"]
pub struct DOG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG1_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG1_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_A) -> Self {
        match variant {
            VBUS_A::NOTDETECTED => false,
            VBUS_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `VBUS`"]
pub type VBUS_R = crate::R<bool, VBUS_A>;
impl VBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_A {
        match self.bits {
            false => VBUS_A::NOTDETECTED,
            true => VBUS_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == VBUS_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == VBUS_A::DETECTED
    }
}
#[doc = "Write proxy for field `VBUS`"]
pub struct VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(VBUS_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(VBUS_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reset from local CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCTRLAP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED,
    #[doc = "1: Detected"]
    DETECTED,
}
impl From<LCTRLAP_A> for bool {
    #[inline(always)]
    fn from(variant: LCTRLAP_A) -> Self {
        match variant {
            LCTRLAP_A::NOTDETECTED => false,
            LCTRLAP_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `LCTRLAP`"]
pub type LCTRLAP_R = crate::R<bool, LCTRLAP_A>;
impl LCTRLAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCTRLAP_A {
        match self.bits {
            false => LCTRLAP_A::NOTDETECTED,
            true => LCTRLAP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LCTRLAP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LCTRLAP_A::DETECTED
    }
}
#[doc = "Write proxy for field `LCTRLAP`"]
pub struct LCTRLAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCTRLAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCTRLAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LCTRLAP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LCTRLAP_A::DETECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> RESETPIN_R {
        RESETPIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset from watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&self) -> DOG0_R {
        DOG0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&self) -> CTRLAP_R {
        CTRLAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset from AIRCR.SYSRESETREQ detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SREQ_R {
        SREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset from CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the debug interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reset from local soft reset detected"]
    #[inline(always)]
    pub fn lsreq(&self) -> LSREQ_R {
        LSREQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reset from local CPU lockup detected"]
    #[inline(always)]
    pub fn llockup(&self) -> LLOCKUP_R {
        LLOCKUP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset from local watchdog timer detected"]
    #[inline(always)]
    pub fn ldog(&self) -> LDOG_R {
        LDOG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Force off reset from Application core detected"]
    #[inline(always)]
    pub fn mforceoff(&self) -> MFORCEOFF_R {
        MFORCEOFF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&self) -> NFC_R {
        NFC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reset from watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&self) -> DOG1_R {
        DOG1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Reset from local CTRL-AP detected"]
    #[inline(always)]
    pub fn lctrlap(&self) -> LCTRLAP_R {
        LCTRLAP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&mut self) -> RESETPIN_W {
        RESETPIN_W { w: self }
    }
    #[doc = "Bit 1 - Reset from watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&mut self) -> DOG0_W {
        DOG0_W { w: self }
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&mut self) -> CTRLAP_W {
        CTRLAP_W { w: self }
    }
    #[doc = "Bit 3 - Reset from AIRCR.SYSRESETREQ detected"]
    #[inline(always)]
    pub fn sreq(&mut self) -> SREQ_W {
        SREQ_W { w: self }
    }
    #[doc = "Bit 4 - Reset from CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W {
        LOCKUP_W { w: self }
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W {
        OFF_W { w: self }
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LPCOMP_W {
        LPCOMP_W { w: self }
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the debug interface mode"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W {
        DIF_W { w: self }
    }
    #[doc = "Bit 16 - Reset from local soft reset detected"]
    #[inline(always)]
    pub fn lsreq(&mut self) -> LSREQ_W {
        LSREQ_W { w: self }
    }
    #[doc = "Bit 17 - Reset from local CPU lockup detected"]
    #[inline(always)]
    pub fn llockup(&mut self) -> LLOCKUP_W {
        LLOCKUP_W { w: self }
    }
    #[doc = "Bit 18 - Reset from local watchdog timer detected"]
    #[inline(always)]
    pub fn ldog(&mut self) -> LDOG_W {
        LDOG_W { w: self }
    }
    #[doc = "Bit 23 - Force off reset from Application core detected"]
    #[inline(always)]
    pub fn mforceoff(&mut self) -> MFORCEOFF_W {
        MFORCEOFF_W { w: self }
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&mut self) -> NFC_W {
        NFC_W { w: self }
    }
    #[doc = "Bit 25 - Reset from watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&mut self) -> DOG1_W {
        DOG1_W { w: self }
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W {
        VBUS_W { w: self }
    }
    #[doc = "Bit 27 - Reset from local CTRL-AP detected"]
    #[inline(always)]
    pub fn lctrlap(&mut self) -> LCTRLAP_W {
        LCTRLAP_W { w: self }
    }
}
