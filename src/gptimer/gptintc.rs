#[doc = "Reader of register GPTINTC"]
pub type R = crate::R<u32, super::GPTINTC>;
#[doc = "Writer for register GPTINTC"]
pub type W = crate::W<u32, super::GPTINTC>;
#[doc = "Register GPTINTC `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTINTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTINTC`"]
pub type GPTINTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTINTC`"]
pub struct GPTINTC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTINTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Writing 0b1 disables the ALARM\\[n\\] interrupt"]
    #[inline(always)]
    pub fn gptintc(&self) -> GPTINTC_R {
        GPTINTC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Writing 0b1 disables the ALARM\\[n\\] interrupt"]
    #[inline(always)]
    pub fn gptintc(&mut self) -> GPTINTC_W {
        GPTINTC_W { w: self }
    }
}
