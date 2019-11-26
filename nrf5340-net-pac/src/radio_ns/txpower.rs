#[doc = "Reader of register TXPOWER"]
pub type R = crate::R<u32, super::TXPOWER>;
#[doc = "Writer for register TXPOWER"]
pub type W = crate::W<u32, super::TXPOWER>;
#[doc = "Register TXPOWER `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPOWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RADIO output power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOWER_A {
    #[doc = "0: 0 dBm"]
    _0DBM,
    #[doc = "255: -1 dBm"]
    NEG1DBM,
    #[doc = "254: -2 dBm"]
    NEG2DBM,
    #[doc = "253: -3 dBm"]
    NEG3DBM,
    #[doc = "252: -4 dBm"]
    NEG4DBM,
    #[doc = "251: -5 dBm"]
    NEG5DBM,
    #[doc = "250: -6 dBm"]
    NEG6DBM,
    #[doc = "249: -7 dBm"]
    NEG7DBM,
    #[doc = "248: -8 dBm"]
    NEG8DBM,
    #[doc = "244: -12 dBm"]
    NEG12DBM,
    #[doc = "240: -16 dBm"]
    NEG16DBM,
    #[doc = "236: -20 dBm"]
    NEG20DBM,
    #[doc = "226: Deprecated enumerator -  -40 dBm"]
    NEG30DBM,
    #[doc = "216: -40 dBm"]
    NEG40DBM,
}
impl From<TXPOWER_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOWER_A) -> Self {
        match variant {
            TXPOWER_A::_0DBM => 0,
            TXPOWER_A::NEG1DBM => 255,
            TXPOWER_A::NEG2DBM => 254,
            TXPOWER_A::NEG3DBM => 253,
            TXPOWER_A::NEG4DBM => 252,
            TXPOWER_A::NEG5DBM => 251,
            TXPOWER_A::NEG6DBM => 250,
            TXPOWER_A::NEG7DBM => 249,
            TXPOWER_A::NEG8DBM => 248,
            TXPOWER_A::NEG12DBM => 244,
            TXPOWER_A::NEG16DBM => 240,
            TXPOWER_A::NEG20DBM => 236,
            TXPOWER_A::NEG30DBM => 226,
            TXPOWER_A::NEG40DBM => 216,
        }
    }
}
#[doc = "Reader of field `TXPOWER`"]
pub type TXPOWER_R = crate::R<u8, TXPOWER_A>;
impl TXPOWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPOWER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPOWER_A::_0DBM),
            255 => Val(TXPOWER_A::NEG1DBM),
            254 => Val(TXPOWER_A::NEG2DBM),
            253 => Val(TXPOWER_A::NEG3DBM),
            252 => Val(TXPOWER_A::NEG4DBM),
            251 => Val(TXPOWER_A::NEG5DBM),
            250 => Val(TXPOWER_A::NEG6DBM),
            249 => Val(TXPOWER_A::NEG7DBM),
            248 => Val(TXPOWER_A::NEG8DBM),
            244 => Val(TXPOWER_A::NEG12DBM),
            240 => Val(TXPOWER_A::NEG16DBM),
            236 => Val(TXPOWER_A::NEG20DBM),
            226 => Val(TXPOWER_A::NEG30DBM),
            216 => Val(TXPOWER_A::NEG40DBM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0DBM`"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == TXPOWER_A::_0DBM
    }
    #[doc = "Checks if the value of the field is `NEG1DBM`"]
    #[inline(always)]
    pub fn is_neg1d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG1DBM
    }
    #[doc = "Checks if the value of the field is `NEG2DBM`"]
    #[inline(always)]
    pub fn is_neg2d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG2DBM
    }
    #[doc = "Checks if the value of the field is `NEG3DBM`"]
    #[inline(always)]
    pub fn is_neg3d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG3DBM
    }
    #[doc = "Checks if the value of the field is `NEG4DBM`"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG4DBM
    }
    #[doc = "Checks if the value of the field is `NEG5DBM`"]
    #[inline(always)]
    pub fn is_neg5d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG5DBM
    }
    #[doc = "Checks if the value of the field is `NEG6DBM`"]
    #[inline(always)]
    pub fn is_neg6d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG6DBM
    }
    #[doc = "Checks if the value of the field is `NEG7DBM`"]
    #[inline(always)]
    pub fn is_neg7d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG7DBM
    }
    #[doc = "Checks if the value of the field is `NEG8DBM`"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG8DBM
    }
    #[doc = "Checks if the value of the field is `NEG12DBM`"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG12DBM
    }
    #[doc = "Checks if the value of the field is `NEG16DBM`"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG16DBM
    }
    #[doc = "Checks if the value of the field is `NEG20DBM`"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG20DBM
    }
    #[doc = "Checks if the value of the field is `NEG30DBM`"]
    #[inline(always)]
    pub fn is_neg30d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG30DBM
    }
    #[doc = "Checks if the value of the field is `NEG40DBM`"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG40DBM
    }
}
#[doc = "Write proxy for field `TXPOWER`"]
pub struct TXPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPOWER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::_0DBM)
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn neg1d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG1DBM)
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn neg2d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG2DBM)
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn neg3d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG3DBM)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG4DBM)
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn neg5d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG5DBM)
    }
    #[doc = "-6 dBm"]
    #[inline(always)]
    pub fn neg6d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG6DBM)
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn neg7d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG7DBM)
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG8DBM)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG12DBM)
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG16DBM)
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG20DBM)
    }
    #[doc = "Deprecated enumerator - -40 dBm"]
    #[inline(always)]
    pub fn neg30d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG30DBM)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG40DBM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&self) -> TXPOWER_R {
        TXPOWER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&mut self) -> TXPOWER_W {
        TXPOWER_W { w: self }
    }
}
