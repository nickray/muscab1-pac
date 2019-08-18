#[doc = "Reader of register SSE_OTP_RD_DATA"]
pub type R = crate::R<u32, super::SSE_OTP_RD_DATA>;
#[doc = "Reader of field `sse_otp_rd_data`"]
pub type SSE_OTP_RD_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SSE-200 OTP read data"]
    #[inline(always)]
    pub fn sse_otp_rd_data(&self) -> SSE_OTP_RD_DATA_R {
        SSE_OTP_RD_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
