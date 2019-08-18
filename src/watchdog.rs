#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load Register"]
    pub wdogload: WDOGLOAD,
    #[doc = "0x04 - Watchdog Value Register"]
    pub wdogvalue: WDOGVALUE,
    #[doc = "0x08 - Watchdog Control Register"]
    pub wdogcontrol: WDOGCONTROL,
    #[doc = "0x0c - Watchdog Interrupt Clear Register"]
    pub wdogintclr: WDOGINTCLR,
    #[doc = "0x10 - Watchdog Raw Interrupt Status Register"]
    pub wdogris: WDOGRIS,
    #[doc = "0x14 - Watchdog Mask Interrupt Status Register"]
    pub wdogmis: WDOGMIS,
    _reserved6: [u8; 3048usize],
    #[doc = "0xc00 - Watchdog Lock Register"]
    pub wdoglock: WDOGLOCK,
}
#[doc = "Watchdog Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogload](wdogload) module"]
pub type WDOGLOAD = crate::Reg<u32, _WDOGLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGLOAD;
#[doc = "`read()` method returns [wdogload::R](wdogload::R) reader structure"]
impl crate::Readable for WDOGLOAD {}
#[doc = "`write(|w| ..)` method takes [wdogload::W](wdogload::W) writer structure"]
impl crate::Writable for WDOGLOAD {}
#[doc = "Watchdog Load Register"]
pub mod wdogload;
#[doc = "Watchdog Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogvalue](wdogvalue) module"]
pub type WDOGVALUE = crate::Reg<u32, _WDOGVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGVALUE;
#[doc = "`read()` method returns [wdogvalue::R](wdogvalue::R) reader structure"]
impl crate::Readable for WDOGVALUE {}
#[doc = "Watchdog Value Register"]
pub mod wdogvalue;
#[doc = "Watchdog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogcontrol](wdogcontrol) module"]
pub type WDOGCONTROL = crate::Reg<u32, _WDOGCONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGCONTROL;
#[doc = "`read()` method returns [wdogcontrol::R](wdogcontrol::R) reader structure"]
impl crate::Readable for WDOGCONTROL {}
#[doc = "`write(|w| ..)` method takes [wdogcontrol::W](wdogcontrol::W) writer structure"]
impl crate::Writable for WDOGCONTROL {}
#[doc = "Watchdog Control Register"]
pub mod wdogcontrol;
#[doc = "Watchdog Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogintclr](wdogintclr) module"]
pub type WDOGINTCLR = crate::Reg<u32, _WDOGINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGINTCLR;
#[doc = "`write(|w| ..)` method takes [wdogintclr::W](wdogintclr::W) writer structure"]
impl crate::Writable for WDOGINTCLR {}
#[doc = "Watchdog Interrupt Clear Register"]
pub mod wdogintclr;
#[doc = "Watchdog Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogris](wdogris) module"]
pub type WDOGRIS = crate::Reg<u32, _WDOGRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGRIS;
#[doc = "`read()` method returns [wdogris::R](wdogris::R) reader structure"]
impl crate::Readable for WDOGRIS {}
#[doc = "Watchdog Raw Interrupt Status Register"]
pub mod wdogris;
#[doc = "Watchdog Mask Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogmis](wdogmis) module"]
pub type WDOGMIS = crate::Reg<u32, _WDOGMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGMIS;
#[doc = "`read()` method returns [wdogmis::R](wdogmis::R) reader structure"]
impl crate::Readable for WDOGMIS {}
#[doc = "Watchdog Mask Interrupt Status Register"]
pub mod wdogmis;
#[doc = "Watchdog Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdoglock](wdoglock) module"]
pub type WDOGLOCK = crate::Reg<u32, _WDOGLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGLOCK;
#[doc = "`read()` method returns [wdoglock::R](wdoglock::R) reader structure"]
impl crate::Readable for WDOGLOCK {}
#[doc = "`write(|w| ..)` method takes [wdoglock::W](wdoglock::W) writer structure"]
impl crate::Writable for WDOGLOCK {}
#[doc = "Watchdog Lock Register"]
pub mod wdoglock;
