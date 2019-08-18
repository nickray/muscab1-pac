#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 144usize],
    #[doc = "0x90 - Non-Secure Unprivileged Access AHB slave Peripheral Protection Control #0"]
    pub ahbnspppc0: AHBNSPPPC0,
    _reserved1: [u8; 12usize],
    #[doc = "0xa0 - Expansion 0 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbnspppcexp0: AHBNSPPPCEXP0,
    #[doc = "0xa4 - Expansion 1 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbnspppcexp1: AHBNSPPPCEXP1,
    #[doc = "0xa8 - Expansion 2 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbnspppcexp2: AHBNSPPPCEXP2,
    #[doc = "0xac - Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    pub ahbnspppcexp3: AHBNSPPPCEXP3,
    #[doc = "0xb0 - Non-Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
    pub apbnspppc0: APBNSPPPC0,
    #[doc = "0xb4 - Non-Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
    pub apbnspppc1: APBNSPPPC1,
    _reserved7: [u8; 8usize],
    #[doc = "0xc0 - Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbnspppcexp0: APBNSPPPCEXP0,
    #[doc = "0xc4 - Expansion 1 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbnspppcexp1: APBNSPPPCEXP1,
    #[doc = "0xc8 - Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbnspppcexp2: APBNSPPPCEXP2,
    #[doc = "0xcc - Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    pub apbnspppcexp3: APBNSPPPCEXP3,
    _reserved11: [u8; 3840usize],
    #[doc = "0xfd0 - Peripheral ID 4"]
    pub pidr4: PIDR4,
    _reserved12: [u8; 12usize],
    #[doc = "0xfe0 - Peripheral ID 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral ID 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral ID 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral ID 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component ID 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component ID 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component ID 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component ID 3"]
    pub cidr3: CIDR3,
}
#[doc = "Non-Secure Unprivileged Access AHB slave Peripheral Protection Control #0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnspppc0](ahbnspppc0) module"]
pub type AHBNSPPPC0 = crate::Reg<u32, _AHBNSPPPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPPC0;
#[doc = "`read()` method returns [ahbnspppc0::R](ahbnspppc0::R) reader structure"]
impl crate::Readable for AHBNSPPPC0 {}
#[doc = "`write(|w| ..)` method takes [ahbnspppc0::W](ahbnspppc0::W) writer structure"]
impl crate::Writable for AHBNSPPPC0 {}
#[doc = "Non-Secure Unprivileged Access AHB slave Peripheral Protection Control #0"]
pub mod ahbnspppc0;
#[doc = "Expansion 0 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnspppcexp0](ahbnspppcexp0) module"]
pub type AHBNSPPPCEXP0 = crate::Reg<u32, _AHBNSPPPCEXP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPPCEXP0;
#[doc = "`read()` method returns [ahbnspppcexp0::R](ahbnspppcexp0::R) reader structure"]
impl crate::Readable for AHBNSPPPCEXP0 {}
#[doc = "`write(|w| ..)` method takes [ahbnspppcexp0::W](ahbnspppcexp0::W) writer structure"]
impl crate::Writable for AHBNSPPPCEXP0 {}
#[doc = "Expansion 0 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp0;
#[doc = "Expansion 1 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnspppcexp1](ahbnspppcexp1) module"]
pub type AHBNSPPPCEXP1 = crate::Reg<u32, _AHBNSPPPCEXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPPCEXP1;
#[doc = "`read()` method returns [ahbnspppcexp1::R](ahbnspppcexp1::R) reader structure"]
impl crate::Readable for AHBNSPPPCEXP1 {}
#[doc = "`write(|w| ..)` method takes [ahbnspppcexp1::W](ahbnspppcexp1::W) writer structure"]
impl crate::Writable for AHBNSPPPCEXP1 {}
#[doc = "Expansion 1 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp1;
#[doc = "Expansion 2 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnspppcexp2](ahbnspppcexp2) module"]
pub type AHBNSPPPCEXP2 = crate::Reg<u32, _AHBNSPPPCEXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPPCEXP2;
#[doc = "`read()` method returns [ahbnspppcexp2::R](ahbnspppcexp2::R) reader structure"]
impl crate::Readable for AHBNSPPPCEXP2 {}
#[doc = "`write(|w| ..)` method takes [ahbnspppcexp2::W](ahbnspppcexp2::W) writer structure"]
impl crate::Writable for AHBNSPPPCEXP2 {}
#[doc = "Expansion 2 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp2;
#[doc = "Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbnspppcexp3](ahbnspppcexp3) module"]
pub type AHBNSPPPCEXP3 = crate::Reg<u32, _AHBNSPPPCEXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBNSPPPCEXP3;
#[doc = "`read()` method returns [ahbnspppcexp3::R](ahbnspppcexp3::R) reader structure"]
impl crate::Readable for AHBNSPPPCEXP3 {}
#[doc = "`write(|w| ..)` method takes [ahbnspppcexp3::W](ahbnspppcexp3::W) writer structure"]
impl crate::Writable for AHBNSPPPCEXP3 {}
#[doc = "Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp3;
#[doc = "Non-Secure Unprivileged Access APB slave Peripheral Protection Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnspppc0](apbnspppc0) module"]
pub type APBNSPPPC0 = crate::Reg<u32, _APBNSPPPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPPC0;
#[doc = "`read()` method returns [apbnspppc0::R](apbnspppc0::R) reader structure"]
impl crate::Readable for APBNSPPPC0 {}
#[doc = "`write(|w| ..)` method takes [apbnspppc0::W](apbnspppc0::W) writer structure"]
impl crate::Writable for APBNSPPPC0 {}
#[doc = "Non-Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
pub mod apbnspppc0;
#[doc = "Non-Secure Unprivileged Access APB slave Peripheral Protection Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnspppc1](apbnspppc1) module"]
pub type APBNSPPPC1 = crate::Reg<u32, _APBNSPPPC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPPC1;
#[doc = "`read()` method returns [apbnspppc1::R](apbnspppc1::R) reader structure"]
impl crate::Readable for APBNSPPPC1 {}
#[doc = "`write(|w| ..)` method takes [apbnspppc1::W](apbnspppc1::W) writer structure"]
impl crate::Writable for APBNSPPPC1 {}
#[doc = "Non-Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
pub mod apbnspppc1;
#[doc = "Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnspppcexp0](apbnspppcexp0) module"]
pub type APBNSPPPCEXP0 = crate::Reg<u32, _APBNSPPPCEXP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPPCEXP0;
#[doc = "`read()` method returns [apbnspppcexp0::R](apbnspppcexp0::R) reader structure"]
impl crate::Readable for APBNSPPPCEXP0 {}
#[doc = "`write(|w| ..)` method takes [apbnspppcexp0::W](apbnspppcexp0::W) writer structure"]
impl crate::Writable for APBNSPPPCEXP0 {}
#[doc = "Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp0;
#[doc = "Expansion 1 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnspppcexp1](apbnspppcexp1) module"]
pub type APBNSPPPCEXP1 = crate::Reg<u32, _APBNSPPPCEXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPPCEXP1;
#[doc = "`read()` method returns [apbnspppcexp1::R](apbnspppcexp1::R) reader structure"]
impl crate::Readable for APBNSPPPCEXP1 {}
#[doc = "`write(|w| ..)` method takes [apbnspppcexp1::W](apbnspppcexp1::W) writer structure"]
impl crate::Writable for APBNSPPPCEXP1 {}
#[doc = "Expansion 1 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp1;
#[doc = "Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnspppcexp2](apbnspppcexp2) module"]
pub type APBNSPPPCEXP2 = crate::Reg<u32, _APBNSPPPCEXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPPCEXP2;
#[doc = "`read()` method returns [apbnspppcexp2::R](apbnspppcexp2::R) reader structure"]
impl crate::Readable for APBNSPPPCEXP2 {}
#[doc = "`write(|w| ..)` method takes [apbnspppcexp2::W](apbnspppcexp2::W) writer structure"]
impl crate::Writable for APBNSPPPCEXP2 {}
#[doc = "Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp2;
#[doc = "Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [apbnspppcexp3](apbnspppcexp3) module"]
pub type APBNSPPPCEXP3 = crate::Reg<u32, _APBNSPPPCEXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBNSPPPCEXP3;
#[doc = "`read()` method returns [apbnspppcexp3::R](apbnspppcexp3::R) reader structure"]
impl crate::Readable for APBNSPPPCEXP3 {}
#[doc = "`write(|w| ..)` method takes [apbnspppcexp3::W](apbnspppcexp3::W) writer structure"]
impl crate::Writable for APBNSPPPCEXP3 {}
#[doc = "Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp3;
#[doc = "Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Peripheral ID 4"]
pub mod pidr4;
#[doc = "Peripheral ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "Peripheral ID 0"]
pub mod pidr0;
#[doc = "Peripheral ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "Peripheral ID 1"]
pub mod pidr1;
#[doc = "Peripheral ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "Peripheral ID 2"]
pub mod pidr2;
#[doc = "Peripheral ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "Peripheral ID 3"]
pub mod pidr3;
#[doc = "Component ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "Component ID 0"]
pub mod cidr0;
#[doc = "Component ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "Component ID 1"]
pub mod cidr1;
#[doc = "Component ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "Component ID 2"]
pub mod cidr2;
#[doc = "Component ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "Component ID 3"]
pub mod cidr3;
