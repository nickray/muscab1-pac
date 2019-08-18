#[doc = "Reader of register FLASH_DIN_1"]
pub type R = crate::R<u32, super::FLASH_DIN_1>;
#[doc = "Writer for register FLASH_DIN_1"]
pub type W = crate::W<u32, super::FLASH_DIN_1>;
#[doc = "Register FLASH_DIN_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DIN_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `scc_flash_din1`"]
pub type SCC_FLASH_DIN1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `scc_flash_din1`"]
pub struct SCC_FLASH_DIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCC_FLASH_DIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input{63:32\\]"]
    #[inline(always)]
    pub fn scc_flash_din1(&self) -> SCC_FLASH_DIN1_R {
        SCC_FLASH_DIN1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input{63:32\\]"]
    #[inline(always)]
    pub fn scc_flash_din1(&mut self) -> SCC_FLASH_DIN1_W {
        SCC_FLASH_DIN1_W { w: self }
    }
}
