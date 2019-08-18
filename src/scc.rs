#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub clk_ctrl_sel: CLK_CTRL_SEL,
    #[doc = "0x04 - "]
    pub clk_pll_prediv_ctrl: CLK_PLL_PREDIV_CTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - "]
    pub clk_postdiv_ctrl_flash: CLK_POSTDIV_CTRL_FLASH,
    #[doc = "0x10 - "]
    pub clk_postdiv_ctrl_qspi: CLK_POSTDIV_CTRL_QSPI,
    #[doc = "0x14 - "]
    pub clk_postdiv_ctrl_rtc: CLK_POSTDIV_CTRL_RTC,
    #[doc = "0x18 - "]
    pub clk_postdiv_ctrl_sd: CLK_POSTDIV_CTRL_SD,
    #[doc = "0x1c - "]
    pub clk_postdiv_ctrl_test: CLK_POSTDIV_CTRL_TEST,
    #[doc = "0x20 - "]
    pub ctrl_bypass_div: CTRL_BYPASS_DIV,
    #[doc = "0x24 - "]
    pub pll_ctrl_pll0_clk: PLL_CTRL_PLL0_CLK,
    #[doc = "0x28 - "]
    pub pll_postdiv_ctrl_pll0_clk: PLL_POSTDIV_CTRL_PLL0_CLK,
    #[doc = "0x2c - "]
    pub pll_ctrl_mult_pll0_clk: PLL_CTRL_MULT_PLL0_CLK,
    #[doc = "0x30 - "]
    pub clk_ctrl_enable: CLK_CTRL_ENABLE,
    #[doc = "0x34 - "]
    pub clk_status: CLK_STATUS,
    _reserved13: [u8; 8usize],
    #[doc = "0x40 - "]
    pub reset_ctrl: RESET_CTRL,
    _reserved14: [u8; 4usize],
    #[doc = "0x48 - "]
    pub dbg_ctrl: DBG_CTRL,
    #[doc = "0x4c - "]
    pub sram_ctrl: SRAM_CTRL,
    #[doc = "0x50 - "]
    pub intr_ctrl: INTR_CTRL,
    #[doc = "0x54 - "]
    pub clk_test_ctrl: CLK_TEST_CTRL,
    #[doc = "0x58 - "]
    pub cpu0_vtor: CPU0_VTOR,
    _reserved19: [u8; 4usize],
    #[doc = "0x60 - "]
    pub cpu1_vtor: CPU1_VTOR,
    #[doc = "0x64 - "]
    pub az_cpu_vtor: AZ_CPU_VTOR,
    #[doc = "0x68 - "]
    pub iomux_main_insel_0: IOMUX_MAIN_INSEL_0,
    #[doc = "0x6c - "]
    pub iomux_main_insel_1: IOMUX_MAIN_INSEL_1,
    #[doc = "0x70 - "]
    pub iomux_main_outsel_0: IOMUX_MAIN_OUTSEL_0,
    #[doc = "0x74 - "]
    pub iomux_main_outsel_1: IOMUX_MAIN_OUTSEL_1,
    #[doc = "0x78 - "]
    pub iomux_main_oensel_0: IOMUX_MAIN_OENSEL_0,
    #[doc = "0x7c - "]
    pub iomux_main_oensel_1: IOMUX_MAIN_OENSEL_1,
    #[doc = "0x80 - "]
    pub iomux_main_default_in_0: IOMUX_MAIN_DEFAULT_IN_0,
    #[doc = "0x84 - "]
    pub iomux_main_default_in_1: IOMUX_MAIN_DEFAULT_IN_1,
    #[doc = "0x88 - "]
    pub iomux_altf1_insel_0: IOMUX_ALTF1_INSEL_0,
    #[doc = "0x8c - "]
    pub iomux_altf1_insel_1: IOMUX_ALTF1_INSEL_1,
    #[doc = "0x90 - "]
    pub iomux_altf1_outsel_0: IOMUX_ALTF1_OUTSEL_0,
    #[doc = "0x94 - "]
    pub iomux_altf1_outsel_1: IOMUX_ALTF1_OUTSEL_1,
    #[doc = "0x98 - "]
    pub iomux_altf1_oensel_0: IOMUX_ALTF1_OENSEL_0,
    #[doc = "0x9c - "]
    pub iomux_altf1_oensel_1: IOMUX_ALTF1_OENSEL_1,
    #[doc = "0xa0 - "]
    pub iomux_altf1_default_in_0: IOMUX_ALTF1_DEFAULT_IN_0,
    #[doc = "0xa4 - "]
    pub iomux_altf1_default_in_1: IOMUX_ALTF1_DEFAULT_IN_1,
    #[doc = "0xa8 - "]
    pub iomux_altf2_insel_0: IOMUX_ALTF2_INSEL_0,
    #[doc = "0xac - "]
    pub iomux_altf2_insel_1: IOMUX_ALTF2_INSEL_1,
    #[doc = "0xb0 - "]
    pub iomux_altf2_outsel_0: IOMUX_ALTF2_OUTSEL_0,
    #[doc = "0xb4 - "]
    pub iomux_altf2_outsel_1: IOMUX_ALTF2_OUTSEL_1,
    #[doc = "0xb8 - "]
    pub iomux_altf2_oensel_0: IOMUX_ALTF2_OENSEL_0,
    #[doc = "0xbc - "]
    pub iomux_altf2_oensel_1: IOMUX_ALTF2_OENSEL_1,
    #[doc = "0xc0 - "]
    pub iomux_altf2_default_in_0: IOMUX_ALTF2_DEFAULT_IN_0,
    #[doc = "0xc4 - "]
    pub iomux_altf2_default_in_1: IOMUX_ALTF2_DEFAULT_IN_1,
    _reserved45: [u8; 32usize],
    #[doc = "0xe8 - "]
    pub iopad_dso_0: IOPAD_DSO_0,
    #[doc = "0xec - "]
    pub iopad_dso_1: IOPAD_DSO_1,
    #[doc = "0xf0 - "]
    pub iopad_ds1_0: IOPAD_DS1_0,
    #[doc = "0xf4 - "]
    pub iopad_ds1_1: IOPAD_DS1_1,
    #[doc = "0xf8 - "]
    pub iopad_pe_0: IOPAD_PE_0,
    #[doc = "0xfc - "]
    pub iopad_pe_1: IOPAD_PE_1,
    #[doc = "0x100 - "]
    pub iopad_ps_0: IOPAD_PS_0,
    #[doc = "0x104 - "]
    pub iopad_ps_1: IOPAD_PS_1,
    #[doc = "0x108 - "]
    pub iopad_sr_0: IOPAD_SR_0,
    #[doc = "0x10c - "]
    pub iopad_sr_1: IOPAD_SR_1,
    #[doc = "0x110 - "]
    pub iopad_is_0: IOPAD_IS_0,
    #[doc = "0x114 - "]
    pub iopad_is_1: IOPAD_IS_1,
    #[doc = "0x118 - "]
    pub pvt_ctrl: PVT_CTRL,
    _reserved58: [u8; 20usize],
    #[doc = "0x130 - "]
    pub spare0: SPARE0,
    _reserved59: [u8; 8usize],
    #[doc = "0x13c - "]
    pub static_conf_sig1: STATIC_CONF_SIG1,
    _reserved60: [u8; 96usize],
    #[doc = "0x1a0 - "]
    pub flash_din_0: FLASH_DIN_0,
    #[doc = "0x1a4 - "]
    pub flash_din_1: FLASH_DIN_1,
    #[doc = "0x1a8 - "]
    pub flash_din_2: FLASH_DIN_2,
    #[doc = "0x1ac - "]
    pub flash_din_3: FLASH_DIN_3,
    _reserved64: [u8; 16usize],
    #[doc = "0x1c0 - "]
    pub flash0_dout_0: FLASH0_DOUT_0,
    #[doc = "0x1c4 - "]
    pub flash0_dout_1: FLASH0_DOUT_1,
    #[doc = "0x1c8 - "]
    pub flash0_dout_2: FLASH0_DOUT_2,
    #[doc = "0x1cc - "]
    pub flash0_dout_3: FLASH0_DOUT_3,
    #[doc = "0x1d0 - "]
    pub flash1_dout_0: FLASH1_DOUT_0,
    #[doc = "0x1d4 - "]
    pub flash1_dout_1: FLASH1_DOUT_1,
    #[doc = "0x1d8 - "]
    pub flash1_dout_2: FLASH1_DOUT_2,
    #[doc = "0x1dc - "]
    pub flash1_dout_3: FLASH1_DOUT_3,
    #[doc = "0x1e0 - "]
    pub selection_control_reg: SELECTION_CONTROL_REG,
    #[doc = "0x1e4 - "]
    pub az_rom_remap_mask: AZ_ROM_REMAP_MASK,
    #[doc = "0x1e8 - "]
    pub az_rom_remap_offset: AZ_ROM_REMAP_OFFSET,
    #[doc = "0x1ec - "]
    pub az_code_remap_mask: AZ_CODE_REMAP_MASK,
    #[doc = "0x1f0 - "]
    pub az_code_remap_offset: AZ_CODE_REMAP_OFFSET,
    #[doc = "0x1f4 - "]
    pub az_sys_remap_mask: AZ_SYS_REMAP_MASK,
    #[doc = "0x1f8 - "]
    pub az_sys_remap_offset: AZ_SYS_REMAP_OFFSET,
    _reserved79: [u8; 4usize],
    #[doc = "0x200 - "]
    pub az_ctrl: AZ_CTRL,
    _reserved80: [u8; 4usize],
    #[doc = "0x208 - "]
    pub sse_otp_rd_data: SSE_OTP_RD_DATA,
    _reserved81: [u8; 4usize],
    #[doc = "0x210 - "]
    pub az_otp_rd_data: AZ_OTP_RD_DATA,
    _reserved82: [u8; 8usize],
    #[doc = "0x21c - "]
    pub spare_ctrl0: SPARE_CTRL0,
    #[doc = "0x220 - "]
    pub spare_ctrl1: SPARE_CTRL1,
    _reserved84: [u8; 476usize],
    #[doc = "0x400 - "]
    pub chip_id: CHIP_ID,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_ctrl_sel](clk_ctrl_sel) module"]
pub type CLK_CTRL_SEL = crate::Reg<u32, _CLK_CTRL_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CTRL_SEL;
#[doc = "`read()` method returns [clk_ctrl_sel::R](clk_ctrl_sel::R) reader structure"]
impl crate::Readable for CLK_CTRL_SEL {}
#[doc = "`write(|w| ..)` method takes [clk_ctrl_sel::W](clk_ctrl_sel::W) writer structure"]
impl crate::Writable for CLK_CTRL_SEL {}
#[doc = ""]
pub mod clk_ctrl_sel;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_pll_prediv_ctrl](clk_pll_prediv_ctrl) module"]
pub type CLK_PLL_PREDIV_CTRL = crate::Reg<u32, _CLK_PLL_PREDIV_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PLL_PREDIV_CTRL;
#[doc = "`read()` method returns [clk_pll_prediv_ctrl::R](clk_pll_prediv_ctrl::R) reader structure"]
impl crate::Readable for CLK_PLL_PREDIV_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_pll_prediv_ctrl::W](clk_pll_prediv_ctrl::W) writer structure"]
impl crate::Writable for CLK_PLL_PREDIV_CTRL {}
#[doc = ""]
pub mod clk_pll_prediv_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_postdiv_ctrl_flash](clk_postdiv_ctrl_flash) module"]
pub type CLK_POSTDIV_CTRL_FLASH = crate::Reg<u32, _CLK_POSTDIV_CTRL_FLASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_POSTDIV_CTRL_FLASH;
#[doc = "`read()` method returns [clk_postdiv_ctrl_flash::R](clk_postdiv_ctrl_flash::R) reader structure"]
impl crate::Readable for CLK_POSTDIV_CTRL_FLASH {}
#[doc = "`write(|w| ..)` method takes [clk_postdiv_ctrl_flash::W](clk_postdiv_ctrl_flash::W) writer structure"]
impl crate::Writable for CLK_POSTDIV_CTRL_FLASH {}
#[doc = ""]
pub mod clk_postdiv_ctrl_flash;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_postdiv_ctrl_qspi](clk_postdiv_ctrl_qspi) module"]
pub type CLK_POSTDIV_CTRL_QSPI = crate::Reg<u32, _CLK_POSTDIV_CTRL_QSPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_POSTDIV_CTRL_QSPI;
#[doc = "`read()` method returns [clk_postdiv_ctrl_qspi::R](clk_postdiv_ctrl_qspi::R) reader structure"]
impl crate::Readable for CLK_POSTDIV_CTRL_QSPI {}
#[doc = "`write(|w| ..)` method takes [clk_postdiv_ctrl_qspi::W](clk_postdiv_ctrl_qspi::W) writer structure"]
impl crate::Writable for CLK_POSTDIV_CTRL_QSPI {}
#[doc = ""]
pub mod clk_postdiv_ctrl_qspi;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_postdiv_ctrl_rtc](clk_postdiv_ctrl_rtc) module"]
pub type CLK_POSTDIV_CTRL_RTC = crate::Reg<u32, _CLK_POSTDIV_CTRL_RTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_POSTDIV_CTRL_RTC;
#[doc = "`read()` method returns [clk_postdiv_ctrl_rtc::R](clk_postdiv_ctrl_rtc::R) reader structure"]
impl crate::Readable for CLK_POSTDIV_CTRL_RTC {}
#[doc = "`write(|w| ..)` method takes [clk_postdiv_ctrl_rtc::W](clk_postdiv_ctrl_rtc::W) writer structure"]
impl crate::Writable for CLK_POSTDIV_CTRL_RTC {}
#[doc = ""]
pub mod clk_postdiv_ctrl_rtc;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_postdiv_ctrl_sd](clk_postdiv_ctrl_sd) module"]
pub type CLK_POSTDIV_CTRL_SD = crate::Reg<u32, _CLK_POSTDIV_CTRL_SD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_POSTDIV_CTRL_SD;
#[doc = "`read()` method returns [clk_postdiv_ctrl_sd::R](clk_postdiv_ctrl_sd::R) reader structure"]
impl crate::Readable for CLK_POSTDIV_CTRL_SD {}
#[doc = "`write(|w| ..)` method takes [clk_postdiv_ctrl_sd::W](clk_postdiv_ctrl_sd::W) writer structure"]
impl crate::Writable for CLK_POSTDIV_CTRL_SD {}
#[doc = ""]
pub mod clk_postdiv_ctrl_sd;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_postdiv_ctrl_test](clk_postdiv_ctrl_test) module"]
pub type CLK_POSTDIV_CTRL_TEST = crate::Reg<u32, _CLK_POSTDIV_CTRL_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_POSTDIV_CTRL_TEST;
#[doc = "`read()` method returns [clk_postdiv_ctrl_test::R](clk_postdiv_ctrl_test::R) reader structure"]
impl crate::Readable for CLK_POSTDIV_CTRL_TEST {}
#[doc = "`write(|w| ..)` method takes [clk_postdiv_ctrl_test::W](clk_postdiv_ctrl_test::W) writer structure"]
impl crate::Writable for CLK_POSTDIV_CTRL_TEST {}
#[doc = ""]
pub mod clk_postdiv_ctrl_test;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl_bypass_div](ctrl_bypass_div) module"]
pub type CTRL_BYPASS_DIV = crate::Reg<u32, _CTRL_BYPASS_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_BYPASS_DIV;
#[doc = "`read()` method returns [ctrl_bypass_div::R](ctrl_bypass_div::R) reader structure"]
impl crate::Readable for CTRL_BYPASS_DIV {}
#[doc = "`write(|w| ..)` method takes [ctrl_bypass_div::W](ctrl_bypass_div::W) writer structure"]
impl crate::Writable for CTRL_BYPASS_DIV {}
#[doc = ""]
pub mod ctrl_bypass_div;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_ctrl_pll0_clk](pll_ctrl_pll0_clk) module"]
pub type PLL_CTRL_PLL0_CLK = crate::Reg<u32, _PLL_CTRL_PLL0_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_CTRL_PLL0_CLK;
#[doc = "`read()` method returns [pll_ctrl_pll0_clk::R](pll_ctrl_pll0_clk::R) reader structure"]
impl crate::Readable for PLL_CTRL_PLL0_CLK {}
#[doc = "`write(|w| ..)` method takes [pll_ctrl_pll0_clk::W](pll_ctrl_pll0_clk::W) writer structure"]
impl crate::Writable for PLL_CTRL_PLL0_CLK {}
#[doc = ""]
pub mod pll_ctrl_pll0_clk;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_postdiv_ctrl_pll0_clk](pll_postdiv_ctrl_pll0_clk) module"]
pub type PLL_POSTDIV_CTRL_PLL0_CLK = crate::Reg<u32, _PLL_POSTDIV_CTRL_PLL0_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_POSTDIV_CTRL_PLL0_CLK;
#[doc = "`read()` method returns [pll_postdiv_ctrl_pll0_clk::R](pll_postdiv_ctrl_pll0_clk::R) reader structure"]
impl crate::Readable for PLL_POSTDIV_CTRL_PLL0_CLK {}
#[doc = "`write(|w| ..)` method takes [pll_postdiv_ctrl_pll0_clk::W](pll_postdiv_ctrl_pll0_clk::W) writer structure"]
impl crate::Writable for PLL_POSTDIV_CTRL_PLL0_CLK {}
#[doc = ""]
pub mod pll_postdiv_ctrl_pll0_clk;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_ctrl_mult_pll0_clk](pll_ctrl_mult_pll0_clk) module"]
pub type PLL_CTRL_MULT_PLL0_CLK = crate::Reg<u32, _PLL_CTRL_MULT_PLL0_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_CTRL_MULT_PLL0_CLK;
#[doc = "`read()` method returns [pll_ctrl_mult_pll0_clk::R](pll_ctrl_mult_pll0_clk::R) reader structure"]
impl crate::Readable for PLL_CTRL_MULT_PLL0_CLK {}
#[doc = "`write(|w| ..)` method takes [pll_ctrl_mult_pll0_clk::W](pll_ctrl_mult_pll0_clk::W) writer structure"]
impl crate::Writable for PLL_CTRL_MULT_PLL0_CLK {}
#[doc = ""]
pub mod pll_ctrl_mult_pll0_clk;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_ctrl_enable](clk_ctrl_enable) module"]
pub type CLK_CTRL_ENABLE = crate::Reg<u32, _CLK_CTRL_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CTRL_ENABLE;
#[doc = "`read()` method returns [clk_ctrl_enable::R](clk_ctrl_enable::R) reader structure"]
impl crate::Readable for CLK_CTRL_ENABLE {}
#[doc = "`write(|w| ..)` method takes [clk_ctrl_enable::W](clk_ctrl_enable::W) writer structure"]
impl crate::Writable for CLK_CTRL_ENABLE {}
#[doc = ""]
pub mod clk_ctrl_enable;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_status](clk_status) module"]
pub type CLK_STATUS = crate::Reg<u32, _CLK_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_STATUS;
#[doc = "`read()` method returns [clk_status::R](clk_status::R) reader structure"]
impl crate::Readable for CLK_STATUS {}
#[doc = ""]
pub mod clk_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset_ctrl](reset_ctrl) module"]
pub type RESET_CTRL = crate::Reg<u32, _RESET_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_CTRL;
#[doc = "`read()` method returns [reset_ctrl::R](reset_ctrl::R) reader structure"]
impl crate::Readable for RESET_CTRL {}
#[doc = "`write(|w| ..)` method takes [reset_ctrl::W](reset_ctrl::W) writer structure"]
impl crate::Writable for RESET_CTRL {}
#[doc = ""]
pub mod reset_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbg_ctrl](dbg_ctrl) module"]
pub type DBG_CTRL = crate::Reg<u32, _DBG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_CTRL;
#[doc = "`read()` method returns [dbg_ctrl::R](dbg_ctrl::R) reader structure"]
impl crate::Readable for DBG_CTRL {}
#[doc = "`write(|w| ..)` method takes [dbg_ctrl::W](dbg_ctrl::W) writer structure"]
impl crate::Writable for DBG_CTRL {}
#[doc = ""]
pub mod dbg_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sram_ctrl](sram_ctrl) module"]
pub type SRAM_CTRL = crate::Reg<u32, _SRAM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_CTRL;
#[doc = "`read()` method returns [sram_ctrl::R](sram_ctrl::R) reader structure"]
impl crate::Readable for SRAM_CTRL {}
#[doc = "`write(|w| ..)` method takes [sram_ctrl::W](sram_ctrl::W) writer structure"]
impl crate::Writable for SRAM_CTRL {}
#[doc = ""]
pub mod sram_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_ctrl](intr_ctrl) module"]
pub type INTR_CTRL = crate::Reg<u32, _INTR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CTRL;
#[doc = "`read()` method returns [intr_ctrl::R](intr_ctrl::R) reader structure"]
impl crate::Readable for INTR_CTRL {}
#[doc = "`write(|w| ..)` method takes [intr_ctrl::W](intr_ctrl::W) writer structure"]
impl crate::Writable for INTR_CTRL {}
#[doc = ""]
pub mod intr_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_test_ctrl](clk_test_ctrl) module"]
pub type CLK_TEST_CTRL = crate::Reg<u32, _CLK_TEST_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TEST_CTRL;
#[doc = "`read()` method returns [clk_test_ctrl::R](clk_test_ctrl::R) reader structure"]
impl crate::Readable for CLK_TEST_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_test_ctrl::W](clk_test_ctrl::W) writer structure"]
impl crate::Writable for CLK_TEST_CTRL {}
#[doc = ""]
pub mod clk_test_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu0_vtor](cpu0_vtor) module"]
pub type CPU0_VTOR = crate::Reg<u32, _CPU0_VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU0_VTOR;
#[doc = "`read()` method returns [cpu0_vtor::R](cpu0_vtor::R) reader structure"]
impl crate::Readable for CPU0_VTOR {}
#[doc = "`write(|w| ..)` method takes [cpu0_vtor::W](cpu0_vtor::W) writer structure"]
impl crate::Writable for CPU0_VTOR {}
#[doc = ""]
pub mod cpu0_vtor;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu1_vtor](cpu1_vtor) module"]
pub type CPU1_VTOR = crate::Reg<u32, _CPU1_VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU1_VTOR;
#[doc = "`read()` method returns [cpu1_vtor::R](cpu1_vtor::R) reader structure"]
impl crate::Readable for CPU1_VTOR {}
#[doc = "`write(|w| ..)` method takes [cpu1_vtor::W](cpu1_vtor::W) writer structure"]
impl crate::Writable for CPU1_VTOR {}
#[doc = ""]
pub mod cpu1_vtor;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_cpu_vtor](az_cpu_vtor) module"]
pub type AZ_CPU_VTOR = crate::Reg<u32, _AZ_CPU_VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_CPU_VTOR;
#[doc = "`read()` method returns [az_cpu_vtor::R](az_cpu_vtor::R) reader structure"]
impl crate::Readable for AZ_CPU_VTOR {}
#[doc = "`write(|w| ..)` method takes [az_cpu_vtor::W](az_cpu_vtor::W) writer structure"]
impl crate::Writable for AZ_CPU_VTOR {}
#[doc = ""]
pub mod az_cpu_vtor;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_insel_0](iomux_main_insel_0) module"]
pub type IOMUX_MAIN_INSEL_0 = crate::Reg<u32, _IOMUX_MAIN_INSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_INSEL_0;
#[doc = "`read()` method returns [iomux_main_insel_0::R](iomux_main_insel_0::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_INSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_insel_0::W](iomux_main_insel_0::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_INSEL_0 {}
#[doc = ""]
pub mod iomux_main_insel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_insel_1](iomux_main_insel_1) module"]
pub type IOMUX_MAIN_INSEL_1 = crate::Reg<u32, _IOMUX_MAIN_INSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_INSEL_1;
#[doc = "`read()` method returns [iomux_main_insel_1::R](iomux_main_insel_1::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_INSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_insel_1::W](iomux_main_insel_1::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_INSEL_1 {}
#[doc = ""]
pub mod iomux_main_insel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_outsel_0](iomux_main_outsel_0) module"]
pub type IOMUX_MAIN_OUTSEL_0 = crate::Reg<u32, _IOMUX_MAIN_OUTSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_OUTSEL_0;
#[doc = "`read()` method returns [iomux_main_outsel_0::R](iomux_main_outsel_0::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_OUTSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_outsel_0::W](iomux_main_outsel_0::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_OUTSEL_0 {}
#[doc = ""]
pub mod iomux_main_outsel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_outsel_1](iomux_main_outsel_1) module"]
pub type IOMUX_MAIN_OUTSEL_1 = crate::Reg<u32, _IOMUX_MAIN_OUTSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_OUTSEL_1;
#[doc = "`read()` method returns [iomux_main_outsel_1::R](iomux_main_outsel_1::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_OUTSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_outsel_1::W](iomux_main_outsel_1::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_OUTSEL_1 {}
#[doc = ""]
pub mod iomux_main_outsel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_oensel_0](iomux_main_oensel_0) module"]
pub type IOMUX_MAIN_OENSEL_0 = crate::Reg<u32, _IOMUX_MAIN_OENSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_OENSEL_0;
#[doc = "`read()` method returns [iomux_main_oensel_0::R](iomux_main_oensel_0::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_OENSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_oensel_0::W](iomux_main_oensel_0::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_OENSEL_0 {}
#[doc = ""]
pub mod iomux_main_oensel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_oensel_1](iomux_main_oensel_1) module"]
pub type IOMUX_MAIN_OENSEL_1 = crate::Reg<u32, _IOMUX_MAIN_OENSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_OENSEL_1;
#[doc = "`read()` method returns [iomux_main_oensel_1::R](iomux_main_oensel_1::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_OENSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_oensel_1::W](iomux_main_oensel_1::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_OENSEL_1 {}
#[doc = ""]
pub mod iomux_main_oensel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_default_in_0](iomux_main_default_in_0) module"]
pub type IOMUX_MAIN_DEFAULT_IN_0 = crate::Reg<u32, _IOMUX_MAIN_DEFAULT_IN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_DEFAULT_IN_0;
#[doc = "`read()` method returns [iomux_main_default_in_0::R](iomux_main_default_in_0::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_DEFAULT_IN_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_default_in_0::W](iomux_main_default_in_0::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_DEFAULT_IN_0 {}
#[doc = ""]
pub mod iomux_main_default_in_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_main_default_in_1](iomux_main_default_in_1) module"]
pub type IOMUX_MAIN_DEFAULT_IN_1 = crate::Reg<u32, _IOMUX_MAIN_DEFAULT_IN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_MAIN_DEFAULT_IN_1;
#[doc = "`read()` method returns [iomux_main_default_in_1::R](iomux_main_default_in_1::R) reader structure"]
impl crate::Readable for IOMUX_MAIN_DEFAULT_IN_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_main_default_in_1::W](iomux_main_default_in_1::W) writer structure"]
impl crate::Writable for IOMUX_MAIN_DEFAULT_IN_1 {}
#[doc = ""]
pub mod iomux_main_default_in_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_insel_0](iomux_altf1_insel_0) module"]
pub type IOMUX_ALTF1_INSEL_0 = crate::Reg<u32, _IOMUX_ALTF1_INSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_INSEL_0;
#[doc = "`read()` method returns [iomux_altf1_insel_0::R](iomux_altf1_insel_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_INSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_insel_0::W](iomux_altf1_insel_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_INSEL_0 {}
#[doc = ""]
pub mod iomux_altf1_insel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_insel_1](iomux_altf1_insel_1) module"]
pub type IOMUX_ALTF1_INSEL_1 = crate::Reg<u32, _IOMUX_ALTF1_INSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_INSEL_1;
#[doc = "`read()` method returns [iomux_altf1_insel_1::R](iomux_altf1_insel_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_INSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_insel_1::W](iomux_altf1_insel_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_INSEL_1 {}
#[doc = ""]
pub mod iomux_altf1_insel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_outsel_0](iomux_altf1_outsel_0) module"]
pub type IOMUX_ALTF1_OUTSEL_0 = crate::Reg<u32, _IOMUX_ALTF1_OUTSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_OUTSEL_0;
#[doc = "`read()` method returns [iomux_altf1_outsel_0::R](iomux_altf1_outsel_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_OUTSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_outsel_0::W](iomux_altf1_outsel_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_OUTSEL_0 {}
#[doc = ""]
pub mod iomux_altf1_outsel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_outsel_1](iomux_altf1_outsel_1) module"]
pub type IOMUX_ALTF1_OUTSEL_1 = crate::Reg<u32, _IOMUX_ALTF1_OUTSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_OUTSEL_1;
#[doc = "`read()` method returns [iomux_altf1_outsel_1::R](iomux_altf1_outsel_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_OUTSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_outsel_1::W](iomux_altf1_outsel_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_OUTSEL_1 {}
#[doc = ""]
pub mod iomux_altf1_outsel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_oensel_0](iomux_altf1_oensel_0) module"]
pub type IOMUX_ALTF1_OENSEL_0 = crate::Reg<u32, _IOMUX_ALTF1_OENSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_OENSEL_0;
#[doc = "`read()` method returns [iomux_altf1_oensel_0::R](iomux_altf1_oensel_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_OENSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_oensel_0::W](iomux_altf1_oensel_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_OENSEL_0 {}
#[doc = ""]
pub mod iomux_altf1_oensel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_oensel_1](iomux_altf1_oensel_1) module"]
pub type IOMUX_ALTF1_OENSEL_1 = crate::Reg<u32, _IOMUX_ALTF1_OENSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_OENSEL_1;
#[doc = "`read()` method returns [iomux_altf1_oensel_1::R](iomux_altf1_oensel_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_OENSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_oensel_1::W](iomux_altf1_oensel_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_OENSEL_1 {}
#[doc = ""]
pub mod iomux_altf1_oensel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_default_in_0](iomux_altf1_default_in_0) module"]
pub type IOMUX_ALTF1_DEFAULT_IN_0 = crate::Reg<u32, _IOMUX_ALTF1_DEFAULT_IN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_DEFAULT_IN_0;
#[doc = "`read()` method returns [iomux_altf1_default_in_0::R](iomux_altf1_default_in_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_DEFAULT_IN_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_default_in_0::W](iomux_altf1_default_in_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_DEFAULT_IN_0 {}
#[doc = ""]
pub mod iomux_altf1_default_in_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf1_default_in_1](iomux_altf1_default_in_1) module"]
pub type IOMUX_ALTF1_DEFAULT_IN_1 = crate::Reg<u32, _IOMUX_ALTF1_DEFAULT_IN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF1_DEFAULT_IN_1;
#[doc = "`read()` method returns [iomux_altf1_default_in_1::R](iomux_altf1_default_in_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF1_DEFAULT_IN_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf1_default_in_1::W](iomux_altf1_default_in_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF1_DEFAULT_IN_1 {}
#[doc = ""]
pub mod iomux_altf1_default_in_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_insel_0](iomux_altf2_insel_0) module"]
pub type IOMUX_ALTF2_INSEL_0 = crate::Reg<u32, _IOMUX_ALTF2_INSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_INSEL_0;
#[doc = "`read()` method returns [iomux_altf2_insel_0::R](iomux_altf2_insel_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_INSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_insel_0::W](iomux_altf2_insel_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_INSEL_0 {}
#[doc = ""]
pub mod iomux_altf2_insel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_insel_1](iomux_altf2_insel_1) module"]
pub type IOMUX_ALTF2_INSEL_1 = crate::Reg<u32, _IOMUX_ALTF2_INSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_INSEL_1;
#[doc = "`read()` method returns [iomux_altf2_insel_1::R](iomux_altf2_insel_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_INSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_insel_1::W](iomux_altf2_insel_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_INSEL_1 {}
#[doc = ""]
pub mod iomux_altf2_insel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_outsel_0](iomux_altf2_outsel_0) module"]
pub type IOMUX_ALTF2_OUTSEL_0 = crate::Reg<u32, _IOMUX_ALTF2_OUTSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_OUTSEL_0;
#[doc = "`read()` method returns [iomux_altf2_outsel_0::R](iomux_altf2_outsel_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_OUTSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_outsel_0::W](iomux_altf2_outsel_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_OUTSEL_0 {}
#[doc = ""]
pub mod iomux_altf2_outsel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_outsel_1](iomux_altf2_outsel_1) module"]
pub type IOMUX_ALTF2_OUTSEL_1 = crate::Reg<u32, _IOMUX_ALTF2_OUTSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_OUTSEL_1;
#[doc = "`read()` method returns [iomux_altf2_outsel_1::R](iomux_altf2_outsel_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_OUTSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_outsel_1::W](iomux_altf2_outsel_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_OUTSEL_1 {}
#[doc = ""]
pub mod iomux_altf2_outsel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_oensel_0](iomux_altf2_oensel_0) module"]
pub type IOMUX_ALTF2_OENSEL_0 = crate::Reg<u32, _IOMUX_ALTF2_OENSEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_OENSEL_0;
#[doc = "`read()` method returns [iomux_altf2_oensel_0::R](iomux_altf2_oensel_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_OENSEL_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_oensel_0::W](iomux_altf2_oensel_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_OENSEL_0 {}
#[doc = ""]
pub mod iomux_altf2_oensel_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_oensel_1](iomux_altf2_oensel_1) module"]
pub type IOMUX_ALTF2_OENSEL_1 = crate::Reg<u32, _IOMUX_ALTF2_OENSEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_OENSEL_1;
#[doc = "`read()` method returns [iomux_altf2_oensel_1::R](iomux_altf2_oensel_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_OENSEL_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_oensel_1::W](iomux_altf2_oensel_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_OENSEL_1 {}
#[doc = ""]
pub mod iomux_altf2_oensel_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_default_in_0](iomux_altf2_default_in_0) module"]
pub type IOMUX_ALTF2_DEFAULT_IN_0 = crate::Reg<u32, _IOMUX_ALTF2_DEFAULT_IN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_DEFAULT_IN_0;
#[doc = "`read()` method returns [iomux_altf2_default_in_0::R](iomux_altf2_default_in_0::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_DEFAULT_IN_0 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_default_in_0::W](iomux_altf2_default_in_0::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_DEFAULT_IN_0 {}
#[doc = ""]
pub mod iomux_altf2_default_in_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iomux_altf2_default_in_1](iomux_altf2_default_in_1) module"]
pub type IOMUX_ALTF2_DEFAULT_IN_1 = crate::Reg<u32, _IOMUX_ALTF2_DEFAULT_IN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMUX_ALTF2_DEFAULT_IN_1;
#[doc = "`read()` method returns [iomux_altf2_default_in_1::R](iomux_altf2_default_in_1::R) reader structure"]
impl crate::Readable for IOMUX_ALTF2_DEFAULT_IN_1 {}
#[doc = "`write(|w| ..)` method takes [iomux_altf2_default_in_1::W](iomux_altf2_default_in_1::W) writer structure"]
impl crate::Writable for IOMUX_ALTF2_DEFAULT_IN_1 {}
#[doc = ""]
pub mod iomux_altf2_default_in_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_dso_0](iopad_dso_0) module"]
pub type IOPAD_DSO_0 = crate::Reg<u32, _IOPAD_DSO_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_DSO_0;
#[doc = "`read()` method returns [iopad_dso_0::R](iopad_dso_0::R) reader structure"]
impl crate::Readable for IOPAD_DSO_0 {}
#[doc = "`write(|w| ..)` method takes [iopad_dso_0::W](iopad_dso_0::W) writer structure"]
impl crate::Writable for IOPAD_DSO_0 {}
#[doc = ""]
pub mod iopad_dso_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_dso_1](iopad_dso_1) module"]
pub type IOPAD_DSO_1 = crate::Reg<u32, _IOPAD_DSO_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_DSO_1;
#[doc = "`read()` method returns [iopad_dso_1::R](iopad_dso_1::R) reader structure"]
impl crate::Readable for IOPAD_DSO_1 {}
#[doc = "`write(|w| ..)` method takes [iopad_dso_1::W](iopad_dso_1::W) writer structure"]
impl crate::Writable for IOPAD_DSO_1 {}
#[doc = ""]
pub mod iopad_dso_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_ds1_0](iopad_ds1_0) module"]
pub type IOPAD_DS1_0 = crate::Reg<u32, _IOPAD_DS1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_DS1_0;
#[doc = "`read()` method returns [iopad_ds1_0::R](iopad_ds1_0::R) reader structure"]
impl crate::Readable for IOPAD_DS1_0 {}
#[doc = "`write(|w| ..)` method takes [iopad_ds1_0::W](iopad_ds1_0::W) writer structure"]
impl crate::Writable for IOPAD_DS1_0 {}
#[doc = ""]
pub mod iopad_ds1_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_ds1_1](iopad_ds1_1) module"]
pub type IOPAD_DS1_1 = crate::Reg<u32, _IOPAD_DS1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_DS1_1;
#[doc = "`read()` method returns [iopad_ds1_1::R](iopad_ds1_1::R) reader structure"]
impl crate::Readable for IOPAD_DS1_1 {}
#[doc = "`write(|w| ..)` method takes [iopad_ds1_1::W](iopad_ds1_1::W) writer structure"]
impl crate::Writable for IOPAD_DS1_1 {}
#[doc = ""]
pub mod iopad_ds1_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_pe_0](iopad_pe_0) module"]
pub type IOPAD_PE_0 = crate::Reg<u32, _IOPAD_PE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_PE_0;
#[doc = "`read()` method returns [iopad_pe_0::R](iopad_pe_0::R) reader structure"]
impl crate::Readable for IOPAD_PE_0 {}
#[doc = "`write(|w| ..)` method takes [iopad_pe_0::W](iopad_pe_0::W) writer structure"]
impl crate::Writable for IOPAD_PE_0 {}
#[doc = ""]
pub mod iopad_pe_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_pe_1](iopad_pe_1) module"]
pub type IOPAD_PE_1 = crate::Reg<u32, _IOPAD_PE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_PE_1;
#[doc = "`read()` method returns [iopad_pe_1::R](iopad_pe_1::R) reader structure"]
impl crate::Readable for IOPAD_PE_1 {}
#[doc = "`write(|w| ..)` method takes [iopad_pe_1::W](iopad_pe_1::W) writer structure"]
impl crate::Writable for IOPAD_PE_1 {}
#[doc = ""]
pub mod iopad_pe_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_ps_0](iopad_ps_0) module"]
pub type IOPAD_PS_0 = crate::Reg<u32, _IOPAD_PS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_PS_0;
#[doc = "`read()` method returns [iopad_ps_0::R](iopad_ps_0::R) reader structure"]
impl crate::Readable for IOPAD_PS_0 {}
#[doc = "`write(|w| ..)` method takes [iopad_ps_0::W](iopad_ps_0::W) writer structure"]
impl crate::Writable for IOPAD_PS_0 {}
#[doc = ""]
pub mod iopad_ps_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_ps_1](iopad_ps_1) module"]
pub type IOPAD_PS_1 = crate::Reg<u32, _IOPAD_PS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_PS_1;
#[doc = "`read()` method returns [iopad_ps_1::R](iopad_ps_1::R) reader structure"]
impl crate::Readable for IOPAD_PS_1 {}
#[doc = "`write(|w| ..)` method takes [iopad_ps_1::W](iopad_ps_1::W) writer structure"]
impl crate::Writable for IOPAD_PS_1 {}
#[doc = ""]
pub mod iopad_ps_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_sr_0](iopad_sr_0) module"]
pub type IOPAD_SR_0 = crate::Reg<u32, _IOPAD_SR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_SR_0;
#[doc = "`read()` method returns [iopad_sr_0::R](iopad_sr_0::R) reader structure"]
impl crate::Readable for IOPAD_SR_0 {}
#[doc = "`write(|w| ..)` method takes [iopad_sr_0::W](iopad_sr_0::W) writer structure"]
impl crate::Writable for IOPAD_SR_0 {}
#[doc = ""]
pub mod iopad_sr_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_sr_1](iopad_sr_1) module"]
pub type IOPAD_SR_1 = crate::Reg<u32, _IOPAD_SR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_SR_1;
#[doc = "`read()` method returns [iopad_sr_1::R](iopad_sr_1::R) reader structure"]
impl crate::Readable for IOPAD_SR_1 {}
#[doc = "`write(|w| ..)` method takes [iopad_sr_1::W](iopad_sr_1::W) writer structure"]
impl crate::Writable for IOPAD_SR_1 {}
#[doc = ""]
pub mod iopad_sr_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_is_0](iopad_is_0) module"]
pub type IOPAD_IS_0 = crate::Reg<u32, _IOPAD_IS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_IS_0;
#[doc = "`read()` method returns [iopad_is_0::R](iopad_is_0::R) reader structure"]
impl crate::Readable for IOPAD_IS_0 {}
#[doc = "`write(|w| ..)` method takes [iopad_is_0::W](iopad_is_0::W) writer structure"]
impl crate::Writable for IOPAD_IS_0 {}
#[doc = ""]
pub mod iopad_is_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iopad_is_1](iopad_is_1) module"]
pub type IOPAD_IS_1 = crate::Reg<u32, _IOPAD_IS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPAD_IS_1;
#[doc = "`read()` method returns [iopad_is_1::R](iopad_is_1::R) reader structure"]
impl crate::Readable for IOPAD_IS_1 {}
#[doc = "`write(|w| ..)` method takes [iopad_is_1::W](iopad_is_1::W) writer structure"]
impl crate::Writable for IOPAD_IS_1 {}
#[doc = ""]
pub mod iopad_is_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pvt_ctrl](pvt_ctrl) module"]
pub type PVT_CTRL = crate::Reg<u32, _PVT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PVT_CTRL;
#[doc = "`read()` method returns [pvt_ctrl::R](pvt_ctrl::R) reader structure"]
impl crate::Readable for PVT_CTRL {}
#[doc = "`write(|w| ..)` method takes [pvt_ctrl::W](pvt_ctrl::W) writer structure"]
impl crate::Writable for PVT_CTRL {}
#[doc = ""]
pub mod pvt_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spare0](spare0) module"]
pub type SPARE0 = crate::Reg<u32, _SPARE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE0;
#[doc = "`read()` method returns [spare0::R](spare0::R) reader structure"]
impl crate::Readable for SPARE0 {}
#[doc = "`write(|w| ..)` method takes [spare0::W](spare0::W) writer structure"]
impl crate::Writable for SPARE0 {}
#[doc = ""]
pub mod spare0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [static_conf_sig1](static_conf_sig1) module"]
pub type STATIC_CONF_SIG1 = crate::Reg<u32, _STATIC_CONF_SIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATIC_CONF_SIG1;
#[doc = "`read()` method returns [static_conf_sig1::R](static_conf_sig1::R) reader structure"]
impl crate::Readable for STATIC_CONF_SIG1 {}
#[doc = "`write(|w| ..)` method takes [static_conf_sig1::W](static_conf_sig1::W) writer structure"]
impl crate::Writable for STATIC_CONF_SIG1 {}
#[doc = ""]
pub mod static_conf_sig1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash_din_0](flash_din_0) module"]
pub type FLASH_DIN_0 = crate::Reg<u32, _FLASH_DIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DIN_0;
#[doc = "`read()` method returns [flash_din_0::R](flash_din_0::R) reader structure"]
impl crate::Readable for FLASH_DIN_0 {}
#[doc = "`write(|w| ..)` method takes [flash_din_0::W](flash_din_0::W) writer structure"]
impl crate::Writable for FLASH_DIN_0 {}
#[doc = ""]
pub mod flash_din_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash_din_1](flash_din_1) module"]
pub type FLASH_DIN_1 = crate::Reg<u32, _FLASH_DIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DIN_1;
#[doc = "`read()` method returns [flash_din_1::R](flash_din_1::R) reader structure"]
impl crate::Readable for FLASH_DIN_1 {}
#[doc = "`write(|w| ..)` method takes [flash_din_1::W](flash_din_1::W) writer structure"]
impl crate::Writable for FLASH_DIN_1 {}
#[doc = ""]
pub mod flash_din_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash_din_2](flash_din_2) module"]
pub type FLASH_DIN_2 = crate::Reg<u32, _FLASH_DIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DIN_2;
#[doc = "`read()` method returns [flash_din_2::R](flash_din_2::R) reader structure"]
impl crate::Readable for FLASH_DIN_2 {}
#[doc = "`write(|w| ..)` method takes [flash_din_2::W](flash_din_2::W) writer structure"]
impl crate::Writable for FLASH_DIN_2 {}
#[doc = ""]
pub mod flash_din_2;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash_din_3](flash_din_3) module"]
pub type FLASH_DIN_3 = crate::Reg<u32, _FLASH_DIN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DIN_3;
#[doc = "`read()` method returns [flash_din_3::R](flash_din_3::R) reader structure"]
impl crate::Readable for FLASH_DIN_3 {}
#[doc = "`write(|w| ..)` method takes [flash_din_3::W](flash_din_3::W) writer structure"]
impl crate::Writable for FLASH_DIN_3 {}
#[doc = ""]
pub mod flash_din_3;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash0_dout_0](flash0_dout_0) module"]
pub type FLASH0_DOUT_0 = crate::Reg<u32, _FLASH0_DOUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH0_DOUT_0;
#[doc = "`read()` method returns [flash0_dout_0::R](flash0_dout_0::R) reader structure"]
impl crate::Readable for FLASH0_DOUT_0 {}
#[doc = ""]
pub mod flash0_dout_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash0_dout_1](flash0_dout_1) module"]
pub type FLASH0_DOUT_1 = crate::Reg<u32, _FLASH0_DOUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH0_DOUT_1;
#[doc = "`read()` method returns [flash0_dout_1::R](flash0_dout_1::R) reader structure"]
impl crate::Readable for FLASH0_DOUT_1 {}
#[doc = ""]
pub mod flash0_dout_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash0_dout_2](flash0_dout_2) module"]
pub type FLASH0_DOUT_2 = crate::Reg<u32, _FLASH0_DOUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH0_DOUT_2;
#[doc = "`read()` method returns [flash0_dout_2::R](flash0_dout_2::R) reader structure"]
impl crate::Readable for FLASH0_DOUT_2 {}
#[doc = ""]
pub mod flash0_dout_2;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash0_dout_3](flash0_dout_3) module"]
pub type FLASH0_DOUT_3 = crate::Reg<u32, _FLASH0_DOUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH0_DOUT_3;
#[doc = "`read()` method returns [flash0_dout_3::R](flash0_dout_3::R) reader structure"]
impl crate::Readable for FLASH0_DOUT_3 {}
#[doc = ""]
pub mod flash0_dout_3;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash1_dout_0](flash1_dout_0) module"]
pub type FLASH1_DOUT_0 = crate::Reg<u32, _FLASH1_DOUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH1_DOUT_0;
#[doc = "`read()` method returns [flash1_dout_0::R](flash1_dout_0::R) reader structure"]
impl crate::Readable for FLASH1_DOUT_0 {}
#[doc = ""]
pub mod flash1_dout_0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash1_dout_1](flash1_dout_1) module"]
pub type FLASH1_DOUT_1 = crate::Reg<u32, _FLASH1_DOUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH1_DOUT_1;
#[doc = "`read()` method returns [flash1_dout_1::R](flash1_dout_1::R) reader structure"]
impl crate::Readable for FLASH1_DOUT_1 {}
#[doc = ""]
pub mod flash1_dout_1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash1_dout_2](flash1_dout_2) module"]
pub type FLASH1_DOUT_2 = crate::Reg<u32, _FLASH1_DOUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH1_DOUT_2;
#[doc = "`read()` method returns [flash1_dout_2::R](flash1_dout_2::R) reader structure"]
impl crate::Readable for FLASH1_DOUT_2 {}
#[doc = ""]
pub mod flash1_dout_2;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash1_dout_3](flash1_dout_3) module"]
pub type FLASH1_DOUT_3 = crate::Reg<u32, _FLASH1_DOUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH1_DOUT_3;
#[doc = "`read()` method returns [flash1_dout_3::R](flash1_dout_3::R) reader structure"]
impl crate::Readable for FLASH1_DOUT_3 {}
#[doc = ""]
pub mod flash1_dout_3;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [selection_control_reg](selection_control_reg) module"]
pub type SELECTION_CONTROL_REG = crate::Reg<u32, _SELECTION_CONTROL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELECTION_CONTROL_REG;
#[doc = "`read()` method returns [selection_control_reg::R](selection_control_reg::R) reader structure"]
impl crate::Readable for SELECTION_CONTROL_REG {}
#[doc = "`write(|w| ..)` method takes [selection_control_reg::W](selection_control_reg::W) writer structure"]
impl crate::Writable for SELECTION_CONTROL_REG {}
#[doc = ""]
pub mod selection_control_reg;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_rom_remap_mask](az_rom_remap_mask) module"]
pub type AZ_ROM_REMAP_MASK = crate::Reg<u32, _AZ_ROM_REMAP_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_ROM_REMAP_MASK;
#[doc = "`read()` method returns [az_rom_remap_mask::R](az_rom_remap_mask::R) reader structure"]
impl crate::Readable for AZ_ROM_REMAP_MASK {}
#[doc = "`write(|w| ..)` method takes [az_rom_remap_mask::W](az_rom_remap_mask::W) writer structure"]
impl crate::Writable for AZ_ROM_REMAP_MASK {}
#[doc = ""]
pub mod az_rom_remap_mask;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_rom_remap_offset](az_rom_remap_offset) module"]
pub type AZ_ROM_REMAP_OFFSET = crate::Reg<u32, _AZ_ROM_REMAP_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_ROM_REMAP_OFFSET;
#[doc = "`read()` method returns [az_rom_remap_offset::R](az_rom_remap_offset::R) reader structure"]
impl crate::Readable for AZ_ROM_REMAP_OFFSET {}
#[doc = "`write(|w| ..)` method takes [az_rom_remap_offset::W](az_rom_remap_offset::W) writer structure"]
impl crate::Writable for AZ_ROM_REMAP_OFFSET {}
#[doc = ""]
pub mod az_rom_remap_offset;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_code_remap_mask](az_code_remap_mask) module"]
pub type AZ_CODE_REMAP_MASK = crate::Reg<u32, _AZ_CODE_REMAP_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_CODE_REMAP_MASK;
#[doc = "`read()` method returns [az_code_remap_mask::R](az_code_remap_mask::R) reader structure"]
impl crate::Readable for AZ_CODE_REMAP_MASK {}
#[doc = "`write(|w| ..)` method takes [az_code_remap_mask::W](az_code_remap_mask::W) writer structure"]
impl crate::Writable for AZ_CODE_REMAP_MASK {}
#[doc = ""]
pub mod az_code_remap_mask;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_code_remap_offset](az_code_remap_offset) module"]
pub type AZ_CODE_REMAP_OFFSET = crate::Reg<u32, _AZ_CODE_REMAP_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_CODE_REMAP_OFFSET;
#[doc = "`read()` method returns [az_code_remap_offset::R](az_code_remap_offset::R) reader structure"]
impl crate::Readable for AZ_CODE_REMAP_OFFSET {}
#[doc = "`write(|w| ..)` method takes [az_code_remap_offset::W](az_code_remap_offset::W) writer structure"]
impl crate::Writable for AZ_CODE_REMAP_OFFSET {}
#[doc = ""]
pub mod az_code_remap_offset;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_sys_remap_mask](az_sys_remap_mask) module"]
pub type AZ_SYS_REMAP_MASK = crate::Reg<u32, _AZ_SYS_REMAP_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_SYS_REMAP_MASK;
#[doc = "`read()` method returns [az_sys_remap_mask::R](az_sys_remap_mask::R) reader structure"]
impl crate::Readable for AZ_SYS_REMAP_MASK {}
#[doc = "`write(|w| ..)` method takes [az_sys_remap_mask::W](az_sys_remap_mask::W) writer structure"]
impl crate::Writable for AZ_SYS_REMAP_MASK {}
#[doc = ""]
pub mod az_sys_remap_mask;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_sys_remap_offset](az_sys_remap_offset) module"]
pub type AZ_SYS_REMAP_OFFSET = crate::Reg<u32, _AZ_SYS_REMAP_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_SYS_REMAP_OFFSET;
#[doc = "`read()` method returns [az_sys_remap_offset::R](az_sys_remap_offset::R) reader structure"]
impl crate::Readable for AZ_SYS_REMAP_OFFSET {}
#[doc = "`write(|w| ..)` method takes [az_sys_remap_offset::W](az_sys_remap_offset::W) writer structure"]
impl crate::Writable for AZ_SYS_REMAP_OFFSET {}
#[doc = ""]
pub mod az_sys_remap_offset;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_ctrl](az_ctrl) module"]
pub type AZ_CTRL = crate::Reg<u32, _AZ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_CTRL;
#[doc = "`read()` method returns [az_ctrl::R](az_ctrl::R) reader structure"]
impl crate::Readable for AZ_CTRL {}
#[doc = "`write(|w| ..)` method takes [az_ctrl::W](az_ctrl::W) writer structure"]
impl crate::Writable for AZ_CTRL {}
#[doc = ""]
pub mod az_ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sse_otp_rd_data](sse_otp_rd_data) module"]
pub type SSE_OTP_RD_DATA = crate::Reg<u32, _SSE_OTP_RD_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSE_OTP_RD_DATA;
#[doc = "`read()` method returns [sse_otp_rd_data::R](sse_otp_rd_data::R) reader structure"]
impl crate::Readable for SSE_OTP_RD_DATA {}
#[doc = ""]
pub mod sse_otp_rd_data;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [az_otp_rd_data](az_otp_rd_data) module"]
pub type AZ_OTP_RD_DATA = crate::Reg<u32, _AZ_OTP_RD_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AZ_OTP_RD_DATA;
#[doc = "`read()` method returns [az_otp_rd_data::R](az_otp_rd_data::R) reader structure"]
impl crate::Readable for AZ_OTP_RD_DATA {}
#[doc = "`write(|w| ..)` method takes [az_otp_rd_data::W](az_otp_rd_data::W) writer structure"]
impl crate::Writable for AZ_OTP_RD_DATA {}
#[doc = ""]
pub mod az_otp_rd_data;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spare_ctrl0](spare_ctrl0) module"]
pub type SPARE_CTRL0 = crate::Reg<u32, _SPARE_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_CTRL0;
#[doc = "`read()` method returns [spare_ctrl0::R](spare_ctrl0::R) reader structure"]
impl crate::Readable for SPARE_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [spare_ctrl0::W](spare_ctrl0::W) writer structure"]
impl crate::Writable for SPARE_CTRL0 {}
#[doc = ""]
pub mod spare_ctrl0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spare_ctrl1](spare_ctrl1) module"]
pub type SPARE_CTRL1 = crate::Reg<u32, _SPARE_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_CTRL1;
#[doc = "`read()` method returns [spare_ctrl1::R](spare_ctrl1::R) reader structure"]
impl crate::Readable for SPARE_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [spare_ctrl1::W](spare_ctrl1::W) writer structure"]
impl crate::Writable for SPARE_CTRL1 {}
#[doc = ""]
pub mod spare_ctrl1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chip_id](chip_id) module"]
pub type CHIP_ID = crate::Reg<u32, _CHIP_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIP_ID;
#[doc = "`read()` method returns [chip_id::R](chip_id::R) reader structure"]
impl crate::Readable for CHIP_ID {}
#[doc = ""]
pub mod chip_id;
