#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Type Register"]
    pub type_: TYPE,
    #[doc = "0x08 - Region Number Register"]
    pub rnr: RNR,
    #[doc = "0x0c - Region Base Address Register"]
    pub rbar: RBAR,
    #[doc = "0x10 - Region Limit Address Register"]
    pub rlar: RLAR,
    #[doc = "0x14 - Secure Fault Status Register"]
    pub sfsr: SFSR,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [type_](type_) module"]
pub type TYPE = crate::Reg<u32, _TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE;
#[doc = "`read()` method returns [type_::R](type_::R) reader structure"]
impl crate::Readable for TYPE {}
#[doc = "Type Register"]
pub mod type_;
#[doc = "Region Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rnr](rnr) module"]
pub type RNR = crate::Reg<u32, _RNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNR;
#[doc = "`read()` method returns [rnr::R](rnr::R) reader structure"]
impl crate::Readable for RNR {}
#[doc = "`write(|w| ..)` method takes [rnr::W](rnr::W) writer structure"]
impl crate::Writable for RNR {}
#[doc = "Region Number Register"]
pub mod rnr;
#[doc = "Region Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbar](rbar) module"]
pub type RBAR = crate::Reg<u32, _RBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBAR;
#[doc = "`read()` method returns [rbar::R](rbar::R) reader structure"]
impl crate::Readable for RBAR {}
#[doc = "`write(|w| ..)` method takes [rbar::W](rbar::W) writer structure"]
impl crate::Writable for RBAR {}
#[doc = "Region Base Address Register"]
pub mod rbar;
#[doc = "Region Limit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rlar](rlar) module"]
pub type RLAR = crate::Reg<u32, _RLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLAR;
#[doc = "`read()` method returns [rlar::R](rlar::R) reader structure"]
impl crate::Readable for RLAR {}
#[doc = "`write(|w| ..)` method takes [rlar::W](rlar::W) writer structure"]
impl crate::Writable for RLAR {}
#[doc = "Region Limit Address Register"]
pub mod rlar;
#[doc = "Secure Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sfsr](sfsr) module"]
pub type SFSR = crate::Reg<u32, _SFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFSR;
#[doc = "`read()` method returns [sfsr::R](sfsr::R) reader structure"]
impl crate::Readable for SFSR {}
#[doc = "`write(|w| ..)` method takes [sfsr::W](sfsr::W) writer structure"]
impl crate::Writable for SFSR {}
#[doc = "Secure Fault Status Register"]
pub mod sfsr;
