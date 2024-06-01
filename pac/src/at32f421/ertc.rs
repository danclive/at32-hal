#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    time: Time,
    date: Date,
    ctrl: Ctrl,
    sts: Sts,
    div: Div,
    _reserved5: [u8; 0x08],
    ala: Ala,
    _reserved6: [u8; 0x04],
    wp: Wp,
    sbs: Sbs,
    tadj: Tadj,
    tstm: Tstm,
    tsdt: Tsdt,
    tssbs: Tssbs,
    scal: Scal,
    tamp: Tamp,
    alasbs: Alasbs,
    _reserved15: [u8; 0x08],
    bpr1dt: Bpr1dt,
    bpr2dt: Bpr2dt,
    bpr3dt: Bpr3dt,
    bpr4dt: Bpr4dt,
    bpr5dt: Bpr5dt,
}
impl RegisterBlock {
    #[doc = "0x00 - time register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x04 - date register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x08 - control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - initialization and status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x10 - Diveder register"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
    #[doc = "0x1c - Alarm A register"]
    #[inline(always)]
    pub const fn ala(&self) -> &Ala {
        &self.ala
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn wp(&self) -> &Wp {
        &self.wp
    }
    #[doc = "0x28 - sub second register"]
    #[inline(always)]
    pub const fn sbs(&self) -> &Sbs {
        &self.sbs
    }
    #[doc = "0x2c - time adjust register"]
    #[inline(always)]
    pub const fn tadj(&self) -> &Tadj {
        &self.tadj
    }
    #[doc = "0x30 - time stamp time register"]
    #[inline(always)]
    pub const fn tstm(&self) -> &Tstm {
        &self.tstm
    }
    #[doc = "0x34 - timestamp date register"]
    #[inline(always)]
    pub const fn tsdt(&self) -> &Tsdt {
        &self.tsdt
    }
    #[doc = "0x38 - timestamp sub second register"]
    #[inline(always)]
    pub const fn tssbs(&self) -> &Tssbs {
        &self.tssbs
    }
    #[doc = "0x3c - calibration register"]
    #[inline(always)]
    pub const fn scal(&self) -> &Scal {
        &self.scal
    }
    #[doc = "0x40 - tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn tamp(&self) -> &Tamp {
        &self.tamp
    }
    #[doc = "0x44 - alarm A sub second register"]
    #[inline(always)]
    pub const fn alasbs(&self) -> &Alasbs {
        &self.alasbs
    }
    #[doc = "0x50 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr1dt(&self) -> &Bpr1dt {
        &self.bpr1dt
    }
    #[doc = "0x54 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr2dt(&self) -> &Bpr2dt {
        &self.bpr2dt
    }
    #[doc = "0x58 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr3dt(&self) -> &Bpr3dt {
        &self.bpr3dt
    }
    #[doc = "0x5c - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr4dt(&self) -> &Bpr4dt {
        &self.bpr4dt
    }
    #[doc = "0x60 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr5dt(&self) -> &Bpr5dt {
        &self.bpr5dt
    }
}
#[doc = "TIME (rw) register accessor: time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "time register"]
pub mod time;
#[doc = "DATE (rw) register accessor: date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`]
module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "date register"]
pub mod date;
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "control register"]
pub mod ctrl;
#[doc = "STS (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "initialization and status register"]
pub mod sts;
#[doc = "DIV (rw) register accessor: Diveder register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Diveder register"]
pub mod div;
#[doc = "ALA (rw) register accessor: Alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ala::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ala::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ala`]
module"]
#[doc(alias = "ALA")]
pub type Ala = crate::Reg<ala::AlaSpec>;
#[doc = "Alarm A register"]
pub mod ala;
#[doc = "WP (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp`]
module"]
#[doc(alias = "WP")]
pub type Wp = crate::Reg<wp::WpSpec>;
#[doc = "write protection register"]
pub mod wp;
#[doc = "SBS (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbs`]
module"]
#[doc(alias = "SBS")]
pub type Sbs = crate::Reg<sbs::SbsSpec>;
#[doc = "sub second register"]
pub mod sbs;
#[doc = "TADJ (w) register accessor: time adjust register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tadj::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tadj`]
module"]
#[doc(alias = "TADJ")]
pub type Tadj = crate::Reg<tadj::TadjSpec>;
#[doc = "time adjust register"]
pub mod tadj;
#[doc = "TSTM (r) register accessor: time stamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstm`]
module"]
#[doc(alias = "TSTM")]
pub type Tstm = crate::Reg<tstm::TstmSpec>;
#[doc = "time stamp time register"]
pub mod tstm;
#[doc = "TSDT (r) register accessor: timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdt`]
module"]
#[doc(alias = "TSDT")]
pub type Tsdt = crate::Reg<tsdt::TsdtSpec>;
#[doc = "timestamp date register"]
pub mod tsdt;
#[doc = "TSSBS (r) register accessor: timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssbs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tssbs`]
module"]
#[doc(alias = "TSSBS")]
pub type Tssbs = crate::Reg<tssbs::TssbsSpec>;
#[doc = "timestamp sub second register"]
pub mod tssbs;
#[doc = "SCAL (rw) register accessor: calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scal`]
module"]
#[doc(alias = "SCAL")]
pub type Scal = crate::Reg<scal::ScalSpec>;
#[doc = "calibration register"]
pub mod scal;
#[doc = "TAMP (rw) register accessor: tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp`]
module"]
#[doc(alias = "TAMP")]
pub type Tamp = crate::Reg<tamp::TampSpec>;
#[doc = "tamper and alternate function configuration register"]
pub mod tamp;
#[doc = "ALASBS (rw) register accessor: alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alasbs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alasbs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alasbs`]
module"]
#[doc(alias = "ALASBS")]
pub type Alasbs = crate::Reg<alasbs::AlasbsSpec>;
#[doc = "alarm A sub second register"]
pub mod alasbs;
#[doc = "BPR1DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr1dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr1dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr1dt`]
module"]
#[doc(alias = "BPR1DT")]
pub type Bpr1dt = crate::Reg<bpr1dt::Bpr1dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr1dt;
#[doc = "BPR2DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr2dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr2dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr2dt`]
module"]
#[doc(alias = "BPR2DT")]
pub type Bpr2dt = crate::Reg<bpr2dt::Bpr2dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr2dt;
#[doc = "BPR3DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr3dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr3dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr3dt`]
module"]
#[doc(alias = "BPR3DT")]
pub type Bpr3dt = crate::Reg<bpr3dt::Bpr3dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr3dt;
#[doc = "BPR4DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr4dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr4dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr4dt`]
module"]
#[doc(alias = "BPR4DT")]
pub type Bpr4dt = crate::Reg<bpr4dt::Bpr4dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr4dt;
#[doc = "BPR5DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr5dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr5dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr5dt`]
module"]
#[doc(alias = "BPR5DT")]
pub type Bpr5dt = crate::Reg<bpr5dt::Bpr5dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr5dt;
