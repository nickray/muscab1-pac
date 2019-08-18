#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub data: DATA,
    #[doc = "0x04 - Data Output Register"]
    pub dataout: DATAOUT,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Ouptut enable set Register"]
    pub outenset: OUTENSET,
    #[doc = "0x14 - Ouptut enable clear Register"]
    pub outenclr: OUTENCLR,
    #[doc = "0x18 - Alternate function set Register"]
    pub altfuncset: ALTFUNCSET,
    #[doc = "0x1c - Alternate function clear Register"]
    pub altfuncclr: ALTFUNCCLR,
    #[doc = "0x20 - Interrupt enable set Register"]
    pub intenset: INTENSET,
    #[doc = "0x24 - Interrupt enable clear Register"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt type set Register"]
    pub inttypeset: INTTYPESET,
    #[doc = "0x2c - Interrupt type clear Register"]
    pub inttypeclr: INTTYPECLR,
    #[doc = "0x30 - Polarity-level, edge interrupt configuration set Register"]
    pub intpolset: INTPOLSET,
    #[doc = "0x34 - Polarity-level, edge interrupt configuration clear Register"]
    pub intpolclr: INTPOLCLR,
    _reserved_12_intclear: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x38 - Interrupt CLEAR Register"]
    #[inline(always)]
    pub fn intclear(&self) -> &INTCLEAR {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const INTCLEAR) }
    }
    #[doc = "0x38 - Interrupt CLEAR Register"]
    #[inline(always)]
    pub fn intclear_mut(&self) -> &mut INTCLEAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut INTCLEAR) }
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub fn intstatus(&self) -> &INTSTATUS {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const INTSTATUS) }
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub fn intstatus_mut(&self) -> &mut INTSTATUS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut INTSTATUS) }
    }
}
#[doc = "Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data Register"]
pub mod data;
#[doc = "Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataout](dataout) module"]
pub type DATAOUT = crate::Reg<u32, _DATAOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAOUT;
#[doc = "`read()` method returns [dataout::R](dataout::R) reader structure"]
impl crate::Readable for DATAOUT {}
#[doc = "`write(|w| ..)` method takes [dataout::W](dataout::W) writer structure"]
impl crate::Writable for DATAOUT {}
#[doc = "Data Output Register"]
pub mod dataout;
#[doc = "Ouptut enable set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outenset](outenset) module"]
pub type OUTENSET = crate::Reg<u32, _OUTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTENSET;
#[doc = "`read()` method returns [outenset::R](outenset::R) reader structure"]
impl crate::Readable for OUTENSET {}
#[doc = "`write(|w| ..)` method takes [outenset::W](outenset::W) writer structure"]
impl crate::Writable for OUTENSET {}
#[doc = "Ouptut enable set Register"]
pub mod outenset;
#[doc = "Ouptut enable clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outenclr](outenclr) module"]
pub type OUTENCLR = crate::Reg<u32, _OUTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTENCLR;
#[doc = "`read()` method returns [outenclr::R](outenclr::R) reader structure"]
impl crate::Readable for OUTENCLR {}
#[doc = "`write(|w| ..)` method takes [outenclr::W](outenclr::W) writer structure"]
impl crate::Writable for OUTENCLR {}
#[doc = "Ouptut enable clear Register"]
pub mod outenclr;
#[doc = "Alternate function set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altfuncset](altfuncset) module"]
pub type ALTFUNCSET = crate::Reg<u32, _ALTFUNCSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTFUNCSET;
#[doc = "`read()` method returns [altfuncset::R](altfuncset::R) reader structure"]
impl crate::Readable for ALTFUNCSET {}
#[doc = "`write(|w| ..)` method takes [altfuncset::W](altfuncset::W) writer structure"]
impl crate::Writable for ALTFUNCSET {}
#[doc = "Alternate function set Register"]
pub mod altfuncset;
#[doc = "Alternate function clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altfuncclr](altfuncclr) module"]
pub type ALTFUNCCLR = crate::Reg<u32, _ALTFUNCCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTFUNCCLR;
#[doc = "`read()` method returns [altfuncclr::R](altfuncclr::R) reader structure"]
impl crate::Readable for ALTFUNCCLR {}
#[doc = "`write(|w| ..)` method takes [altfuncclr::W](altfuncclr::W) writer structure"]
impl crate::Writable for ALTFUNCCLR {}
#[doc = "Alternate function clear Register"]
pub mod altfuncclr;
#[doc = "Interrupt enable set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt enable set Register"]
pub mod intenset;
#[doc = "Interrupt enable clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt enable clear Register"]
pub mod intenclr;
#[doc = "Interrupt type set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inttypeset](inttypeset) module"]
pub type INTTYPESET = crate::Reg<u32, _INTTYPESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTTYPESET;
#[doc = "`read()` method returns [inttypeset::R](inttypeset::R) reader structure"]
impl crate::Readable for INTTYPESET {}
#[doc = "`write(|w| ..)` method takes [inttypeset::W](inttypeset::W) writer structure"]
impl crate::Writable for INTTYPESET {}
#[doc = "Interrupt type set Register"]
pub mod inttypeset;
#[doc = "Interrupt type clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inttypeclr](inttypeclr) module"]
pub type INTTYPECLR = crate::Reg<u32, _INTTYPECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTTYPECLR;
#[doc = "`read()` method returns [inttypeclr::R](inttypeclr::R) reader structure"]
impl crate::Readable for INTTYPECLR {}
#[doc = "`write(|w| ..)` method takes [inttypeclr::W](inttypeclr::W) writer structure"]
impl crate::Writable for INTTYPECLR {}
#[doc = "Interrupt type clear Register"]
pub mod inttypeclr;
#[doc = "Polarity-level, edge interrupt configuration set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpolset](intpolset) module"]
pub type INTPOLSET = crate::Reg<u32, _INTPOLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPOLSET;
#[doc = "`read()` method returns [intpolset::R](intpolset::R) reader structure"]
impl crate::Readable for INTPOLSET {}
#[doc = "`write(|w| ..)` method takes [intpolset::W](intpolset::W) writer structure"]
impl crate::Writable for INTPOLSET {}
#[doc = "Polarity-level, edge interrupt configuration set Register"]
pub mod intpolset;
#[doc = "Polarity-level, edge interrupt configuration clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpolclr](intpolclr) module"]
pub type INTPOLCLR = crate::Reg<u32, _INTPOLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPOLCLR;
#[doc = "`read()` method returns [intpolclr::R](intpolclr::R) reader structure"]
impl crate::Readable for INTPOLCLR {}
#[doc = "`write(|w| ..)` method takes [intpolclr::W](intpolclr::W) writer structure"]
impl crate::Writable for INTPOLCLR {}
#[doc = "Polarity-level, edge interrupt configuration clear Register"]
pub mod intpolclr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "Interrupt CLEAR Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intclear](intclear) module"]
pub type INTCLEAR = crate::Reg<u32, _INTCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLEAR;
#[doc = "`write(|w| ..)` method takes [intclear::W](intclear::W) writer structure"]
impl crate::Writable for INTCLEAR {}
#[doc = "Interrupt CLEAR Register"]
pub mod intclear;
