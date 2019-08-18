#[doc = "Reader of register AZ_OTP_RD_DATA"]
pub type R = crate::R<u32, super::AZ_OTP_RD_DATA>;
#[doc = "Writer for register AZ_OTP_RD_DATA"]
pub type W = crate::W<u32, super::AZ_OTP_RD_DATA>;
#[doc = "Register AZ_OTP_RD_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::AZ_OTP_RD_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `az_otp_rd_data`"]
pub type AZ_OTP_RD_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `az_otp_rd_data`"]
pub struct AZ_OTP_RD_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_OTP_RD_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alcatraz OTP read data"]
    #[inline(always)]
    pub fn az_otp_rd_data(&self) -> AZ_OTP_RD_DATA_R {
        AZ_OTP_RD_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz OTP read data"]
    #[inline(always)]
    pub fn az_otp_rd_data(&mut self) -> AZ_OTP_RD_DATA_W {
        AZ_OTP_RD_DATA_W { w: self }
    }
}
