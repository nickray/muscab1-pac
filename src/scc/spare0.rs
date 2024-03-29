#[doc = "Reader of register SPARE0"]
pub type R = crate::R<u32, super::SPARE0>;
#[doc = "Writer for register SPARE0"]
pub type W = crate::W<u32, super::SPARE0>;
#[doc = "Register SPARE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPARE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `spare0`"]
pub type SPARE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `spare0`"]
pub struct SPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Spare read-write register for software"]
    #[inline(always)]
    pub fn spare0(&self) -> SPARE0_R {
        SPARE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare read-write register for software"]
    #[inline(always)]
    pub fn spare0(&mut self) -> SPARE0_W {
        SPARE0_W { w: self }
    }
}
