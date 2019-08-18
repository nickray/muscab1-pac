#[doc = "Reader of register WDOGLOAD"]
pub type R = crate::R<u32, super::WDOGLOAD>;
#[doc = "Writer for register WDOGLOAD"]
pub type W = crate::W<u32, super::WDOGLOAD>;
#[doc = "Register WDOGLOAD `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::WDOGLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
