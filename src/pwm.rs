#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control Register"]
    pub pwmcr: PWMCR,
    #[doc = "0x04 - PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low"]
    pub pwmpr: PWMPR,
    #[doc = "0x08 - PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle"]
    pub pwmhr: PWMHR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - PWM Enable Interrupt Register"]
    pub pwmei: PWMEI,
    #[doc = "0x14 - PWM Disable Interrupt Register"]
    pub pwmdi: PWMDI,
    #[doc = "0x18 - PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers"]
    pub pwmri: PWMRI,
    #[doc = "0x1c - PWM Read Interrupt Status Register"]
    pub pwmis: PWMIS,
}
#[doc = "PWM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmcr](pwmcr) module"]
pub type PWMCR = crate::Reg<u32, _PWMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMCR;
#[doc = "`read()` method returns [pwmcr::R](pwmcr::R) reader structure"]
impl crate::Readable for PWMCR {}
#[doc = "`write(|w| ..)` method takes [pwmcr::W](pwmcr::W) writer structure"]
impl crate::Writable for PWMCR {}
#[doc = "PWM Control Register"]
pub mod pwmcr;
#[doc = "PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmpr](pwmpr) module"]
pub type PWMPR = crate::Reg<u32, _PWMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMPR;
#[doc = "`read()` method returns [pwmpr::R](pwmpr::R) reader structure"]
impl crate::Readable for PWMPR {}
#[doc = "`write(|w| ..)` method takes [pwmpr::W](pwmpr::W) writer structure"]
impl crate::Writable for PWMPR {}
#[doc = "PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low"]
pub mod pwmpr;
#[doc = "PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmhr](pwmhr) module"]
pub type PWMHR = crate::Reg<u32, _PWMHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMHR;
#[doc = "`read()` method returns [pwmhr::R](pwmhr::R) reader structure"]
impl crate::Readable for PWMHR {}
#[doc = "`write(|w| ..)` method takes [pwmhr::W](pwmhr::W) writer structure"]
impl crate::Writable for PWMHR {}
#[doc = "PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle"]
pub mod pwmhr;
#[doc = "PWM Enable Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmei](pwmei) module"]
pub type PWMEI = crate::Reg<u32, _PWMEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMEI;
#[doc = "`write(|w| ..)` method takes [pwmei::W](pwmei::W) writer structure"]
impl crate::Writable for PWMEI {}
#[doc = "PWM Enable Interrupt Register"]
pub mod pwmei;
#[doc = "PWM Disable Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmdi](pwmdi) module"]
pub type PWMDI = crate::Reg<u32, _PWMDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMDI;
#[doc = "`write(|w| ..)` method takes [pwmdi::W](pwmdi::W) writer structure"]
impl crate::Writable for PWMDI {}
#[doc = "PWM Disable Interrupt Register"]
pub mod pwmdi;
#[doc = "PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmri](pwmri) module"]
pub type PWMRI = crate::Reg<u32, _PWMRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMRI;
#[doc = "`read()` method returns [pwmri::R](pwmri::R) reader structure"]
impl crate::Readable for PWMRI {}
#[doc = "PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers"]
pub mod pwmri;
#[doc = "PWM Read Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmis](pwmis) module"]
pub type PWMIS = crate::Reg<u32, _PWMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMIS;
#[doc = "`read()` method returns [pwmis::R](pwmis::R) reader structure"]
impl crate::Readable for PWMIS {}
#[doc = "PWM Read Interrupt Status Register"]
pub mod pwmis;
