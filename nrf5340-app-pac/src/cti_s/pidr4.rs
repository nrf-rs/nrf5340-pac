#[doc = "Reader of register PIDR4"]
pub type R = crate::R<u32, super::PIDR4>;
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DES_2_A {
    #[doc = "4: JEDEC continuation code"]
    CODE,
}
impl From<DES_2_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_2_A) -> Self {
        match variant {
            DES_2_A::CODE => 4,
        }
    }
}
#[doc = "Reader of field `DES_2`"]
pub type DES_2_R = crate::R<u8, DES_2_A>;
impl DES_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DES_2_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(DES_2_A::CODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODE`"]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        *self == DES_2_A::CODE
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Always 0b0000. Indicates that the device only occupies 4KB of memory"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
