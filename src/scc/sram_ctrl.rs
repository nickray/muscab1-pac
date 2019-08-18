#[doc = "Reader of register SRAM_CTRL"]
pub type R = crate::R<u32, super::SRAM_CTRL>;
#[doc = "Writer for register SRAM_CTRL"]
pub type W = crate::W<u32, super::SRAM_CTRL>;
#[doc = "Register SRAM_CTRL `reset()`'s with value 0x4810_0000"]
impl crate::ResetValue for super::SRAM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4810_0000
    }
}
#[doc = "Reader of field `CODE_SRAMx_PGEN`"]
pub type CODE_SRAMX_PGEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CODE_SRAMx_PGEN`"]
pub struct CODE_SRAMX_PGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODE_SRAMX_PGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SRAM cell power gate enable"]
    #[inline(always)]
    pub fn code_sramx_pgen(&self) -> CODE_SRAMX_PGEN_R {
        CODE_SRAMX_PGEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRAM cell power gate enable"]
    #[inline(always)]
    pub fn code_sramx_pgen(&mut self) -> CODE_SRAMX_PGEN_W {
        CODE_SRAMX_PGEN_W { w: self }
    }
}
