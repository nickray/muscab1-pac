#[doc = "Reader of register GPTINTM"]
pub type R = crate::R<u32, super::GPTINTM>;
#[doc = "Writer for register GPTINTM"]
pub type W = crate::W<u32, super::GPTINTM>;
#[doc = "Register GPTINTM `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTINTM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTINTM`"]
pub type GPTINTM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTINTM`"]
pub struct GPTINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTINTM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Current masked status of the interrupt"]
    #[inline(always)]
    pub fn gptintm(&self) -> GPTINTM_R {
        GPTINTM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current masked status of the interrupt"]
    #[inline(always)]
    pub fn gptintm(&mut self) -> GPTINTM_W {
        GPTINTM_W { w: self }
    }
}
