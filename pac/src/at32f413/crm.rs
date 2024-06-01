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
    _reserved10: [u8; 0x08],
    misc1: Misc1,
    _reserved11: [u8; 0x1c],
    misc2: Misc2,
    misc3: Misc3,
    _reserved13: [u8; 0x04],
    intmap: Intmap,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Clock configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Clock interrupt register"]
    #[inline(always)]
    pub const fn clkint(&self) -> &Clkint {
        &self.clkint
    }
    #[doc = "0x0c - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &Apb2rst {
        &self.apb2rst
    }
    #[doc = "0x10 - APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &Apb1rst {
        &self.apb1rst
    }
    #[doc = "0x14 - AHB Peripheral Clock enable register"]
    #[inline(always)]
    pub const fn ahben(&self) -> &Ahben {
        &self.ahben
    }
    #[doc = "0x18 - APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &Apb2en {
        &self.apb2en
    }
    #[doc = "0x1c - APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &Apb1en {
        &self.apb1en
    }
    #[doc = "0x20 - Battery powered domain control register"]
    #[inline(always)]
    pub const fn bpdc(&self) -> &Bpdc {
        &self.bpdc
    }
    #[doc = "0x24 - Control/status register"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
    #[doc = "0x30 - Miscellaneous register 1"]
    #[inline(always)]
    pub const fn misc1(&self) -> &Misc1 {
        &self.misc1
    }
    #[doc = "0x50 - Miscellaneous register 2"]
    #[inline(always)]
    pub const fn misc2(&self) -> &Misc2 {
        &self.misc2
    }
    #[doc = "0x54 - Miscellaneous register 3"]
    #[inline(always)]
    pub const fn misc3(&self) -> &Misc3 {
        &self.misc3
    }
    #[doc = "0x5c - Interrupt remap register"]
    #[inline(always)]
    pub const fn intmap(&self) -> &Intmap {
        &self.intmap
    }
}
#[doc = "CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Clock control register"]
pub mod ctrl;
#[doc = "CFG (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Clock configuration register"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkint`]
module"]
#[doc(alias = "CLKINT")]
pub type Clkint = crate::Reg<clkint::ClkintSpec>;
#[doc = "Clock interrupt register"]
pub mod clkint;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
#[doc(alias = "APB2RST")]
pub type Apb2rst = crate::Reg<apb2rst::Apb2rstSpec>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
#[doc(alias = "APB1RST")]
pub type Apb1rst = crate::Reg<apb1rst::Apb1rstSpec>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB Peripheral Clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`]
module"]
#[doc(alias = "AHBEN")]
pub type Ahben = crate::Reg<ahben::AhbenSpec>;
#[doc = "AHB Peripheral Clock enable register"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
#[doc(alias = "APB2EN")]
pub type Apb2en = crate::Reg<apb2en::Apb2enSpec>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
#[doc(alias = "APB1EN")]
pub type Apb1en = crate::Reg<apb1en::Apb1enSpec>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1en;
#[doc = "BPDC (rw) register accessor: Battery powered domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpdc`]
module"]
#[doc(alias = "BPDC")]
pub type Bpdc = crate::Reg<bpdc::BpdcSpec>;
#[doc = "Battery powered domain control register"]
pub mod bpdc;
#[doc = "CTRLSTS (rw) register accessor: Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
#[doc(alias = "CTRLSTS")]
pub type Ctrlsts = crate::Reg<ctrlsts::CtrlstsSpec>;
#[doc = "Control/status register"]
pub mod ctrlsts;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc1`]
module"]
#[doc(alias = "MISC1")]
pub type Misc1 = crate::Reg<misc1::Misc1Spec>;
#[doc = "Miscellaneous register 1"]
pub mod misc1;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc2`]
module"]
#[doc(alias = "MISC2")]
pub type Misc2 = crate::Reg<misc2::Misc2Spec>;
#[doc = "Miscellaneous register 2"]
pub mod misc2;
#[doc = "MISC3 (rw) register accessor: Miscellaneous register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc3`]
module"]
#[doc(alias = "MISC3")]
pub type Misc3 = crate::Reg<misc3::Misc3Spec>;
#[doc = "Miscellaneous register 3"]
pub mod misc3;
#[doc = "INTMAP (rw) register accessor: Interrupt remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap`]
module"]
#[doc(alias = "INTMAP")]
pub type Intmap = crate::Reg<intmap::IntmapSpec>;
#[doc = "Interrupt remap register"]
pub mod intmap;
