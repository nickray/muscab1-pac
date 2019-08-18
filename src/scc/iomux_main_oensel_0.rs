#[doc = "Reader of register IOMUX_MAIN_OENSEL_0"]
pub type R = crate::R<u32, super::IOMUX_MAIN_OENSEL_0>;
#[doc = "Writer for register IOMUX_MAIN_OENSEL_0"]
pub type W = crate::W<u32, super::IOMUX_MAIN_OENSEL_0>;
#[doc = "Register IOMUX_MAIN_OENSEL_0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IOMUX_MAIN_OENSEL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `iomux_main_oensel_0`"]
pub type IOMUX_MAIN_OENSEL_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `iomux_main_oensel_0`"]
pub struct IOMUX_MAIN_OENSEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUX_MAIN_OENSEL_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 0: Select ATF1 1: Select Main Function"]
    #[inline(always)]
    pub fn iomux_main_oensel_0(&self) -> IOMUX_MAIN_OENSEL_0_R {
        IOMUX_MAIN_OENSEL_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: Select ATF1 1: Select Main Function"]
    #[inline(always)]
    pub fn iomux_main_oensel_0(&mut self) -> IOMUX_MAIN_OENSEL_0_W {
        IOMUX_MAIN_OENSEL_0_W { w: self }
    }
}
