#[doc = "Reader of register SPARE_CTRL1"]
pub type R = crate::R<u32, super::SPARE_CTRL1>;
#[doc = "Writer for register SPARE_CTRL1"]
pub type W = crate::W<u32, super::SPARE_CTRL1>;
#[doc = "Register SPARE_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPARE_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `spare_ctrl1`"]
pub type SPARE_CTRL1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `spare_ctrl1`"]
pub struct SPARE_CTRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_CTRL1_W<'a> {
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
    pub fn spare_ctrl1(&self) -> SPARE_CTRL1_R {
        SPARE_CTRL1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare control register"]
    #[inline(always)]
    pub fn spare_ctrl1(&mut self) -> SPARE_CTRL1_W {
        SPARE_CTRL1_W { w: self }
    }
}
