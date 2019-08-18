#[doc = "Reader of register CLK_TEST_CTRL"]
pub type R = crate::R<u32, super::CLK_TEST_CTRL>;
#[doc = "Writer for register CLK_TEST_CTRL"]
pub type W = crate::W<u32, super::CLK_TEST_CTRL>;
#[doc = "Register CLK_TEST_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TEST_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_TEST_SEL`"]
pub type CLK_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_TEST_SEL`"]
pub struct CLK_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CLK_TEST_EN`"]
pub type CLK_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_TEST_EN`"]
pub struct CLK_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_TEST_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLK_MAIN_FORCE_RDY`"]
pub type CLK_MAIN_FORCE_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_MAIN_FORCE_RDY`"]
pub struct CLK_MAIN_FORCE_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MAIN_FORCE_RDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select TESTMUX input"]
    #[inline(always)]
    pub fn clk_test_sel(&self) -> CLK_TEST_SEL_R {
        CLK_TEST_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn clk_test_en(&self) -> CLK_TEST_EN_R {
        CLK_TEST_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CLK_MAIN_FORCE_RDY"]
    #[inline(always)]
    pub fn clk_main_force_rdy(&self) -> CLK_MAIN_FORCE_RDY_R {
        CLK_MAIN_FORCE_RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select TESTMUX input"]
    #[inline(always)]
    pub fn clk_test_sel(&mut self) -> CLK_TEST_SEL_W {
        CLK_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 5 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn clk_test_en(&mut self) -> CLK_TEST_EN_W {
        CLK_TEST_EN_W { w: self }
    }
    #[doc = "Bit 6 - CLK_MAIN_FORCE_RDY"]
    #[inline(always)]
    pub fn clk_main_force_rdy(&mut self) -> CLK_MAIN_FORCE_RDY_W {
        CLK_MAIN_FORCE_RDY_W { w: self }
    }
}
