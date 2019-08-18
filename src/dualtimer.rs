#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 1 Load Register"]
    pub timer1load: TIMER1LOAD,
    #[doc = "0x04 - Timer 1 Value Register"]
    pub timer1value: TIMER1VALUE,
    #[doc = "0x08 - Timer 1 Control Register"]
    pub timer1control: TIMER1CONTROL,
    #[doc = "0x0c - Timer 1 Interrupt Clear Register"]
    pub timer1intclr: TIMER1INTCLR,
    #[doc = "0x10 - Timer 1 Raw Interrupt Status Register"]
    pub timer1ris: TIMER1RIS,
    #[doc = "0x14 - Timer 1 Mask Interrupt Status Register"]
    pub timer1mis: TIMER1MIS,
    #[doc = "0x18 - Timer 1 Background Load Register"]
    pub timer1bgload: TIMER1BGLOAD,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Timer 2 Load Register"]
    pub timer2load: TIMER2LOAD,
    #[doc = "0x24 - Timer 2 Value Register"]
    pub timer2value: TIMER2VALUE,
    #[doc = "0x28 - Timer 2 Control Register"]
    pub timer2control: TIMER2CONTROL,
    #[doc = "0x2c - Timer 2 Interrupt Clear Register"]
    pub timer2intclr: TIMER2INTCLR,
    #[doc = "0x30 - Timer 2 Raw Interrupt Status Register"]
    pub timer2ris: TIMER2RIS,
    #[doc = "0x34 - Timer 2 Mask Interrupt Status Register"]
    pub timer2mis: TIMER2MIS,
    #[doc = "0x38 - Timer 2 Background Load Register"]
    pub timer2bgload: TIMER2BGLOAD,
}
#[doc = "Timer 1 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1load](timer1load) module"]
pub type TIMER1LOAD = crate::Reg<u32, _TIMER1LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1LOAD;
#[doc = "`read()` method returns [timer1load::R](timer1load::R) reader structure"]
impl crate::Readable for TIMER1LOAD {}
#[doc = "`write(|w| ..)` method takes [timer1load::W](timer1load::W) writer structure"]
impl crate::Writable for TIMER1LOAD {}
#[doc = "Timer 1 Load Register"]
pub mod timer1load;
#[doc = "Timer 1 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1value](timer1value) module"]
pub type TIMER1VALUE = crate::Reg<u32, _TIMER1VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1VALUE;
#[doc = "`read()` method returns [timer1value::R](timer1value::R) reader structure"]
impl crate::Readable for TIMER1VALUE {}
#[doc = "Timer 1 Value Register"]
pub mod timer1value;
#[doc = "Timer 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1control](timer1control) module"]
pub type TIMER1CONTROL = crate::Reg<u32, _TIMER1CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1CONTROL;
#[doc = "`read()` method returns [timer1control::R](timer1control::R) reader structure"]
impl crate::Readable for TIMER1CONTROL {}
#[doc = "`write(|w| ..)` method takes [timer1control::W](timer1control::W) writer structure"]
impl crate::Writable for TIMER1CONTROL {}
#[doc = "Timer 1 Control Register"]
pub mod timer1control;
#[doc = "Timer 1 Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1intclr](timer1intclr) module"]
pub type TIMER1INTCLR = crate::Reg<u32, _TIMER1INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1INTCLR;
#[doc = "`write(|w| ..)` method takes [timer1intclr::W](timer1intclr::W) writer structure"]
impl crate::Writable for TIMER1INTCLR {}
#[doc = "Timer 1 Interrupt Clear Register"]
pub mod timer1intclr;
#[doc = "Timer 1 Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1ris](timer1ris) module"]
pub type TIMER1RIS = crate::Reg<u32, _TIMER1RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1RIS;
#[doc = "`read()` method returns [timer1ris::R](timer1ris::R) reader structure"]
impl crate::Readable for TIMER1RIS {}
#[doc = "Timer 1 Raw Interrupt Status Register"]
pub mod timer1ris;
#[doc = "Timer 1 Mask Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1mis](timer1mis) module"]
pub type TIMER1MIS = crate::Reg<u32, _TIMER1MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1MIS;
#[doc = "`read()` method returns [timer1mis::R](timer1mis::R) reader structure"]
impl crate::Readable for TIMER1MIS {}
#[doc = "Timer 1 Mask Interrupt Status Register"]
pub mod timer1mis;
#[doc = "Timer 1 Background Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1bgload](timer1bgload) module"]
pub type TIMER1BGLOAD = crate::Reg<u32, _TIMER1BGLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1BGLOAD;
#[doc = "`read()` method returns [timer1bgload::R](timer1bgload::R) reader structure"]
impl crate::Readable for TIMER1BGLOAD {}
#[doc = "`write(|w| ..)` method takes [timer1bgload::W](timer1bgload::W) writer structure"]
impl crate::Writable for TIMER1BGLOAD {}
#[doc = "Timer 1 Background Load Register"]
pub mod timer1bgload;
#[doc = "Timer 2 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2load](timer2load) module"]
pub type TIMER2LOAD = crate::Reg<u32, _TIMER2LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2LOAD;
#[doc = "`read()` method returns [timer2load::R](timer2load::R) reader structure"]
impl crate::Readable for TIMER2LOAD {}
#[doc = "`write(|w| ..)` method takes [timer2load::W](timer2load::W) writer structure"]
impl crate::Writable for TIMER2LOAD {}
#[doc = "Timer 2 Load Register"]
pub mod timer2load;
#[doc = "Timer 2 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2value](timer2value) module"]
pub type TIMER2VALUE = crate::Reg<u32, _TIMER2VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2VALUE;
#[doc = "`read()` method returns [timer2value::R](timer2value::R) reader structure"]
impl crate::Readable for TIMER2VALUE {}
#[doc = "Timer 2 Value Register"]
pub mod timer2value;
#[doc = "Timer 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2control](timer2control) module"]
pub type TIMER2CONTROL = crate::Reg<u32, _TIMER2CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2CONTROL;
#[doc = "`read()` method returns [timer2control::R](timer2control::R) reader structure"]
impl crate::Readable for TIMER2CONTROL {}
#[doc = "`write(|w| ..)` method takes [timer2control::W](timer2control::W) writer structure"]
impl crate::Writable for TIMER2CONTROL {}
#[doc = "Timer 2 Control Register"]
pub mod timer2control;
#[doc = "Timer 2 Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2intclr](timer2intclr) module"]
pub type TIMER2INTCLR = crate::Reg<u32, _TIMER2INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2INTCLR;
#[doc = "`write(|w| ..)` method takes [timer2intclr::W](timer2intclr::W) writer structure"]
impl crate::Writable for TIMER2INTCLR {}
#[doc = "Timer 2 Interrupt Clear Register"]
pub mod timer2intclr;
#[doc = "Timer 2 Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2ris](timer2ris) module"]
pub type TIMER2RIS = crate::Reg<u32, _TIMER2RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2RIS;
#[doc = "`read()` method returns [timer2ris::R](timer2ris::R) reader structure"]
impl crate::Readable for TIMER2RIS {}
#[doc = "Timer 2 Raw Interrupt Status Register"]
pub mod timer2ris;
#[doc = "Timer 2 Mask Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2mis](timer2mis) module"]
pub type TIMER2MIS = crate::Reg<u32, _TIMER2MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2MIS;
#[doc = "`read()` method returns [timer2mis::R](timer2mis::R) reader structure"]
impl crate::Readable for TIMER2MIS {}
#[doc = "Timer 2 Mask Interrupt Status Register"]
pub mod timer2mis;
#[doc = "Timer 2 Background Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2bgload](timer2bgload) module"]
pub type TIMER2BGLOAD = crate::Reg<u32, _TIMER2BGLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2BGLOAD;
#[doc = "`read()` method returns [timer2bgload::R](timer2bgload::R) reader structure"]
impl crate::Readable for TIMER2BGLOAD {}
#[doc = "`write(|w| ..)` method takes [timer2bgload::W](timer2bgload::W) writer structure"]
impl crate::Writable for TIMER2BGLOAD {}
#[doc = "Timer 2 Background Load Register"]
pub mod timer2bgload;
