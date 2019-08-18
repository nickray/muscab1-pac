#[doc = "Reader of register FLASH_DIN_2"]
pub type R = crate::R<u32, super::FLASH_DIN_2>;
#[doc = "Writer for register FLASH_DIN_2"]
pub type W = crate::W<u32, super::FLASH_DIN_2>;
#[doc = "Register FLASH_DIN_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DIN_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `scc_flash_din2`"]
pub type SCC_FLASH_DIN2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `scc_flash_din2`"]
pub struct SCC_FLASH_DIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCC_FLASH_DIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash_din2(&self) -> SCC_FLASH_DIN2_R {
        SCC_FLASH_DIN2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - eFlash 0 and eFlash 1 data input\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash_din2(&mut self) -> SCC_FLASH_DIN2_W {
        SCC_FLASH_DIN2_W { w: self }
    }
}
