#[doc = "Reader of register AZ_CPU_VTOR"]
pub type R = crate::R<u32, super::AZ_CPU_VTOR>;
#[doc = "Writer for register AZ_CPU_VTOR"]
pub type W = crate::W<u32, super::AZ_CPU_VTOR>;
#[doc = "Register AZ_CPU_VTOR `reset()`'s with value 0x00a0_3800"]
impl crate::ResetValue for super::AZ_CPU_VTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00a0_3800
    }
}
#[doc = "Reader of field `AZ_ROM_REMAP`"]
pub type AZ_ROM_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AZ_ROM_REMAP`"]
pub struct AZ_ROM_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_ROM_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AZ_CODE_REMAP`"]
pub type AZ_CODE_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AZ_CODE_REMAP`"]
pub struct AZ_CODE_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_CODE_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AZ_SYS_REMAP`"]
pub type AZ_SYS_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AZ_SYS_REMAP`"]
pub struct AZ_SYS_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_SYS_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Remap vector for Alcatraz ROM address space."]
    #[inline(always)]
    pub fn az_rom_remap(&self) -> AZ_ROM_REMAP_R {
        AZ_ROM_REMAP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Remap vector for Alcatraz Code address space"]
    #[inline(always)]
    pub fn az_code_remap(&self) -> AZ_CODE_REMAP_R {
        AZ_CODE_REMAP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Remap vector for Alcatraz System address space"]
    #[inline(always)]
    pub fn az_sys_remap(&self) -> AZ_SYS_REMAP_R {
        AZ_SYS_REMAP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Remap vector for Alcatraz ROM address space."]
    #[inline(always)]
    pub fn az_rom_remap(&mut self) -> AZ_ROM_REMAP_W {
        AZ_ROM_REMAP_W { w: self }
    }
    #[doc = "Bits 8:15 - Remap vector for Alcatraz Code address space"]
    #[inline(always)]
    pub fn az_code_remap(&mut self) -> AZ_CODE_REMAP_W {
        AZ_CODE_REMAP_W { w: self }
    }
    #[doc = "Bits 16:23 - Remap vector for Alcatraz System address space"]
    #[inline(always)]
    pub fn az_sys_remap(&mut self) -> AZ_SYS_REMAP_W {
        AZ_SYS_REMAP_W { w: self }
    }
}
