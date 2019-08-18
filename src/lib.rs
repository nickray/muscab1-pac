#![doc = "Peripheral access API for MUSCA_B1 microcontrollers (generated using svd2rust v0.16.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.0/svd2rust/#peripheral-api"]
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
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn NONSEC_WATCHDOG_IRQ();
    fn TIMER0();
    fn DUALTIMER();
    fn MPC();
    fn GPTIMERINTR();
    fn QSPIINTR();
    fn UARTRXINTR0();
    fn GPIO0();
    fn PWMINT0();
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
pub static __INTERRUPTS: [Vector; 71] = [
    Vector { _reserved: 0 },
    Vector {
        _handler: NONSEC_WATCHDOG_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DUALTIMER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: MPC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: GPTIMERINTR,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: QSPIINTR },
    Vector {
        _handler: UARTRXINTR0,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIO0 },
    Vector { _reserved: 0 },
    Vector { _handler: PWMINT0 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "1 - Non-Secure Watchdog Interrupt"]
    NONSEC_WATCHDOG_IRQ,
    #[doc = "3 - Timer 0"]
    TIMER0,
    #[doc = "5 - Dual Timer"]
    DUALTIMER,
    #[doc = "9 - MPC Combined"]
    MPC,
    #[doc = "33 - General-Purpose Timer interrupt"]
    GPTIMERINTR,
    #[doc = "38 - QSPI interrupt"]
    QSPIINTR,
    #[doc = "39 - UART0 receive FIFO interrupt"]
    UARTRXINTR0,
    #[doc = "68 - GPIO 0 combined"]
    GPIO0,
    #[doc = "70 - PWM0 interrupt"]
    PWMINT0,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::NONSEC_WATCHDOG_IRQ => 1,
            Interrupt::TIMER0 => 3,
            Interrupt::DUALTIMER => 5,
            Interrupt::MPC => 9,
            Interrupt::GPTIMERINTR => 33,
            Interrupt::QSPIINTR => 38,
            Interrupt::UARTRXINTR0 => 39,
            Interrupt::GPIO0 => 68,
            Interrupt::PWMINT0 => 70,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "System Information"]
pub struct SYSINFO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSINFO {}
impl SYSINFO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysinfo::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for SYSINFO {
    type Target = sysinfo::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSINFO::ptr() }
    }
}
#[doc = "System Information"]
pub mod sysinfo;
#[doc = "System Information (Secure)"]
pub struct SYSINFO_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSINFO_SECURE {}
impl SYSINFO_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysinfo::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for SYSINFO_SECURE {
    type Target = sysinfo::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSINFO_SECURE::ptr() }
    }
}
#[doc = "System Control"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0x5002_1000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control"]
pub mod system_control;
#[doc = "Security Attribution Unit"]
pub struct SAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAU {}
impl SAU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sau::RegisterBlock {
        0xe000_edd0 as *const _
    }
}
impl Deref for SAU {
    type Target = sau::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAU::ptr() }
    }
}
#[doc = "Security Attribution Unit"]
pub mod sau;
#[doc = "Timer 0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer 0"]
pub mod timer0;
#[doc = "Dual Timer"]
pub struct DUALTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DUALTIMER {}
impl DUALTIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dualtimer::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for DUALTIMER {
    type Target = dualtimer::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DUALTIMER::ptr() }
    }
}
#[doc = "Dual Timer"]
pub mod dualtimer;
#[doc = "General-Purpose Timer"]
pub struct GPTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMER {}
impl GPTIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimer::RegisterBlock {
        0x4010_c000 as *const _
    }
}
impl Deref for GPTIMER {
    type Target = gptimer::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTIMER::ptr() }
    }
}
#[doc = "General-Purpose Timer"]
pub mod gptimer;
#[doc = "Timer 0 (Secure)"]
pub struct TIMER0_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_SECURE {}
impl TIMER0_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for TIMER0_SECURE {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0_SECURE::ptr() }
    }
}
#[doc = "Dual Timer (Secure)"]
pub struct DUALTIMER_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DUALTIMER_SECURE {}
impl DUALTIMER_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dualtimer::RegisterBlock {
        0x5000_2000 as *const _
    }
}
impl Deref for DUALTIMER_SECURE {
    type Target = dualtimer::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DUALTIMER_SECURE::ptr() }
    }
}
#[doc = "General-Purpose Timer (Secure)"]
pub struct GPTIMER_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMER_SECURE {}
impl GPTIMER_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimer::RegisterBlock {
        0x5010_c000 as *const _
    }
}
impl Deref for GPTIMER_SECURE {
    type Target = gptimer::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTIMER_SECURE::ptr() }
    }
}
#[doc = "General-purpose I/O 0"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x4100_0000 as *const _
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO0::ptr() }
    }
}
#[doc = "General-purpose I/O 0"]
pub mod gpio0;
#[doc = "General-purpose I/O 0 (Secure)"]
pub struct GPIO0_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0_SECURE {}
impl GPIO0_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x5100_0000 as *const _
    }
}
impl Deref for GPIO0_SECURE {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO0_SECURE::ptr() }
    }
}
#[doc = "UART 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4010_5000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART 0"]
pub mod uart0;
#[doc = "UART 0 (Secure)"]
pub struct UART0_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0_SECURE {}
impl UART0_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x5010_5000 as *const _
    }
}
impl Deref for UART0_SECURE {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0_SECURE::ptr() }
    }
}
#[doc = "Non-secure Watchdog Timer"]
pub struct WATCHDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG {}
impl WATCHDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog::RegisterBlock {
        0x4008_1000 as *const _
    }
}
impl Deref for WATCHDOG {
    type Target = watchdog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG::ptr() }
    }
}
#[doc = "Non-secure Watchdog Timer"]
pub mod watchdog;
#[doc = "Cache"]
pub struct ICACHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICACHE {}
impl ICACHE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i_cache::RegisterBlock {
        0x5001_0000 as *const _
    }
}
impl Deref for ICACHE {
    type Target = i_cache::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICACHE::ptr() }
    }
}
#[doc = "Cache"]
pub mod i_cache;
#[doc = "PWM_IP6512"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4010_7000 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "PWM_IP6512"]
pub mod pwm;
#[doc = "Watchdog (Secure)"]
pub struct WATCHDOG_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_SECURE {}
impl WATCHDOG_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog::RegisterBlock {
        0x5008_1000 as *const _
    }
}
impl Deref for WATCHDOG_SECURE {
    type Target = watchdog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG_SECURE::ptr() }
    }
}
#[doc = "S32K Watchdog (Secure)"]
pub struct S32KWATCHDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for S32KWATCHDOG {}
impl S32KWATCHDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog::RegisterBlock {
        0x5002_e000 as *const _
    }
}
impl Deref for S32KWATCHDOG {
    type Target = watchdog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*S32KWATCHDOG::ptr() }
    }
}
#[doc = "Serial Communication Controller"]
pub struct SCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCC {}
impl SCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scc::RegisterBlock {
        0x5010_b000 as *const _
    }
}
impl Deref for SCC {
    type Target = scc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCC::ptr() }
    }
}
#[doc = "Serial Communication Controller"]
pub mod scc;
#[doc = "Secure Privilege Control Block"]
pub struct SPCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPCTRL {}
impl SPCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spctrl::RegisterBlock {
        0x5008_0000 as *const _
    }
}
impl Deref for SPCTRL {
    type Target = spctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPCTRL::ptr() }
    }
}
#[doc = "Secure Privilege Control Block"]
pub mod spctrl;
#[doc = "Non-secure Privilege Control Block"]
pub struct NSPCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NSPCTRL {}
impl NSPCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nspctrl::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for NSPCTRL {
    type Target = nspctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NSPCTRL::ptr() }
    }
}
#[doc = "Non-secure Privilege Control Block"]
pub mod nspctrl;
#[doc = "Memory Protection Controller 0"]
pub struct SRAM0MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRAM0MPC {}
impl SRAM0MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5008_3000 as *const _
    }
}
impl Deref for SRAM0MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRAM0MPC::ptr() }
    }
}
#[doc = "Memory Protection Controller 0"]
pub mod sram0mpc;
#[doc = "SRAM 1 Memory Protection Controller"]
pub struct SRAM1MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRAM1MPC {}
impl SRAM1MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5008_4000 as *const _
    }
}
impl Deref for SRAM1MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRAM1MPC::ptr() }
    }
}
#[doc = "SRAM 2 Memory Protection Controller"]
pub struct SRAM2MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRAM2MPC {}
impl SRAM2MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5008_5000 as *const _
    }
}
impl Deref for SRAM2MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRAM2MPC::ptr() }
    }
}
#[doc = "SRAM 3 Memory Protection Controller"]
pub struct SRAM3MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRAM3MPC {}
impl SRAM3MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5008_6000 as *const _
    }
}
impl Deref for SRAM3MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRAM3MPC::ptr() }
    }
}
#[doc = "Code SRAM Memory Protection Controller"]
pub struct CODE_SRAM_MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CODE_SRAM_MPC {}
impl CODE_SRAM_MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5210_0000 as *const _
    }
}
impl Deref for CODE_SRAM_MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CODE_SRAM_MPC::ptr() }
    }
}
#[doc = "QSPI Flash Memory Protection Controller"]
pub struct QSPI_MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI_MPC {}
impl QSPI_MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5200_0000 as *const _
    }
}
impl Deref for QSPI_MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI_MPC::ptr() }
    }
}
#[doc = "EFlash0 Memory Protection Controller"]
pub struct EFLASH0_MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFLASH0_MPC {}
impl EFLASH0_MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5220_0000 as *const _
    }
}
impl Deref for EFLASH0_MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFLASH0_MPC::ptr() }
    }
}
#[doc = "EFlash1 Memory Protection Controller"]
pub struct EFLASH1_MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFLASH1_MPC {}
impl EFLASH1_MPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram0mpc::RegisterBlock {
        0x5230_0000 as *const _
    }
}
impl Deref for EFLASH1_MPC {
    type Target = sram0mpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFLASH1_MPC::ptr() }
    }
}
#[doc = "QSPI Flash Controller"]
pub struct QSPIFCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPIFCTRL {}
impl QSPIFCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspifctrl::RegisterBlock {
        0x4280_0000 as *const _
    }
}
impl Deref for QSPIFCTRL {
    type Target = qspifctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPIFCTRL::ptr() }
    }
}
#[doc = "QSPI Flash Controller"]
pub mod qspifctrl;
#[doc = "QSPI Flash Controller (Secure)"]
pub struct QSPIFCTRL_SECURE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPIFCTRL_SECURE {}
impl QSPIFCTRL_SECURE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspifctrl::RegisterBlock {
        0x5280_0000 as *const _
    }
}
impl Deref for QSPIFCTRL_SECURE {
    type Target = qspifctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPIFCTRL_SECURE::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYSINFO"]
    pub SYSINFO: SYSINFO,
    #[doc = "SYSINFO_SECURE"]
    pub SYSINFO_SECURE: SYSINFO_SECURE,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SAU"]
    pub SAU: SAU,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "DUALTIMER"]
    pub DUALTIMER: DUALTIMER,
    #[doc = "GPTIMER"]
    pub GPTIMER: GPTIMER,
    #[doc = "TIMER0_SECURE"]
    pub TIMER0_SECURE: TIMER0_SECURE,
    #[doc = "DUALTIMER_SECURE"]
    pub DUALTIMER_SECURE: DUALTIMER_SECURE,
    #[doc = "GPTIMER_SECURE"]
    pub GPTIMER_SECURE: GPTIMER_SECURE,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "GPIO0_SECURE"]
    pub GPIO0_SECURE: GPIO0_SECURE,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART0_SECURE"]
    pub UART0_SECURE: UART0_SECURE,
    #[doc = "WATCHDOG"]
    pub WATCHDOG: WATCHDOG,
    #[doc = "ICACHE"]
    pub ICACHE: ICACHE,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "WATCHDOG_SECURE"]
    pub WATCHDOG_SECURE: WATCHDOG_SECURE,
    #[doc = "S32KWATCHDOG"]
    pub S32KWATCHDOG: S32KWATCHDOG,
    #[doc = "SCC"]
    pub SCC: SCC,
    #[doc = "SPCTRL"]
    pub SPCTRL: SPCTRL,
    #[doc = "NSPCTRL"]
    pub NSPCTRL: NSPCTRL,
    #[doc = "SRAM0MPC"]
    pub SRAM0MPC: SRAM0MPC,
    #[doc = "SRAM1MPC"]
    pub SRAM1MPC: SRAM1MPC,
    #[doc = "SRAM2MPC"]
    pub SRAM2MPC: SRAM2MPC,
    #[doc = "SRAM3MPC"]
    pub SRAM3MPC: SRAM3MPC,
    #[doc = "CODE_SRAM_MPC"]
    pub CODE_SRAM_MPC: CODE_SRAM_MPC,
    #[doc = "QSPI_MPC"]
    pub QSPI_MPC: QSPI_MPC,
    #[doc = "EFLASH0_MPC"]
    pub EFLASH0_MPC: EFLASH0_MPC,
    #[doc = "EFLASH1_MPC"]
    pub EFLASH1_MPC: EFLASH1_MPC,
    #[doc = "QSPIFCTRL"]
    pub QSPIFCTRL: QSPIFCTRL,
    #[doc = "QSPIFCTRL_SECURE"]
    pub QSPIFCTRL_SECURE: QSPIFCTRL_SECURE,
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
            SYSINFO: SYSINFO {
                _marker: PhantomData,
            },
            SYSINFO_SECURE: SYSINFO_SECURE {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SAU: SAU {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            DUALTIMER: DUALTIMER {
                _marker: PhantomData,
            },
            GPTIMER: GPTIMER {
                _marker: PhantomData,
            },
            TIMER0_SECURE: TIMER0_SECURE {
                _marker: PhantomData,
            },
            DUALTIMER_SECURE: DUALTIMER_SECURE {
                _marker: PhantomData,
            },
            GPTIMER_SECURE: GPTIMER_SECURE {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            GPIO0_SECURE: GPIO0_SECURE {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART0_SECURE: UART0_SECURE {
                _marker: PhantomData,
            },
            WATCHDOG: WATCHDOG {
                _marker: PhantomData,
            },
            ICACHE: ICACHE {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            WATCHDOG_SECURE: WATCHDOG_SECURE {
                _marker: PhantomData,
            },
            S32KWATCHDOG: S32KWATCHDOG {
                _marker: PhantomData,
            },
            SCC: SCC {
                _marker: PhantomData,
            },
            SPCTRL: SPCTRL {
                _marker: PhantomData,
            },
            NSPCTRL: NSPCTRL {
                _marker: PhantomData,
            },
            SRAM0MPC: SRAM0MPC {
                _marker: PhantomData,
            },
            SRAM1MPC: SRAM1MPC {
                _marker: PhantomData,
            },
            SRAM2MPC: SRAM2MPC {
                _marker: PhantomData,
            },
            SRAM3MPC: SRAM3MPC {
                _marker: PhantomData,
            },
            CODE_SRAM_MPC: CODE_SRAM_MPC {
                _marker: PhantomData,
            },
            QSPI_MPC: QSPI_MPC {
                _marker: PhantomData,
            },
            EFLASH0_MPC: EFLASH0_MPC {
                _marker: PhantomData,
            },
            EFLASH1_MPC: EFLASH1_MPC {
                _marker: PhantomData,
            },
            QSPIFCTRL: QSPIFCTRL {
                _marker: PhantomData,
            },
            QSPIFCTRL_SECURE: QSPIFCTRL_SECURE {
                _marker: PhantomData,
            },
        }
    }
}
