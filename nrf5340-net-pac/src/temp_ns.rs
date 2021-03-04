#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start temperature measurement"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop temperature measurement"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    _reserved4: [u8; 120usize],
    #[doc = "0x100 - Temperature measurement complete, data ready"]
    pub events_datardy: crate::Reg<events_datardy::EVENTS_DATARDY_SPEC>,
    _reserved5: [u8; 124usize],
    #[doc = "0x180 - Publish configuration for event DATARDY"]
    pub publish_datardy: crate::Reg<publish_datardy::PUBLISH_DATARDY_SPEC>,
    _reserved6: [u8; 384usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved8: [u8; 508usize],
    #[doc = "0x508 - Temperature in degC (0.25deg steps)"]
    pub temp: crate::Reg<temp::TEMP_SPEC>,
    _reserved9: [u8; 20usize],
    #[doc = "0x520 - Slope of first piecewise linear function"]
    pub a0: crate::Reg<a0::A0_SPEC>,
    #[doc = "0x524 - Slope of second piecewise linear function"]
    pub a1: crate::Reg<a1::A1_SPEC>,
    #[doc = "0x528 - Slope of third piecewise linear function"]
    pub a2: crate::Reg<a2::A2_SPEC>,
    #[doc = "0x52c - Slope of fourth piecewise linear function"]
    pub a3: crate::Reg<a3::A3_SPEC>,
    #[doc = "0x530 - Slope of fifth piecewise linear function"]
    pub a4: crate::Reg<a4::A4_SPEC>,
    #[doc = "0x534 - Slope of sixth piecewise linear function"]
    pub a5: crate::Reg<a5::A5_SPEC>,
    _reserved15: [u8; 8usize],
    #[doc = "0x540 - y-intercept of first piecewise linear function"]
    pub b0: crate::Reg<b0::B0_SPEC>,
    #[doc = "0x544 - y-intercept of second piecewise linear function"]
    pub b1: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x548 - y-intercept of third piecewise linear function"]
    pub b2: crate::Reg<b2::B2_SPEC>,
    #[doc = "0x54c - y-intercept of fourth piecewise linear function"]
    pub b3: crate::Reg<b3::B3_SPEC>,
    #[doc = "0x550 - y-intercept of fifth piecewise linear function"]
    pub b4: crate::Reg<b4::B4_SPEC>,
    #[doc = "0x554 - y-intercept of sixth piecewise linear function"]
    pub b5: crate::Reg<b5::B5_SPEC>,
    _reserved21: [u8; 8usize],
    #[doc = "0x560 - Endpoint of first piecewise linear function"]
    pub t0: crate::Reg<t0::T0_SPEC>,
    #[doc = "0x564 - Endpoint of second piecewise linear function"]
    pub t1: crate::Reg<t1::T1_SPEC>,
    #[doc = "0x568 - Endpoint of third piecewise linear function"]
    pub t2: crate::Reg<t2::T2_SPEC>,
    #[doc = "0x56c - Endpoint of fourth piecewise linear function"]
    pub t3: crate::Reg<t3::T3_SPEC>,
    #[doc = "0x570 - Endpoint of fifth piecewise linear function"]
    pub t4: crate::Reg<t4::T4_SPEC>,
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start temperature measurement"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop temperature measurement"]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_DATARDY register accessor: an alias for `Reg<EVENTS_DATARDY_SPEC>`"]
pub type EVENTS_DATARDY = crate::Reg<events_datardy::EVENTS_DATARDY_SPEC>;
#[doc = "Temperature measurement complete, data ready"]
pub mod events_datardy;
#[doc = "PUBLISH_DATARDY register accessor: an alias for `Reg<PUBLISH_DATARDY_SPEC>`"]
pub type PUBLISH_DATARDY = crate::Reg<publish_datardy::PUBLISH_DATARDY_SPEC>;
#[doc = "Publish configuration for event DATARDY"]
pub mod publish_datardy;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "TEMP register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temperature in degC (0.25deg steps)"]
pub mod temp;
#[doc = "A0 register accessor: an alias for `Reg<A0_SPEC>`"]
pub type A0 = crate::Reg<a0::A0_SPEC>;
#[doc = "Slope of first piecewise linear function"]
pub mod a0;
#[doc = "A1 register accessor: an alias for `Reg<A1_SPEC>`"]
pub type A1 = crate::Reg<a1::A1_SPEC>;
#[doc = "Slope of second piecewise linear function"]
pub mod a1;
#[doc = "A2 register accessor: an alias for `Reg<A2_SPEC>`"]
pub type A2 = crate::Reg<a2::A2_SPEC>;
#[doc = "Slope of third piecewise linear function"]
pub mod a2;
#[doc = "A3 register accessor: an alias for `Reg<A3_SPEC>`"]
pub type A3 = crate::Reg<a3::A3_SPEC>;
#[doc = "Slope of fourth piecewise linear function"]
pub mod a3;
#[doc = "A4 register accessor: an alias for `Reg<A4_SPEC>`"]
pub type A4 = crate::Reg<a4::A4_SPEC>;
#[doc = "Slope of fifth piecewise linear function"]
pub mod a4;
#[doc = "A5 register accessor: an alias for `Reg<A5_SPEC>`"]
pub type A5 = crate::Reg<a5::A5_SPEC>;
#[doc = "Slope of sixth piecewise linear function"]
pub mod a5;
#[doc = "B0 register accessor: an alias for `Reg<B0_SPEC>`"]
pub type B0 = crate::Reg<b0::B0_SPEC>;
#[doc = "y-intercept of first piecewise linear function"]
pub mod b0;
#[doc = "B1 register accessor: an alias for `Reg<B1_SPEC>`"]
pub type B1 = crate::Reg<b1::B1_SPEC>;
#[doc = "y-intercept of second piecewise linear function"]
pub mod b1;
#[doc = "B2 register accessor: an alias for `Reg<B2_SPEC>`"]
pub type B2 = crate::Reg<b2::B2_SPEC>;
#[doc = "y-intercept of third piecewise linear function"]
pub mod b2;
#[doc = "B3 register accessor: an alias for `Reg<B3_SPEC>`"]
pub type B3 = crate::Reg<b3::B3_SPEC>;
#[doc = "y-intercept of fourth piecewise linear function"]
pub mod b3;
#[doc = "B4 register accessor: an alias for `Reg<B4_SPEC>`"]
pub type B4 = crate::Reg<b4::B4_SPEC>;
#[doc = "y-intercept of fifth piecewise linear function"]
pub mod b4;
#[doc = "B5 register accessor: an alias for `Reg<B5_SPEC>`"]
pub type B5 = crate::Reg<b5::B5_SPEC>;
#[doc = "y-intercept of sixth piecewise linear function"]
pub mod b5;
#[doc = "T0 register accessor: an alias for `Reg<T0_SPEC>`"]
pub type T0 = crate::Reg<t0::T0_SPEC>;
#[doc = "Endpoint of first piecewise linear function"]
pub mod t0;
#[doc = "T1 register accessor: an alias for `Reg<T1_SPEC>`"]
pub type T1 = crate::Reg<t1::T1_SPEC>;
#[doc = "Endpoint of second piecewise linear function"]
pub mod t1;
#[doc = "T2 register accessor: an alias for `Reg<T2_SPEC>`"]
pub type T2 = crate::Reg<t2::T2_SPEC>;
#[doc = "Endpoint of third piecewise linear function"]
pub mod t2;
#[doc = "T3 register accessor: an alias for `Reg<T3_SPEC>`"]
pub type T3 = crate::Reg<t3::T3_SPEC>;
#[doc = "Endpoint of fourth piecewise linear function"]
pub mod t3;
#[doc = "T4 register accessor: an alias for `Reg<T4_SPEC>`"]
pub type T4 = crate::Reg<t4::T4_SPEC>;
#[doc = "Endpoint of fifth piecewise linear function"]
pub mod t4;
