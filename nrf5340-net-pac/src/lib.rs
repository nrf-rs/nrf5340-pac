#![doc = "Peripheral access API for NRF5340_NETWORK microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn CLOCK_POWER();
    fn RADIO();
    fn RNG();
    fn GPIOTE();
    fn WDT();
    fn TIMER0();
    fn ECB();
    fn AAR_CCM();
    fn TEMP();
    fn RTC0();
    fn IPC();
    fn SPIM0_SPIS0_TWIM0_TWIS0_UARTE0();
    fn EGU0();
    fn RTC1();
    fn TIMER1();
    fn TIMER2();
    fn SWI0();
    fn SWI1();
    fn SWI2();
    fn SWI3();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 30] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: CLOCK_POWER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RADIO },
    Vector { _handler: RNG },
    Vector { _handler: GPIOTE },
    Vector { _handler: WDT },
    Vector { _handler: TIMER0 },
    Vector { _handler: ECB },
    Vector { _handler: AAR_CCM },
    Vector { _reserved: 0 },
    Vector { _handler: TEMP },
    Vector { _handler: RTC0 },
    Vector { _handler: IPC },
    Vector {
        _handler: SPIM0_SPIS0_TWIM0_TWIS0_UARTE0,
    },
    Vector { _handler: EGU0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC1 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: SWI0 },
    Vector { _handler: SWI1 },
    Vector { _handler: SWI2 },
    Vector { _handler: SWI3 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER,
    #[doc = "8 - RADIO"]
    RADIO,
    #[doc = "9 - RNG"]
    RNG,
    #[doc = "10 - GPIOTE"]
    GPIOTE,
    #[doc = "11 - WDT"]
    WDT,
    #[doc = "12 - TIMER0"]
    TIMER0,
    #[doc = "13 - ECB"]
    ECB,
    #[doc = "14 - AAR_CCM"]
    AAR_CCM,
    #[doc = "16 - TEMP"]
    TEMP,
    #[doc = "17 - RTC0"]
    RTC0,
    #[doc = "18 - IPC"]
    IPC,
    #[doc = "19 - SPIM0_SPIS0_TWIM0_TWIS0_UARTE0"]
    SPIM0_SPIS0_TWIM0_TWIS0_UARTE0,
    #[doc = "20 - EGU0"]
    EGU0,
    #[doc = "22 - RTC1"]
    RTC1,
    #[doc = "24 - TIMER1"]
    TIMER1,
    #[doc = "25 - TIMER2"]
    TIMER2,
    #[doc = "26 - SWI0"]
    SWI0,
    #[doc = "27 - SWI1"]
    SWI1,
    #[doc = "28 - SWI2"]
    SWI2,
    #[doc = "29 - SWI3"]
    SWI3,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::CLOCK_POWER => 5,
            Interrupt::RADIO => 8,
            Interrupt::RNG => 9,
            Interrupt::GPIOTE => 10,
            Interrupt::WDT => 11,
            Interrupt::TIMER0 => 12,
            Interrupt::ECB => 13,
            Interrupt::AAR_CCM => 14,
            Interrupt::TEMP => 16,
            Interrupt::RTC0 => 17,
            Interrupt::IPC => 18,
            Interrupt::SPIM0_SPIS0_TWIM0_TWIS0_UARTE0 => 19,
            Interrupt::EGU0 => 20,
            Interrupt::RTC1 => 22,
            Interrupt::TIMER1 => 24,
            Interrupt::TIMER2 => 25,
            Interrupt::SWI0 => 26,
            Interrupt::SWI1 => 27,
            Interrupt::SWI2 => 28,
            Interrupt::SWI3 => 29,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Factory Information Configuration Registers"]
pub struct FICR_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR_NS {}
impl FICR_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ficr_ns::RegisterBlock {
        0x01ff_0000 as *const _
    }
}
impl Deref for FICR_NS {
    type Target = ficr_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FICR_NS::ptr() }
    }
}
#[doc = "Factory Information Configuration Registers"]
pub mod ficr_ns;
#[doc = "User Information Configuration Registers"]
pub struct UICR_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR_NS {}
impl UICR_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uicr_ns::RegisterBlock {
        0x01ff_8000 as *const _
    }
}
impl Deref for UICR_NS {
    type Target = uicr_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UICR_NS::ptr() }
    }
}
#[doc = "User Information Configuration Registers"]
pub mod uicr_ns;
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub struct CTI_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTI_NS {}
impl CTI_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cti_ns::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for CTI_NS {
    type Target = cti_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTI_NS::ptr() }
    }
}
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub mod cti_ns;
#[doc = "Domain configuration management"]
pub struct DCNF_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCNF_NS {}
impl DCNF_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcnf_ns::RegisterBlock {
        0x4100_0000 as *const _
    }
}
impl Deref for DCNF_NS {
    type Target = dcnf_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCNF_NS::ptr() }
    }
}
#[doc = "Domain configuration management"]
pub mod dcnf_ns;
#[doc = "Voltage request control"]
pub struct VREQCTRL_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREQCTRL_NS {}
impl VREQCTRL_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vreqctrl_ns::RegisterBlock {
        0x4100_4000 as *const _
    }
}
impl Deref for VREQCTRL_NS {
    type Target = vreqctrl_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREQCTRL_NS::ptr() }
    }
}
#[doc = "Voltage request control"]
pub mod vreqctrl_ns;
#[doc = "Clock management"]
pub struct CLOCK_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK_NS {}
impl CLOCK_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clock_ns::RegisterBlock {
        0x4100_5000 as *const _
    }
}
impl Deref for CLOCK_NS {
    type Target = clock_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLOCK_NS::ptr() }
    }
}
#[doc = "Clock management"]
pub mod clock_ns;
#[doc = "Power control"]
pub struct POWER_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER_NS {}
impl POWER_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const power_ns::RegisterBlock {
        0x4100_5000 as *const _
    }
}
impl Deref for POWER_NS {
    type Target = power_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*POWER_NS::ptr() }
    }
}
#[doc = "Power control"]
pub mod power_ns;
#[doc = "Reset control"]
pub struct RESET_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RESET_NS {}
impl RESET_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const reset_ns::RegisterBlock {
        0x4100_5000 as *const _
    }
}
impl Deref for RESET_NS {
    type Target = reset_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RESET_NS::ptr() }
    }
}
#[doc = "Reset control"]
pub mod reset_ns;
#[doc = "Control access port"]
pub struct CTRLAP_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRLAP_NS {}
impl CTRLAP_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctrlap_ns::RegisterBlock {
        0x4100_6000 as *const _
    }
}
impl Deref for CTRLAP_NS {
    type Target = ctrlap_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTRLAP_NS::ptr() }
    }
}
#[doc = "Control access port"]
pub mod ctrlap_ns;
#[doc = "2.4 GHz radio"]
pub struct RADIO_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RADIO_NS {}
impl RADIO_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const radio_ns::RegisterBlock {
        0x4100_8000 as *const _
    }
}
impl Deref for RADIO_NS {
    type Target = radio_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RADIO_NS::ptr() }
    }
}
#[doc = "2.4 GHz radio"]
pub mod radio_ns;
#[doc = "Random Number Generator"]
pub struct RNG_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG_NS {}
impl RNG_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng_ns::RegisterBlock {
        0x4100_9000 as *const _
    }
}
impl Deref for RNG_NS {
    type Target = rng_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG_NS::ptr() }
    }
}
#[doc = "Random Number Generator"]
pub mod rng_ns;
#[doc = "GPIO Tasks and Events"]
pub struct GPIOTE_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE_NS {}
impl GPIOTE_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiote_ns::RegisterBlock {
        0x4100_a000 as *const _
    }
}
impl Deref for GPIOTE_NS {
    type Target = gpiote_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOTE_NS::ptr() }
    }
}
#[doc = "GPIO Tasks and Events"]
pub mod gpiote_ns;
#[doc = "Watchdog Timer"]
pub struct WDT_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_NS {}
impl WDT_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt_ns::RegisterBlock {
        0x4100_b000 as *const _
    }
}
impl Deref for WDT_NS {
    type Target = wdt_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT_NS::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt_ns;
#[doc = "Timer/Counter 0"]
pub struct TIMER0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_NS {}
impl TIMER0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x4100_c000 as *const _
    }
}
impl Deref for TIMER0_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0_NS::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod timer0_ns;
#[doc = "AES ECB Mode Encryption"]
pub struct ECB_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECB_NS {}
impl ECB_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ecb_ns::RegisterBlock {
        0x4100_d000 as *const _
    }
}
impl Deref for ECB_NS {
    type Target = ecb_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ECB_NS::ptr() }
    }
}
#[doc = "AES ECB Mode Encryption"]
pub mod ecb_ns;
#[doc = "Accelerated Address Resolver"]
pub struct AAR_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AAR_NS {}
impl AAR_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aar_ns::RegisterBlock {
        0x4100_e000 as *const _
    }
}
impl Deref for AAR_NS {
    type Target = aar_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AAR_NS::ptr() }
    }
}
#[doc = "Accelerated Address Resolver"]
pub mod aar_ns;
#[doc = "AES CCM mode encryption"]
pub struct CCM_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM_NS {}
impl CCM_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm_ns::RegisterBlock {
        0x4100_e000 as *const _
    }
}
impl Deref for CCM_NS {
    type Target = ccm_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCM_NS::ptr() }
    }
}
#[doc = "AES CCM mode encryption"]
pub mod ccm_ns;
#[doc = "Distributed programmable peripheral interconnect controller"]
pub struct DPPIC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPPIC_NS {}
impl DPPIC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dppic_ns::RegisterBlock {
        0x4100_f000 as *const _
    }
}
impl Deref for DPPIC_NS {
    type Target = dppic_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DPPIC_NS::ptr() }
    }
}
#[doc = "Distributed programmable peripheral interconnect controller"]
pub mod dppic_ns;
#[doc = "Temperature Sensor"]
pub struct TEMP_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP_NS {}
impl TEMP_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const temp_ns::RegisterBlock {
        0x4101_0000 as *const _
    }
}
impl Deref for TEMP_NS {
    type Target = temp_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TEMP_NS::ptr() }
    }
}
#[doc = "Temperature Sensor"]
pub mod temp_ns;
#[doc = "Real-time counter 0"]
pub struct RTC0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0_NS {}
impl RTC0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0_ns::RegisterBlock {
        0x4101_1000 as *const _
    }
}
impl Deref for RTC0_NS {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC0_NS::ptr() }
    }
}
#[doc = "Real-time counter 0"]
pub mod rtc0_ns;
#[doc = "Interprocessor communication"]
pub struct IPC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC_NS {}
impl IPC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipc_ns::RegisterBlock {
        0x4101_2000 as *const _
    }
}
impl Deref for IPC_NS {
    type Target = ipc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IPC_NS::ptr() }
    }
}
#[doc = "Interprocessor communication"]
pub mod ipc_ns;
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub struct SPIM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0_NS {}
impl SPIM0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0_ns::RegisterBlock {
        0x4101_3000 as *const _
    }
}
impl Deref for SPIM0_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM0_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub mod spim0_ns;
#[doc = "SPI Slave"]
pub struct SPIS0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0_NS {}
impl SPIS0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0_ns::RegisterBlock {
        0x4101_3000 as *const _
    }
}
impl Deref for SPIS0_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS0_NS::ptr() }
    }
}
#[doc = "SPI Slave"]
pub mod spis0_ns;
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub struct TWIM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0_NS {}
impl TWIM0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0_ns::RegisterBlock {
        0x4101_3000 as *const _
    }
}
impl Deref for TWIM0_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM0_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub mod twim0_ns;
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub struct TWIS0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0_NS {}
impl TWIS0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0_ns::RegisterBlock {
        0x4101_3000 as *const _
    }
}
impl Deref for TWIS0_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS0_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub mod twis0_ns;
#[doc = "UART with EasyDMA"]
pub struct UARTE0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0_NS {}
impl UARTE0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarte0_ns::RegisterBlock {
        0x4101_3000 as *const _
    }
}
impl Deref for UARTE0_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTE0_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA"]
pub mod uarte0_ns;
#[doc = "Event generator unit"]
pub struct EGU0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0_NS {}
impl EGU0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0_ns::RegisterBlock {
        0x4101_4000 as *const _
    }
}
impl Deref for EGU0_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU0_NS::ptr() }
    }
}
#[doc = "Event generator unit"]
pub mod egu0_ns;
#[doc = "Real-time counter 1"]
pub struct RTC1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1_NS {}
impl RTC1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0_ns::RegisterBlock {
        0x4101_6000 as *const _
    }
}
impl Deref for RTC1_NS {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC1_NS::ptr() }
    }
}
#[doc = "Timer/Counter 1"]
pub struct TIMER1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1_NS {}
impl TIMER1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x4101_8000 as *const _
    }
}
impl Deref for TIMER1_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1_NS::ptr() }
    }
}
#[doc = "Timer/Counter 2"]
pub struct TIMER2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2_NS {}
impl TIMER2_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_ns::RegisterBlock {
        0x4101_9000 as *const _
    }
}
impl Deref for TIMER2_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2_NS::ptr() }
    }
}
#[doc = "Software interrupt 0"]
pub struct SWI0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI0_NS {}
impl SWI0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0_ns::RegisterBlock {
        0x4101_a000 as *const _
    }
}
impl Deref for SWI0_NS {
    type Target = swi0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI0_NS::ptr() }
    }
}
#[doc = "Software interrupt 0"]
pub mod swi0_ns;
#[doc = "Software interrupt 1"]
pub struct SWI1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI1_NS {}
impl SWI1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0_ns::RegisterBlock {
        0x4101_b000 as *const _
    }
}
impl Deref for SWI1_NS {
    type Target = swi0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI1_NS::ptr() }
    }
}
#[doc = "Software interrupt 2"]
pub struct SWI2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI2_NS {}
impl SWI2_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0_ns::RegisterBlock {
        0x4101_c000 as *const _
    }
}
impl Deref for SWI2_NS {
    type Target = swi0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI2_NS::ptr() }
    }
}
#[doc = "Software interrupt 3"]
pub struct SWI3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI3_NS {}
impl SWI3_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0_ns::RegisterBlock {
        0x4101_d000 as *const _
    }
}
impl Deref for SWI3_NS {
    type Target = swi0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI3_NS::ptr() }
    }
}
#[doc = "MUTEX 0"]
pub struct APPMUTEX_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APPMUTEX_NS {}
impl APPMUTEX_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const appmutex_ns::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for APPMUTEX_NS {
    type Target = appmutex_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*APPMUTEX_NS::ptr() }
    }
}
#[doc = "MUTEX 0"]
pub mod appmutex_ns;
#[doc = "MUTEX 1"]
pub struct APPMUTEX_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APPMUTEX_S {}
impl APPMUTEX_S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const appmutex_ns::RegisterBlock {
        0x5003_0000 as *const _
    }
}
impl Deref for APPMUTEX_S {
    type Target = appmutex_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*APPMUTEX_S::ptr() }
    }
}
#[doc = "Access control lists"]
pub struct ACL_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACL_NS {}
impl ACL_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acl_ns::RegisterBlock {
        0x4108_0000 as *const _
    }
}
impl Deref for ACL_NS {
    type Target = acl_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACL_NS::ptr() }
    }
}
#[doc = "Access control lists"]
pub mod acl_ns;
#[doc = "Non-volatile memory controller"]
pub struct NVMC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC_NS {}
impl NVMC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmc_ns::RegisterBlock {
        0x4108_0000 as *const _
    }
}
impl Deref for NVMC_NS {
    type Target = nvmc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMC_NS::ptr() }
    }
}
#[doc = "Non-volatile memory controller"]
pub mod nvmc_ns;
#[doc = "Volatile Memory controller"]
pub struct VMC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VMC_NS {}
impl VMC_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vmc_ns::RegisterBlock {
        0x4108_1000 as *const _
    }
}
impl Deref for VMC_NS {
    type Target = vmc_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VMC_NS::ptr() }
    }
}
#[doc = "Volatile Memory controller"]
pub mod vmc_ns;
#[doc = "GPIO Port 0"]
pub struct P0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0_NS {}
impl P0_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0_ns::RegisterBlock {
        0x418c_0500 as *const _
    }
}
impl Deref for P0_NS {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P0_NS::ptr() }
    }
}
#[doc = "GPIO Port 0"]
pub mod p0_ns;
#[doc = "GPIO Port 1"]
pub struct P1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P1_NS {}
impl P1_NS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0_ns::RegisterBlock {
        0x418c_0800 as *const _
    }
}
impl Deref for P1_NS {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P1_NS::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FICR_NS"]
    pub FICR_NS: FICR_NS,
    #[doc = "UICR_NS"]
    pub UICR_NS: UICR_NS,
    #[doc = "CTI_NS"]
    pub CTI_NS: CTI_NS,
    #[doc = "DCNF_NS"]
    pub DCNF_NS: DCNF_NS,
    #[doc = "VREQCTRL_NS"]
    pub VREQCTRL_NS: VREQCTRL_NS,
    #[doc = "CLOCK_NS"]
    pub CLOCK_NS: CLOCK_NS,
    #[doc = "POWER_NS"]
    pub POWER_NS: POWER_NS,
    #[doc = "RESET_NS"]
    pub RESET_NS: RESET_NS,
    #[doc = "CTRLAP_NS"]
    pub CTRLAP_NS: CTRLAP_NS,
    #[doc = "RADIO_NS"]
    pub RADIO_NS: RADIO_NS,
    #[doc = "RNG_NS"]
    pub RNG_NS: RNG_NS,
    #[doc = "GPIOTE_NS"]
    pub GPIOTE_NS: GPIOTE_NS,
    #[doc = "WDT_NS"]
    pub WDT_NS: WDT_NS,
    #[doc = "TIMER0_NS"]
    pub TIMER0_NS: TIMER0_NS,
    #[doc = "ECB_NS"]
    pub ECB_NS: ECB_NS,
    #[doc = "AAR_NS"]
    pub AAR_NS: AAR_NS,
    #[doc = "CCM_NS"]
    pub CCM_NS: CCM_NS,
    #[doc = "DPPIC_NS"]
    pub DPPIC_NS: DPPIC_NS,
    #[doc = "TEMP_NS"]
    pub TEMP_NS: TEMP_NS,
    #[doc = "RTC0_NS"]
    pub RTC0_NS: RTC0_NS,
    #[doc = "IPC_NS"]
    pub IPC_NS: IPC_NS,
    #[doc = "SPIM0_NS"]
    pub SPIM0_NS: SPIM0_NS,
    #[doc = "SPIS0_NS"]
    pub SPIS0_NS: SPIS0_NS,
    #[doc = "TWIM0_NS"]
    pub TWIM0_NS: TWIM0_NS,
    #[doc = "TWIS0_NS"]
    pub TWIS0_NS: TWIS0_NS,
    #[doc = "UARTE0_NS"]
    pub UARTE0_NS: UARTE0_NS,
    #[doc = "EGU0_NS"]
    pub EGU0_NS: EGU0_NS,
    #[doc = "RTC1_NS"]
    pub RTC1_NS: RTC1_NS,
    #[doc = "TIMER1_NS"]
    pub TIMER1_NS: TIMER1_NS,
    #[doc = "TIMER2_NS"]
    pub TIMER2_NS: TIMER2_NS,
    #[doc = "SWI0_NS"]
    pub SWI0_NS: SWI0_NS,
    #[doc = "SWI1_NS"]
    pub SWI1_NS: SWI1_NS,
    #[doc = "SWI2_NS"]
    pub SWI2_NS: SWI2_NS,
    #[doc = "SWI3_NS"]
    pub SWI3_NS: SWI3_NS,
    #[doc = "APPMUTEX_NS"]
    pub APPMUTEX_NS: APPMUTEX_NS,
    #[doc = "APPMUTEX_S"]
    pub APPMUTEX_S: APPMUTEX_S,
    #[doc = "ACL_NS"]
    pub ACL_NS: ACL_NS,
    #[doc = "NVMC_NS"]
    pub NVMC_NS: NVMC_NS,
    #[doc = "VMC_NS"]
    pub VMC_NS: VMC_NS,
    #[doc = "P0_NS"]
    pub P0_NS: P0_NS,
    #[doc = "P1_NS"]
    pub P1_NS: P1_NS,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FICR_NS: FICR_NS {
                _marker: PhantomData,
            },
            UICR_NS: UICR_NS {
                _marker: PhantomData,
            },
            CTI_NS: CTI_NS {
                _marker: PhantomData,
            },
            DCNF_NS: DCNF_NS {
                _marker: PhantomData,
            },
            VREQCTRL_NS: VREQCTRL_NS {
                _marker: PhantomData,
            },
            CLOCK_NS: CLOCK_NS {
                _marker: PhantomData,
            },
            POWER_NS: POWER_NS {
                _marker: PhantomData,
            },
            RESET_NS: RESET_NS {
                _marker: PhantomData,
            },
            CTRLAP_NS: CTRLAP_NS {
                _marker: PhantomData,
            },
            RADIO_NS: RADIO_NS {
                _marker: PhantomData,
            },
            RNG_NS: RNG_NS {
                _marker: PhantomData,
            },
            GPIOTE_NS: GPIOTE_NS {
                _marker: PhantomData,
            },
            WDT_NS: WDT_NS {
                _marker: PhantomData,
            },
            TIMER0_NS: TIMER0_NS {
                _marker: PhantomData,
            },
            ECB_NS: ECB_NS {
                _marker: PhantomData,
            },
            AAR_NS: AAR_NS {
                _marker: PhantomData,
            },
            CCM_NS: CCM_NS {
                _marker: PhantomData,
            },
            DPPIC_NS: DPPIC_NS {
                _marker: PhantomData,
            },
            TEMP_NS: TEMP_NS {
                _marker: PhantomData,
            },
            RTC0_NS: RTC0_NS {
                _marker: PhantomData,
            },
            IPC_NS: IPC_NS {
                _marker: PhantomData,
            },
            SPIM0_NS: SPIM0_NS {
                _marker: PhantomData,
            },
            SPIS0_NS: SPIS0_NS {
                _marker: PhantomData,
            },
            TWIM0_NS: TWIM0_NS {
                _marker: PhantomData,
            },
            TWIS0_NS: TWIS0_NS {
                _marker: PhantomData,
            },
            UARTE0_NS: UARTE0_NS {
                _marker: PhantomData,
            },
            EGU0_NS: EGU0_NS {
                _marker: PhantomData,
            },
            RTC1_NS: RTC1_NS {
                _marker: PhantomData,
            },
            TIMER1_NS: TIMER1_NS {
                _marker: PhantomData,
            },
            TIMER2_NS: TIMER2_NS {
                _marker: PhantomData,
            },
            SWI0_NS: SWI0_NS {
                _marker: PhantomData,
            },
            SWI1_NS: SWI1_NS {
                _marker: PhantomData,
            },
            SWI2_NS: SWI2_NS {
                _marker: PhantomData,
            },
            SWI3_NS: SWI3_NS {
                _marker: PhantomData,
            },
            APPMUTEX_NS: APPMUTEX_NS {
                _marker: PhantomData,
            },
            APPMUTEX_S: APPMUTEX_S {
                _marker: PhantomData,
            },
            ACL_NS: ACL_NS {
                _marker: PhantomData,
            },
            NVMC_NS: NVMC_NS {
                _marker: PhantomData,
            },
            VMC_NS: VMC_NS {
                _marker: PhantomData,
            },
            P0_NS: P0_NS {
                _marker: PhantomData,
            },
            P1_NS: P1_NS {
                _marker: PhantomData,
            },
        }
    }
}
