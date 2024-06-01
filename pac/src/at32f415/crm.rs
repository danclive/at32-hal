#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cfg: Cfg,
    clkint: Clkint,
    apb2rst: Apb2rst,
    apb1rst: Apb1rst,
    ahben: Ahben,
    apb2en: Apb2en,
    apb1en: Apb1en,
    bpdc: Bpdc,
    ctrlsts: Ctrlsts,
    ahbrst: Ahbrst,
    pll: Pll,
    misc1: Misc1,
    _reserved13: [u8; 0x10],
    otg_extctrl: OtgExtctrl,
    _reserved14: [u8; 0x0c],
    misc2: Misc2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Clock configuration register (CRM_CFG)"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Clock interrupt register (CRM_CLKINT)"]
    #[inline(always)]
    pub const fn clkint(&self) -> &Clkint {
        &self.clkint
    }
    #[doc = "0x0c - APB2 peripheral reset register (CRM_APB2RST)"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &Apb2rst {
        &self.apb2rst
    }
    #[doc = "0x10 - APB1 peripheral reset register (CRM_APB1RST)"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &Apb1rst {
        &self.apb1rst
    }
    #[doc = "0x14 - AHB Peripheral Clock enable register (CRM_AHBEN)"]
    #[inline(always)]
    pub const fn ahben(&self) -> &Ahben {
        &self.ahben
    }
    #[doc = "0x18 - APB2 peripheral clock enable register (CRM_APB2EN)"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &Apb2en {
        &self.apb2en
    }
    #[doc = "0x1c - APB1 peripheral clock enable register (CRM_APB1EN)"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &Apb1en {
        &self.apb1en
    }
    #[doc = "0x20 - Battery powered domain control register (CRM_BPDC)"]
    #[inline(always)]
    pub const fn bpdc(&self) -> &Bpdc {
        &self.bpdc
    }
    #[doc = "0x24 - Control/status register (CRM_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
    #[doc = "0x28 - AHB reset register"]
    #[inline(always)]
    pub const fn ahbrst(&self) -> &Ahbrst {
        &self.ahbrst
    }
    #[doc = "0x2c - PLL configuration register (RCC_PLL)"]
    #[inline(always)]
    pub const fn pll(&self) -> &Pll {
        &self.pll
    }
    #[doc = "0x30 - Miscellaneous register1"]
    #[inline(always)]
    pub const fn misc1(&self) -> &Misc1 {
        &self.misc1
    }
    #[doc = "0x44 - OTGFS external ctrl register1"]
    #[inline(always)]
    pub const fn otg_extctrl(&self) -> &OtgExtctrl {
        &self.otg_extctrl
    }
    #[doc = "0x54 - Miscellaneous register2"]
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
#[doc = "CFG (rw) register accessor: Clock configuration register (CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Clock configuration register (CRM_CFG)"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkint`]
module"]
#[doc(alias = "CLKINT")]
pub type Clkint = crate::Reg<clkint::ClkintSpec>;
#[doc = "Clock interrupt register (CRM_CLKINT)"]
pub mod clkint;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
#[doc(alias = "APB2RST")]
pub type Apb2rst = crate::Reg<apb2rst::Apb2rstSpec>;
#[doc = "APB2 peripheral reset register (CRM_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
#[doc(alias = "APB1RST")]
pub type Apb1rst = crate::Reg<apb1rst::Apb1rstSpec>;
#[doc = "APB1 peripheral reset register (CRM_APB1RST)"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB Peripheral Clock enable register (CRM_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`]
module"]
#[doc(alias = "AHBEN")]
pub type Ahben = crate::Reg<ahben::AhbenSpec>;
#[doc = "AHB Peripheral Clock enable register (CRM_AHBEN)"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
#[doc(alias = "APB2EN")]
pub type Apb2en = crate::Reg<apb2en::Apb2enSpec>;
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
#[doc(alias = "APB1EN")]
pub type Apb1en = crate::Reg<apb1en::Apb1enSpec>;
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)"]
pub mod apb1en;
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
#[doc = "AHBRST (rw) register accessor: AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`]
module"]
#[doc(alias = "AHBRST")]
pub type Ahbrst = crate::Reg<ahbrst::AhbrstSpec>;
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "PLL (rw) register accessor: PLL configuration register (RCC_PLL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll`]
module"]
#[doc(alias = "PLL")]
pub type Pll = crate::Reg<pll::PllSpec>;
#[doc = "PLL configuration register (RCC_PLL)"]
pub mod pll;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc1`]
module"]
#[doc(alias = "MISC1")]
pub type Misc1 = crate::Reg<misc1::Misc1Spec>;
#[doc = "Miscellaneous register1"]
pub mod misc1;
#[doc = "OTG_EXTCTRL (rw) register accessor: OTGFS external ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_extctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_extctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_extctrl`]
module"]
#[doc(alias = "OTG_EXTCTRL")]
pub type OtgExtctrl = crate::Reg<otg_extctrl::OtgExtctrlSpec>;
#[doc = "OTGFS external ctrl register1"]
pub mod otg_extctrl;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc2`]
module"]
#[doc(alias = "MISC2")]
pub type Misc2 = crate::Reg<misc2::Misc2Spec>;
#[doc = "Miscellaneous register2"]
pub mod misc2;
