#[doc = "Reader of register NMI_ENABLE"]
pub type R = crate::R<u32, super::NMI_ENABLE>;
#[doc = "Writer for register NMI_ENABLE"]
pub type W = crate::W<u32, super::NMI_ENABLE>;
#[doc = "Register NMI_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::NMI_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CPU0_INTNMI_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0_INTNMI_ENABLE_A {
    #[doc = "CPU0 Internally Sourced NMI Enabled"]
    ENABLE,
    #[doc = "CPU0 Internally Sourced NMI Disabled"]
    DISABLED,
}
impl From<CPU0_INTNMI_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0_INTNMI_ENABLE_A) -> Self {
        match variant {
            CPU0_INTNMI_ENABLE_A::ENABLE => true,
            CPU0_INTNMI_ENABLE_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU0_INTNMI_ENABLE`"]
pub type CPU0_INTNMI_ENABLE_R = crate::R<bool, CPU0_INTNMI_ENABLE_A>;
impl CPU0_INTNMI_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0_INTNMI_ENABLE_A {
        match self.bits {
            true => CPU0_INTNMI_ENABLE_A::ENABLE,
            false => CPU0_INTNMI_ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_INTNMI_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU0_INTNMI_ENABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `CPU0_INTNMI_ENABLE`"]
pub struct CPU0_INTNMI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_INTNMI_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_INTNMI_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU0 Internally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_INTNMI_ENABLE_A::ENABLE)
    }
    #[doc = "CPU0 Internally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPU0_INTNMI_ENABLE_A::DISABLED)
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
#[doc = "Possible values of the field `CPU1_INTNMI_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_INTNMI_ENABLE_A {
    #[doc = "CPU1 Internally Sourced NMI Enabled"]
    ENABLE,
    #[doc = "CPU1 Internally Sourced NMI Disabled"]
    DISABLED,
}
impl From<CPU1_INTNMI_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_INTNMI_ENABLE_A) -> Self {
        match variant {
            CPU1_INTNMI_ENABLE_A::ENABLE => true,
            CPU1_INTNMI_ENABLE_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU1_INTNMI_ENABLE`"]
pub type CPU1_INTNMI_ENABLE_R = crate::R<bool, CPU1_INTNMI_ENABLE_A>;
impl CPU1_INTNMI_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_INTNMI_ENABLE_A {
        match self.bits {
            true => CPU1_INTNMI_ENABLE_A::ENABLE,
            false => CPU1_INTNMI_ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1_INTNMI_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU1_INTNMI_ENABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `CPU1_INTNMI_ENABLE`"]
pub struct CPU1_INTNMI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_INTNMI_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_INTNMI_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU1 Internally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1_INTNMI_ENABLE_A::ENABLE)
    }
    #[doc = "CPU1 Internally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPU1_INTNMI_ENABLE_A::DISABLED)
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
#[doc = "Possible values of the field `CPU0_EXPNMI_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0_EXPNMI_ENABLE_A {
    #[doc = "CPU0 Externally Sourced NMI Enabled"]
    ENABLE,
    #[doc = "CPU0 Externally Sourced NMI Disabled"]
    DISABLED,
}
impl From<CPU0_EXPNMI_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0_EXPNMI_ENABLE_A) -> Self {
        match variant {
            CPU0_EXPNMI_ENABLE_A::ENABLE => true,
            CPU0_EXPNMI_ENABLE_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU0_EXPNMI_ENABLE`"]
pub type CPU0_EXPNMI_ENABLE_R = crate::R<bool, CPU0_EXPNMI_ENABLE_A>;
impl CPU0_EXPNMI_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0_EXPNMI_ENABLE_A {
        match self.bits {
            true => CPU0_EXPNMI_ENABLE_A::ENABLE,
            false => CPU0_EXPNMI_ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_EXPNMI_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU0_EXPNMI_ENABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `CPU0_EXPNMI_ENABLE`"]
pub struct CPU0_EXPNMI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_EXPNMI_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_EXPNMI_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU0 Externally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_EXPNMI_ENABLE_A::ENABLE)
    }
    #[doc = "CPU0 Externally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPU0_EXPNMI_ENABLE_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CPU1_EXPNMI_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_EXPNMI_ENABLE_A {
    #[doc = "CPU1 Externally Sourced NMI Enabled"]
    ENABLE,
    #[doc = "CPU1 Externally Sourced NMI Disabled"]
    DISABLED,
}
impl From<CPU1_EXPNMI_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_EXPNMI_ENABLE_A) -> Self {
        match variant {
            CPU1_EXPNMI_ENABLE_A::ENABLE => true,
            CPU1_EXPNMI_ENABLE_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU1_EXPNMI_ENABLE`"]
pub type CPU1_EXPNMI_ENABLE_R = crate::R<bool, CPU1_EXPNMI_ENABLE_A>;
impl CPU1_EXPNMI_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_EXPNMI_ENABLE_A {
        match self.bits {
            true => CPU1_EXPNMI_ENABLE_A::ENABLE,
            false => CPU1_EXPNMI_ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1_EXPNMI_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU1_EXPNMI_ENABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `CPU1_EXPNMI_ENABLE`"]
pub struct CPU1_EXPNMI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_EXPNMI_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_EXPNMI_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU1 Externally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1_EXPNMI_ENABLE_A::ENABLE)
    }
    #[doc = "CPU1 Externally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPU1_EXPNMI_ENABLE_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CPU0 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_intnmi_enable(&self) -> CPU0_INTNMI_ENABLE_R {
        CPU0_INTNMI_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU1 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_intnmi_enable(&self) -> CPU1_INTNMI_ENABLE_R {
        CPU1_INTNMI_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU0 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_expnmi_enable(&self) -> CPU0_EXPNMI_ENABLE_R {
        CPU0_EXPNMI_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU1 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_expnmi_enable(&self) -> CPU1_EXPNMI_ENABLE_R {
        CPU1_EXPNMI_ENABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU0 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_intnmi_enable(&mut self) -> CPU0_INTNMI_ENABLE_W {
        CPU0_INTNMI_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - CPU1 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_intnmi_enable(&mut self) -> CPU1_INTNMI_ENABLE_W {
        CPU1_INTNMI_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - CPU0 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_expnmi_enable(&mut self) -> CPU0_EXPNMI_ENABLE_W {
        CPU0_EXPNMI_ENABLE_W { w: self }
    }
    #[doc = "Bit 17 - CPU1 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_expnmi_enable(&mut self) -> CPU1_EXPNMI_ENABLE_W {
        CPU1_EXPNMI_ENABLE_W { w: self }
    }
}
