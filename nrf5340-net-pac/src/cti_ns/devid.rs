#[doc = "Reader of register DEVID"]
pub type R = crate::R<u32, super::DEVID>;
#[doc = "Reader of field `EXTMUXNUM`"]
pub type EXTMUXNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUMTRIG`"]
pub type NUMTRIG_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUMCH`"]
pub type NUMCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
    #[inline(always)]
    pub fn extmuxnum(&self) -> EXTMUXNUM_R {
        EXTMUXNUM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Number of ECT triggers available"]
    #[inline(always)]
    pub fn numtrig(&self) -> NUMTRIG_R {
        NUMTRIG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Number of ECT channels available"]
    #[inline(always)]
    pub fn numch(&self) -> NUMCH_R {
        NUMCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
