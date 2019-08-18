#[doc = "Reader of register IOMUX_ALTF2_INSEL_0"]
pub type R = crate::R<u32, super::IOMUX_ALTF2_INSEL_0>;
#[doc = "Writer for register IOMUX_ALTF2_INSEL_0"]
pub type W = crate::W<u32, super::IOMUX_ALTF2_INSEL_0>;
#[doc = "Register IOMUX_ALTF2_INSEL_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOMUX_ALTF2_INSEL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `iomux_altf2_insel_0`"]
pub type IOMUX_ALTF2_INSEL_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `iomux_altf2_insel_0`"]
pub struct IOMUX_ALTF2_INSEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUX_ALTF2_INSEL_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_insel_0(&self) -> IOMUX_ALTF2_INSEL_0_R {
        IOMUX_ALTF2_INSEL_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_insel_0(&mut self) -> IOMUX_ALTF2_INSEL_0_W {
        IOMUX_ALTF2_INSEL_0_W { w: self }
    }
}
