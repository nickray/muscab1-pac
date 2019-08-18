#[doc = "Reader of register PLL_CTRL_PLL0_CLK"]
pub type R = crate::R<u32, super::PLL_CTRL_PLL0_CLK>;
#[doc = "Writer for register PLL_CTRL_PLL0_CLK"]
pub type W = crate::W<u32, super::PLL_CTRL_PLL0_CLK>;
#[doc = "Register PLL_CTRL_PLL0_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_CTRL_PLL0_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pd_pll0`"]
pub type PD_PLL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pd_pll0`"]
pub struct PD_PLL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_PLL0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `pd_foutpostdiv1pd`"]
pub type PD_FOUTPOSTDIV1PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pd_foutpostdiv1pd`"]
pub struct PD_FOUTPOSTDIV1PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_FOUTPOSTDIV1PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `pd_foutpostdiv2pd`"]
pub type PD_FOUTPOSTDIV2PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pd_foutpostdiv2pd`"]
pub struct PD_FOUTPOSTDIV2PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_FOUTPOSTDIV2PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `pd_foutvcopd`"]
pub type PD_FOUTVCOPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pd_foutvcopd`"]
pub struct PD_FOUTVCOPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_FOUTVCOPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `bypass_pll0`"]
pub type BYPASS_PLL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass_pll0`"]
pub struct BYPASS_PLL0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_PLL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power down PLL0"]
    #[inline(always)]
    pub fn pd_pll0(&self) -> PD_PLL0_R {
        PD_PLL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power down FOUTPOSTDIV1PD:"]
    #[inline(always)]
    pub fn pd_foutpostdiv1pd(&self) -> PD_FOUTPOSTDIV1PD_R {
        PD_FOUTPOSTDIV1PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power down FOUTPOSTDIV2PD"]
    #[inline(always)]
    pub fn pd_foutpostdiv2pd(&self) -> PD_FOUTPOSTDIV2PD_R {
        PD_FOUTPOSTDIV2PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power down FOUTVCOPD"]
    #[inline(always)]
    pub fn pd_foutvcopd(&self) -> PD_FOUTVCOPD_R {
        PD_FOUTVCOPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bypass PLL0"]
    #[inline(always)]
    pub fn bypass_pll0(&self) -> BYPASS_PLL0_R {
        BYPASS_PLL0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down PLL0"]
    #[inline(always)]
    pub fn pd_pll0(&mut self) -> PD_PLL0_W {
        PD_PLL0_W { w: self }
    }
    #[doc = "Bit 1 - Power down FOUTPOSTDIV1PD:"]
    #[inline(always)]
    pub fn pd_foutpostdiv1pd(&mut self) -> PD_FOUTPOSTDIV1PD_W {
        PD_FOUTPOSTDIV1PD_W { w: self }
    }
    #[doc = "Bit 2 - Power down FOUTPOSTDIV2PD"]
    #[inline(always)]
    pub fn pd_foutpostdiv2pd(&mut self) -> PD_FOUTPOSTDIV2PD_W {
        PD_FOUTPOSTDIV2PD_W { w: self }
    }
    #[doc = "Bit 3 - Power down FOUTVCOPD"]
    #[inline(always)]
    pub fn pd_foutvcopd(&mut self) -> PD_FOUTVCOPD_W {
        PD_FOUTVCOPD_W { w: self }
    }
    #[doc = "Bit 4 - Bypass PLL0"]
    #[inline(always)]
    pub fn bypass_pll0(&mut self) -> BYPASS_PLL0_W {
        BYPASS_PLL0_W { w: self }
    }
}
