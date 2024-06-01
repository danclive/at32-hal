#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    stctrl: Stctrl,
    iden: Iden,
    ists: Ists,
    swevt: Swevt,
    _reserved_6_cm1: [u8; 0x04],
    _reserved_7_cm2: [u8; 0x04],
    cctrl: Cctrl,
    cval: Cval,
    div: Div,
    pr: Pr,
    _reserved12: [u8; 0x04],
    c1dt: C1dt,
    c2dt: C2dt,
    c3dt: C3dt,
    c4dt: C4dt,
    _reserved16: [u8; 0x04],
    dmactrl: Dmactrl,
    dmadt: Dmadt,
    tmr5_rmp: Tmr5Rmp,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x08 - Subordinate TMR control register"]
    #[inline(always)]
    pub const fn stctrl(&self) -> &Stctrl {
        &self.stctrl
    }
    #[doc = "0x0c - Interrupt/DMA enable register"]
    #[inline(always)]
    pub const fn iden(&self) -> &Iden {
        &self.iden
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn ists(&self) -> &Ists {
        &self.ists
    }
    #[doc = "0x14 - Software event register"]
    #[inline(always)]
    pub const fn swevt(&self) -> &Swevt {
        &self.swevt
    }
    #[doc = "0x18 - Channel input mode register 1"]
    #[inline(always)]
    pub const fn cm1_input(&self) -> &Cm1Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Channel output mode register"]
    #[inline(always)]
    pub const fn cm1_output(&self) -> &Cm1Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Channel input mode register 2"]
    #[inline(always)]
    pub const fn cm2_input(&self) -> &Cm2Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Channel output mode register 2"]
    #[inline(always)]
    pub const fn cm2_output(&self) -> &Cm2Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Channel control register"]
    #[inline(always)]
    pub const fn cctrl(&self) -> &Cctrl {
        &self.cctrl
    }
    #[doc = "0x24 - Counter value"]
    #[inline(always)]
    pub const fn cval(&self) -> &Cval {
        &self.cval
    }
    #[doc = "0x28 - Divider value"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
    #[doc = "0x2c - Period value"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x34 - Channel 1 data register"]
    #[inline(always)]
    pub const fn c1dt(&self) -> &C1dt {
        &self.c1dt
    }
    #[doc = "0x38 - Channel 2 data register"]
    #[inline(always)]
    pub const fn c2dt(&self) -> &C2dt {
        &self.c2dt
    }
    #[doc = "0x3c - Channel 3 data register"]
    #[inline(always)]
    pub const fn c3dt(&self) -> &C3dt {
        &self.c3dt
    }
    #[doc = "0x40 - Channel 4 data register"]
    #[inline(always)]
    pub const fn c4dt(&self) -> &C4dt {
        &self.c4dt
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    #[doc = "0x4c - DMA data register"]
    #[inline(always)]
    pub const fn dmadt(&self) -> &Dmadt {
        &self.dmadt
    }
    #[doc = "0x50 - TMR5 channel input remap register"]
    #[inline(always)]
    pub const fn tmr5_rmp(&self) -> &Tmr5Rmp {
        &self.tmr5_rmp
    }
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "STCTRL (rw) register accessor: Subordinate TMR control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stctrl`]
module"]
#[doc(alias = "STCTRL")]
pub type Stctrl = crate::Reg<stctrl::StctrlSpec>;
#[doc = "Subordinate TMR control register"]
pub mod stctrl;
#[doc = "IDEN (rw) register accessor: Interrupt/DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iden::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iden::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iden`]
module"]
#[doc(alias = "IDEN")]
pub type Iden = crate::Reg<iden::IdenSpec>;
#[doc = "Interrupt/DMA enable register"]
pub mod iden;
#[doc = "ISTS (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ists::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ists`]
module"]
#[doc(alias = "ISTS")]
pub type Ists = crate::Reg<ists::IstsSpec>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "SWEVT (rw) register accessor: Software event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevt`]
module"]
#[doc(alias = "SWEVT")]
pub type Swevt = crate::Reg<swevt::SwevtSpec>;
#[doc = "Software event register"]
pub mod swevt;
#[doc = "CM1_OUTPUT (rw) register accessor: Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm1_output`]
module"]
#[doc(alias = "CM1_OUTPUT")]
pub type Cm1Output = crate::Reg<cm1_output::Cm1OutputSpec>;
#[doc = "Channel output mode register"]
pub mod cm1_output;
#[doc = "CM1_INPUT (rw) register accessor: Channel input mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm1_input`]
module"]
#[doc(alias = "CM1_INPUT")]
pub type Cm1Input = crate::Reg<cm1_input::Cm1InputSpec>;
#[doc = "Channel input mode register 1"]
pub mod cm1_input;
#[doc = "CM2_OUTPUT (rw) register accessor: Channel output mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm2_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm2_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm2_output`]
module"]
#[doc(alias = "CM2_OUTPUT")]
pub type Cm2Output = crate::Reg<cm2_output::Cm2OutputSpec>;
#[doc = "Channel output mode register 2"]
pub mod cm2_output;
#[doc = "CM2_INPUT (rw) register accessor: Channel input mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm2_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm2_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm2_input`]
module"]
#[doc(alias = "CM2_INPUT")]
pub type Cm2Input = crate::Reg<cm2_input::Cm2InputSpec>;
#[doc = "Channel input mode register 2"]
pub mod cm2_input;
#[doc = "CCTRL (rw) register accessor: Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrl`]
module"]
#[doc(alias = "CCTRL")]
pub type Cctrl = crate::Reg<cctrl::CctrlSpec>;
#[doc = "Channel control register"]
pub mod cctrl;
#[doc = "CVAL (rw) register accessor: Counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cval`]
module"]
#[doc(alias = "CVAL")]
pub type Cval = crate::Reg<cval::CvalSpec>;
#[doc = "Counter value"]
pub mod cval;
#[doc = "DIV (rw) register accessor: Divider value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Divider value"]
pub mod div;
#[doc = "PR (rw) register accessor: Period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Period value"]
pub mod pr;
#[doc = "C1DT (rw) register accessor: Channel 1 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1dt`]
module"]
#[doc(alias = "C1DT")]
pub type C1dt = crate::Reg<c1dt::C1dtSpec>;
#[doc = "Channel 1 data register"]
pub mod c1dt;
#[doc = "C2DT (rw) register accessor: Channel 2 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2dt`]
module"]
#[doc(alias = "C2DT")]
pub type C2dt = crate::Reg<c2dt::C2dtSpec>;
#[doc = "Channel 2 data register"]
pub mod c2dt;
#[doc = "C3DT (rw) register accessor: Channel 3 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3dt`]
module"]
#[doc(alias = "C3DT")]
pub type C3dt = crate::Reg<c3dt::C3dtSpec>;
#[doc = "Channel 3 data register"]
pub mod c3dt;
#[doc = "C4DT (rw) register accessor: Channel 4 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4dt`]
module"]
#[doc(alias = "C4DT")]
pub type C4dt = crate::Reg<c4dt::C4dtSpec>;
#[doc = "Channel 4 data register"]
pub mod c4dt;
#[doc = "DMACTRL (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`]
module"]
#[doc(alias = "DMACTRL")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
#[doc = "DMA control register"]
pub mod dmactrl;
#[doc = "DMADT (rw) register accessor: DMA data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmadt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmadt`]
module"]
#[doc(alias = "DMADT")]
pub type Dmadt = crate::Reg<dmadt::DmadtSpec>;
#[doc = "DMA data register"]
pub mod dmadt;
#[doc = "TMR5_RMP (rw) register accessor: TMR5 channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr5_rmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr5_rmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr5_rmp`]
module"]
#[doc(alias = "TMR5_RMP")]
pub type Tmr5Rmp = crate::Reg<tmr5_rmp::Tmr5RmpSpec>;
#[doc = "TMR5 channel input remap register"]
pub mod tmr5_rmp;
