#[doc = "Reader of register PIDR3"]
pub type R = crate::R<u32, super::PIDR3>;
#[doc = "Reader of field `bit[3_0]`"]
pub type BIT3_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `bit[7_4]`"]
pub type BIT7_4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Customer modification number"]
    #[inline(always)]
    pub fn bit3_0(&self) -> BIT3_0_R {
        BIT3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ECO revision number"]
    #[inline(always)]
    pub fn bit7_4(&self) -> BIT7_4_R {
        BIT7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
