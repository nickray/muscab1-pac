#[doc = "Reader of register IOMUX_ALTF2_INSEL_1"]
pub type R = crate::R<u32, super::IOMUX_ALTF2_INSEL_1>;
#[doc = "Writer for register IOMUX_ALTF2_INSEL_1"]
pub type W = crate::W<u32, super::IOMUX_ALTF2_INSEL_1>;
#[doc = "Register IOMUX_ALTF2_INSEL_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOMUX_ALTF2_INSEL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `iomux_altf2_insel_1`"]
pub type IOMUX_ALTF2_INSEL_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `iomux_altf2_insel_1`"]
pub struct IOMUX_ALTF2_INSEL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUX_ALTF2_INSEL_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_insel_1(&self) -> IOMUX_ALTF2_INSEL_1_R {
        IOMUX_ALTF2_INSEL_1_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 0: Select ATF3 1: Select ATF2"]
    #[inline(always)]
    pub fn iomux_altf2_insel_1(&mut self) -> IOMUX_ALTF2_INSEL_1_W {
        IOMUX_ALTF2_INSEL_1_W { w: self }
    }
}
