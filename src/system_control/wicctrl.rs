#[doc = "Reader of register WICCTRL"]
pub type R = crate::R<u32, super::WICCTRL>;
#[doc = "Writer for register WICCTRL"]
pub type W = crate::W<u32, super::WICCTRL>;
#[doc = "Register WICCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WICCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CPU0WICEN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0WICEN_STATUS_A {
    #[doc = "CPU 0 WIC request enabled"]
    ENABLE,
    #[doc = "CPU 0 WIC request disabled"]
    DISABLED,
}
impl From<CPU0WICEN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0WICEN_STATUS_A) -> Self {
        match variant {
            CPU0WICEN_STATUS_A::ENABLE => true,
            CPU0WICEN_STATUS_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU0WICEN_STATUS`"]
pub type CPU0WICEN_STATUS_R = crate::R<bool, CPU0WICEN_STATUS_A>;
impl CPU0WICEN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0WICEN_STATUS_A {
        match self.bits {
            true => CPU0WICEN_STATUS_A::ENABLE,
            false => CPU0WICEN_STATUS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0WICEN_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU0WICEN_STATUS_A::DISABLED
    }
}
#[doc = "Possible values of the field `CPU1WICEN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1WICEN_STATUS_A {
    #[doc = "CPU 1 WIC request enabled"]
    ENABLE,
    #[doc = "CPU 1 WIC request disabled"]
    DISABLED,
}
impl From<CPU1WICEN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1WICEN_STATUS_A) -> Self {
        match variant {
            CPU1WICEN_STATUS_A::ENABLE => true,
            CPU1WICEN_STATUS_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU1WICEN_STATUS`"]
pub type CPU1WICEN_STATUS_R = crate::R<bool, CPU1WICEN_STATUS_A>;
impl CPU1WICEN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1WICEN_STATUS_A {
        match self.bits {
            true => CPU1WICEN_STATUS_A::ENABLE,
            false => CPU1WICEN_STATUS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1WICEN_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU1WICEN_STATUS_A::DISABLED
    }
}
#[doc = "Write proxy for field `CPU0WICEN_SET`"]
pub struct CPU0WICEN_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0WICEN_SET_W<'a> {
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
#[doc = "Write proxy for field `CPU1WICEN_SET`"]
pub struct CPU1WICEN_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1WICEN_SET_W<'a> {
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
#[doc = "Write proxy for field `CPU0WICEN_CLR`"]
pub struct CPU0WICEN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0WICEN_CLR_W<'a> {
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
#[doc = "Write proxy for field `CPU1WICEN_CLR`"]
pub struct CPU1WICEN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1WICEN_CLR_W<'a> {
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
#[doc = "Possible values of the field `CPU0WICRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0WICRDY_A {
    #[doc = "CPU 0 WIC Enabled"]
    ENABLED,
    #[doc = "CPU 0 WIC Disabled"]
    DISABLED,
}
impl From<CPU0WICRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0WICRDY_A) -> Self {
        match variant {
            CPU0WICRDY_A::ENABLED => true,
            CPU0WICRDY_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU0WICRDY`"]
pub type CPU0WICRDY_R = crate::R<bool, CPU0WICRDY_A>;
impl CPU0WICRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0WICRDY_A {
        match self.bits {
            true => CPU0WICRDY_A::ENABLED,
            false => CPU0WICRDY_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPU0WICRDY_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU0WICRDY_A::DISABLED
    }
}
#[doc = "Possible values of the field `CPU1WICRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1WICRDY_A {
    #[doc = "CPU 1 WIC Enabled"]
    ENABLED,
    #[doc = "CPU 1 WIC Disabled"]
    DISABLED,
}
impl From<CPU1WICRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1WICRDY_A) -> Self {
        match variant {
            CPU1WICRDY_A::ENABLED => true,
            CPU1WICRDY_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CPU1WICRDY`"]
pub type CPU1WICRDY_R = crate::R<bool, CPU1WICRDY_A>;
impl CPU1WICRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1WICRDY_A {
        match self.bits {
            true => CPU1WICRDY_A::ENABLED,
            false => CPU1WICRDY_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPU1WICRDY_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPU1WICRDY_A::DISABLED
    }
}
impl R {
    #[doc = "Bit 0 - CPU 0 WIC Enable Request Status"]
    #[inline(always)]
    pub fn cpu0wicen_status(&self) -> CPU0WICEN_STATUS_R {
        CPU0WICEN_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU 1 WIC Enable Request Status"]
    #[inline(always)]
    pub fn cpu1wicen_status(&self) -> CPU1WICEN_STATUS_R {
        CPU1WICEN_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU 0 WIC Enable Acknowledge"]
    #[inline(always)]
    pub fn cpu0wicrdy(&self) -> CPU0WICRDY_R {
        CPU0WICRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU 1 WIC Enable Acknowledge"]
    #[inline(always)]
    pub fn cpu1wicrdy(&self) -> CPU1WICRDY_R {
        CPU1WICRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - High Active CPU 0 WIC Enable Request Set"]
    #[inline(always)]
    pub fn cpu0wicen_set(&mut self) -> CPU0WICEN_SET_W {
        CPU0WICEN_SET_W { w: self }
    }
    #[doc = "Bit 5 - High Active CPU 1 WIC Enable Request Set"]
    #[inline(always)]
    pub fn cpu1wicen_set(&mut self) -> CPU1WICEN_SET_W {
        CPU1WICEN_SET_W { w: self }
    }
    #[doc = "Bit 8 - High Active CPU 0 WIC Enable Request Clear"]
    #[inline(always)]
    pub fn cpu0wicen_clr(&mut self) -> CPU0WICEN_CLR_W {
        CPU0WICEN_CLR_W { w: self }
    }
    #[doc = "Bit 9 - High Active CPU 1 WIC Enable Request Clear"]
    #[inline(always)]
    pub fn cpu1wicen_clr(&mut self) -> CPU1WICEN_CLR_W {
        CPU1WICEN_CLR_W { w: self }
    }
}
