#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub uartdr: UARTDR,
    #[doc = "0x04 - Receive status register/error clear register"]
    pub uartrsr_uartecr: UARTRSR_UARTECR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - Flag register"]
    pub uartrfr: UARTRFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - IrDA low-power counter register"]
    pub uartilpr: UARTILPR,
    #[doc = "0x24 - Integer baud rate register"]
    pub uartibrd: UARTIBRD,
    #[doc = "0x28 - Fractional baud rate register"]
    pub uartfbrd: UARTFBRD,
    #[doc = "0x2c - Line control register"]
    pub uartlcr_h: UARTLCR_H,
    #[doc = "0x30 - Control register"]
    pub uartcr: UARTCR,
    #[doc = "0x34 - Interrupt FIFO level select register"]
    pub uartifls: UARTIFLS,
    #[doc = "0x38 - Interrupt mask set/clear register"]
    pub uartimsc: UARTIMSC,
    #[doc = "0x3c - Raw interrupt status register"]
    pub uartris: UARTRIS,
    #[doc = "0x40 - Masked interrupt status register"]
    pub uartmis: UARTMIS,
    #[doc = "0x44 - Interrupt clear register"]
    pub uarticr: UARTICR,
    #[doc = "0x48 - DMA control register"]
    pub uartdmacr: UARTDMACR,
}
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartdr](uartdr) module"]
pub type UARTDR = crate::Reg<u32, _UARTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTDR;
#[doc = "`read()` method returns [uartdr::R](uartdr::R) reader structure"]
impl crate::Readable for UARTDR {}
#[doc = "`write(|w| ..)` method takes [uartdr::W](uartdr::W) writer structure"]
impl crate::Writable for UARTDR {}
#[doc = "Data register"]
pub mod uartdr;
#[doc = "Receive status register/error clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartrsr_uartecr](uartrsr_uartecr) module"]
pub type UARTRSR_UARTECR = crate::Reg<u32, _UARTRSR_UARTECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRSR_UARTECR;
#[doc = "`read()` method returns [uartrsr_uartecr::R](uartrsr_uartecr::R) reader structure"]
impl crate::Readable for UARTRSR_UARTECR {}
#[doc = "`write(|w| ..)` method takes [uartrsr_uartecr::W](uartrsr_uartecr::W) writer structure"]
impl crate::Writable for UARTRSR_UARTECR {}
#[doc = "Receive status register/error clear register"]
pub mod uartrsr_uartecr;
#[doc = "Flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartrfr](uartrfr) module"]
pub type UARTRFR = crate::Reg<u32, _UARTRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRFR;
#[doc = "`read()` method returns [uartrfr::R](uartrfr::R) reader structure"]
impl crate::Readable for UARTRFR {}
#[doc = "Flag register"]
pub mod uartrfr;
#[doc = "IrDA low-power counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartilpr](uartilpr) module"]
pub type UARTILPR = crate::Reg<u32, _UARTILPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTILPR;
#[doc = "`read()` method returns [uartilpr::R](uartilpr::R) reader structure"]
impl crate::Readable for UARTILPR {}
#[doc = "`write(|w| ..)` method takes [uartilpr::W](uartilpr::W) writer structure"]
impl crate::Writable for UARTILPR {}
#[doc = "IrDA low-power counter register"]
pub mod uartilpr;
#[doc = "Integer baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartibrd](uartibrd) module"]
pub type UARTIBRD = crate::Reg<u32, _UARTIBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIBRD;
#[doc = "`read()` method returns [uartibrd::R](uartibrd::R) reader structure"]
impl crate::Readable for UARTIBRD {}
#[doc = "`write(|w| ..)` method takes [uartibrd::W](uartibrd::W) writer structure"]
impl crate::Writable for UARTIBRD {}
#[doc = "Integer baud rate register"]
pub mod uartibrd;
#[doc = "Fractional baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartfbrd](uartfbrd) module"]
pub type UARTFBRD = crate::Reg<u32, _UARTFBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTFBRD;
#[doc = "`read()` method returns [uartfbrd::R](uartfbrd::R) reader structure"]
impl crate::Readable for UARTFBRD {}
#[doc = "`write(|w| ..)` method takes [uartfbrd::W](uartfbrd::W) writer structure"]
impl crate::Writable for UARTFBRD {}
#[doc = "Fractional baud rate register"]
pub mod uartfbrd;
#[doc = "Line control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartlcr_h](uartlcr_h) module"]
pub type UARTLCR_H = crate::Reg<u32, _UARTLCR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTLCR_H;
#[doc = "`read()` method returns [uartlcr_h::R](uartlcr_h::R) reader structure"]
impl crate::Readable for UARTLCR_H {}
#[doc = "`write(|w| ..)` method takes [uartlcr_h::W](uartlcr_h::W) writer structure"]
impl crate::Writable for UARTLCR_H {}
#[doc = "Line control register"]
pub mod uartlcr_h;
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartcr](uartcr) module"]
pub type UARTCR = crate::Reg<u32, _UARTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCR;
#[doc = "`read()` method returns [uartcr::R](uartcr::R) reader structure"]
impl crate::Readable for UARTCR {}
#[doc = "`write(|w| ..)` method takes [uartcr::W](uartcr::W) writer structure"]
impl crate::Writable for UARTCR {}
#[doc = "Control register"]
pub mod uartcr;
#[doc = "Interrupt FIFO level select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartifls](uartifls) module"]
pub type UARTIFLS = crate::Reg<u32, _UARTIFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIFLS;
#[doc = "`read()` method returns [uartifls::R](uartifls::R) reader structure"]
impl crate::Readable for UARTIFLS {}
#[doc = "`write(|w| ..)` method takes [uartifls::W](uartifls::W) writer structure"]
impl crate::Writable for UARTIFLS {}
#[doc = "Interrupt FIFO level select register"]
pub mod uartifls;
#[doc = "Interrupt mask set/clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartimsc](uartimsc) module"]
pub type UARTIMSC = crate::Reg<u32, _UARTIMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIMSC;
#[doc = "`read()` method returns [uartimsc::R](uartimsc::R) reader structure"]
impl crate::Readable for UARTIMSC {}
#[doc = "`write(|w| ..)` method takes [uartimsc::W](uartimsc::W) writer structure"]
impl crate::Writable for UARTIMSC {}
#[doc = "Interrupt mask set/clear register"]
pub mod uartimsc;
#[doc = "Raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartris](uartris) module"]
pub type UARTRIS = crate::Reg<u32, _UARTRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRIS;
#[doc = "`read()` method returns [uartris::R](uartris::R) reader structure"]
impl crate::Readable for UARTRIS {}
#[doc = "Raw interrupt status register"]
pub mod uartris;
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartmis](uartmis) module"]
pub type UARTMIS = crate::Reg<u32, _UARTMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTMIS;
#[doc = "`read()` method returns [uartmis::R](uartmis::R) reader structure"]
impl crate::Readable for UARTMIS {}
#[doc = "Masked interrupt status register"]
pub mod uartmis;
#[doc = "Interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uarticr](uarticr) module"]
pub type UARTICR = crate::Reg<u32, _UARTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTICR;
#[doc = "`write(|w| ..)` method takes [uarticr::W](uarticr::W) writer structure"]
impl crate::Writable for UARTICR {}
#[doc = "Interrupt clear register"]
pub mod uarticr;
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartdmacr](uartdmacr) module"]
pub type UARTDMACR = crate::Reg<u32, _UARTDMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTDMACR;
#[doc = "`read()` method returns [uartdmacr::R](uartdmacr::R) reader structure"]
impl crate::Readable for UARTDMACR {}
#[doc = "`write(|w| ..)` method takes [uartdmacr::W](uartdmacr::W) writer structure"]
impl crate::Writable for UARTDMACR {}
#[doc = "DMA control register"]
pub mod uartdmacr;
