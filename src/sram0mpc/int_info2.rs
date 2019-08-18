#[doc = "Reader of register INT_INFO2"]
pub type R = crate::R<u32, super::INT_INFO2>;
#[doc = "Reader of field `bit[15_0]`"]
pub type BIT15_0_R = crate::R<u16, u16>;
#[doc = "Reader of field `bit[16]`"]
pub type BIT16_R = crate::R<bool, bool>;
#[doc = "Reader of field `bit[17]`"]
pub type BIT17_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - hmaster"]
    #[inline(always)]
    pub fn bit15_0(&self) -> BIT15_0_R {
        BIT15_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - hnonsec"]
    #[inline(always)]
    pub fn bit16(&self) -> BIT16_R {
        BIT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - cfg_ns"]
    #[inline(always)]
    pub fn bit17(&self) -> BIT17_R {
        BIT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
