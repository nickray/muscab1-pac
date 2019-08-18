#[doc = "Reader of register CLK_PLL_PREDIV_CTRL"]
pub type R = crate::R<u32, super::CLK_PLL_PREDIV_CTRL>;
#[doc = "Writer for register CLK_PLL_PREDIV_CTRL"]
pub type W = crate::W<u32, super::CLK_PLL_PREDIV_CTRL>;
#[doc = "Register CLK_PLL_PREDIV_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_PLL_PREDIV_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `prediv_ctrl`"]
pub type PREDIV_CTRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `prediv_ctrl`"]
pub struct PREDIV_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - prediv_ctrl"]
    #[inline(always)]
    pub fn prediv_ctrl(&self) -> PREDIV_CTRL_R {
        PREDIV_CTRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - prediv_ctrl"]
    #[inline(always)]
    pub fn prediv_ctrl(&mut self) -> PREDIV_CTRL_W {
        PREDIV_CTRL_W { w: self }
    }
}
