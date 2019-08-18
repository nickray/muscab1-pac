#[doc = "Reader of register FCLK_DIV"]
pub type R = crate::R<u32, super::FCLK_DIV>;
#[doc = "Writer for register FCLK_DIV"]
pub type W = crate::W<u32, super::FCLK_DIV>;
#[doc = "Register FCLK_DIV `reset()`'s with value 0"]
impl crate::ResetValue for super::FCLK_DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCLKDIV`"]
pub type FCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCLKDIV`"]
pub struct FCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `FCLKDIV_CUR`"]
pub type FCLKDIV_CUR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - FCLK from MAINCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn fclkdiv(&self) -> FCLKDIV_R {
        FCLKDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Clock Divider Current Value."]
    #[inline(always)]
    pub fn fclkdiv_cur(&self) -> FCLKDIV_CUR_R {
        FCLKDIV_CUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - FCLK from MAINCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn fclkdiv(&mut self) -> FCLKDIV_W {
        FCLKDIV_W { w: self }
    }
}
