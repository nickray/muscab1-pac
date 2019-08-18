#[doc = "Reader of register AZ_CTRL"]
pub type R = crate::R<u32, super::AZ_CTRL>;
#[doc = "Writer for register AZ_CTRL"]
pub type W = crate::W<u32, super::AZ_CTRL>;
#[doc = "Register AZ_CTRL `reset()`'s with value 0x0600"]
impl crate::ResetValue for super::AZ_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
#[doc = "Reader of field `AZ_BOOT_REMAP`"]
pub type AZ_BOOT_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AZ_BOOT_REMAP`"]
pub struct AZ_BOOT_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_BOOT_REMAP_W<'a> {
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
#[doc = "Reader of field `CPUWAIT`"]
pub type CPUWAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPUWAIT`"]
pub struct CPUWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUWAIT_W<'a> {
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
#[doc = "Reader of field `REMOVE_CHACHA_ENGINE`"]
pub type REMOVE_CHACHA_ENGINE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REMOVE_CHACHA_ENGINE`"]
pub struct REMOVE_CHACHA_ENGINE_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_CHACHA_ENGINE_W<'a> {
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
#[doc = "Reader of field `REMOVE_GHASH_ENGINE`"]
pub type REMOVE_GHASH_ENGINE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REMOVE_GHASH_ENGINE`"]
pub struct REMOVE_GHASH_ENGINE_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_GHASH_ENGINE_W<'a> {
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
#[doc = "Reader of field `CHSEC_ISO_ENB`"]
pub type CHSEC_ISO_ENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSEC_ISO_ENB`"]
pub struct CHSEC_ISO_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEC_ISO_ENB_W<'a> {
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
#[doc = "Reader of field `CHSEC_MISC_7`"]
pub type CHSEC_MISC_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSEC_MISC_7`"]
pub struct CHSEC_MISC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEC_MISC_7_W<'a> {
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
#[doc = "Reader of field `DBGRESETn`"]
pub type DBGRESETN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGRESETn`"]
pub struct DBGRESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRESETN_W<'a> {
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
#[doc = "Reader of field `HRESETn`"]
pub type HRESETN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRESETn`"]
pub struct HRESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> HRESETN_W<'a> {
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
#[doc = "Reader of field `SCC_nPORESETAON_nPORESET_SEL`"]
pub type SCC_NPORESETAON_NPORESET_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCC_nPORESETAON_nPORESET_SEL`"]
pub struct SCC_NPORESETAON_NPORESET_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCC_NPORESETAON_NPORESET_SEL_W<'a> {
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
#[doc = "Reader of field `SCC_PSI_FEATURE_EN`"]
pub type SCC_PSI_FEATURE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCC_PSI_FEATURE_EN`"]
pub struct SCC_PSI_FEATURE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCC_PSI_FEATURE_EN_W<'a> {
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
#[doc = "Reader of field `SCC_PSI_FEATURE_EN_SEL`"]
pub type SCC_PSI_FEATURE_EN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCC_PSI_FEATURE_EN_SEL`"]
pub struct SCC_PSI_FEATURE_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCC_PSI_FEATURE_EN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Alcatraz remap at boot"]
    #[inline(always)]
    pub fn az_boot_remap(&self) -> AZ_BOOT_REMAP_R {
        AZ_BOOT_REMAP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alcatraz CPU wait at boot:"]
    #[inline(always)]
    pub fn cpuwait(&self) -> CPUWAIT_R {
        CPUWAIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alcatraz CryptoCell remove CHACHA engine"]
    #[inline(always)]
    pub fn remove_chacha_engine(&self) -> REMOVE_CHACHA_ENGINE_R {
        REMOVE_CHACHA_ENGINE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Alcatraz CryptoCell remove Ghash engine"]
    #[inline(always)]
    pub fn remove_ghash_engine(&self) -> REMOVE_GHASH_ENGINE_R {
        REMOVE_GHASH_ENGINE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Alcatraz CryptoCell Secure Frame Isolation enable"]
    #[inline(always)]
    pub fn chsec_iso_enb(&self) -> CHSEC_ISO_ENB_R {
        CHSEC_ISO_ENB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Alcatraz CryptoCell secure Secure Frame control"]
    #[inline(always)]
    pub fn chsec_misc_7(&self) -> CHSEC_MISC_7_R {
        CHSEC_MISC_7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Alcatraz reset DBGRESETn"]
    #[inline(always)]
    pub fn dbgresetn(&self) -> DBGRESETN_R {
        DBGRESETN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alcatraz reset HRESETn"]
    #[inline(always)]
    pub fn hresetn(&self) -> HRESETN_R {
        HRESETN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alcatraz reset control"]
    #[inline(always)]
    pub fn scc_n_poresetaon_n_poreset_sel(&self) -> SCC_NPORESETAON_NPORESET_SEL_R {
        SCC_NPORESETAON_NPORESET_SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Value of SCC_PSI_FEATURE_EN from SCC"]
    #[inline(always)]
    pub fn scc_psi_feature_en(&self) -> SCC_PSI_FEATURE_EN_R {
        SCC_PSI_FEATURE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select PSI_FEATURE_EN source"]
    #[inline(always)]
    pub fn scc_psi_feature_en_sel(&self) -> SCC_PSI_FEATURE_EN_SEL_R {
        SCC_PSI_FEATURE_EN_SEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alcatraz remap at boot"]
    #[inline(always)]
    pub fn az_boot_remap(&mut self) -> AZ_BOOT_REMAP_W {
        AZ_BOOT_REMAP_W { w: self }
    }
    #[doc = "Bit 1 - Alcatraz CPU wait at boot:"]
    #[inline(always)]
    pub fn cpuwait(&mut self) -> CPUWAIT_W {
        CPUWAIT_W { w: self }
    }
    #[doc = "Bit 2 - Alcatraz CryptoCell remove CHACHA engine"]
    #[inline(always)]
    pub fn remove_chacha_engine(&mut self) -> REMOVE_CHACHA_ENGINE_W {
        REMOVE_CHACHA_ENGINE_W { w: self }
    }
    #[doc = "Bit 3 - Alcatraz CryptoCell remove Ghash engine"]
    #[inline(always)]
    pub fn remove_ghash_engine(&mut self) -> REMOVE_GHASH_ENGINE_W {
        REMOVE_GHASH_ENGINE_W { w: self }
    }
    #[doc = "Bit 4 - Alcatraz CryptoCell Secure Frame Isolation enable"]
    #[inline(always)]
    pub fn chsec_iso_enb(&mut self) -> CHSEC_ISO_ENB_W {
        CHSEC_ISO_ENB_W { w: self }
    }
    #[doc = "Bit 5 - Alcatraz CryptoCell secure Secure Frame control"]
    #[inline(always)]
    pub fn chsec_misc_7(&mut self) -> CHSEC_MISC_7_W {
        CHSEC_MISC_7_W { w: self }
    }
    #[doc = "Bit 7 - Alcatraz reset DBGRESETn"]
    #[inline(always)]
    pub fn dbgresetn(&mut self) -> DBGRESETN_W {
        DBGRESETN_W { w: self }
    }
    #[doc = "Bit 8 - Alcatraz reset HRESETn"]
    #[inline(always)]
    pub fn hresetn(&mut self) -> HRESETN_W {
        HRESETN_W { w: self }
    }
    #[doc = "Bit 9 - Alcatraz reset control"]
    #[inline(always)]
    pub fn scc_n_poresetaon_n_poreset_sel(&mut self) -> SCC_NPORESETAON_NPORESET_SEL_W {
        SCC_NPORESETAON_NPORESET_SEL_W { w: self }
    }
    #[doc = "Bit 10 - Value of SCC_PSI_FEATURE_EN from SCC"]
    #[inline(always)]
    pub fn scc_psi_feature_en(&mut self) -> SCC_PSI_FEATURE_EN_W {
        SCC_PSI_FEATURE_EN_W { w: self }
    }
    #[doc = "Bit 11 - Select PSI_FEATURE_EN source"]
    #[inline(always)]
    pub fn scc_psi_feature_en_sel(&mut self) -> SCC_PSI_FEATURE_EN_SEL_W {
        SCC_PSI_FEATURE_EN_SEL_W { w: self }
    }
}
