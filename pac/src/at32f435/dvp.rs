#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    sts: Sts,
    ests: Ests,
    iena: Iena,
    ists: Ists,
    iclr: Iclr,
    scr: Scr,
    sur: Sur,
    cwst: Cwst,
    cwsz: Cwsz,
    dt: Dt,
    _reserved11: [u8; 0x14],
    actrl: Actrl,
    _reserved12: [u8; 0x04],
    hscf: Hscf,
    vscf: Vscf,
    frf: Frf,
    bth: Bth,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x08 - Event status register"]
    #[inline(always)]
    pub const fn ests(&self) -> &Ests {
        &self.ests
    }
    #[doc = "0x0c - interrupt enable register"]
    #[inline(always)]
    pub const fn iena(&self) -> &Iena {
        &self.iena
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn ists(&self) -> &Ists {
        &self.ists
    }
    #[doc = "0x14 - Interrupt clear register"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x18 - Synchronization code register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x1c - Synchronization unmask register"]
    #[inline(always)]
    pub const fn sur(&self) -> &Sur {
        &self.sur
    }
    #[doc = "0x20 - Crop window start"]
    #[inline(always)]
    pub const fn cwst(&self) -> &Cwst {
        &self.cwst
    }
    #[doc = "0x24 - Crop window size"]
    #[inline(always)]
    pub const fn cwsz(&self) -> &Cwsz {
        &self.cwsz
    }
    #[doc = "0x28 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x40 - Advanced Control register"]
    #[inline(always)]
    pub const fn actrl(&self) -> &Actrl {
        &self.actrl
    }
    #[doc = "0x48 - Horizontal scaling control flow"]
    #[inline(always)]
    pub const fn hscf(&self) -> &Hscf {
        &self.hscf
    }
    #[doc = "0x4c - Vertical scaling control flow"]
    #[inline(always)]
    pub const fn vscf(&self) -> &Vscf {
        &self.vscf
    }
    #[doc = "0x50 - Frame rate flow"]
    #[inline(always)]
    pub const fn frf(&self) -> &Frf {
        &self.frf
    }
    #[doc = "0x54 - Binarization threshold"]
    #[inline(always)]
    pub const fn bth(&self) -> &Bth {
        &self.bth
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STS (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "status register"]
pub mod sts;
#[doc = "ESTS (r) register accessor: Event status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ests`]
module"]
#[doc(alias = "ESTS")]
pub type Ests = crate::Reg<ests::EstsSpec>;
#[doc = "Event status register"]
pub mod ests;
#[doc = "IENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iena`]
module"]
#[doc(alias = "IENA")]
pub type Iena = crate::Reg<iena::IenaSpec>;
#[doc = "interrupt enable register"]
pub mod iena;
#[doc = "ISTS (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ists`]
module"]
#[doc(alias = "ISTS")]
pub type Ists = crate::Reg<ists::IstsSpec>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "ICLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register"]
pub mod iclr;
#[doc = "SCR (rw) register accessor: Synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Synchronization code register"]
pub mod scr;
#[doc = "SUR (rw) register accessor: Synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sur`]
module"]
#[doc(alias = "SUR")]
pub type Sur = crate::Reg<sur::SurSpec>;
#[doc = "Synchronization unmask register"]
pub mod sur;
#[doc = "CWST (rw) register accessor: Crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwst`]
module"]
#[doc(alias = "CWST")]
pub type Cwst = crate::Reg<cwst::CwstSpec>;
#[doc = "Crop window start"]
pub mod cwst;
#[doc = "CWSZ (rw) register accessor: Crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwsz`]
module"]
#[doc(alias = "CWSZ")]
pub type Cwsz = crate::Reg<cwsz::CwszSpec>;
#[doc = "Crop window size"]
pub mod cwsz;
#[doc = "DT (r) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "Data register"]
pub mod dt;
#[doc = "ACTRL (rw) register accessor: Advanced Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actrl`]
module"]
#[doc(alias = "ACTRL")]
pub type Actrl = crate::Reg<actrl::ActrlSpec>;
#[doc = "Advanced Control register"]
pub mod actrl;
#[doc = "HSCF (rw) register accessor: Horizontal scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hscf`]
module"]
#[doc(alias = "HSCF")]
pub type Hscf = crate::Reg<hscf::HscfSpec>;
#[doc = "Horizontal scaling control flow"]
pub mod hscf;
#[doc = "VSCF (rw) register accessor: Vertical scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vscf`]
module"]
#[doc(alias = "VSCF")]
pub type Vscf = crate::Reg<vscf::VscfSpec>;
#[doc = "Vertical scaling control flow"]
pub mod vscf;
#[doc = "FRF (rw) register accessor: Frame rate flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frf`]
module"]
#[doc(alias = "FRF")]
pub type Frf = crate::Reg<frf::FrfSpec>;
#[doc = "Frame rate flow"]
pub mod frf;
#[doc = "BTH (rw) register accessor: Binarization threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bth`]
module"]
#[doc(alias = "BTH")]
pub type Bth = crate::Reg<bth::BthSpec>;
#[doc = "Binarization threshold"]
pub mod bth;
