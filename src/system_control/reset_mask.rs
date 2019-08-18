#[doc = "Reader of register RESET_MASK"]
pub type R = crate::R<u32, super::RESET_MASK>;
#[doc = "Writer for register RESET_MASK"]
pub type W = crate::W<u32, super::RESET_MASK>;
#[doc = "Register RESET_MASK `reset()`'s with value 0x30"]
impl crate::ResetValue for super::RESET_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Possible values of the field `NSWD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWD_EN_A {
    #[doc = "Enable NON-SECURE WATCHDOG Reset"]
    ENABLED,
    #[doc = "Disabled NON-SECURE WATCHDOG Reset"]
    DISABLED,
}
impl From<NSWD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NSWD_EN_A) -> Self {
        match variant {
            NSWD_EN_A::ENABLED => true,
            NSWD_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `NSWD_EN`"]
pub type NSWD_EN_R = crate::R<bool, NSWD_EN_A>;
impl NSWD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSWD_EN_A {
        match self.bits {
            true => NSWD_EN_A::ENABLED,
            false => NSWD_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NSWD_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NSWD_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `NSWD_EN`"]
pub struct NSWD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSWD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NSWD_EN_A::ENABLED)
    }
    #[doc = "Disabled NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NSWD_EN_A::DISABLED)
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
#[doc = "Possible values of the field `SYSRSTREQ0_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTREQ0_EN_A {
    #[doc = "Enable Merging CPU 0 System Reset Request"]
    ENABLED,
    #[doc = "Disabled Merging CPU 0 System Reset Request"]
    DISABLED,
}
impl From<SYSRSTREQ0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRSTREQ0_EN_A) -> Self {
        match variant {
            SYSRSTREQ0_EN_A::ENABLED => true,
            SYSRSTREQ0_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `SYSRSTREQ0_EN`"]
pub type SYSRSTREQ0_EN_R = crate::R<bool, SYSRSTREQ0_EN_A>;
impl SYSRSTREQ0_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRSTREQ0_EN_A {
        match self.bits {
            true => SYSRSTREQ0_EN_A::ENABLED,
            false => SYSRSTREQ0_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSRSTREQ0_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRSTREQ0_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `SYSRSTREQ0_EN`"]
pub struct SYSRSTREQ0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTREQ0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRSTREQ0_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSRSTREQ0_EN_A::ENABLED)
    }
    #[doc = "Disabled Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSRSTREQ0_EN_A::DISABLED)
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
#[doc = "Possible values of the field `SYSRSTREQ1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTREQ1_EN_A {
    #[doc = "Enable Merging CPU 1 System Reset Request"]
    ENABLED,
    #[doc = "Disabled Merging CPU 1 System Reset Request"]
    DISABLED,
}
impl From<SYSRSTREQ1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRSTREQ1_EN_A) -> Self {
        match variant {
            SYSRSTREQ1_EN_A::ENABLED => true,
            SYSRSTREQ1_EN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `SYSRSTREQ1_EN`"]
pub type SYSRSTREQ1_EN_R = crate::R<bool, SYSRSTREQ1_EN_A>;
impl SYSRSTREQ1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRSTREQ1_EN_A {
        match self.bits {
            true => SYSRSTREQ1_EN_A::ENABLED,
            false => SYSRSTREQ1_EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSRSTREQ1_EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRSTREQ1_EN_A::DISABLED
    }
}
#[doc = "Write proxy for field `SYSRSTREQ1_EN`"]
pub struct SYSRSTREQ1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTREQ1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRSTREQ1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Merging CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSRSTREQ1_EN_A::ENABLED)
    }
    #[doc = "Disabled Merging CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSRSTREQ1_EN_A::DISABLED)
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
    #[doc = "Bit 1 - Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn nswd_en(&self) -> NSWD_EN_R {
        NSWD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq0_en(&self) -> SYSRSTREQ0_EN_R {
        SYSRSTREQ0_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq1_en(&self) -> SYSRSTREQ1_EN_R {
        SYSRSTREQ1_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn nswd_en(&mut self) -> NSWD_EN_W {
        NSWD_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq0_en(&mut self) -> SYSRSTREQ0_EN_W {
        SYSRSTREQ0_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq1_en(&mut self) -> SYSRSTREQ1_EN_W {
        SYSRSTREQ1_EN_W { w: self }
    }
}
