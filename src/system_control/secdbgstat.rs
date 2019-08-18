#[doc = "Reader of register SECDBGSTAT"]
pub type R = crate::R<u32, super::SECDBGSTAT>;
#[doc = "Possible values of the field `DBGEN_I_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_I_STATUS_A {
    #[doc = "debug enable"]
    ENABLE,
    #[doc = "debug disable"]
    DISABLE,
}
impl From<DBGEN_I_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_I_STATUS_A) -> Self {
        match variant {
            DBGEN_I_STATUS_A::ENABLE => true,
            DBGEN_I_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `DBGEN_I_STATUS`"]
pub type DBGEN_I_STATUS_R = crate::R<bool, DBGEN_I_STATUS_A>;
impl DBGEN_I_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_I_STATUS_A {
        match self.bits {
            true => DBGEN_I_STATUS_A::ENABLE,
            false => DBGEN_I_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBGEN_I_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGEN_I_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `DBGEN_SEL_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_SEL_STATUS_A {
    #[doc = "debug enable selector"]
    ENABLE,
    #[doc = "debug disable selector"]
    DISABLE,
}
impl From<DBGEN_SEL_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_SEL_STATUS_A) -> Self {
        match variant {
            DBGEN_SEL_STATUS_A::ENABLE => true,
            DBGEN_SEL_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `DBGEN_SEL_STATUS`"]
pub type DBGEN_SEL_STATUS_R = crate::R<bool, DBGEN_SEL_STATUS_A>;
impl DBGEN_SEL_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_SEL_STATUS_A {
        match self.bits {
            true => DBGEN_SEL_STATUS_A::ENABLE,
            false => DBGEN_SEL_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBGEN_SEL_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGEN_SEL_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `NIDEN_I_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDEN_I_STATUS_A {
    #[doc = "non-invasive debug enable"]
    ENABLE,
    #[doc = "non-invasive debug disable"]
    DISABLE,
}
impl From<NIDEN_I_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_I_STATUS_A) -> Self {
        match variant {
            NIDEN_I_STATUS_A::ENABLE => true,
            NIDEN_I_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `NIDEN_I_STATUS`"]
pub type NIDEN_I_STATUS_R = crate::R<bool, NIDEN_I_STATUS_A>;
impl NIDEN_I_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN_I_STATUS_A {
        match self.bits {
            true => NIDEN_I_STATUS_A::ENABLE,
            false => NIDEN_I_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NIDEN_I_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NIDEN_I_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `NIDEN_SEL_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDEN_SEL_STATUS_A {
    #[doc = "non-invasive debug enable selector"]
    ENABLE,
    #[doc = "non-invasive debug disable selector"]
    DISABLE,
}
impl From<NIDEN_SEL_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_SEL_STATUS_A) -> Self {
        match variant {
            NIDEN_SEL_STATUS_A::ENABLE => true,
            NIDEN_SEL_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `NIDEN_SEL_STATUS`"]
pub type NIDEN_SEL_STATUS_R = crate::R<bool, NIDEN_SEL_STATUS_A>;
impl NIDEN_SEL_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN_SEL_STATUS_A {
        match self.bits {
            true => NIDEN_SEL_STATUS_A::ENABLE,
            false => NIDEN_SEL_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NIDEN_SEL_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NIDEN_SEL_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `SPIDEN_I_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDEN_I_STATUS_A {
    #[doc = "Secure privilege invasive debug enable"]
    ENABLE,
    #[doc = "Secure privilege invasive debug disable"]
    DISABLE,
}
impl From<SPIDEN_I_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPIDEN_I_STATUS_A) -> Self {
        match variant {
            SPIDEN_I_STATUS_A::ENABLE => true,
            SPIDEN_I_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `SPIDEN_I_STATUS`"]
pub type SPIDEN_I_STATUS_R = crate::R<bool, SPIDEN_I_STATUS_A>;
impl SPIDEN_I_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIDEN_I_STATUS_A {
        match self.bits {
            true => SPIDEN_I_STATUS_A::ENABLE,
            false => SPIDEN_I_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPIDEN_I_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPIDEN_I_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `SPIDEN_SEL_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDEN_SEL_STATUS_A {
    #[doc = "Secure privilege invasive debug enable selector"]
    ENABLE,
    #[doc = "Secure privilege invasive debug disable selector"]
    DISABLE,
}
impl From<SPIDEN_SEL_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPIDEN_SEL_STATUS_A) -> Self {
        match variant {
            SPIDEN_SEL_STATUS_A::ENABLE => true,
            SPIDEN_SEL_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `SPIDEN_SEL_STATUS`"]
pub type SPIDEN_SEL_STATUS_R = crate::R<bool, SPIDEN_SEL_STATUS_A>;
impl SPIDEN_SEL_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIDEN_SEL_STATUS_A {
        match self.bits {
            true => SPIDEN_SEL_STATUS_A::ENABLE,
            false => SPIDEN_SEL_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPIDEN_SEL_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPIDEN_SEL_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `SPNIDEN_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNIDEN_STATUS_A {
    #[doc = "Secure privilege non-invasive debug enable"]
    ENABLE,
    #[doc = "Secure privilege non-invasive debug disable"]
    DISABLE,
}
impl From<SPNIDEN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPNIDEN_STATUS_A) -> Self {
        match variant {
            SPNIDEN_STATUS_A::ENABLE => true,
            SPNIDEN_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `SPNIDEN_STATUS`"]
pub type SPNIDEN_STATUS_R = crate::R<bool, SPNIDEN_STATUS_A>;
impl SPNIDEN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNIDEN_STATUS_A {
        match self.bits {
            true => SPNIDEN_STATUS_A::ENABLE,
            false => SPNIDEN_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPNIDEN_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPNIDEN_STATUS_A::DISABLE
    }
}
#[doc = "Possible values of the field `SPNIDEN_SEL_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNIDEN_SEL_STATUS_A {
    #[doc = "Secure privilege non-invasive debug enable selector"]
    ENABLE,
    #[doc = "Secure privilege non-invasive debug disable selector"]
    DISABLE,
}
impl From<SPNIDEN_SEL_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPNIDEN_SEL_STATUS_A) -> Self {
        match variant {
            SPNIDEN_SEL_STATUS_A::ENABLE => true,
            SPNIDEN_SEL_STATUS_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `SPNIDEN_SEL_STATUS`"]
pub type SPNIDEN_SEL_STATUS_R = crate::R<bool, SPNIDEN_SEL_STATUS_A>;
impl SPNIDEN_SEL_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNIDEN_SEL_STATUS_A {
        match self.bits {
            true => SPNIDEN_SEL_STATUS_A::ENABLE,
            false => SPNIDEN_SEL_STATUS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPNIDEN_SEL_STATUS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPNIDEN_SEL_STATUS_A::DISABLE
    }
}
impl R {
    #[doc = "Bit 0 - Debug enable value"]
    #[inline(always)]
    pub fn dbgen_i_status(&self) -> DBGEN_I_STATUS_R {
        DBGEN_I_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debug enable selector value"]
    #[inline(always)]
    pub fn dbgen_sel_status(&self) -> DBGEN_SEL_STATUS_R {
        DBGEN_SEL_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-invasive debug enable value"]
    #[inline(always)]
    pub fn niden_i_status(&self) -> NIDEN_I_STATUS_R {
        NIDEN_I_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-invasive debug enable selector value"]
    #[inline(always)]
    pub fn niden_sel_status(&self) -> NIDEN_SEL_STATUS_R {
        NIDEN_SEL_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Secure privilege invasive debug enable value"]
    #[inline(always)]
    pub fn spiden_i_status(&self) -> SPIDEN_I_STATUS_R {
        SPIDEN_I_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Secure privilege invasive debug enable selector value"]
    #[inline(always)]
    pub fn spiden_sel_status(&self) -> SPIDEN_SEL_STATUS_R {
        SPIDEN_SEL_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Secure privilege non-invasive debug enable value"]
    #[inline(always)]
    pub fn spniden_status(&self) -> SPNIDEN_STATUS_R {
        SPNIDEN_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Secure privilege non-invasive debug enable selector value"]
    #[inline(always)]
    pub fn spniden_sel_status(&self) -> SPNIDEN_SEL_STATUS_R {
        SPNIDEN_SEL_STATUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
