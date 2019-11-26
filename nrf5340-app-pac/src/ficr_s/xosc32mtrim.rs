#[doc = "Reader of register XOSC32MTRIM"]
pub type R = crate::R<u32, super::XOSC32MTRIM>;
#[doc = "Reader of field `SLOPE`"]
pub type SLOPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Slope trim factor on twos complement form"]
    #[inline(always)]
    pub fn slope(&self) -> SLOPE_R {
        SLOPE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Offset trim factor on integer form"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
