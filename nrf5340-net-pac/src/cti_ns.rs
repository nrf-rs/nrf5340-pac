#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTI Control register"]
    pub cticontrol: CTICONTROL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - CTI Interrupt Acknowledge register"]
    pub ctiintack: CTIINTACK,
    #[doc = "0x14 - CTI Application Trigger Set register"]
    pub ctiappset: CTIAPPSET,
    #[doc = "0x18 - CTI Application Trigger Clear register"]
    pub ctiappclear: CTIAPPCLEAR,
    #[doc = "0x1c - CTI Application Pulse register"]
    pub ctiapppulse: CTIAPPPULSE,
    #[doc = "0x20 - Description collection: CTI Trigger input"]
    pub ctiinen: [CTIINEN; 8],
    _reserved6: [u8; 96usize],
    #[doc = "0xa0 - Description collection: CTI Trigger output"]
    pub ctiouten: [CTIOUTEN; 8],
    _reserved7: [u8; 112usize],
    #[doc = "0x130 - CTI Trigger In Status register"]
    pub ctitriginstatus: CTITRIGINSTATUS,
    #[doc = "0x134 - CTI Trigger Out Status register"]
    pub ctitrigoutstatus: CTITRIGOUTSTATUS,
    #[doc = "0x138 - CTI Channel In Status register"]
    pub ctichinstatus: CTICHINSTATUS,
    _reserved10: [u8; 4usize],
    #[doc = "0x140 - Enable CTI Channel Gate register"]
    pub ctigate: CTIGATE,
    _reserved11: [u8; 3704usize],
    #[doc = "0xfbc - Device Architecture register"]
    pub devarch: DEVARCH,
    _reserved12: [u8; 8usize],
    #[doc = "0xfc8 - Device Configuration register"]
    pub devid: DEVID,
    #[doc = "0xfcc - Device Type Identifier register"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Peripheral ID5 register"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Peripheral ID6 register"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Peripheral ID7 register"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral ID3 Register"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component ID0 Register"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component ID1 Register"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component ID2 Register"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component ID3 Register"]
    pub cidr3: CIDR3,
}
#[doc = "CTI Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cticontrol](cticontrol) module"]
pub type CTICONTROL = crate::Reg<u32, _CTICONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTICONTROL;
#[doc = "`read()` method returns [cticontrol::R](cticontrol::R) reader structure"]
impl crate::Readable for CTICONTROL {}
#[doc = "`write(|w| ..)` method takes [cticontrol::W](cticontrol::W) writer structure"]
impl crate::Writable for CTICONTROL {}
#[doc = "CTI Control register"]
pub mod cticontrol;
#[doc = "CTI Interrupt Acknowledge register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctiintack](ctiintack) module"]
pub type CTIINTACK = crate::Reg<u32, _CTIINTACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIINTACK;
#[doc = "`write(|w| ..)` method takes [ctiintack::W](ctiintack::W) writer structure"]
impl crate::Writable for CTIINTACK {}
#[doc = "CTI Interrupt Acknowledge register"]
pub mod ctiintack;
#[doc = "CTI Application Trigger Set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctiappset](ctiappset) module"]
pub type CTIAPPSET = crate::Reg<u32, _CTIAPPSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIAPPSET;
#[doc = "`read()` method returns [ctiappset::R](ctiappset::R) reader structure"]
impl crate::Readable for CTIAPPSET {}
#[doc = "`write(|w| ..)` method takes [ctiappset::W](ctiappset::W) writer structure"]
impl crate::Writable for CTIAPPSET {}
#[doc = "CTI Application Trigger Set register"]
pub mod ctiappset;
#[doc = "CTI Application Trigger Clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctiappclear](ctiappclear) module"]
pub type CTIAPPCLEAR = crate::Reg<u32, _CTIAPPCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIAPPCLEAR;
#[doc = "`write(|w| ..)` method takes [ctiappclear::W](ctiappclear::W) writer structure"]
impl crate::Writable for CTIAPPCLEAR {}
#[doc = "CTI Application Trigger Clear register"]
pub mod ctiappclear;
#[doc = "CTI Application Pulse register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctiapppulse](ctiapppulse) module"]
pub type CTIAPPPULSE = crate::Reg<u32, _CTIAPPPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIAPPPULSE;
#[doc = "`write(|w| ..)` method takes [ctiapppulse::W](ctiapppulse::W) writer structure"]
impl crate::Writable for CTIAPPPULSE {}
#[doc = "CTI Application Pulse register"]
pub mod ctiapppulse;
#[doc = "Description collection: CTI Trigger input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctiinen](ctiinen) module"]
pub type CTIINEN = crate::Reg<u32, _CTIINEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIINEN;
#[doc = "`read()` method returns [ctiinen::R](ctiinen::R) reader structure"]
impl crate::Readable for CTIINEN {}
#[doc = "`write(|w| ..)` method takes [ctiinen::W](ctiinen::W) writer structure"]
impl crate::Writable for CTIINEN {}
#[doc = "Description collection: CTI Trigger input"]
pub mod ctiinen;
#[doc = "Description collection: CTI Trigger output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctiouten](ctiouten) module"]
pub type CTIOUTEN = crate::Reg<u32, _CTIOUTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIOUTEN;
#[doc = "`read()` method returns [ctiouten::R](ctiouten::R) reader structure"]
impl crate::Readable for CTIOUTEN {}
#[doc = "`write(|w| ..)` method takes [ctiouten::W](ctiouten::W) writer structure"]
impl crate::Writable for CTIOUTEN {}
#[doc = "Description collection: CTI Trigger output"]
pub mod ctiouten;
#[doc = "CTI Trigger In Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctitriginstatus](ctitriginstatus) module"]
pub type CTITRIGINSTATUS = crate::Reg<u32, _CTITRIGINSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTITRIGINSTATUS;
#[doc = "`read()` method returns [ctitriginstatus::R](ctitriginstatus::R) reader structure"]
impl crate::Readable for CTITRIGINSTATUS {}
#[doc = "CTI Trigger In Status register"]
pub mod ctitriginstatus;
#[doc = "CTI Trigger Out Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctitrigoutstatus](ctitrigoutstatus) module"]
pub type CTITRIGOUTSTATUS = crate::Reg<u32, _CTITRIGOUTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTITRIGOUTSTATUS;
#[doc = "`read()` method returns [ctitrigoutstatus::R](ctitrigoutstatus::R) reader structure"]
impl crate::Readable for CTITRIGOUTSTATUS {}
#[doc = "CTI Trigger Out Status register"]
pub mod ctitrigoutstatus;
#[doc = "CTI Channel In Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctichinstatus](ctichinstatus) module"]
pub type CTICHINSTATUS = crate::Reg<u32, _CTICHINSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTICHINSTATUS;
#[doc = "`read()` method returns [ctichinstatus::R](ctichinstatus::R) reader structure"]
impl crate::Readable for CTICHINSTATUS {}
#[doc = "CTI Channel In Status register"]
pub mod ctichinstatus;
#[doc = "Enable CTI Channel Gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctigate](ctigate) module"]
pub type CTIGATE = crate::Reg<u32, _CTIGATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIGATE;
#[doc = "`read()` method returns [ctigate::R](ctigate::R) reader structure"]
impl crate::Readable for CTIGATE {}
#[doc = "`write(|w| ..)` method takes [ctigate::W](ctigate::W) writer structure"]
impl crate::Writable for CTIGATE {}
#[doc = "Enable CTI Channel Gate register"]
pub mod ctigate;
#[doc = "Device Architecture register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devarch](devarch) module"]
pub type DEVARCH = crate::Reg<u32, _DEVARCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVARCH;
#[doc = "`read()` method returns [devarch::R](devarch::R) reader structure"]
impl crate::Readable for DEVARCH {}
#[doc = "Device Architecture register"]
pub mod devarch;
#[doc = "Device Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devid](devid) module"]
pub type DEVID = crate::Reg<u32, _DEVID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVID;
#[doc = "`read()` method returns [devid::R](devid::R) reader structure"]
impl crate::Readable for DEVID {}
#[doc = "Device Configuration register"]
pub mod devid;
#[doc = "Device Type Identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devtype](devtype) module"]
pub type DEVTYPE = crate::Reg<u32, _DEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVTYPE;
#[doc = "`read()` method returns [devtype::R](devtype::R) reader structure"]
impl crate::Readable for DEVTYPE {}
#[doc = "Device Type Identifier register"]
pub mod devtype;
#[doc = "Peripheral ID4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Peripheral ID4 Register"]
pub mod pidr4;
#[doc = "Peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr5](pidr5) module"]
pub type PIDR5 = crate::Reg<u32, _PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR5;
#[doc = "`read()` method returns [pidr5::R](pidr5::R) reader structure"]
impl crate::Readable for PIDR5 {}
#[doc = "Peripheral ID5 register"]
pub mod pidr5;
#[doc = "Peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr6](pidr6) module"]
pub type PIDR6 = crate::Reg<u32, _PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR6;
#[doc = "`read()` method returns [pidr6::R](pidr6::R) reader structure"]
impl crate::Readable for PIDR6 {}
#[doc = "Peripheral ID6 register"]
pub mod pidr6;
#[doc = "Peripheral ID7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr7](pidr7) module"]
pub type PIDR7 = crate::Reg<u32, _PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR7;
#[doc = "`read()` method returns [pidr7::R](pidr7::R) reader structure"]
impl crate::Readable for PIDR7 {}
#[doc = "Peripheral ID7 register"]
pub mod pidr7;
#[doc = "Peripheral ID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "Peripheral ID0 Register"]
pub mod pidr0;
#[doc = "Peripheral ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "Peripheral ID1 Register"]
pub mod pidr1;
#[doc = "Peripheral ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "Peripheral ID2 Register"]
pub mod pidr2;
#[doc = "Peripheral ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "Peripheral ID3 Register"]
pub mod pidr3;
#[doc = "Component ID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "Component ID0 Register"]
pub mod cidr0;
#[doc = "Component ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "Component ID1 Register"]
pub mod cidr1;
#[doc = "Component ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "Component ID2 Register"]
pub mod cidr2;
#[doc = "Component ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "Component ID3 Register"]
pub mod cidr3;
