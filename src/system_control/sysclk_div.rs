#[doc = "Reader of register SYSCLK_DIV"]
pub type R = crate::R<u32, super::SYSCLK_DIV>;
#[doc = "Writer for register SYSCLK_DIV"]
pub type W = crate::W<u32, super::SYSCLK_DIV>;
#[doc = "Register SYSCLK_DIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCLK_DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCLKDIV`"]
pub type SYSCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCLKDIV`"]
pub struct SYSCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `SYSCLKDIV_CUR`"]
pub type SYSCLKDIV_CUR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - SYSCLK from FCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn sysclkdiv(&self) -> SYSCLKDIV_R {
        SYSCLKDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Clock Divider Current Value"]
    #[inline(always)]
    pub fn sysclkdiv_cur(&self) -> SYSCLKDIV_CUR_R {
        SYSCLKDIV_CUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SYSCLK from FCLK Clock Divider Ratio Request"]
    #[inline(always)]
    pub fn sysclkdiv(&mut self) -> SYSCLKDIV_W {
        SYSCLKDIV_W { w: self }
    }
}
