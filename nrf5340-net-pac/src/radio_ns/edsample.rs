#[doc = "Reader of register EDSAMPLE"]
pub type R = crate::R<u32, super::EDSAMPLE>;
#[doc = "Reader of field `EDLVL`"]
pub type EDLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EDLVL_R {
        EDLVL_R::new((self.bits & 0xff) as u8)
    }
}
