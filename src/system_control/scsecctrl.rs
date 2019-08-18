#[doc = "Reader of register SCSECCTRL"]
pub type R = crate::R<u32, super::SCSECCTRL>;
#[doc = "Writer for register SCSECCTRL"]
pub type W = crate::W<u32, super::SCSECCTRL>;
#[doc = "Register SCSECCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SCSECCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CERTDISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERTDISABLE_A {
    #[doc = "control to disable certification path"]
    DISABLE,
    #[doc = "control to enable certification path"]
    ENABLE,
}
impl From<CERTDISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CERTDISABLE_A) -> Self {
        match variant {
            CERTDISABLE_A::DISABLE => true,
            CERTDISABLE_A::ENABLE => false,
        }
    }
}
#[doc = "Reader of field `CERTDISABLE`"]
pub type CERTDISABLE_R = crate::R<bool, CERTDISABLE_A>;
impl CERTDISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERTDISABLE_A {
        match self.bits {
            true => CERTDISABLE_A::DISABLE,
            false => CERTDISABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CERTDISABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CERTDISABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `CERTDISABLE`"]
pub struct CERTDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CERTDISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERTDISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "control to disable certification path"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CERTDISABLE_A::DISABLE)
    }
    #[doc = "control to enable certification path"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CERTDISABLE_A::ENABLE)
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
#[doc = "Possible values of the field `CERTREADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERTREADEN_A {
    #[doc = "control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    ENABLE,
    #[doc = "control to disable read access on the certification path as long as CERTDISABLE is also LOW"]
    DISABLE,
}
impl From<CERTREADEN_A> for bool {
    #[inline(always)]
    fn from(variant: CERTREADEN_A) -> Self {
        match variant {
            CERTREADEN_A::ENABLE => true,
            CERTREADEN_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `CERTREADEN`"]
pub type CERTREADEN_R = crate::R<bool, CERTREADEN_A>;
impl CERTREADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERTREADEN_A {
        match self.bits {
            true => CERTREADEN_A::ENABLE,
            false => CERTREADEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CERTREADEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CERTREADEN_A::DISABLE
    }
}
#[doc = "Write proxy for field `CERTREADEN`"]
pub struct CERTREADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CERTREADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERTREADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CERTREADEN_A::ENABLE)
    }
    #[doc = "control to disable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CERTREADEN_A::DISABLE)
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
#[doc = "Possible values of the field `SCSECCFGLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSECCFGLOCK_A {
    #[doc = "control to disable writes to security-related control registers in this register block"]
    DISABLE,
    #[doc = "control to enable writes to security-related control registers in this register block"]
    ENABLE,
}
impl From<SCSECCFGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SCSECCFGLOCK_A) -> Self {
        match variant {
            SCSECCFGLOCK_A::DISABLE => true,
            SCSECCFGLOCK_A::ENABLE => false,
        }
    }
}
#[doc = "Reader of field `SCSECCFGLOCK`"]
pub type SCSECCFGLOCK_R = crate::R<bool, SCSECCFGLOCK_A>;
impl SCSECCFGLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCSECCFGLOCK_A {
        match self.bits {
            true => SCSECCFGLOCK_A::DISABLE,
            false => SCSECCFGLOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCSECCFGLOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCSECCFGLOCK_A::ENABLE
    }
}
#[doc = "Write proxy for field `SCSECCFGLOCK`"]
pub struct SCSECCFGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSECCFGLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCSECCFGLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCSECCFGLOCK_A::DISABLE)
    }
    #[doc = "control to enable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCSECCFGLOCK_A::ENABLE)
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
#[doc = "Possible values of the field `CERTDISABLED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERTDISABLED_A {
    #[doc = "Certification write path has been disabled"]
    DISABLED,
    #[doc = "Certification write path has been enabled"]
    ENABLED,
}
impl From<CERTDISABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CERTDISABLED_A) -> Self {
        match variant {
            CERTDISABLED_A::DISABLED => true,
            CERTDISABLED_A::ENABLED => false,
        }
    }
}
#[doc = "Reader of field `CERTDISABLED`"]
pub type CERTDISABLED_R = crate::R<bool, CERTDISABLED_A>;
impl CERTDISABLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERTDISABLED_A {
        match self.bits {
            true => CERTDISABLED_A::DISABLED,
            false => CERTDISABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CERTDISABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CERTDISABLED_A::ENABLED
    }
}
#[doc = "Write proxy for field `CERTDISABLED`"]
pub struct CERTDISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> CERTDISABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERTDISABLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Certification write path has been disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CERTDISABLED_A::DISABLED)
    }
    #[doc = "Certification write path has been enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CERTDISABLED_A::ENABLED)
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
#[doc = "Possible values of the field `CERTREADENABLED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERTREADENABLED_A {
    #[doc = "certification read access is enabled"]
    ENABLED,
    #[doc = "certification read access is disabled"]
    DISABLED,
}
impl From<CERTREADENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CERTREADENABLED_A) -> Self {
        match variant {
            CERTREADENABLED_A::ENABLED => true,
            CERTREADENABLED_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CERTREADENABLED`"]
pub type CERTREADENABLED_R = crate::R<bool, CERTREADENABLED_A>;
impl CERTREADENABLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERTREADENABLED_A {
        match self.bits {
            true => CERTREADENABLED_A::ENABLED,
            false => CERTREADENABLED_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CERTREADENABLED_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CERTREADENABLED_A::DISABLED
    }
}
#[doc = "Write proxy for field `CERTREADENABLED`"]
pub struct CERTREADENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> CERTREADENABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERTREADENABLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "certification read access is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CERTREADENABLED_A::ENABLED)
    }
    #[doc = "certification read access is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CERTREADENABLED_A::DISABLED)
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
    #[doc = "Bit 0 - Control to disable certification path"]
    #[inline(always)]
    pub fn certdisable(&self) -> CERTDISABLE_R {
        CERTDISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn certreaden(&self) -> CERTREADEN_R {
        CERTREADEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn scseccfglock(&self) -> SCSECCFGLOCK_R {
        SCSECCFGLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Indicates that the Certification write path has been disabled"]
    #[inline(always)]
    pub fn certdisabled(&self) -> CERTDISABLED_R {
        CERTDISABLED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates whether the certification read access is enabled"]
    #[inline(always)]
    pub fn certreadenabled(&self) -> CERTREADENABLED_R {
        CERTREADENABLED_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control to disable certification path"]
    #[inline(always)]
    pub fn certdisable(&mut self) -> CERTDISABLE_W {
        CERTDISABLE_W { w: self }
    }
    #[doc = "Bit 1 - Control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn certreaden(&mut self) -> CERTREADEN_W {
        CERTREADEN_W { w: self }
    }
    #[doc = "Bit 2 - Control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn scseccfglock(&mut self) -> SCSECCFGLOCK_W {
        SCSECCFGLOCK_W { w: self }
    }
    #[doc = "Bit 16 - Indicates that the Certification write path has been disabled"]
    #[inline(always)]
    pub fn certdisabled(&mut self) -> CERTDISABLED_W {
        CERTDISABLED_W { w: self }
    }
    #[doc = "Bit 17 - Indicates whether the certification read access is enabled"]
    #[inline(always)]
    pub fn certreadenabled(&mut self) -> CERTREADENABLED_W {
        CERTREADENABLED_W { w: self }
    }
}
