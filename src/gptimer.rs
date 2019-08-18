#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Reset Register"]
    pub gptreset: GPTRESET,
    #[doc = "0x04 - Masked interrupt status register"]
    pub gptintm: GPTINTM,
    #[doc = "0x08 - Interrupt clear register"]
    pub gptintc: GPTINTC,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - ALARM0 data value register"]
    pub gptalarm0: GPTALARM0,
    #[doc = "0x14 - ALARM1 data value register"]
    pub gptalarm1: GPTALARM1,
    #[doc = "0x18 - Raw interrupt status register"]
    pub gptintr: GPTINTR,
    #[doc = "0x1c - Counter data value register"]
    pub gptcounter: GPTCOUNTER,
}
#[doc = "Control Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptreset](gptreset) module"]
pub type GPTRESET = crate::Reg<u32, _GPTRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTRESET;
#[doc = "`read()` method returns [gptreset::R](gptreset::R) reader structure"]
impl crate::Readable for GPTRESET {}
#[doc = "Control Reset Register"]
pub mod gptreset;
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptintm](gptintm) module"]
pub type GPTINTM = crate::Reg<u32, _GPTINTM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTINTM;
#[doc = "`read()` method returns [gptintm::R](gptintm::R) reader structure"]
impl crate::Readable for GPTINTM {}
#[doc = "`write(|w| ..)` method takes [gptintm::W](gptintm::W) writer structure"]
impl crate::Writable for GPTINTM {}
#[doc = "Masked interrupt status register"]
pub mod gptintm;
#[doc = "Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptintc](gptintc) module"]
pub type GPTINTC = crate::Reg<u32, _GPTINTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTINTC;
#[doc = "`read()` method returns [gptintc::R](gptintc::R) reader structure"]
impl crate::Readable for GPTINTC {}
#[doc = "`write(|w| ..)` method takes [gptintc::W](gptintc::W) writer structure"]
impl crate::Writable for GPTINTC {}
#[doc = "Interrupt clear register"]
pub mod gptintc;
#[doc = "ALARM0 data value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptalarm0](gptalarm0) module"]
pub type GPTALARM0 = crate::Reg<u32, _GPTALARM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTALARM0;
#[doc = "`read()` method returns [gptalarm0::R](gptalarm0::R) reader structure"]
impl crate::Readable for GPTALARM0 {}
#[doc = "`write(|w| ..)` method takes [gptalarm0::W](gptalarm0::W) writer structure"]
impl crate::Writable for GPTALARM0 {}
#[doc = "ALARM0 data value register"]
pub mod gptalarm0;
#[doc = "ALARM1 data value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptalarm1](gptalarm1) module"]
pub type GPTALARM1 = crate::Reg<u32, _GPTALARM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTALARM1;
#[doc = "`read()` method returns [gptalarm1::R](gptalarm1::R) reader structure"]
impl crate::Readable for GPTALARM1 {}
#[doc = "`write(|w| ..)` method takes [gptalarm1::W](gptalarm1::W) writer structure"]
impl crate::Writable for GPTALARM1 {}
#[doc = "ALARM1 data value register"]
pub mod gptalarm1;
#[doc = "Raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptintr](gptintr) module"]
pub type GPTINTR = crate::Reg<u32, _GPTINTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTINTR;
#[doc = "`read()` method returns [gptintr::R](gptintr::R) reader structure"]
impl crate::Readable for GPTINTR {}
#[doc = "Raw interrupt status register"]
pub mod gptintr;
#[doc = "Counter data value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptcounter](gptcounter) module"]
pub type GPTCOUNTER = crate::Reg<u32, _GPTCOUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTCOUNTER;
#[doc = "`read()` method returns [gptcounter::R](gptcounter::R) reader structure"]
impl crate::Readable for GPTCOUNTER {}
#[doc = "Counter data value register"]
pub mod gptcounter;
