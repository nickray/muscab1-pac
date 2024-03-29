#[doc = "Reader of register CLK_POSTDIV_CTRL_RTC"]
pub type R = crate::R<u32, super::CLK_POSTDIV_CTRL_RTC>;
#[doc = "Writer for register CLK_POSTDIV_CTRL_RTC"]
pub type W = crate::W<u32, super::CLK_POSTDIV_CTRL_RTC>;
#[doc = "Register CLK_POSTDIV_CTRL_RTC `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CLK_POSTDIV_CTRL_RTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `postdiv_ctrl_rtc_div`"]
pub type POSTDIV_CTRL_RTC_DIV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `postdiv_ctrl_rtc_div`"]
pub struct POSTDIV_CTRL_RTC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIV_CTRL_RTC_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - postdiv_ctrl_rtc_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_rtc_div(&self) -> POSTDIV_CTRL_RTC_DIV_R {
        POSTDIV_CTRL_RTC_DIV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - postdiv_ctrl_rtc_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_rtc_div(&mut self) -> POSTDIV_CTRL_RTC_DIV_W {
        POSTDIV_CTRL_RTC_DIV_W { w: self }
    }
}
