#[doc = "Reader of register CTESTATUS"]
pub type R = crate::R<u32, super::CTESTATUS>;
#[doc = "Reader of field `CTETIME`"]
pub type CTETIME_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFU`"]
pub type RFU_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTETYPE`"]
pub type CTETYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - CTETime parsed from packet"]
    #[inline(always)]
    pub fn ctetime(&self) -> CTETIME_R {
        CTETIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - RFU parsed from packet"]
    #[inline(always)]
    pub fn rfu(&self) -> RFU_R {
        RFU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - CTEType parsed from packet"]
    #[inline(always)]
    pub fn ctetype(&self) -> CTETYPE_R {
        CTETYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
