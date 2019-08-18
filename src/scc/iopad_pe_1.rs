#[doc = "Reader of register IOPAD_PE_1"]
pub type R = crate::R<u32, super::IOPAD_PE_1>;
#[doc = "Writer for register IOPAD_PE_1"]
pub type W = crate::W<u32, super::IOPAD_PE_1>;
#[doc = "Register IOPAD_PE_1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IOPAD_PE_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `pull_enable`"]
pub type PULL_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pull_enable`"]
pub struct PULL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Enables pull resistors of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Enables pull resistors of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W {
        PULL_ENABLE_W { w: self }
    }
}
