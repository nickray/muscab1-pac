#[doc = "Writer for register ICIRQSCLR"]
pub type W = crate::W<u32, super::ICIRQSCLR>;
#[doc = "Register ICIRQSCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICIRQSCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `IC_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_CLR_AW {
    #[doc = "Clear the Invalidate Complete\n                          IRQ Status"]
    CLEAR,
}
impl From<IC_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: IC_CLR_AW) -> Self {
        match variant {
            IC_CLR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `IC_CLR`"]
pub struct IC_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Invalidate Complete IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC_CLR_AW::CLEAR)
    }
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
#[doc = "Possible values of the field `CDC_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDC_CLR_AW {
    #[doc = "Clear Cache Disable Complete IRQ\n                          Status"]
    CLEAR,
}
impl From<CDC_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: CDC_CLR_AW) -> Self {
        match variant {
            CDC_CLR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `CDC_CLR`"]
pub struct CDC_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CDC_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDC_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear Cache Disable Complete IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDC_CLR_AW::CLEAR)
    }
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
#[doc = "Possible values of the field `CEC_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEC_CLR_AW {
    #[doc = "Clear the Cache Enable Complete\n                          IRQ Status"]
    CLEAR,
}
impl From<CEC_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: CEC_CLR_AW) -> Self {
        match variant {
            CEC_CLR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `CEC_CLR`"]
pub struct CEC_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CEC_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEC_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Cache Enable Complete IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CEC_CLR_AW::CLEAR)
    }
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
#[doc = "Possible values of the field `CFE_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFE_CLR_AW {
    #[doc = "Clear the Cache Fill Error\n                          IRQ Status"]
    CLEAR,
}
impl From<CFE_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: CFE_CLR_AW) -> Self {
        match variant {
            CFE_CLR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `CFE_CLR`"]
pub struct CFE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFE_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFE_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Cache Fill Error IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFE_CLR_AW::CLEAR)
    }
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
#[doc = "Possible values of the field `SV_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV_CLR_AW {
    #[doc = "Clear the Security violation\n                          IRQ Status"]
    CLEAR,
}
impl From<SV_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SV_CLR_AW) -> Self {
        match variant {
            SV_CLR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `SV_CLR`"]
pub struct SV_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SV_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Security violation IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SV_CLR_AW::CLEAR)
    }
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
#[doc = "Possible values of the field `SS_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_CLR_AW {
    #[doc = "Clear the Statistics Saturated\n                          Status"]
    CLEAR,
}
impl From<SS_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SS_CLR_AW) -> Self {
        match variant {
            SS_CLR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `SS_CLR`"]
pub struct SS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Statistics Saturated Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SS_CLR_AW::CLEAR)
    }
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
impl W {
    #[doc = "Bit 0 - Invalidate Complete IRQ Status Clear"]
    #[inline(always)]
    pub fn ic_clr(&mut self) -> IC_CLR_W {
        IC_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Status Clear"]
    #[inline(always)]
    pub fn cdc_clr(&mut self) -> CDC_CLR_W {
        CDC_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Status Clear"]
    #[inline(always)]
    pub fn cec_clr(&mut self) -> CEC_CLR_W {
        CEC_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Status Clear"]
    #[inline(always)]
    pub fn cfe_clr(&mut self) -> CFE_CLR_W {
        CFE_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Security violation IRQ Status Clear"]
    #[inline(always)]
    pub fn sv_clr(&mut self) -> SV_CLR_W {
        SV_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Statistics Saturated Status Clear"]
    #[inline(always)]
    pub fn ss_clr(&mut self) -> SS_CLR_W {
        SS_CLR_W { w: self }
    }
}
