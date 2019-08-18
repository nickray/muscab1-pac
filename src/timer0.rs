#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current Timer Counter Value"]
    pub value: VALUE,
    #[doc = "0x08 - Counter Reload Value"]
    pub reload: RELOAD,
    _reserved_3_intclear: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x0c - Timer Interrupt clear register"]
    #[inline(always)]
    pub fn intclear(&self) -> &INTCLEAR {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const INTCLEAR) }
    }
    #[doc = "0x0c - Timer Interrupt clear register"]
    #[inline(always)]
    pub fn intclear_mut(&self) -> &mut INTCLEAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut INTCLEAR) }
    }
    #[doc = "0x0c - Timer Interrupt status register"]
    #[inline(always)]
    pub fn intstatus(&self) -> &INTSTATUS {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const INTSTATUS) }
    }
    #[doc = "0x0c - Timer Interrupt status register"]
    #[inline(always)]
    pub fn intstatus_mut(&self) -> &mut INTSTATUS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut INTSTATUS) }
    }
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
#[doc = "Current Timer Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [value](value) module"]
pub type VALUE = crate::Reg<u32, _VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE;
#[doc = "`read()` method returns [value::R](value::R) reader structure"]
impl crate::Readable for VALUE {}
#[doc = "`write(|w| ..)` method takes [value::W](value::W) writer structure"]
impl crate::Writable for VALUE {}
#[doc = "Current Timer Counter Value"]
pub mod value;
#[doc = "Counter Reload Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reload](reload) module"]
pub type RELOAD = crate::Reg<u32, _RELOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD;
#[doc = "`read()` method returns [reload::R](reload::R) reader structure"]
impl crate::Readable for RELOAD {}
#[doc = "`write(|w| ..)` method takes [reload::W](reload::W) writer structure"]
impl crate::Writable for RELOAD {}
#[doc = "Counter Reload Value"]
pub mod reload;
#[doc = "Timer Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Timer Interrupt status register"]
pub mod intstatus;
#[doc = "Timer Interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intclear](intclear) module"]
pub type INTCLEAR = crate::Reg<u32, _INTCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLEAR;
#[doc = "`write(|w| ..)` method takes [intclear::W](intclear::W) writer structure"]
impl crate::Writable for INTCLEAR {}
#[doc = "Timer Interrupt clear register"]
pub mod intclear;
