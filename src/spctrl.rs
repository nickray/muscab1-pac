#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Secure Privilege Controller Secure Configuration Control register"]
    pub spcsectrl: SPCSECTRL,
    #[doc = "0x04 - Bus Access wait control after reset"]
    pub buswait: BUSWAIT,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Security Violation Response Configuration register"]
    pub secrespcfg: SECRESPCFG,
    #[doc = "0x14 - Non Secure Callable Configuration for IDAU"]
    pub nsccfg: NSCCFG,
    _reserved4: [u8; 4usize],
    #[doc = "0x1c - Secure MPC Interrupt Status"]
    pub secmpcintstatus: SECMPCINTSTATUS,
    #[doc = "0x20 - Secure PPC Interrupt Status"]
    pub secppcintstat: SECPPCINTSTAT,
    #[doc = "0x24 - Secure PPC Interrupt Clear"]
    pub secppcintclr: SECPPCINTCLR,
    #[doc = "0x28 - Secure PPC Interrupt Enable"]
    pub secppcinten: SECPPCINTEN,
    _reserved8: [u8; 4usize],
    #[doc = "0x30 - Secure MSC Interrupt Status"]
    pub secmscintstat: SECMSCINTSTAT,
    #[doc = "0x34 - Secure MSC Interrupt Clear"]
    pub secmscintclr: SECMSCINTCLR,
    #[doc = "0x38 - Secure MSC Interrupt Enable"]
    pub secmscinten: SECMSCINTEN,
    _reserved11: [u8; 4usize],
    #[doc = "0x40 - Bridge Buffer Error Interrupt Status"]
    pub brgintstat: BRGINTSTAT,
    #[doc = "0x44 - Bridge Buffer Error Interrupt Clear"]
    pub brgintclr: BRGINTCLR,
    #[doc = "0x48 - Bridge Buffer Error Interrupt Enable"]
    pub brginten: BRGINTEN,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - Non-Secure Access AHB slave Peripheral Protection Control 0"]
    pub ahbnsppc0: AHBNSPPC0,
    _reserved15: [u8; 12usize],
    #[doc = "0x60 - Expansion 0 Non_Secure Access AHB slave Peripheral Protection Control"]
    pub ahbnsppcexp0: AHBNSPPCEXP0,
    #[doc = "0x64 - Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control"]
    pub ahbnsppcexp1: AHBNSPPCEXP1,
    #[doc = "0x68 - Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control"]
    pub ahbnsppcexp2: AHBNSPPCEXP2,
    #[doc = "0x6c - Expansion 3 Non_Secure Access AHB slave Peripheral Protection Control"]
    pub ahbnsppcexp3: AHBNSPPCEXP3,
    #[doc = "0x70 - Non-Secure Access APB slave Peripheral Protection Control 0"]
    pub apbnsppc0: APBNSPPC0,
    #[doc = "0x74 - Non-Secure Access APB slave Peripheral Protection Control 1"]
    pub apbnsppc1: APBNSPPC1,
    _reserved21: [u8; 8usize],
    #[doc = "0x80 - Expansion 0 Non_Secure Access APB slave Peripheral Protection Control"]
    pub apbnsppcexp0: APBNSPPCEXP0,
    #[doc = "0x84 - Expansion 1 Non_Secure Access APB slave Peripheral Protection Control"]
    pub apbnsppcexp1: APBNSPPCEXP1,
    #[doc = "0x88 - Expansion 2 Non_Secure Access APB slave Peripheral Protection Control"]
    pub apbnsppcexp2: APBNSPPCEXP2,
    #[doc = "0x8c - Expansion 3 Non_Secure Access APB slave Peripheral Protection Control"]
    pub apbnsppcexp3: APBNSPPCEXP3,
    #[doc = "0x90 - Secure Unprivileged Access AHB slave Peripheral Protection Control 0"]
    pub ahbspppc0: AHBSPPPC0,
    _reserved26: [u8; 12usize],
    #[doc = "0xa0 - Expansion 0 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbspppcexp0: AHBSPPPCEXP0,
    #[doc = "0xa4 - Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbspppcexp1: AHBSPPPCEXP1,
    #[doc = "0xa8 - Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbspppcexp2: AHBSPPPCEXP2,
    #[doc = "0xac - Expansion 3 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbspppcexp3: AHBSPPPCEXP3,
    #[doc = "0xb0 - Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
    pub apbspppc0: APBSPPPC0,
    #[doc = "0xb4 - Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
    pub apbspppc1: APBSPPPC1,
    _reserved32: [u8; 8usize],
    #[doc = "0xc0 - Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbspppcexp0: APBSPPPCEXP0,
    #[doc = "0xc4 - Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbspppcexp1: APBSPPPCEXP1,
    #[doc = "0xc8 - Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbspppcexp2: APBSPPPCEXP2,
    #[doc = "0xcc - Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbspppcexp3: APBSPPPCEXP3,
    #[doc = "0xd0 - Expansion MSC Non-Secure Configuration"]
    pub nsmscexp: NSMSCEXP,
    _reserved37: [u8; 3836usize],
    #[doc = "0xfd0 - Peripheral ID 4"]
    pub pid4: PID4,
    _reserved38: [u8; 12usize],
    #[doc = "0xfe0 - Peripheral ID 0"]
    pub pid0: PID0,
    #[doc = "0xfe4 - Peripheral ID 1"]
    pub pid1: PID1,
    #[doc = "0xfe8 - Peripheral ID 2"]
    pub pid2: PID2,
    #[doc = "0xfec - Peripheral ID 3"]
    pub pid3: PID3,
    #[doc = "0xff0 - Component ID 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component ID 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component ID 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component ID 3"]
    pub cidr3: CIDR3,
}
#[doc = "Secure Privilege Controller Secure Configuration Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spcsectrl](spcsectrl) module"]
pub type SPCSECTRL = crate::Reg<u32, _SPCSECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPCSECTRL;
#[doc = "`read()` method returns [spcsectrl::R](spcsectrl::R) reader structure"]
impl crate::Readable for SPCSECTRL {}
#[doc = "`write(|w| ..)` method takes [spcsectrl::W](spcsectrl::W) writer structure"]
impl crate::Writable for SPCSECTRL {}
#[doc = "Secure Privilege Controller Secure Configuration Control register"]
pub mod spcsectrl;
#[doc = "Bus Access wait control after reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [buswait](buswait) module"]
pub type BUSWAIT = crate::Reg<u32, _BUSWAIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWAIT;
#[doc = "`read()` method returns [buswait::R](buswait::R) reader structure"]
impl crate::Readable for BUSWAIT {}
#[doc = "`write(|w| ..)` method takes [buswait::W](buswait::W) writer structure"]
impl crate::Writable for BUSWAIT {}
#[doc = "Bus Access wait control after reset"]
pub mod buswait;
#[doc = "Security Violation Response Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secrespcfg](secrespcfg) module"]
pub type SECRESPCFG = crate::Reg<u32, _SECRESPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECRESPCFG;
#[doc = "`read()` method returns [secrespcfg::R](secrespcfg::R) reader structure"]
impl crate::Readable for SECRESPCFG {}
#[doc = "`write(|w| ..)` method takes [secrespcfg::W](secrespcfg::W) writer structure"]
impl crate::Writable for SECRESPCFG {}
#[doc = "Security Violation Response Configuration register"]
pub mod secrespcfg;
#[doc = "Non Secure Callable Configuration for IDAU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nsccfg](nsccfg) module"]
pub type NSCCFG = crate::Reg<u32, _NSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSCCFG;
#[doc = "`read()` method returns [nsccfg::R](nsccfg::R) reader structure"]
impl crate::Readable for NSCCFG {}
#[doc = "`write(|w| ..)` method takes [nsccfg::W](nsccfg::W) writer structure"]
impl crate::Writable for NSCCFG {}
#[doc = "Non Secure Callable Configuration for IDAU"]
pub mod nsccfg;
#[doc = "Secure MPC Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secmpcintstatus](secmpcintstatus) module"]
pub type SECMPCINTSTATUS = crate::Reg<u32, _SECMPCINTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECMPCINTSTATUS;
#[doc = "`read()` method returns [secmpcintstatus::R](secmpcintstatus::R) reader structure"]
impl crate::Readable for SECMPCINTSTATUS {}
#[doc = "Secure MPC Interrupt Status"]
pub mod secmpcintstatus;
#[doc = "Secure PPC Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secppcintstat](secppcintstat) module"]
pub type SECPPCINTSTAT = crate::Reg<u32, _SECPPCINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECPPCINTSTAT;
#[doc = "`read()` method returns [secppcintstat::R](secppcintstat::R) reader structure"]
impl crate::Readable for SECPPCINTSTAT {}
#[doc = "Secure PPC Interrupt Status"]
pub mod secppcintstat;
#[doc = "Secure PPC Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secppcintclr](secppcintclr) module"]
pub type SECPPCINTCLR = crate::Reg<u32, _SECPPCINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECPPCINTCLR;
#[doc = "`write(|w| ..)` method takes [secppcintclr::W](secppcintclr::W) writer structure"]
impl crate::Writable for SECPPCINTCLR {}
#[doc = "Secure PPC Interrupt Clear"]
pub mod secppcintclr;
#[doc = "Secure PPC Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secppcinten](secppcinten) module"]
pub type SECPPCINTEN = crate::Reg<u32, _SECPPCINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECPPCINTEN;
#[doc = "`read()` method returns [secppcinten::R](secppcinten::R) reader structure"]
impl crate::Readable for SECPPCINTEN {}
#[doc = "`write(|w| ..)` method takes [secppcinten::W](secppcinten::W) writer structure"]
impl crate::Writable for SECPPCINTEN {}
#[doc = "Secure PPC Interrupt Enable"]
pub mod secppcinten;
#[doc = "Secure MSC Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secmscintstat](secmscintstat) module"]
pub type SECMSCINTSTAT = crate::Reg<u32, _SECMSCINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECMSCINTSTAT;
#[doc = "`read()` method returns [secmscintstat::R](secmscintstat::R) reader structure"]
impl crate::Readable for SECMSCINTSTAT {}
#[doc = "Secure MSC Interrupt Status"]
pub mod secmscintstat;
#[doc = "Secure MSC Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secmscintclr](secmscintclr) module"]
pub type SECMSCINTCLR = crate::Reg<u32, _SECMSCINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECMSCINTCLR;
#[doc = "`read()` method returns [secmscintclr::R](secmscintclr::R) reader structure"]
impl crate::Readable for SECMSCINTCLR {}
#[doc = "`write(|w| ..)` method takes [secmscintclr::W](secmscintclr::W) writer structure"]
impl crate::Writable for SECMSCINTCLR {}
#[doc = "Secure MSC Interrupt Clear"]
pub mod secmscintclr;
#[doc = "Secure MSC Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secmscinten](secmscinten) module"]
pub type SECMSCINTEN = crate::Reg<u32, _SECMSCINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECMSCINTEN;
#[doc = "`read()` method returns [secmscinten::R](secmscinten::R) reader structure"]
impl crate::Readable for SECMSCINTEN {}
#[doc = "`write(|w| ..)` method takes [secmscinten::W](secmscinten::W) writer structure"]
impl crate::Writable for SECMSCINTEN {}
#[doc = "Secure MSC Interrupt Enable"]
pub mod secmscinten;
#[doc = "Bridge Buffer Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [brgintstat](brgintstat) module"]
pub type BRGINTSTAT = crate::Reg<u32, _BRGINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGINTSTAT;
#[doc = "`read()` method returns [brgintstat::R](brgintstat::R) reader structure"]
impl crate::Readable for BRGINTSTAT {}
#[doc = "Bridge Buffer Error Interrupt Status"]
pub mod brgintstat;
#[doc = "Bridge Buffer Error Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [brgintclr](brgintclr) module"]
pub type BRGINTCLR = crate::Reg<u32, _BRGINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGINTCLR;
#[doc = "`write(|w| ..)` method takes [brgintclr::W](brgintclr::W) writer structure"]
impl crate::Writable for BRGINTCLR {}
#[doc = "Bridge Buffer Error Interrupt Clear"]
pub mod brgintclr;
#[doc = "Bridge Buffer Error Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [brginten](brginten) module"]
pub type BRGINTEN = crate::Reg<u32, _BRGINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGINTEN;
#[doc = "`read()` method returns [brginten::R](brginten::R) reader structure"]
impl crate::Readable for BRGINTEN {}
#[doc = "`write(|w| ..)` method takes [brginten::W](brginten::W) writer structure"]
impl crate::Writable for BRGINTEN {}
#[doc = "Bridge Buffer Error Interrupt Enable"]
pub mod brginten;
#[doc = "Non-Secure Access AHB slave Peripheral Protection Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnsppc0](ahbnsppc0) module"]
pub type AHBNSPPC0 = crate::Reg<u32, _AHBNSPPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPC0;
#[doc = "`read()` method returns [ahbnsppc0::R](ahbnsppc0::R) reader structure"]
impl crate::Readable for AHBNSPPC0 {}
#[doc = "`write(|w| ..)` method takes [ahbnsppc0::W](ahbnsppc0::W) writer structure"]
impl crate::Writable for AHBNSPPC0 {}
#[doc = "Non-Secure Access AHB slave Peripheral Protection Control 0"]
pub mod ahbnsppc0;
#[doc = "Expansion 0 Non_Secure Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnsppcexp0](ahbnsppcexp0) module"]
pub type AHBNSPPCEXP0 = crate::Reg<u32, _AHBNSPPCEXP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPCEXP0;
#[doc = "`read()` method returns [ahbnsppcexp0::R](ahbnsppcexp0::R) reader structure"]
impl crate::Readable for AHBNSPPCEXP0 {}
#[doc = "`write(|w| ..)` method takes [ahbnsppcexp0::W](ahbnsppcexp0::W) writer structure"]
impl crate::Writable for AHBNSPPCEXP0 {}
#[doc = "Expansion 0 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp0;
#[doc = "Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnsppcexp1](ahbnsppcexp1) module"]
pub type AHBNSPPCEXP1 = crate::Reg<u32, _AHBNSPPCEXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPCEXP1;
#[doc = "`read()` method returns [ahbnsppcexp1::R](ahbnsppcexp1::R) reader structure"]
impl crate::Readable for AHBNSPPCEXP1 {}
#[doc = "`write(|w| ..)` method takes [ahbnsppcexp1::W](ahbnsppcexp1::W) writer structure"]
impl crate::Writable for AHBNSPPCEXP1 {}
#[doc = "Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp1;
#[doc = "Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnsppcexp2](ahbnsppcexp2) module"]
pub type AHBNSPPCEXP2 = crate::Reg<u32, _AHBNSPPCEXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPCEXP2;
#[doc = "`read()` method returns [ahbnsppcexp2::R](ahbnsppcexp2::R) reader structure"]
impl crate::Readable for AHBNSPPCEXP2 {}
#[doc = "`write(|w| ..)` method takes [ahbnsppcexp2::W](ahbnsppcexp2::W) writer structure"]
impl crate::Writable for AHBNSPPCEXP2 {}
#[doc = "Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp2;
#[doc = "Expansion 3 Non_Secure Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnsppcexp3](ahbnsppcexp3) module"]
pub type AHBNSPPCEXP3 = crate::Reg<u32, _AHBNSPPCEXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPCEXP3;
#[doc = "`read()` method returns [ahbnsppcexp3::R](ahbnsppcexp3::R) reader structure"]
impl crate::Readable for AHBNSPPCEXP3 {}
#[doc = "`write(|w| ..)` method takes [ahbnsppcexp3::W](ahbnsppcexp3::W) writer structure"]
impl crate::Writable for AHBNSPPCEXP3 {}
#[doc = "Expansion 3 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp3;
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnsppc0](apbnsppc0) module"]
pub type APBNSPPC0 = crate::Reg<u32, _APBNSPPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPC0;
#[doc = "`read()` method returns [apbnsppc0::R](apbnsppc0::R) reader structure"]
impl crate::Readable for APBNSPPC0 {}
#[doc = "`write(|w| ..)` method takes [apbnsppc0::W](apbnsppc0::W) writer structure"]
impl crate::Writable for APBNSPPC0 {}
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 0"]
pub mod apbnsppc0;
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnsppc1](apbnsppc1) module"]
pub type APBNSPPC1 = crate::Reg<u32, _APBNSPPC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPC1;
#[doc = "`read()` method returns [apbnsppc1::R](apbnsppc1::R) reader structure"]
impl crate::Readable for APBNSPPC1 {}
#[doc = "`write(|w| ..)` method takes [apbnsppc1::W](apbnsppc1::W) writer structure"]
impl crate::Writable for APBNSPPC1 {}
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 1"]
pub mod apbnsppc1;
#[doc = "Expansion 0 Non_Secure Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnsppcexp0](apbnsppcexp0) module"]
pub type APBNSPPCEXP0 = crate::Reg<u32, _APBNSPPCEXP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPCEXP0;
#[doc = "`read()` method returns [apbnsppcexp0::R](apbnsppcexp0::R) reader structure"]
impl crate::Readable for APBNSPPCEXP0 {}
#[doc = "`write(|w| ..)` method takes [apbnsppcexp0::W](apbnsppcexp0::W) writer structure"]
impl crate::Writable for APBNSPPCEXP0 {}
#[doc = "Expansion 0 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp0;
#[doc = "Expansion 1 Non_Secure Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnsppcexp1](apbnsppcexp1) module"]
pub type APBNSPPCEXP1 = crate::Reg<u32, _APBNSPPCEXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPCEXP1;
#[doc = "`read()` method returns [apbnsppcexp1::R](apbnsppcexp1::R) reader structure"]
impl crate::Readable for APBNSPPCEXP1 {}
#[doc = "`write(|w| ..)` method takes [apbnsppcexp1::W](apbnsppcexp1::W) writer structure"]
impl crate::Writable for APBNSPPCEXP1 {}
#[doc = "Expansion 1 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp1;
#[doc = "Expansion 2 Non_Secure Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnsppcexp2](apbnsppcexp2) module"]
pub type APBNSPPCEXP2 = crate::Reg<u32, _APBNSPPCEXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPCEXP2;
#[doc = "`read()` method returns [apbnsppcexp2::R](apbnsppcexp2::R) reader structure"]
impl crate::Readable for APBNSPPCEXP2 {}
#[doc = "`write(|w| ..)` method takes [apbnsppcexp2::W](apbnsppcexp2::W) writer structure"]
impl crate::Writable for APBNSPPCEXP2 {}
#[doc = "Expansion 2 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp2;
#[doc = "Expansion 3 Non_Secure Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnsppcexp3](apbnsppcexp3) module"]
pub type APBNSPPCEXP3 = crate::Reg<u32, _APBNSPPCEXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPCEXP3;
#[doc = "`read()` method returns [apbnsppcexp3::R](apbnsppcexp3::R) reader structure"]
impl crate::Readable for APBNSPPCEXP3 {}
#[doc = "`write(|w| ..)` method takes [apbnsppcexp3::W](apbnsppcexp3::W) writer structure"]
impl crate::Writable for APBNSPPCEXP3 {}
#[doc = "Expansion 3 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp3;
#[doc = "Secure Unprivileged Access AHB slave Peripheral Protection Control 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbspppc0](ahbspppc0) module"]
pub type AHBSPPPC0 = crate::Reg<u32, _AHBSPPPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSPPPC0;
#[doc = "`read()` method returns [ahbspppc0::R](ahbspppc0::R) reader structure"]
impl crate::Readable for AHBSPPPC0 {}
#[doc = "Secure Unprivileged Access AHB slave Peripheral Protection Control 0"]
pub mod ahbspppc0;
#[doc = "Expansion 0 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbspppcexp0](ahbspppcexp0) module"]
pub type AHBSPPPCEXP0 = crate::Reg<u32, _AHBSPPPCEXP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSPPPCEXP0;
#[doc = "`read()` method returns [ahbspppcexp0::R](ahbspppcexp0::R) reader structure"]
impl crate::Readable for AHBSPPPCEXP0 {}
#[doc = "`write(|w| ..)` method takes [ahbspppcexp0::W](ahbspppcexp0::W) writer structure"]
impl crate::Writable for AHBSPPPCEXP0 {}
#[doc = "Expansion 0 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp0;
#[doc = "Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbspppcexp1](ahbspppcexp1) module"]
pub type AHBSPPPCEXP1 = crate::Reg<u32, _AHBSPPPCEXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSPPPCEXP1;
#[doc = "`read()` method returns [ahbspppcexp1::R](ahbspppcexp1::R) reader structure"]
impl crate::Readable for AHBSPPPCEXP1 {}
#[doc = "`write(|w| ..)` method takes [ahbspppcexp1::W](ahbspppcexp1::W) writer structure"]
impl crate::Writable for AHBSPPPCEXP1 {}
#[doc = "Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp1;
#[doc = "Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbspppcexp2](ahbspppcexp2) module"]
pub type AHBSPPPCEXP2 = crate::Reg<u32, _AHBSPPPCEXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSPPPCEXP2;
#[doc = "`read()` method returns [ahbspppcexp2::R](ahbspppcexp2::R) reader structure"]
impl crate::Readable for AHBSPPPCEXP2 {}
#[doc = "`write(|w| ..)` method takes [ahbspppcexp2::W](ahbspppcexp2::W) writer structure"]
impl crate::Writable for AHBSPPPCEXP2 {}
#[doc = "Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp2;
#[doc = "Expansion 3 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbspppcexp3](ahbspppcexp3) module"]
pub type AHBSPPPCEXP3 = crate::Reg<u32, _AHBSPPPCEXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSPPPCEXP3;
#[doc = "`read()` method returns [ahbspppcexp3::R](ahbspppcexp3::R) reader structure"]
impl crate::Readable for AHBSPPPCEXP3 {}
#[doc = "`write(|w| ..)` method takes [ahbspppcexp3::W](ahbspppcexp3::W) writer structure"]
impl crate::Writable for AHBSPPPCEXP3 {}
#[doc = "Expansion 3 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp3;
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbspppc0](apbspppc0) module"]
pub type APBSPPPC0 = crate::Reg<u32, _APBSPPPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSPPPC0;
#[doc = "`read()` method returns [apbspppc0::R](apbspppc0::R) reader structure"]
impl crate::Readable for APBSPPPC0 {}
#[doc = "`write(|w| ..)` method takes [apbspppc0::W](apbspppc0::W) writer structure"]
impl crate::Writable for APBSPPPC0 {}
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
pub mod apbspppc0;
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbspppc1](apbspppc1) module"]
pub type APBSPPPC1 = crate::Reg<u32, _APBSPPPC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSPPPC1;
#[doc = "`read()` method returns [apbspppc1::R](apbspppc1::R) reader structure"]
impl crate::Readable for APBSPPPC1 {}
#[doc = "`write(|w| ..)` method takes [apbspppc1::W](apbspppc1::W) writer structure"]
impl crate::Writable for APBSPPPC1 {}
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
pub mod apbspppc1;
#[doc = "Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbspppcexp0](apbspppcexp0) module"]
pub type APBSPPPCEXP0 = crate::Reg<u32, _APBSPPPCEXP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSPPPCEXP0;
#[doc = "`read()` method returns [apbspppcexp0::R](apbspppcexp0::R) reader structure"]
impl crate::Readable for APBSPPPCEXP0 {}
#[doc = "`write(|w| ..)` method takes [apbspppcexp0::W](apbspppcexp0::W) writer structure"]
impl crate::Writable for APBSPPPCEXP0 {}
#[doc = "Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp0;
#[doc = "Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbspppcexp1](apbspppcexp1) module"]
pub type APBSPPPCEXP1 = crate::Reg<u32, _APBSPPPCEXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSPPPCEXP1;
#[doc = "`read()` method returns [apbspppcexp1::R](apbspppcexp1::R) reader structure"]
impl crate::Readable for APBSPPPCEXP1 {}
#[doc = "`write(|w| ..)` method takes [apbspppcexp1::W](apbspppcexp1::W) writer structure"]
impl crate::Writable for APBSPPPCEXP1 {}
#[doc = "Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp1;
#[doc = "Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbspppcexp2](apbspppcexp2) module"]
pub type APBSPPPCEXP2 = crate::Reg<u32, _APBSPPPCEXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSPPPCEXP2;
#[doc = "`read()` method returns [apbspppcexp2::R](apbspppcexp2::R) reader structure"]
impl crate::Readable for APBSPPPCEXP2 {}
#[doc = "`write(|w| ..)` method takes [apbspppcexp2::W](apbspppcexp2::W) writer structure"]
impl crate::Writable for APBSPPPCEXP2 {}
#[doc = "Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp2;
#[doc = "Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbspppcexp3](apbspppcexp3) module"]
pub type APBSPPPCEXP3 = crate::Reg<u32, _APBSPPPCEXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBSPPPCEXP3;
#[doc = "`read()` method returns [apbspppcexp3::R](apbspppcexp3::R) reader structure"]
impl crate::Readable for APBSPPPCEXP3 {}
#[doc = "`write(|w| ..)` method takes [apbspppcexp3::W](apbspppcexp3::W) writer structure"]
impl crate::Writable for APBSPPPCEXP3 {}
#[doc = "Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp3;
#[doc = "Expansion MSC Non-Secure Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nsmscexp](nsmscexp) module"]
pub type NSMSCEXP = crate::Reg<u32, _NSMSCEXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSMSCEXP;
#[doc = "`read()` method returns [nsmscexp::R](nsmscexp::R) reader structure"]
impl crate::Readable for NSMSCEXP {}
#[doc = "Expansion MSC Non-Secure Configuration"]
pub mod nsmscexp;
#[doc = "Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid4](pid4) module"]
pub type PID4 = crate::Reg<u32, _PID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID4;
#[doc = "`read()` method returns [pid4::R](pid4::R) reader structure"]
impl crate::Readable for PID4 {}
#[doc = "Peripheral ID 4"]
pub mod pid4;
#[doc = "Peripheral ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid0](pid0) module"]
pub type PID0 = crate::Reg<u32, _PID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID0;
#[doc = "`read()` method returns [pid0::R](pid0::R) reader structure"]
impl crate::Readable for PID0 {}
#[doc = "Peripheral ID 0"]
pub mod pid0;
#[doc = "Peripheral ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid1](pid1) module"]
pub type PID1 = crate::Reg<u32, _PID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID1;
#[doc = "`read()` method returns [pid1::R](pid1::R) reader structure"]
impl crate::Readable for PID1 {}
#[doc = "Peripheral ID 1"]
pub mod pid1;
#[doc = "Peripheral ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid2](pid2) module"]
pub type PID2 = crate::Reg<u32, _PID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID2;
#[doc = "`read()` method returns [pid2::R](pid2::R) reader structure"]
impl crate::Readable for PID2 {}
#[doc = "Peripheral ID 2"]
pub mod pid2;
#[doc = "Peripheral ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid3](pid3) module"]
pub type PID3 = crate::Reg<u32, _PID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID3;
#[doc = "`read()` method returns [pid3::R](pid3::R) reader structure"]
impl crate::Readable for PID3 {}
#[doc = "Peripheral ID 3"]
pub mod pid3;
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
