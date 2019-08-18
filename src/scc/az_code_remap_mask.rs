#[doc = "Reader of register AZ_CODE_REMAP_MASK"]
pub type R = crate::R<u32, super::AZ_CODE_REMAP_MASK>;
#[doc = "Writer for register AZ_CODE_REMAP_MASK"]
pub type W = crate::W<u32, super::AZ_CODE_REMAP_MASK>;
#[doc = "Register AZ_CODE_REMAP_MASK `reset()`'s with value 0x00ff_ffff"]
impl crate::ResetValue for super::AZ_CODE_REMAP_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ffff
    }
}
#[doc = "Reader of field `az_code_remap_mask`"]
pub type AZ_CODE_REMAP_MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `az_code_remap_mask`"]
pub struct AZ_CODE_REMAP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_CODE_REMAP_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alcatraz code remap mask"]
    #[inline(always)]
    pub fn az_code_remap_mask(&self) -> AZ_CODE_REMAP_MASK_R {
        AZ_CODE_REMAP_MASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz code remap mask"]
    #[inline(always)]
    pub fn az_code_remap_mask(&mut self) -> AZ_CODE_REMAP_MASK_W {
        AZ_CODE_REMAP_MASK_W { w: self }
    }
}
