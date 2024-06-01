#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    pllcfg: Pllcfg,
    cfg: Cfg,
    clkint: Clkint,
    ahbrst1: Ahbrst1,
    ahbrst2: Ahbrst2,
    ahbrst3: Ahbrst3,
    _reserved7: [u8; 0x04],
    apb1rst: Apb1rst,
    apb2rst: Apb2rst,
    _reserved9: [u8; 0x08],
    ahben1: Ahben1,
    ahben2: Ahben2,
    ahben3: Ahben3,
    _reserved12: [u8; 0x04],
    apb1en: Apb1en,
    apb2en: Apb2en,
    _reserved14: [u8; 0x08],
    ahblpen1: Ahblpen1,
    ahblpen2: Ahblpen2,
    ahblpen3: Ahblpen3,
    _reserved17: [u8; 0x04],
    apb1lpen: Apb1lpen,
    apb2lpen: Apb2lpen,
    _reserved19: [u8; 0x08],
    bpdc: Bpdc,
    ctrlsts: Ctrlsts,
    _reserved21: [u8; 0x28],
    misc1: Misc1,
    misc2: Misc2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - PLL configuration register (CRM_PLLCFG)"]
    #[inline(always)]
    pub const fn pllcfg(&self) -> &Pllcfg {
        &self.pllcfg
    }
    #[doc = "0x08 - Clock configuration register(CRM_CFG)"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x0c - Clock interrupt register (CRM_CLKINT)"]
    #[inline(always)]
    pub const fn clkint(&self) -> &Clkint {
        &self.clkint
    }
    #[doc = "0x10 - AHB peripheral reset register1 (CRM_AHBRST1)"]
    #[inline(always)]
    pub const fn ahbrst1(&self) -> &Ahbrst1 {
        &self.ahbrst1
    }
    #[doc = "0x14 - AHB peripheral reset register 2 (CRM_AHBRST2)"]
    #[inline(always)]
    pub const fn ahbrst2(&self) -> &Ahbrst2 {
        &self.ahbrst2
    }
    #[doc = "0x18 - AHB peripheral reset register 3 (CRM_AHBRST3)"]
    #[inline(always)]
    pub const fn ahbrst3(&self) -> &Ahbrst3 {
        &self.ahbrst3
    }
    #[doc = "0x20 - APB1 peripheral reset register (CRM_APB1RST)"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &Apb1rst {
        &self.apb1rst
    }
    #[doc = "0x24 - APB2 peripheral reset register (CRM_APB2RST)"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &Apb2rst {
        &self.apb2rst
    }
    #[doc = "0x30 - AHB Peripheral Clock enable register 1 (CRM_AHBEN1)"]
    #[inline(always)]
    pub const fn ahben1(&self) -> &Ahben1 {
        &self.ahben1
    }
    #[doc = "0x34 - AHB peripheral clock enable register 2 (CRM_AHBEN2)"]
    #[inline(always)]
    pub const fn ahben2(&self) -> &Ahben2 {
        &self.ahben2
    }
    #[doc = "0x38 - AHB peripheral clock enable register 3 (CRM_AHBEN3)"]
    #[inline(always)]
    pub const fn ahben3(&self) -> &Ahben3 {
        &self.ahben3
    }
    #[doc = "0x40 - APB1 peripheral clock enable register (CRM_APB1EN)"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &Apb1en {
        &self.apb1en
    }
    #[doc = "0x44 - APB2 peripheral clock enable register (CRM_APB2EN)"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &Apb2en {
        &self.apb2en
    }
    #[doc = "0x50 - AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)"]
    #[inline(always)]
    pub const fn ahblpen1(&self) -> &Ahblpen1 {
        &self.ahblpen1
    }
    #[doc = "0x54 - AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)"]
    #[inline(always)]
    pub const fn ahblpen2(&self) -> &Ahblpen2 {
        &self.ahblpen2
    }
    #[doc = "0x58 - AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)"]
    #[inline(always)]
    pub const fn ahblpen3(&self) -> &Ahblpen3 {
        &self.ahblpen3
    }
    #[doc = "0x60 - APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)"]
    #[inline(always)]
    pub const fn apb1lpen(&self) -> &Apb1lpen {
        &self.apb1lpen
    }
    #[doc = "0x64 - APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)"]
    #[inline(always)]
    pub const fn apb2lpen(&self) -> &Apb2lpen {
        &self.apb2lpen
    }
    #[doc = "0x70 - Battery powered domain control register (CRM_BPDC)"]
    #[inline(always)]
    pub const fn bpdc(&self) -> &Bpdc {
        &self.bpdc
    }
    #[doc = "0x74 - Control/status register (CRM_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
    #[doc = "0xa0 - Miscellaneous register1"]
    #[inline(always)]
    pub const fn misc1(&self) -> &Misc1 {
        &self.misc1
    }
    #[doc = "0xa4 - Miscellaneous register2"]
    #[inline(always)]
    pub const fn misc2(&self) -> &Misc2 {
        &self.misc2
    }
}
#[doc = "CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Clock control register"]
pub mod ctrl;
#[doc = "PLLCFG (rw) register accessor: PLL configuration register (CRM_PLLCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfg`]
module"]
#[doc(alias = "PLLCFG")]
pub type Pllcfg = crate::Reg<pllcfg::PllcfgSpec>;
#[doc = "PLL configuration register (CRM_PLLCFG)"]
pub mod pllcfg;
#[doc = "CFG (rw) register accessor: Clock configuration register(CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Clock configuration register(CRM_CFG)"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkint`]
module"]
#[doc(alias = "CLKINT")]
pub type Clkint = crate::Reg<clkint::ClkintSpec>;
#[doc = "Clock interrupt register (CRM_CLKINT)"]
pub mod clkint;
#[doc = "AHBRST1 (rw) register accessor: AHB peripheral reset register1 (CRM_AHBRST1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst1`]
module"]
#[doc(alias = "AHBRST1")]
pub type Ahbrst1 = crate::Reg<ahbrst1::Ahbrst1Spec>;
#[doc = "AHB peripheral reset register1 (CRM_AHBRST1)"]
pub mod ahbrst1;
#[doc = "AHBRST2 (rw) register accessor: AHB peripheral reset register 2 (CRM_AHBRST2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst2`]
module"]
#[doc(alias = "AHBRST2")]
pub type Ahbrst2 = crate::Reg<ahbrst2::Ahbrst2Spec>;
#[doc = "AHB peripheral reset register 2 (CRM_AHBRST2)"]
pub mod ahbrst2;
#[doc = "AHBRST3 (rw) register accessor: AHB peripheral reset register 3 (CRM_AHBRST3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst3`]
module"]
#[doc(alias = "AHBRST3")]
pub type Ahbrst3 = crate::Reg<ahbrst3::Ahbrst3Spec>;
#[doc = "AHB peripheral reset register 3 (CRM_AHBRST3)"]
pub mod ahbrst3;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
#[doc(alias = "APB1RST")]
pub type Apb1rst = crate::Reg<apb1rst::Apb1rstSpec>;
#[doc = "APB1 peripheral reset register (CRM_APB1RST)"]
pub mod apb1rst;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
#[doc(alias = "APB2RST")]
pub type Apb2rst = crate::Reg<apb2rst::Apb2rstSpec>;
#[doc = "APB2 peripheral reset register (CRM_APB2RST)"]
pub mod apb2rst;
#[doc = "AHBEN1 (rw) register accessor: AHB Peripheral Clock enable register 1 (CRM_AHBEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben1`]
module"]
#[doc(alias = "AHBEN1")]
pub type Ahben1 = crate::Reg<ahben1::Ahben1Spec>;
#[doc = "AHB Peripheral Clock enable register 1 (CRM_AHBEN1)"]
pub mod ahben1;
#[doc = "AHBEN2 (rw) register accessor: AHB peripheral clock enable register 2 (CRM_AHBEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben2`]
module"]
#[doc(alias = "AHBEN2")]
pub type Ahben2 = crate::Reg<ahben2::Ahben2Spec>;
#[doc = "AHB peripheral clock enable register 2 (CRM_AHBEN2)"]
pub mod ahben2;
#[doc = "AHBEN3 (rw) register accessor: AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben3`]
module"]
#[doc(alias = "AHBEN3")]
pub type Ahben3 = crate::Reg<ahben3::Ahben3Spec>;
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)"]
pub mod ahben3;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
#[doc(alias = "APB1EN")]
pub type Apb1en = crate::Reg<apb1en::Apb1enSpec>;
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)"]
pub mod apb1en;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
#[doc(alias = "APB2EN")]
pub type Apb2en = crate::Reg<apb2en::Apb2enSpec>;
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)"]
pub mod apb2en;
#[doc = "AHBLPEN1 (rw) register accessor: AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblpen1`]
module"]
#[doc(alias = "AHBLPEN1")]
pub type Ahblpen1 = crate::Reg<ahblpen1::Ahblpen1Spec>;
#[doc = "AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)"]
pub mod ahblpen1;
#[doc = "AHBLPEN2 (rw) register accessor: AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblpen2`]
module"]
#[doc(alias = "AHBLPEN2")]
pub type Ahblpen2 = crate::Reg<ahblpen2::Ahblpen2Spec>;
#[doc = "AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)"]
pub mod ahblpen2;
#[doc = "AHBLPEN3 (rw) register accessor: AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblpen3`]
module"]
#[doc(alias = "AHBLPEN3")]
pub type Ahblpen3 = crate::Reg<ahblpen3::Ahblpen3Spec>;
#[doc = "AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)"]
pub mod ahblpen3;
#[doc = "APB1LPEN (rw) register accessor: APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lpen`]
module"]
#[doc(alias = "APB1LPEN")]
pub type Apb1lpen = crate::Reg<apb1lpen::Apb1lpenSpec>;
#[doc = "APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)"]
pub mod apb1lpen;
#[doc = "APB2LPEN (rw) register accessor: APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpen`]
module"]
#[doc(alias = "APB2LPEN")]
pub type Apb2lpen = crate::Reg<apb2lpen::Apb2lpenSpec>;
#[doc = "APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)"]
pub mod apb2lpen;
#[doc = "BPDC (rw) register accessor: Battery powered domain control register (CRM_BPDC)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpdc`]
module"]
#[doc(alias = "BPDC")]
pub type Bpdc = crate::Reg<bpdc::BpdcSpec>;
#[doc = "Battery powered domain control register (CRM_BPDC)"]
pub mod bpdc;
#[doc = "CTRLSTS (rw) register accessor: Control/status register (CRM_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
#[doc(alias = "CTRLSTS")]
pub type Ctrlsts = crate::Reg<ctrlsts::CtrlstsSpec>;
#[doc = "Control/status register (CRM_CTRLSTS)"]
pub mod ctrlsts;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc1`]
module"]
#[doc(alias = "MISC1")]
pub type Misc1 = crate::Reg<misc1::Misc1Spec>;
#[doc = "Miscellaneous register1"]
pub mod misc1;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc2`]
module"]
#[doc(alias = "MISC2")]
pub type Misc2 = crate::Reg<misc2::Misc2Spec>;
#[doc = "Miscellaneous register2"]
pub mod misc2;
