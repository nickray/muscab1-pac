#[doc = "Reader of register ICIRQEN"]
pub type R = crate::R<u32, super::ICIRQEN>;
#[doc = "Writer for register ICIRQEN"]
pub type W = crate::W<u32, super::ICIRQEN>;
#[doc = "Register ICIRQEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ICIRQEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `IC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_EN_A {
    #[doc = "Enable the Invalidate\n                          Complete IRQ"]
    ENABLED,
    #[doc = "Disable the Invalidate\n                          Complete IRQ"]
    DISABLED,
}
impl From<IC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IC_EN_A) -> Self {
        match variant {
            IC_EN_A::ENABLED => true,
            IC_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `IC_EN`"]
pub type IC_EN_R = crate::R<bool, IC_EN_A>;
impl IC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_EN_A {
        match self.bits {
            true => IC_EN_A::ENABLED,
            false => IC_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IC_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IC_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `IC_EN`"]
pub struct IC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Invalidate Complete IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IC_EN_A::ENABLED)
    }
    #[doc = "Disable the Invalidate Complete IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IC_EN_A::DISABLED)
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
#[doc = "Possible values of the field `CDC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDC_EN_A {
    #[doc = "Enable the Cache Disable\n                          Complete IRQ"]
    ENABLED,
    #[doc = "Disable the Cache Disable\n                          Complete IRQ"]
    DISABLED,
}
impl From<CDC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CDC_EN_A) -> Self {
        match variant {
            CDC_EN_A::ENABLED => true,
            CDC_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CDC_EN`"]
pub type CDC_EN_R = crate::R<bool, CDC_EN_A>;
impl CDC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDC_EN_A {
        match self.bits {
            true => CDC_EN_A::ENABLED,
            false => CDC_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CDC_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CDC_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CDC_EN`"]
pub struct CDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Cache Disable Complete IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CDC_EN_A::ENABLED)
    }
    #[doc = "Disable the Cache Disable Complete IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CDC_EN_A::DISABLED)
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
#[doc = "Possible values of the field `CEC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEC_EN_A {
    #[doc = "Enable the Cache Enable\n                          Complete IRQ"]
    ENABLED,
    #[doc = "Disable the Cache Enable\n                          Complete IRQ"]
    DISABLED,
}
impl From<CEC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CEC_EN_A) -> Self {
        match variant {
            CEC_EN_A::ENABLED => true,
            CEC_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CEC_EN`"]
pub type CEC_EN_R = crate::R<bool, CEC_EN_A>;
impl CEC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEC_EN_A {
        match self.bits {
            true => CEC_EN_A::ENABLED,
            false => CEC_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEC_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEC_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CEC_EN`"]
pub struct CEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Cache Enable Complete IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEC_EN_A::ENABLED)
    }
    #[doc = "Disable the Cache Enable Complete IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEC_EN_A::DISABLED)
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
#[doc = "Possible values of the field `CFE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFE_EN_A {
    #[doc = "Enable the Cache Fill\n                          Error IRQ"]
    ENABLED,
    #[doc = "Disable the Cache Fill\n                          Error IRQ"]
    DISABLED,
}
impl From<CFE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CFE_EN_A) -> Self {
        match variant {
            CFE_EN_A::ENABLED => true,
            CFE_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CFE_EN`"]
pub type CFE_EN_R = crate::R<bool, CFE_EN_A>;
impl CFE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFE_EN_A {
        match self.bits {
            true => CFE_EN_A::ENABLED,
            false => CFE_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CFE_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CFE_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CFE_EN`"]
pub struct CFE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Cache Fill Error IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CFE_EN_A::ENABLED)
    }
    #[doc = "Disable the Cache Fill Error IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CFE_EN_A::DISABLED)
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
#[doc = "Possible values of the field `SV_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV_EN_A {
    #[doc = "Enable the Security\n                          violation IRQ"]
    ENABLED,
    #[doc = "Disable the Security\n                          violation IRQ"]
    DISABLED,
}
impl From<SV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV_EN_A) -> Self {
        match variant {
            SV_EN_A::ENABLED => true,
            SV_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `SV_EN`"]
pub type SV_EN_R = crate::R<bool, SV_EN_A>;
impl SV_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV_EN_A {
        match self.bits {
            true => SV_EN_A::ENABLED,
            false => SV_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `SV_EN`"]
pub struct SV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Security violation IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV_EN_A::ENABLED)
    }
    #[doc = "Disable the Security violation IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV_EN_A::DISABLED)
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
#[doc = "Possible values of the field `SS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_EN_A {
    #[doc = "Enable the Statistics Saturated"]
    ENABLED,
    #[doc = "Disable the Statistics Saturated"]
    DISABLED,
}
impl From<SS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SS_EN_A) -> Self {
        match variant {
            SS_EN_A::ENABLED => true,
            SS_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `SS_EN`"]
pub type SS_EN_R = crate::R<bool, SS_EN_A>;
impl SS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_EN_A {
        match self.bits {
            true => SS_EN_A::ENABLED,
            false => SS_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SS_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SS_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `SS_EN`"]
pub struct SS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Statistics Saturated"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SS_EN_A::ENABLED)
    }
    #[doc = "Disable the Statistics Saturated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SS_EN_A::DISABLED)
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
impl R {
    #[doc = "Bit 0 - Invalidate Complete IRQ Enable"]
    #[inline(always)]
    pub fn ic_en(&self) -> IC_EN_R {
        IC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cdc_en(&self) -> CDC_EN_R {
        CDC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cec_en(&self) -> CEC_EN_R {
        CEC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Enable"]
    #[inline(always)]
    pub fn cfe_en(&self) -> CFE_EN_R {
        CFE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security violation IRQ Enable"]
    #[inline(always)]
    pub fn sv_en(&self) -> SV_EN_R {
        SV_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Statistics Saturated Enable"]
    #[inline(always)]
    pub fn ss_en(&self) -> SS_EN_R {
        SS_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate Complete IRQ Enable"]
    #[inline(always)]
    pub fn ic_en(&mut self) -> IC_EN_W {
        IC_EN_W { w: self }
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cdc_en(&mut self) -> CDC_EN_W {
        CDC_EN_W { w: self }
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cec_en(&mut self) -> CEC_EN_W {
        CEC_EN_W { w: self }
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Enable"]
    #[inline(always)]
    pub fn cfe_en(&mut self) -> CFE_EN_W {
        CFE_EN_W { w: self }
    }
    #[doc = "Bit 4 - Security violation IRQ Enable"]
    #[inline(always)]
    pub fn sv_en(&mut self) -> SV_EN_W {
        SV_EN_W { w: self }
    }
    #[doc = "Bit 5 - Statistics Saturated Enable"]
    #[inline(always)]
    pub fn ss_en(&mut self) -> SS_EN_W {
        SS_EN_W { w: self }
    }
}
