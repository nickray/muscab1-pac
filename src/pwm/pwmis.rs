#[doc = "Reader of register PWMIS"]
pub type R = crate::R<u32, super::PWMIS>;
#[doc = "Possible values of the field `Status`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "Interrupt is active"]
    ACTIVE,
    #[doc = "Interrupt is not active"]
    NOTACTIVE,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        match variant {
            STATUS_A::ACTIVE => true,
            STATUS_A::NOTACTIVE => false,
        }
    }
}
#[doc = "Reader of field `Status`"]
pub type STATUS_R = crate::R<bool, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            true => STATUS_A::ACTIVE,
            false => STATUS_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == STATUS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == STATUS_A::NOTACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Reading from this address returns the current state of the PWM Interrupt output, and then sets the bit low"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
}
