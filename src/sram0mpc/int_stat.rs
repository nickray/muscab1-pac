#[doc = "Reader of register INT_STAT"]
pub type R = crate::R<u32, super::INT_STAT>;
#[doc = "Reader of field `bit[0]`"]
pub type BIT0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - mpc_irq triggered"]
    #[inline(always)]
    pub fn bit0(&self) -> BIT0_R {
        BIT0_R::new((self.bits & 0x01) != 0)
    }
}
