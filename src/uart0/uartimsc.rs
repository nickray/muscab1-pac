#[doc = "Reader of register UARTIMSC"]
pub type R = crate::R<u32, super::UARTIMSC>;
#[doc = "Writer for register UARTIMSC"]
pub type W = crate::W<u32, super::UARTIMSC>;
#[doc = "Register UARTIMSC `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTIMSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RIMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIMIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<RIMIM_A> for bool {
    #[inline(always)]
    fn from(variant: RIMIM_A) -> Self {
        match variant {
            RIMIM_A::CLEAR => false,
            RIMIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `RIMIM`"]
pub type RIMIM_R = crate::R<bool, RIMIM_A>;
impl RIMIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIMIM_A {
        match self.bits {
            false => RIMIM_A::CLEAR,
            true => RIMIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RIMIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RIMIM_A::SET
    }
}
#[doc = "Write proxy for field `RIMIM`"]
pub struct RIMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIMIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RIMIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RIMIM_A::SET)
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
#[doc = "Possible values of the field `CTSMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSMIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<CTSMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CTSMIM_A) -> Self {
        match variant {
            CTSMIM_A::CLEAR => false,
            CTSMIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `CTSMIM`"]
pub type CTSMIM_R = crate::R<bool, CTSMIM_A>;
impl CTSMIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSMIM_A {
        match self.bits {
            false => CTSMIM_A::CLEAR,
            true => CTSMIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTSMIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CTSMIM_A::SET
    }
}
#[doc = "Write proxy for field `CTSMIM`"]
pub struct CTSMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSMIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSMIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTSMIM_A::SET)
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
#[doc = "Possible values of the field `DCDMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDMIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<DCDMIM_A> for bool {
    #[inline(always)]
    fn from(variant: DCDMIM_A) -> Self {
        match variant {
            DCDMIM_A::CLEAR => false,
            DCDMIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `DCDMIM`"]
pub type DCDMIM_R = crate::R<bool, DCDMIM_A>;
impl DCDMIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDMIM_A {
        match self.bits {
            false => DCDMIM_A::CLEAR,
            true => DCDMIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DCDMIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DCDMIM_A::SET
    }
}
#[doc = "Write proxy for field `DCDMIM`"]
pub struct DCDMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDMIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DCDMIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DCDMIM_A::SET)
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
#[doc = "Possible values of the field `DSRMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRMIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<DSRMIM_A> for bool {
    #[inline(always)]
    fn from(variant: DSRMIM_A) -> Self {
        match variant {
            DSRMIM_A::CLEAR => false,
            DSRMIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `DSRMIM`"]
pub type DSRMIM_R = crate::R<bool, DSRMIM_A>;
impl DSRMIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRMIM_A {
        match self.bits {
            false => DSRMIM_A::CLEAR,
            true => DSRMIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DSRMIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DSRMIM_A::SET
    }
}
#[doc = "Write proxy for field `DSRMIM`"]
pub struct DSRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRMIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DSRMIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DSRMIM_A::SET)
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
#[doc = "Possible values of the field `RXIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<RXIM_A> for bool {
    #[inline(always)]
    fn from(variant: RXIM_A) -> Self {
        match variant {
            RXIM_A::CLEAR => false,
            RXIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `RXIM`"]
pub type RXIM_R = crate::R<bool, RXIM_A>;
impl RXIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIM_A {
        match self.bits {
            false => RXIM_A::CLEAR,
            true => RXIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RXIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RXIM_A::SET
    }
}
#[doc = "Write proxy for field `RXIM`"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXIM_A::SET)
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
#[doc = "Possible values of the field `TXIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<TXIM_A> for bool {
    #[inline(always)]
    fn from(variant: TXIM_A) -> Self {
        match variant {
            TXIM_A::CLEAR => false,
            TXIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `TXIM`"]
pub type TXIM_R = crate::R<bool, TXIM_A>;
impl TXIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIM_A {
        match self.bits {
            false => TXIM_A::CLEAR,
            true => TXIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TXIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TXIM_A::SET
    }
}
#[doc = "Write proxy for field `TXIM`"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXIM_A::SET)
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
#[doc = "Possible values of the field `RTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<RTIM_A> for bool {
    #[inline(always)]
    fn from(variant: RTIM_A) -> Self {
        match variant {
            RTIM_A::CLEAR => false,
            RTIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `RTIM`"]
pub type RTIM_R = crate::R<bool, RTIM_A>;
impl RTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTIM_A {
        match self.bits {
            false => RTIM_A::CLEAR,
            true => RTIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RTIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RTIM_A::SET
    }
}
#[doc = "Write proxy for field `RTIM`"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RTIM_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `FEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<FEIM_A> for bool {
    #[inline(always)]
    fn from(variant: FEIM_A) -> Self {
        match variant {
            FEIM_A::CLEAR => false,
            FEIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `FEIM`"]
pub type FEIM_R = crate::R<bool, FEIM_A>;
impl FEIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIM_A {
        match self.bits {
            false => FEIM_A::CLEAR,
            true => FEIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FEIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == FEIM_A::SET
    }
}
#[doc = "Write proxy for field `FEIM`"]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FEIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(FEIM_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `PEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<PEIM_A> for bool {
    #[inline(always)]
    fn from(variant: PEIM_A) -> Self {
        match variant {
            PEIM_A::CLEAR => false,
            PEIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `PEIM`"]
pub type PEIM_R = crate::R<bool, PEIM_A>;
impl PEIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIM_A {
        match self.bits {
            false => PEIM_A::CLEAR,
            true => PEIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PEIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PEIM_A::SET
    }
}
#[doc = "Write proxy for field `PEIM`"]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PEIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PEIM_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `BEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<BEIM_A> for bool {
    #[inline(always)]
    fn from(variant: BEIM_A) -> Self {
        match variant {
            BEIM_A::CLEAR => false,
            BEIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `BEIM`"]
pub type BEIM_R = crate::R<bool, BEIM_A>;
impl BEIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEIM_A {
        match self.bits {
            false => BEIM_A::CLEAR,
            true => BEIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BEIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BEIM_A::SET
    }
}
#[doc = "Write proxy for field `BEIM`"]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BEIM_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `OEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEIM_A {
    #[doc = "Clears the mask"]
    CLEAR,
    #[doc = "Sets the mask"]
    SET,
}
impl From<OEIM_A> for bool {
    #[inline(always)]
    fn from(variant: OEIM_A) -> Self {
        match variant {
            OEIM_A::CLEAR => false,
            OEIM_A::SET => true,
        }
    }
}
#[doc = "Reader of field `OEIM`"]
pub type OEIM_R = crate::R<bool, OEIM_A>;
impl OEIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEIM_A {
        match self.bits {
            false => OEIM_A::CLEAR,
            true => OEIM_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OEIM_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OEIM_A::SET
    }
}
#[doc = "Write proxy for field `OEIM`"]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the mask"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OEIM_A::CLEAR)
    }
    #[doc = "Sets the mask"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OEIM_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&mut self) -> RIMIM_W {
        RIMIM_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask."]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CTSMIM_W {
        CTSMIM_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DCDMIM_W {
        DCDMIM_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DSRMIM_W {
        DSRMIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
}
