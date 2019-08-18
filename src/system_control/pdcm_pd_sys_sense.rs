#[doc = "Reader of register PDCM_PD_SYS_SENSE"]
pub type R = crate::R<u32, super::PDCM_PD_SYS_SENSE>;
#[doc = "Writer for register PDCM_PD_SYS_SENSE"]
pub type W = crate::W<u32, super::PDCM_PD_SYS_SENSE>;
#[doc = "Register PDCM_PD_SYS_SENSE `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::PDCM_PD_SYS_SENSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
#[doc = "Possible values of the field `S_PD_SYS_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_SYS_ON_A {
    #[doc = "Keep PD_SYS awake after powered ON"]
    ENABLE,
}
impl From<S_PD_SYS_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_SYS_ON_A) -> Self {
        match variant {
            S_PD_SYS_ON_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `S_PD_SYS_ON`"]
pub type S_PD_SYS_ON_R = crate::R<bool, S_PD_SYS_ON_A>;
impl S_PD_SYS_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_SYS_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_SYS_ON_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == S_PD_SYS_ON_A::ENABLE
    }
}
#[doc = "Write proxy for field `S_PD_SYS_ON`"]
pub struct S_PD_SYS_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> S_PD_SYS_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_PD_SYS_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Keep PD_SYS awake after powered ON"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(S_PD_SYS_ON_A::ENABLE)
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
#[doc = "Possible values of the field `S_PD_CPU0CORE_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_CPU0CORE_ON_A {
    #[doc = "PD_SYS always tries to stay ON if PD_CPU0CORE is ON"]
    HIGH,
}
impl From<S_PD_CPU0CORE_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_CPU0CORE_ON_A) -> Self {
        match variant {
            S_PD_CPU0CORE_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_CPU0CORE_ON`"]
pub type S_PD_CPU0CORE_ON_R = crate::R<bool, S_PD_CPU0CORE_ON_A>;
impl S_PD_CPU0CORE_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_CPU0CORE_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_CPU0CORE_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_CPU0CORE_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_CPU1CORE_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_CPU1CORE_ON_A {
    #[doc = "PD_SYS always tries to stay ON if PD_CPU1CORE is ON"]
    HIGH,
}
impl From<S_PD_CPU1CORE_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_CPU1CORE_ON_A) -> Self {
        match variant {
            S_PD_CPU1CORE_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_CPU1CORE_ON`"]
pub type S_PD_CPU1CORE_ON_R = crate::R<bool, S_PD_CPU1CORE_ON_A>;
impl S_PD_CPU1CORE_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_CPU1CORE_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_CPU1CORE_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_CPU1CORE_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_SRAM0_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_SRAM0_ON_A {
    #[doc = "PD_SYS always tries to keep ON if SRAM0 power domain is ON"]
    HIGH,
}
impl From<S_PD_SRAM0_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_SRAM0_ON_A) -> Self {
        match variant {
            S_PD_SRAM0_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_SRAM0_ON`"]
pub type S_PD_SRAM0_ON_R = crate::R<bool, S_PD_SRAM0_ON_A>;
impl S_PD_SRAM0_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_SRAM0_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_SRAM0_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_SRAM0_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_SRAM1_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_SRAM1_ON_A {
    #[doc = "PD_SYS always tries to keep ON if SRAM1 power domain is ON"]
    HIGH,
}
impl From<S_PD_SRAM1_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_SRAM1_ON_A) -> Self {
        match variant {
            S_PD_SRAM1_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_SRAM1_ON`"]
pub type S_PD_SRAM1_ON_R = crate::R<bool, S_PD_SRAM1_ON_A>;
impl S_PD_SRAM1_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_SRAM1_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_SRAM1_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_SRAM1_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_SRAM2_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_SRAM2_ON_A {
    #[doc = "PD_SYS always tries to keep ON if SRAM2 power domain is ON"]
    HIGH,
}
impl From<S_PD_SRAM2_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_SRAM2_ON_A) -> Self {
        match variant {
            S_PD_SRAM2_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_SRAM2_ON`"]
pub type S_PD_SRAM2_ON_R = crate::R<bool, S_PD_SRAM2_ON_A>;
impl S_PD_SRAM2_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_SRAM2_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_SRAM2_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_SRAM2_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_SRAM3_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_SRAM3_ON_A {
    #[doc = "PD_SYS always tries to keep ON if SRAM3 power domain is ON"]
    HIGH,
}
impl From<S_PD_SRAM3_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_SRAM3_ON_A) -> Self {
        match variant {
            S_PD_SRAM3_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_SRAM3_ON`"]
pub type S_PD_SRAM3_ON_R = crate::R<bool, S_PD_SRAM3_ON_A>;
impl S_PD_SRAM3_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_SRAM3_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_SRAM3_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_SRAM3_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_CRYPTO_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_CRYPTO_ON_A {
    #[doc = "PD_SYS always tries to keep ON if S_PD_CRYPTO_ON is ON"]
    HIGH,
}
impl From<S_PD_CRYPTO_ON_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_CRYPTO_ON_A) -> Self {
        match variant {
            S_PD_CRYPTO_ON_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `S_PD_CRYPTO_ON`"]
pub type S_PD_CRYPTO_ON_R = crate::R<bool, S_PD_CRYPTO_ON_A>;
impl S_PD_CRYPTO_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, S_PD_CRYPTO_ON_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(S_PD_CRYPTO_ON_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == S_PD_CRYPTO_ON_A::HIGH
    }
}
#[doc = "Possible values of the field `S_PD_EXP0_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_EXP0_IN_A {
    #[doc = "Enable PDEXPIN\\[0\\] signal Sensitivity."]
    ENABLE,
    #[doc = "Disable PDEXPIN\\[0\\] signal Sensitivity."]
    DISABLED,
}
impl From<S_PD_EXP0_IN_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_EXP0_IN_A) -> Self {
        match variant {
            S_PD_EXP0_IN_A::ENABLE => true,
            S_PD_EXP0_IN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `S_PD_EXP0_IN`"]
pub type S_PD_EXP0_IN_R = crate::R<bool, S_PD_EXP0_IN_A>;
impl S_PD_EXP0_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_PD_EXP0_IN_A {
        match self.bits {
            true => S_PD_EXP0_IN_A::ENABLE,
            false => S_PD_EXP0_IN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == S_PD_EXP0_IN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == S_PD_EXP0_IN_A::DISABLED
    }
}
#[doc = "Write proxy for field `S_PD_EXP0_IN`"]
pub struct S_PD_EXP0_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_PD_EXP0_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_PD_EXP0_IN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PDEXPIN\\[0\\] signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(S_PD_EXP0_IN_A::ENABLE)
    }
    #[doc = "Disable PDEXPIN\\[0\\] signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(S_PD_EXP0_IN_A::DISABLED)
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
#[doc = "Possible values of the field `S_PD_EXP1_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_EXP1_IN_A {
    #[doc = "Enable PDEXPIN\\[1\\] signal Sensitivity."]
    ENABLE,
    #[doc = "Disable PDEXPIN\\[1\\] signal Sensitivity."]
    DISABLED,
}
impl From<S_PD_EXP1_IN_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_EXP1_IN_A) -> Self {
        match variant {
            S_PD_EXP1_IN_A::ENABLE => true,
            S_PD_EXP1_IN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `S_PD_EXP1_IN`"]
pub type S_PD_EXP1_IN_R = crate::R<bool, S_PD_EXP1_IN_A>;
impl S_PD_EXP1_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_PD_EXP1_IN_A {
        match self.bits {
            true => S_PD_EXP1_IN_A::ENABLE,
            false => S_PD_EXP1_IN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == S_PD_EXP1_IN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == S_PD_EXP1_IN_A::DISABLED
    }
}
#[doc = "Write proxy for field `S_PD_EXP1_IN`"]
pub struct S_PD_EXP1_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_PD_EXP1_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_PD_EXP1_IN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PDEXPIN\\[1\\] signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(S_PD_EXP1_IN_A::ENABLE)
    }
    #[doc = "Disable PDEXPIN\\[1\\] signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(S_PD_EXP1_IN_A::DISABLED)
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
#[doc = "Possible values of the field `S_PD_EXP2_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_EXP2_IN_A {
    #[doc = "Enable PDEXPIN\\[2\\] signal Sensitivity."]
    ENABLE,
    #[doc = "Disable PDEXPIN\\[2\\] signal Sensitivity."]
    DISABLED,
}
impl From<S_PD_EXP2_IN_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_EXP2_IN_A) -> Self {
        match variant {
            S_PD_EXP2_IN_A::ENABLE => true,
            S_PD_EXP2_IN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `S_PD_EXP2_IN`"]
pub type S_PD_EXP2_IN_R = crate::R<bool, S_PD_EXP2_IN_A>;
impl S_PD_EXP2_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_PD_EXP2_IN_A {
        match self.bits {
            true => S_PD_EXP2_IN_A::ENABLE,
            false => S_PD_EXP2_IN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == S_PD_EXP2_IN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == S_PD_EXP2_IN_A::DISABLED
    }
}
#[doc = "Write proxy for field `S_PD_EXP2_IN`"]
pub struct S_PD_EXP2_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_PD_EXP2_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_PD_EXP2_IN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PDEXPIN\\[2\\] signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(S_PD_EXP2_IN_A::ENABLE)
    }
    #[doc = "Disable PDEXPIN\\[2\\] signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(S_PD_EXP2_IN_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `S_PD_EXP3_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_PD_EXP3_IN_A {
    #[doc = "Enable PDEXPIN\\[3\\] signal Sensitivity."]
    ENABLE,
    #[doc = "Disable PDEXPIN\\[3\\] signal Sensitivity."]
    DISABLED,
}
impl From<S_PD_EXP3_IN_A> for bool {
    #[inline(always)]
    fn from(variant: S_PD_EXP3_IN_A) -> Self {
        match variant {
            S_PD_EXP3_IN_A::ENABLE => true,
            S_PD_EXP3_IN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `S_PD_EXP3_IN`"]
pub type S_PD_EXP3_IN_R = crate::R<bool, S_PD_EXP3_IN_A>;
impl S_PD_EXP3_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_PD_EXP3_IN_A {
        match self.bits {
            true => S_PD_EXP3_IN_A::ENABLE,
            false => S_PD_EXP3_IN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == S_PD_EXP3_IN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == S_PD_EXP3_IN_A::DISABLED
    }
}
#[doc = "Write proxy for field `S_PD_EXP3_IN`"]
pub struct S_PD_EXP3_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_PD_EXP3_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_PD_EXP3_IN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PDEXPIN\\[3\\] signal Sensitivity."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(S_PD_EXP3_IN_A::ENABLE)
    }
    #[doc = "Disable PDEXPIN\\[3\\] signal Sensitivity."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(S_PD_EXP3_IN_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable PD_SYS ON Sensitivity"]
    #[inline(always)]
    pub fn s_pd_sys_on(&self) -> S_PD_SYS_ON_R {
        S_PD_SYS_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_cpu0core_on(&self) -> S_PD_CPU0CORE_ON_R {
        S_PD_CPU0CORE_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_cpu1core_on(&self) -> S_PD_CPU1CORE_ON_R {
        S_PD_CPU1CORE_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram0_on(&self) -> S_PD_SRAM0_ON_R {
        S_PD_SRAM0_ON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram1_on(&self) -> S_PD_SRAM1_ON_R {
        S_PD_SRAM1_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram2_on(&self) -> S_PD_SRAM2_ON_R {
        S_PD_SRAM2_ON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_sram3_on(&self) -> S_PD_SRAM3_ON_R {
        S_PD_SRAM3_ON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tied to HIGH"]
    #[inline(always)]
    pub fn s_pd_crypto_on(&self) -> S_PD_CRYPTO_ON_R {
        S_PD_CRYPTO_ON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable PDEXPIN\\[0\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp0_in(&self) -> S_PD_EXP0_IN_R {
        S_PD_EXP0_IN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable PDEXPIN\\[1\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp1_in(&self) -> S_PD_EXP1_IN_R {
        S_PD_EXP1_IN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable PDEXPIN\\[2\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp2_in(&self) -> S_PD_EXP2_IN_R {
        S_PD_EXP2_IN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable PDEXPIN\\[3\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp3_in(&self) -> S_PD_EXP3_IN_R {
        S_PD_EXP3_IN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PD_SYS ON Sensitivity"]
    #[inline(always)]
    pub fn s_pd_sys_on(&mut self) -> S_PD_SYS_ON_W {
        S_PD_SYS_ON_W { w: self }
    }
    #[doc = "Bit 16 - Enable PDEXPIN\\[0\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp0_in(&mut self) -> S_PD_EXP0_IN_W {
        S_PD_EXP0_IN_W { w: self }
    }
    #[doc = "Bit 17 - Enable PDEXPIN\\[1\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp1_in(&mut self) -> S_PD_EXP1_IN_W {
        S_PD_EXP1_IN_W { w: self }
    }
    #[doc = "Bit 18 - Enable PDEXPIN\\[2\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp2_in(&mut self) -> S_PD_EXP2_IN_W {
        S_PD_EXP2_IN_W { w: self }
    }
    #[doc = "Bit 19 - Enable PDEXPIN\\[3\\] signal Sensitivity"]
    #[inline(always)]
    pub fn s_pd_exp3_in(&mut self) -> S_PD_EXP3_IN_W {
        S_PD_EXP3_IN_W { w: self }
    }
}
