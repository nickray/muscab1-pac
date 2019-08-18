#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Secure Debug Configuration Status"]
    pub secdbgstat: SECDBGSTAT,
    #[doc = "0x04 - Secure Debug Configuration Set"]
    pub secdbgset: SECDBGSET,
    #[doc = "0x08 - Secure Debug Configuration Clear"]
    pub secdbgclr: SECDBGCLR,
    #[doc = "0x0c - System Security Control"]
    pub scsecctrl: SCSECCTRL,
    #[doc = "0x10 - Fast Clock Divider Configuration"]
    pub fclk_div: FCLK_DIV,
    #[doc = "0x14 - System Clock Divider Configuration"]
    pub sysclk_div: SYSCLK_DIV,
    #[doc = "0x18 - Clock Force"]
    pub clock_force: CLOCK_FORCE,
    _reserved7: [u8; 228usize],
    #[doc = "0x100 - Reset Syndrome"]
    pub reset_syndrome: RESET_SYNDROME,
    #[doc = "0x104 - Reset Mask"]
    pub reset_mask: RESET_MASK,
    #[doc = "0x108 - Software Reset"]
    pub swreset: SWRESET,
    #[doc = "0x10c - General Purpose Retention"]
    pub gretreg: GRETREG,
    #[doc = "0x110 - Initial Secure Reset Vector Register For CPU 0"]
    pub initsvrtor0: INITSVRTOR0,
    #[doc = "0x114 - Initial Secure Reset Vector Register For CPU 1"]
    pub initsvrtor1: INITSVRTOR1,
    #[doc = "0x118 - CPU Boot wait control after reset"]
    pub cpuwait: CPUWAIT,
    #[doc = "0x11c - NMI Enable Register"]
    pub nmi_enable: NMI_ENABLE,
    #[doc = "0x120 - WIC request and acknowledge handshake"]
    pub wicctrl: WICCTRL,
    #[doc = "0x124 - External Wakeup Control"]
    pub ewctrl: EWCTRL,
    _reserved17: [u8; 216usize],
    #[doc = "0x200 - External Wakeup Control"]
    pub pdcm_pd_sys_sense: PDCM_PD_SYS_SENSE,
    _reserved18: [u8; 8usize],
    #[doc = "0x20c - Power Control Depedendency Matrix PD_SRAM0 Power Domain Sensitivity"]
    pub pdcm_pd_sram0_sense: PDCM_PD_SRAM0_SENSE,
    #[doc = "0x210 - Power Control Depedendency Matrix PD_SRAM1 Power Domain Sensitivity"]
    pub pdcm_pd_sram1_sense: PDCM_PD_SRAM1_SENSE,
    #[doc = "0x214 - Power Control Depedendency Matrix PD_SRAM2 Power Domain Sensitivity"]
    pub pdcm_pd_sram2_sense: PDCM_PD_SRAM2_SENSE,
    #[doc = "0x218 - Power Control Depedendency Matrix PD_SRAM3 Power Domain Sensitivity"]
    pub pdcm_pd_sram3_sense: PDCM_PD_SRAM3_SENSE,
    _reserved22: [u8; 3508usize],
    #[doc = "0xfd0 - Peripheral ID 4"]
    pub pidr4: PIDR4,
    _reserved23: [u8; 12usize],
    #[doc = "0xfe0 - Peripheral ID 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral ID 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral ID 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral ID 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component ID 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component ID 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component ID 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component ID 3"]
    pub cidr3: CIDR3,
}
#[doc = "Secure Debug Configuration Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secdbgstat](secdbgstat) module"]
pub type SECDBGSTAT = crate::Reg<u32, _SECDBGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDBGSTAT;
#[doc = "`read()` method returns [secdbgstat::R](secdbgstat::R) reader structure"]
impl crate::Readable for SECDBGSTAT {}
#[doc = "Secure Debug Configuration Status"]
pub mod secdbgstat;
#[doc = "Secure Debug Configuration Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secdbgset](secdbgset) module"]
pub type SECDBGSET = crate::Reg<u32, _SECDBGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDBGSET;
#[doc = "`write(|w| ..)` method takes [secdbgset::W](secdbgset::W) writer structure"]
impl crate::Writable for SECDBGSET {}
#[doc = "Secure Debug Configuration Set"]
pub mod secdbgset;
#[doc = "Secure Debug Configuration Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secdbgclr](secdbgclr) module"]
pub type SECDBGCLR = crate::Reg<u32, _SECDBGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDBGCLR;
#[doc = "`write(|w| ..)` method takes [secdbgclr::W](secdbgclr::W) writer structure"]
impl crate::Writable for SECDBGCLR {}
#[doc = "Secure Debug Configuration Clear"]
pub mod secdbgclr;
#[doc = "System Security Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scsecctrl](scsecctrl) module"]
pub type SCSECCTRL = crate::Reg<u32, _SCSECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSECCTRL;
#[doc = "`read()` method returns [scsecctrl::R](scsecctrl::R) reader structure"]
impl crate::Readable for SCSECCTRL {}
#[doc = "`write(|w| ..)` method takes [scsecctrl::W](scsecctrl::W) writer structure"]
impl crate::Writable for SCSECCTRL {}
#[doc = "System Security Control"]
pub mod scsecctrl;
#[doc = "Fast Clock Divider Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fclk_div](fclk_div) module"]
pub type FCLK_DIV = crate::Reg<u32, _FCLK_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCLK_DIV;
#[doc = "`read()` method returns [fclk_div::R](fclk_div::R) reader structure"]
impl crate::Readable for FCLK_DIV {}
#[doc = "`write(|w| ..)` method takes [fclk_div::W](fclk_div::W) writer structure"]
impl crate::Writable for FCLK_DIV {}
#[doc = "Fast Clock Divider Configuration"]
pub mod fclk_div;
#[doc = "System Clock Divider Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysclk_div](sysclk_div) module"]
pub type SYSCLK_DIV = crate::Reg<u32, _SYSCLK_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCLK_DIV;
#[doc = "`read()` method returns [sysclk_div::R](sysclk_div::R) reader structure"]
impl crate::Readable for SYSCLK_DIV {}
#[doc = "`write(|w| ..)` method takes [sysclk_div::W](sysclk_div::W) writer structure"]
impl crate::Writable for SYSCLK_DIV {}
#[doc = "System Clock Divider Configuration"]
pub mod sysclk_div;
#[doc = "Clock Force\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock_force](clock_force) module"]
pub type CLOCK_FORCE = crate::Reg<u32, _CLOCK_FORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_FORCE;
#[doc = "`read()` method returns [clock_force::R](clock_force::R) reader structure"]
impl crate::Readable for CLOCK_FORCE {}
#[doc = "`write(|w| ..)` method takes [clock_force::W](clock_force::W) writer structure"]
impl crate::Writable for CLOCK_FORCE {}
#[doc = "Clock Force"]
pub mod clock_force;
#[doc = "Reset Syndrome\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset_syndrome](reset_syndrome) module"]
pub type RESET_SYNDROME = crate::Reg<u32, _RESET_SYNDROME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_SYNDROME;
#[doc = "`read()` method returns [reset_syndrome::R](reset_syndrome::R) reader structure"]
impl crate::Readable for RESET_SYNDROME {}
#[doc = "`write(|w| ..)` method takes [reset_syndrome::W](reset_syndrome::W) writer structure"]
impl crate::Writable for RESET_SYNDROME {}
#[doc = "Reset Syndrome"]
pub mod reset_syndrome;
#[doc = "Reset Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset_mask](reset_mask) module"]
pub type RESET_MASK = crate::Reg<u32, _RESET_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_MASK;
#[doc = "`read()` method returns [reset_mask::R](reset_mask::R) reader structure"]
impl crate::Readable for RESET_MASK {}
#[doc = "`write(|w| ..)` method takes [reset_mask::W](reset_mask::W) writer structure"]
impl crate::Writable for RESET_MASK {}
#[doc = "Reset Mask"]
pub mod reset_mask;
#[doc = "Software Reset\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swreset](swreset) module"]
pub type SWRESET = crate::Reg<u32, _SWRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRESET;
#[doc = "`write(|w| ..)` method takes [swreset::W](swreset::W) writer structure"]
impl crate::Writable for SWRESET {}
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "General Purpose Retention\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gretreg](gretreg) module"]
pub type GRETREG = crate::Reg<u32, _GRETREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRETREG;
#[doc = "`read()` method returns [gretreg::R](gretreg::R) reader structure"]
impl crate::Readable for GRETREG {}
#[doc = "`write(|w| ..)` method takes [gretreg::W](gretreg::W) writer structure"]
impl crate::Writable for GRETREG {}
#[doc = "General Purpose Retention"]
pub mod gretreg;
#[doc = "Initial Secure Reset Vector Register For CPU 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [initsvrtor0](initsvrtor0) module"]
pub type INITSVRTOR0 = crate::Reg<u32, _INITSVRTOR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INITSVRTOR0;
#[doc = "`read()` method returns [initsvrtor0::R](initsvrtor0::R) reader structure"]
impl crate::Readable for INITSVRTOR0 {}
#[doc = "`write(|w| ..)` method takes [initsvrtor0::W](initsvrtor0::W) writer structure"]
impl crate::Writable for INITSVRTOR0 {}
#[doc = "Initial Secure Reset Vector Register For CPU 0"]
pub mod initsvrtor0;
#[doc = "Initial Secure Reset Vector Register For CPU 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [initsvrtor1](initsvrtor1) module"]
pub type INITSVRTOR1 = crate::Reg<u32, _INITSVRTOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INITSVRTOR1;
#[doc = "`read()` method returns [initsvrtor1::R](initsvrtor1::R) reader structure"]
impl crate::Readable for INITSVRTOR1 {}
#[doc = "`write(|w| ..)` method takes [initsvrtor1::W](initsvrtor1::W) writer structure"]
impl crate::Writable for INITSVRTOR1 {}
#[doc = "Initial Secure Reset Vector Register For CPU 1"]
pub mod initsvrtor1;
#[doc = "CPU Boot wait control after reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpuwait](cpuwait) module"]
pub type CPUWAIT = crate::Reg<u32, _CPUWAIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUWAIT;
#[doc = "`read()` method returns [cpuwait::R](cpuwait::R) reader structure"]
impl crate::Readable for CPUWAIT {}
#[doc = "`write(|w| ..)` method takes [cpuwait::W](cpuwait::W) writer structure"]
impl crate::Writable for CPUWAIT {}
#[doc = "CPU Boot wait control after reset"]
pub mod cpuwait;
#[doc = "NMI Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmi_enable](nmi_enable) module"]
pub type NMI_ENABLE = crate::Reg<u32, _NMI_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMI_ENABLE;
#[doc = "`read()` method returns [nmi_enable::R](nmi_enable::R) reader structure"]
impl crate::Readable for NMI_ENABLE {}
#[doc = "`write(|w| ..)` method takes [nmi_enable::W](nmi_enable::W) writer structure"]
impl crate::Writable for NMI_ENABLE {}
#[doc = "NMI Enable Register"]
pub mod nmi_enable;
#[doc = "WIC request and acknowledge handshake\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wicctrl](wicctrl) module"]
pub type WICCTRL = crate::Reg<u32, _WICCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WICCTRL;
#[doc = "`read()` method returns [wicctrl::R](wicctrl::R) reader structure"]
impl crate::Readable for WICCTRL {}
#[doc = "`write(|w| ..)` method takes [wicctrl::W](wicctrl::W) writer structure"]
impl crate::Writable for WICCTRL {}
#[doc = "WIC request and acknowledge handshake"]
pub mod wicctrl;
#[doc = "External Wakeup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ewctrl](ewctrl) module"]
pub type EWCTRL = crate::Reg<u32, _EWCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EWCTRL;
#[doc = "`read()` method returns [ewctrl::R](ewctrl::R) reader structure"]
impl crate::Readable for EWCTRL {}
#[doc = "`write(|w| ..)` method takes [ewctrl::W](ewctrl::W) writer structure"]
impl crate::Writable for EWCTRL {}
#[doc = "External Wakeup Control"]
pub mod ewctrl;
#[doc = "External Wakeup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdcm_pd_sys_sense](pdcm_pd_sys_sense) module"]
pub type PDCM_PD_SYS_SENSE = crate::Reg<u32, _PDCM_PD_SYS_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCM_PD_SYS_SENSE;
#[doc = "`read()` method returns [pdcm_pd_sys_sense::R](pdcm_pd_sys_sense::R) reader structure"]
impl crate::Readable for PDCM_PD_SYS_SENSE {}
#[doc = "`write(|w| ..)` method takes [pdcm_pd_sys_sense::W](pdcm_pd_sys_sense::W) writer structure"]
impl crate::Writable for PDCM_PD_SYS_SENSE {}
#[doc = "External Wakeup Control"]
pub mod pdcm_pd_sys_sense;
#[doc = "Power Control Depedendency Matrix PD_SRAM0 Power Domain Sensitivity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdcm_pd_sram0_sense](pdcm_pd_sram0_sense) module"]
pub type PDCM_PD_SRAM0_SENSE = crate::Reg<u32, _PDCM_PD_SRAM0_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCM_PD_SRAM0_SENSE;
#[doc = "`read()` method returns [pdcm_pd_sram0_sense::R](pdcm_pd_sram0_sense::R) reader structure"]
impl crate::Readable for PDCM_PD_SRAM0_SENSE {}
#[doc = "`write(|w| ..)` method takes [pdcm_pd_sram0_sense::W](pdcm_pd_sram0_sense::W) writer structure"]
impl crate::Writable for PDCM_PD_SRAM0_SENSE {}
#[doc = "Power Control Depedendency Matrix PD_SRAM0 Power Domain Sensitivity"]
pub mod pdcm_pd_sram0_sense;
#[doc = "Power Control Depedendency Matrix PD_SRAM1 Power Domain Sensitivity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdcm_pd_sram1_sense](pdcm_pd_sram1_sense) module"]
pub type PDCM_PD_SRAM1_SENSE = crate::Reg<u32, _PDCM_PD_SRAM1_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCM_PD_SRAM1_SENSE;
#[doc = "`read()` method returns [pdcm_pd_sram1_sense::R](pdcm_pd_sram1_sense::R) reader structure"]
impl crate::Readable for PDCM_PD_SRAM1_SENSE {}
#[doc = "`write(|w| ..)` method takes [pdcm_pd_sram1_sense::W](pdcm_pd_sram1_sense::W) writer structure"]
impl crate::Writable for PDCM_PD_SRAM1_SENSE {}
#[doc = "Power Control Depedendency Matrix PD_SRAM1 Power Domain Sensitivity"]
pub mod pdcm_pd_sram1_sense;
#[doc = "Power Control Depedendency Matrix PD_SRAM2 Power Domain Sensitivity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdcm_pd_sram2_sense](pdcm_pd_sram2_sense) module"]
pub type PDCM_PD_SRAM2_SENSE = crate::Reg<u32, _PDCM_PD_SRAM2_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCM_PD_SRAM2_SENSE;
#[doc = "`read()` method returns [pdcm_pd_sram2_sense::R](pdcm_pd_sram2_sense::R) reader structure"]
impl crate::Readable for PDCM_PD_SRAM2_SENSE {}
#[doc = "`write(|w| ..)` method takes [pdcm_pd_sram2_sense::W](pdcm_pd_sram2_sense::W) writer structure"]
impl crate::Writable for PDCM_PD_SRAM2_SENSE {}
#[doc = "Power Control Depedendency Matrix PD_SRAM2 Power Domain Sensitivity"]
pub mod pdcm_pd_sram2_sense;
#[doc = "Power Control Depedendency Matrix PD_SRAM3 Power Domain Sensitivity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdcm_pd_sram3_sense](pdcm_pd_sram3_sense) module"]
pub type PDCM_PD_SRAM3_SENSE = crate::Reg<u32, _PDCM_PD_SRAM3_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCM_PD_SRAM3_SENSE;
#[doc = "`read()` method returns [pdcm_pd_sram3_sense::R](pdcm_pd_sram3_sense::R) reader structure"]
impl crate::Readable for PDCM_PD_SRAM3_SENSE {}
#[doc = "`write(|w| ..)` method takes [pdcm_pd_sram3_sense::W](pdcm_pd_sram3_sense::W) writer structure"]
impl crate::Writable for PDCM_PD_SRAM3_SENSE {}
#[doc = "Power Control Depedendency Matrix PD_SRAM3 Power Domain Sensitivity"]
pub mod pdcm_pd_sram3_sense;
#[doc = "Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Peripheral ID 4"]
pub mod pidr4;
#[doc = "Peripheral ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "Peripheral ID 0"]
pub mod pidr0;
#[doc = "Peripheral ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "Peripheral ID 1"]
pub mod pidr1;
#[doc = "Peripheral ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "Peripheral ID 2"]
pub mod pidr2;
#[doc = "Peripheral ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "Peripheral ID 3"]
pub mod pidr3;
#[doc = "Component ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "Component ID 0"]
pub mod cidr0;
#[doc = "Component ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "Component ID 1"]
pub mod cidr1;
#[doc = "Component ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "Component ID 2"]
pub mod cidr2;
#[doc = "Component ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "Component ID 3"]
pub mod cidr3;
