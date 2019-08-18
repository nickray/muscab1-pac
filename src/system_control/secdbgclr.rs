#[doc = "Writer for register SECDBGCLR"]
pub type W = crate::W<u32, super::SECDBGCLR>;
#[doc = "Register SECDBGCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECDBGCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DBGEN_I_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_I_CLR_AW {
    #[doc = "debug enable clear control"]
    ENABLE,
    #[doc = "debug disable clear control"]
    DISABLE,
}
impl From<DBGEN_I_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_I_CLR_AW) -> Self {
        match variant {
            DBGEN_I_CLR_AW::ENABLE => true,
            DBGEN_I_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `DBGEN_I_CLR`"]
pub struct DBGEN_I_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_I_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_I_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBGEN_I_CLR_AW::ENABLE)
    }
    #[doc = "debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGEN_I_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `DBGEN_SEL_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_SEL_CLR_AW {
    #[doc = "debug enable selector clear control"]
    ENABLE,
    #[doc = "debug disable selector clear control"]
    DISABLE,
}
impl From<DBGEN_SEL_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_SEL_CLR_AW) -> Self {
        match variant {
            DBGEN_SEL_CLR_AW::ENABLE => true,
            DBGEN_SEL_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `DBGEN_SEL_CLR`"]
pub struct DBGEN_SEL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_SEL_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_SEL_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBGEN_SEL_CLR_AW::ENABLE)
    }
    #[doc = "debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGEN_SEL_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `NIDEN_I_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDEN_I_CLR_AW {
    #[doc = "non-invasive debug enable clear control"]
    ENABLE,
    #[doc = "non-invasive debug disable clear control"]
    DISABLE,
}
impl From<NIDEN_I_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_I_CLR_AW) -> Self {
        match variant {
            NIDEN_I_CLR_AW::ENABLE => true,
            NIDEN_I_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `NIDEN_I_CLR`"]
pub struct NIDEN_I_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDEN_I_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIDEN_I_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NIDEN_I_CLR_AW::ENABLE)
    }
    #[doc = "non-invasive debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NIDEN_I_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `NIDEN_SEL_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDEN_SEL_CLR_AW {
    #[doc = "non-invasive debug enable selector clear control"]
    ENABLE,
    #[doc = "non-invasive debug disable selector clear control"]
    DISABLE,
}
impl From<NIDEN_SEL_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_SEL_CLR_AW) -> Self {
        match variant {
            NIDEN_SEL_CLR_AW::ENABLE => true,
            NIDEN_SEL_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `NIDEN_SEL_CLR`"]
pub struct NIDEN_SEL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDEN_SEL_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIDEN_SEL_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NIDEN_SEL_CLR_AW::ENABLE)
    }
    #[doc = "non-invasive debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NIDEN_SEL_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `SPIDEN_I_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDEN_I_CLR_AW {
    #[doc = "Secure privilege invasive debug enable clear control"]
    ENABLE,
    #[doc = "Secure privilege invasive debug disable clear control"]
    DISABLE,
}
impl From<SPIDEN_I_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIDEN_I_CLR_AW) -> Self {
        match variant {
            SPIDEN_I_CLR_AW::ENABLE => true,
            SPIDEN_I_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `SPIDEN_I_CLR`"]
pub struct SPIDEN_I_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIDEN_I_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIDEN_I_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure privilege invasive debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPIDEN_I_CLR_AW::ENABLE)
    }
    #[doc = "Secure privilege invasive debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPIDEN_I_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `SPIDEN_SEL_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDEN_SEL_CLR_AW {
    #[doc = "Secure privilege invasive debug enable selector clear control"]
    ENABLE,
    #[doc = "Secure privilege invasive debug disable selector clear control"]
    DISABLE,
}
impl From<SPIDEN_SEL_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIDEN_SEL_CLR_AW) -> Self {
        match variant {
            SPIDEN_SEL_CLR_AW::ENABLE => true,
            SPIDEN_SEL_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `SPIDEN_SEL_CLR`"]
pub struct SPIDEN_SEL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIDEN_SEL_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIDEN_SEL_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure privilege invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPIDEN_SEL_CLR_AW::ENABLE)
    }
    #[doc = "Secure privilege invasive debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPIDEN_SEL_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `SPNIDEN_I_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNIDEN_I_CLR_AW {
    #[doc = "Secure privilege non-invasive debug enable clear control"]
    ENABLE,
    #[doc = "Secure privilege non-invasive debug disable clear control"]
    DISABLE,
}
impl From<SPNIDEN_I_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SPNIDEN_I_CLR_AW) -> Self {
        match variant {
            SPNIDEN_I_CLR_AW::ENABLE => true,
            SPNIDEN_I_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `SPNIDEN_I_CLR`"]
pub struct SPNIDEN_I_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPNIDEN_I_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPNIDEN_I_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure privilege non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPNIDEN_I_CLR_AW::ENABLE)
    }
    #[doc = "Secure privilege non-invasive debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPNIDEN_I_CLR_AW::DISABLE)
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
#[doc = "Possible values of the field `SPNIDEN_SEL_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNIDEN_SEL_CLR_AW {
    #[doc = "Secure privilege non-invasive debug enable selector clear control"]
    ENABLE,
    #[doc = "Secure privilege non-invasive debug disable selector clear control"]
    DISABLE,
}
impl From<SPNIDEN_SEL_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SPNIDEN_SEL_CLR_AW) -> Self {
        match variant {
            SPNIDEN_SEL_CLR_AW::ENABLE => true,
            SPNIDEN_SEL_CLR_AW::DISABLE => false,
        }
    }
}
#[doc = "Write proxy for field `SPNIDEN_SEL_CLR`"]
pub struct SPNIDEN_SEL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPNIDEN_SEL_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPNIDEN_SEL_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure privilege non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPNIDEN_SEL_CLR_AW::ENABLE)
    }
    #[doc = "Secure privilege non-invasive debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPNIDEN_SEL_CLR_AW::DISABLE)
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
impl W {
    #[doc = "Bit 0 - Debug enable clear control"]
    #[inline(always)]
    pub fn dbgen_i_clr(&mut self) -> DBGEN_I_CLR_W {
        DBGEN_I_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Debug enable selector clear control"]
    #[inline(always)]
    pub fn dbgen_sel_clr(&mut self) -> DBGEN_SEL_CLR_W {
        DBGEN_SEL_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn niden_i_clr(&mut self) -> NIDEN_I_CLR_W {
        NIDEN_I_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn niden_sel_clr(&mut self) -> NIDEN_SEL_CLR_W {
        NIDEN_SEL_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Secure privilege invasive debug enable clear control"]
    #[inline(always)]
    pub fn spiden_i_clr(&mut self) -> SPIDEN_I_CLR_W {
        SPIDEN_I_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Secure privilege invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn spiden_sel_clr(&mut self) -> SPIDEN_SEL_CLR_W {
        SPIDEN_SEL_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Secure privilege non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn spniden_i_clr(&mut self) -> SPNIDEN_I_CLR_W {
        SPNIDEN_I_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Secure privilege non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn spniden_sel_clr(&mut self) -> SPNIDEN_SEL_CLR_W {
        SPNIDEN_SEL_CLR_W { w: self }
    }
}
