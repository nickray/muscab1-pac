#[doc = "Reader of register SPARE_CTRL0"]
pub type R = crate::R<u32, super::SPARE_CTRL0>;
#[doc = "Writer for register SPARE_CTRL0"]
pub type W = crate::W<u32, super::SPARE_CTRL0>;
#[doc = "Register SPARE_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPARE_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `spare_ctrl0`"]
pub type SPARE_CTRL0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `spare_ctrl0`"]
pub struct SPARE_CTRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_CTRL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl0(&self) -> SPARE_CTRL0_R {
        SPARE_CTRL0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl0(&mut self) -> SPARE_CTRL0_W {
        SPARE_CTRL0_W { w: self }
    }
}
