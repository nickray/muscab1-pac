#[doc = "Reader of register CPU0_VTOR"]
pub type R = crate::R<u32, super::CPU0_VTOR>;
#[doc = "Writer for register CPU0_VTOR"]
pub type W = crate::W<u32, super::CPU0_VTOR>;
#[doc = "Register CPU0_VTOR `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::CPU0_VTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `CPU0_VTOR_SECURE`"]
pub type CPU0_VTOR_SECURE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CPU0_VTOR_SECURE`"]
pub struct CPU0_VTOR_SECURE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_VTOR_SECURE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Reset vector for CPU0 secure mode"]
    #[inline(always)]
    pub fn cpu0_vtor_secure(&self) -> CPU0_VTOR_SECURE_R {
        CPU0_VTOR_SECURE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Reset vector for CPU0 secure mode"]
    #[inline(always)]
    pub fn cpu0_vtor_secure(&mut self) -> CPU0_VTOR_SECURE_W {
        CPU0_VTOR_SECURE_W { w: self }
    }
}
