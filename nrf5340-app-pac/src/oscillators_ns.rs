#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1476usize],
    #[doc = "0x5c4 - Programmable capacitance of XC1 and XC2"]
    pub xosc32mcaps: XOSC32MCAPS,
    _reserved1: [u8; 248usize],
    #[doc = "0x6c0 - Unspecified"]
    pub xosc32ki: XOSC32KI,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct XOSC32KI {
    #[doc = "0x00 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    pub bypass: self::xosc32ki::BYPASS,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Control usage of internal load capacitors"]
    pub intcap: self::xosc32ki::INTCAP,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod xosc32ki;
#[doc = "Programmable capacitance of XC1 and XC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xosc32mcaps](xosc32mcaps) module"]
pub type XOSC32MCAPS = crate::Reg<u32, _XOSC32MCAPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC32MCAPS;
#[doc = "`read()` method returns [xosc32mcaps::R](xosc32mcaps::R) reader structure"]
impl crate::Readable for XOSC32MCAPS {}
#[doc = "`write(|w| ..)` method takes [xosc32mcaps::W](xosc32mcaps::W) writer structure"]
impl crate::Writable for XOSC32MCAPS {}
#[doc = "Programmable capacitance of XC1 and XC2"]
pub mod xosc32mcaps;
