#[doc = "Reader of register DFECTRL1"]
pub type R = crate::R<u32, super::DFECTRL1>;
#[doc = "Writer for register DFECTRL1"]
pub type W = crate::W<u32, super::DFECTRL1>;
#[doc = "Register DFECTRL1 `reset()`'s with value 0x0002_3282"]
impl crate::ResetValue for super::DFECTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_3282
    }
}
#[doc = "Reader of field `NUMBEROF8US`"]
pub type NUMBEROF8US_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMBEROF8US`"]
pub struct NUMBEROF8US_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMBEROF8US_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Add CTE extension and do antenna switching/sampling in this extension\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEINEXTENSION_A {
    #[doc = "1: AoA/AoD procedure triggered at end of CRC"]
    CRC,
    #[doc = "0: Antenna switching/sampling is done in the packet payload"]
    PAYLOAD,
}
impl From<DFEINEXTENSION_A> for bool {
    #[inline(always)]
    fn from(variant: DFEINEXTENSION_A) -> Self {
        match variant {
            DFEINEXTENSION_A::CRC => true,
            DFEINEXTENSION_A::PAYLOAD => false,
        }
    }
}
#[doc = "Reader of field `DFEINEXTENSION`"]
pub type DFEINEXTENSION_R = crate::R<bool, DFEINEXTENSION_A>;
impl DFEINEXTENSION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEINEXTENSION_A {
        match self.bits {
            true => DFEINEXTENSION_A::CRC,
            false => DFEINEXTENSION_A::PAYLOAD,
        }
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == DFEINEXTENSION_A::CRC
    }
    #[doc = "Checks if the value of the field is `PAYLOAD`"]
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == DFEINEXTENSION_A::PAYLOAD
    }
}
#[doc = "Write proxy for field `DFEINEXTENSION`"]
pub struct DFEINEXTENSION_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEINEXTENSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEINEXTENSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AoA/AoD procedure triggered at end of CRC"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(DFEINEXTENSION_A::CRC)
    }
    #[doc = "Antenna switching/sampling is done in the packet payload"]
    #[inline(always)]
    pub fn payload(self) -> &'a mut W {
        self.variant(DFEINEXTENSION_A::PAYLOAD)
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
#[doc = "Interval between every time the antenna is changed in the SWITCHING state\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSWITCHSPACING_A {
    #[doc = "1: 4us"]
    _4US,
    #[doc = "2: 2us"]
    _2US,
    #[doc = "3: 1us"]
    _1US,
}
impl From<TSWITCHSPACING_A> for u8 {
    #[inline(always)]
    fn from(variant: TSWITCHSPACING_A) -> Self {
        match variant {
            TSWITCHSPACING_A::_4US => 1,
            TSWITCHSPACING_A::_2US => 2,
            TSWITCHSPACING_A::_1US => 3,
        }
    }
}
#[doc = "Reader of field `TSWITCHSPACING`"]
pub type TSWITCHSPACING_R = crate::R<u8, TSWITCHSPACING_A>;
impl TSWITCHSPACING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSWITCHSPACING_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TSWITCHSPACING_A::_4US),
            2 => Val(TSWITCHSPACING_A::_2US),
            3 => Val(TSWITCHSPACING_A::_1US),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == TSWITCHSPACING_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == TSWITCHSPACING_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == TSWITCHSPACING_A::_1US
    }
}
#[doc = "Write proxy for field `TSWITCHSPACING`"]
pub struct TSWITCHSPACING_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWITCHSPACING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSWITCHSPACING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(TSWITCHSPACING_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(TSWITCHSPACING_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(TSWITCHSPACING_A::_1US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Interval between samples in the REFERENCE period\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSAMPLESPACINGREF_A {
    #[doc = "1: 4us"]
    _4US,
    #[doc = "2: 2us"]
    _2US,
    #[doc = "3: 1us"]
    _1US,
    #[doc = "4: 0.5us"]
    _500NS,
    #[doc = "5: 0.25us"]
    _250NS,
    #[doc = "6: 0.125us"]
    _125NS,
}
impl From<TSAMPLESPACINGREF_A> for u8 {
    #[inline(always)]
    fn from(variant: TSAMPLESPACINGREF_A) -> Self {
        match variant {
            TSAMPLESPACINGREF_A::_4US => 1,
            TSAMPLESPACINGREF_A::_2US => 2,
            TSAMPLESPACINGREF_A::_1US => 3,
            TSAMPLESPACINGREF_A::_500NS => 4,
            TSAMPLESPACINGREF_A::_250NS => 5,
            TSAMPLESPACINGREF_A::_125NS => 6,
        }
    }
}
#[doc = "Reader of field `TSAMPLESPACINGREF`"]
pub type TSAMPLESPACINGREF_R = crate::R<u8, TSAMPLESPACINGREF_A>;
impl TSAMPLESPACINGREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSAMPLESPACINGREF_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TSAMPLESPACINGREF_A::_4US),
            2 => Val(TSAMPLESPACINGREF_A::_2US),
            3 => Val(TSAMPLESPACINGREF_A::_1US),
            4 => Val(TSAMPLESPACINGREF_A::_500NS),
            5 => Val(TSAMPLESPACINGREF_A::_250NS),
            6 => Val(TSAMPLESPACINGREF_A::_125NS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == TSAMPLESPACINGREF_A::_125NS
    }
}
#[doc = "Write proxy for field `TSAMPLESPACINGREF`"]
pub struct TSAMPLESPACINGREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLESPACINGREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSAMPLESPACINGREF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_1US)
    }
    #[doc = "0.5us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_500NS)
    }
    #[doc = "0.25us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_250NS)
    }
    #[doc = "0.125us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACINGREF_A::_125NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Whether to sample I/Q or magnitude/phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLETYPE_A {
    #[doc = "0: Complex samples in I and Q"]
    IQ,
    #[doc = "1: Complex samples as magnitude and phase"]
    MAGPHASE,
}
impl From<SAMPLETYPE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLETYPE_A) -> Self {
        match variant {
            SAMPLETYPE_A::IQ => false,
            SAMPLETYPE_A::MAGPHASE => true,
        }
    }
}
#[doc = "Reader of field `SAMPLETYPE`"]
pub type SAMPLETYPE_R = crate::R<bool, SAMPLETYPE_A>;
impl SAMPLETYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLETYPE_A {
        match self.bits {
            false => SAMPLETYPE_A::IQ,
            true => SAMPLETYPE_A::MAGPHASE,
        }
    }
    #[doc = "Checks if the value of the field is `IQ`"]
    #[inline(always)]
    pub fn is_iq(&self) -> bool {
        *self == SAMPLETYPE_A::IQ
    }
    #[doc = "Checks if the value of the field is `MAGPHASE`"]
    #[inline(always)]
    pub fn is_mag_phase(&self) -> bool {
        *self == SAMPLETYPE_A::MAGPHASE
    }
}
#[doc = "Write proxy for field `SAMPLETYPE`"]
pub struct SAMPLETYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLETYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLETYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Complex samples in I and Q"]
    #[inline(always)]
    pub fn iq(self) -> &'a mut W {
        self.variant(SAMPLETYPE_A::IQ)
    }
    #[doc = "Complex samples as magnitude and phase"]
    #[inline(always)]
    pub fn mag_phase(self) -> &'a mut W {
        self.variant(SAMPLETYPE_A::MAGPHASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSAMPLESPACING_A {
    #[doc = "1: 4us"]
    _4US,
    #[doc = "2: 2us"]
    _2US,
    #[doc = "3: 1us"]
    _1US,
    #[doc = "4: 0.5us"]
    _500NS,
    #[doc = "5: 0.25us"]
    _250NS,
    #[doc = "6: 0.125us"]
    _125NS,
}
impl From<TSAMPLESPACING_A> for u8 {
    #[inline(always)]
    fn from(variant: TSAMPLESPACING_A) -> Self {
        match variant {
            TSAMPLESPACING_A::_4US => 1,
            TSAMPLESPACING_A::_2US => 2,
            TSAMPLESPACING_A::_1US => 3,
            TSAMPLESPACING_A::_500NS => 4,
            TSAMPLESPACING_A::_250NS => 5,
            TSAMPLESPACING_A::_125NS => 6,
        }
    }
}
#[doc = "Reader of field `TSAMPLESPACING`"]
pub type TSAMPLESPACING_R = crate::R<u8, TSAMPLESPACING_A>;
impl TSAMPLESPACING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSAMPLESPACING_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TSAMPLESPACING_A::_4US),
            2 => Val(TSAMPLESPACING_A::_2US),
            3 => Val(TSAMPLESPACING_A::_1US),
            4 => Val(TSAMPLESPACING_A::_500NS),
            5 => Val(TSAMPLESPACING_A::_250NS),
            6 => Val(TSAMPLESPACING_A::_125NS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == TSAMPLESPACING_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == TSAMPLESPACING_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == TSAMPLESPACING_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == TSAMPLESPACING_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == TSAMPLESPACING_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == TSAMPLESPACING_A::_125NS
    }
}
#[doc = "Write proxy for field `TSAMPLESPACING`"]
pub struct TSAMPLESPACING_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLESPACING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSAMPLESPACING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_1US)
    }
    #[doc = "0.5us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_500NS)
    }
    #[doc = "0.25us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_250NS)
    }
    #[doc = "0.125us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(TSAMPLESPACING_A::_125NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Repeat every antenna pattern N times.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPEATPATTERN_A {
    #[doc = "0: Do not repeat (1 time in total)"]
    NOREPEAT,
}
impl From<REPEATPATTERN_A> for u8 {
    #[inline(always)]
    fn from(variant: REPEATPATTERN_A) -> Self {
        match variant {
            REPEATPATTERN_A::NOREPEAT => 0,
        }
    }
}
#[doc = "Reader of field `REPEATPATTERN`"]
pub type REPEATPATTERN_R = crate::R<u8, REPEATPATTERN_A>;
impl REPEATPATTERN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REPEATPATTERN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REPEATPATTERN_A::NOREPEAT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOREPEAT`"]
    #[inline(always)]
    pub fn is_no_repeat(&self) -> bool {
        *self == REPEATPATTERN_A::NOREPEAT
    }
}
#[doc = "Write proxy for field `REPEATPATTERN`"]
pub struct REPEATPATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEATPATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPEATPATTERN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not repeat (1 time in total)"]
    #[inline(always)]
    pub fn no_repeat(self) -> &'a mut W {
        self.variant(REPEATPATTERN_A::NOREPEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `AGCBACKOFFGAIN`"]
pub type AGCBACKOFFGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AGCBACKOFFGAIN`"]
pub struct AGCBACKOFFGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AGCBACKOFFGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&self) -> NUMBEROF8US_R {
        NUMBEROF8US_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&self) -> DFEINEXTENSION_R {
        DFEINEXTENSION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&self) -> TSWITCHSPACING_R {
        TSWITCHSPACING_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&self) -> TSAMPLESPACINGREF_R {
        TSAMPLESPACINGREF_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&self) -> SAMPLETYPE_R {
        SAMPLETYPE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&self) -> TSAMPLESPACING_R {
        TSAMPLESPACING_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Repeat every antenna pattern N times."]
    #[inline(always)]
    pub fn repeatpattern(&self) -> REPEATPATTERN_R {
        REPEATPATTERN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&self) -> AGCBACKOFFGAIN_R {
        AGCBACKOFFGAIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&mut self) -> NUMBEROF8US_W {
        NUMBEROF8US_W { w: self }
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&mut self) -> DFEINEXTENSION_W {
        DFEINEXTENSION_W { w: self }
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&mut self) -> TSWITCHSPACING_W {
        TSWITCHSPACING_W { w: self }
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&mut self) -> TSAMPLESPACINGREF_W {
        TSAMPLESPACINGREF_W { w: self }
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&mut self) -> SAMPLETYPE_W {
        SAMPLETYPE_W { w: self }
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&mut self) -> TSAMPLESPACING_W {
        TSAMPLESPACING_W { w: self }
    }
    #[doc = "Bits 20:23 - Repeat every antenna pattern N times."]
    #[inline(always)]
    pub fn repeatpattern(&mut self) -> REPEATPATTERN_W {
        REPEATPATTERN_W { w: self }
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&mut self) -> AGCBACKOFFGAIN_W {
        AGCBACKOFFGAIN_W { w: self }
    }
}
