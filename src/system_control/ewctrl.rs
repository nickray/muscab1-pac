#[doc = "Reader of register EWCTRL"]
pub type R = crate::R<u32, super::EWCTRL>;
#[doc = "Writer for register EWCTRL"]
pub type W = crate::W<u32, super::EWCTRL>;
#[doc = "Register EWCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EWCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `EWC0EN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWC0EN_STATUS_A {
    #[doc = "External Wakeup Controller 0 Enabled"]
    ENABLE,
    #[doc = "External Wakeup Controller 0 Disabled"]
    DISABLED,
}
impl From<EWC0EN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EWC0EN_STATUS_A) -> Self {
        match variant {
            EWC0EN_STATUS_A::ENABLE => true,
            EWC0EN_STATUS_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `EWC0EN_STATUS`"]
pub type EWC0EN_STATUS_R = crate::R<bool, EWC0EN_STATUS_A>;
impl EWC0EN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWC0EN_STATUS_A {
        match self.bits {
            true => EWC0EN_STATUS_A::ENABLE,
            false => EWC0EN_STATUS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWC0EN_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWC0EN_STATUS_A::DISABLED
    }
}
#[doc = "Possible values of the field `EWC1EN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWC1EN_STATUS_A {
    #[doc = "External Wakeup Controller 1 Enabled"]
    ENABLE,
    #[doc = "External Wakeup Controller 1 Disabled"]
    DISABLED,
}
impl From<EWC1EN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EWC1EN_STATUS_A) -> Self {
        match variant {
            EWC1EN_STATUS_A::ENABLE => true,
            EWC1EN_STATUS_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `EWC1EN_STATUS`"]
pub type EWC1EN_STATUS_R = crate::R<bool, EWC1EN_STATUS_A>;
impl EWC1EN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWC1EN_STATUS_A {
        match self.bits {
            true => EWC1EN_STATUS_A::ENABLE,
            false => EWC1EN_STATUS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWC1EN_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWC1EN_STATUS_A::DISABLED
    }
}
#[doc = "Write proxy for field `EWC0EN_SET`"]
pub struct EWC0EN_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EWC0EN_SET_W<'a> {
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
#[doc = "Write proxy for field `EWC1EN_SET`"]
pub struct EWC1EN_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EWC1EN_SET_W<'a> {
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
#[doc = "Write proxy for field `EWC0EN_CLR`"]
pub struct EWC0EN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EWC0EN_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `EWC1EN_CLR`"]
pub struct EWC1EN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EWC1EN_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Wakeup Controller 0 Enable"]
    #[inline(always)]
    pub fn ewc0en_status(&self) -> EWC0EN_STATUS_R {
        EWC0EN_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Wakeup Controller 1 Enable"]
    #[inline(always)]
    pub fn ewc1en_status(&self) -> EWC1EN_STATUS_R {
        EWC1EN_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - EHigh Active External Wakeup Controller 0 Set"]
    #[inline(always)]
    pub fn ewc0en_set(&mut self) -> EWC0EN_SET_W {
        EWC0EN_SET_W { w: self }
    }
    #[doc = "Bit 5 - High Active External Wakeup Controller 1 Set"]
    #[inline(always)]
    pub fn ewc1en_set(&mut self) -> EWC1EN_SET_W {
        EWC1EN_SET_W { w: self }
    }
    #[doc = "Bit 8 - High Active External Wakeup Controller 0 Clear"]
    #[inline(always)]
    pub fn ewc0en_clr(&mut self) -> EWC0EN_CLR_W {
        EWC0EN_CLR_W { w: self }
    }
    #[doc = "Bit 9 - High Active External Wakeup Controller 1 Clear"]
    #[inline(always)]
    pub fn ewc1en_clr(&mut self) -> EWC1EN_CLR_W {
        EWC1EN_CLR_W { w: self }
    }
}
