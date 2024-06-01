#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    time: Time,
    date: Date,
    ctrl: Ctrl,
    sts: Sts,
    div: Div,
    wat: Wat,
    ccal: Ccal,
    ala: Ala,
    alb: Alb,
    wp: Wp,
    sbs: Sbs,
    tadj: Tadj,
    tstm: Tstm,
    tsdt: Tsdt,
    tssbs: Tssbs,
    scal: Scal,
    tamp: Tamp,
    alasbs: Alasbs,
    albsbs: Albsbs,
    _reserved19: [u8; 0x04],
    bpr1dt: Bpr1dt,
    bpr2dt: Bpr2dt,
    bpr3dt: Bpr3dt,
    bpr4dt: Bpr4dt,
    bpr5dt: Bpr5dt,
    bpr6dt: Bpr6dt,
    bpr7dt: Bpr7dt,
    bpr8dt: Bpr8dt,
    bpr9dt: Bpr9dt,
    bpr10dt: Bpr10dt,
    bpr11dt: Bpr11dt,
    bpr12dt: Bpr12dt,
    bpr13dt: Bpr13dt,
    bpr14dt: Bpr14dt,
    bpr15dt: Bpr15dt,
    bpr16dt: Bpr16dt,
    bpr17dt: Bpr17dt,
    bpr18dt: Bpr18dt,
    bpr19dt: Bpr19dt,
    bpr20dt: Bpr20dt,
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
    #[doc = "0x14 - Wakeup timer register"]
    #[inline(always)]
    pub const fn wat(&self) -> &Wat {
        &self.wat
    }
    #[doc = "0x18 - Calibration register"]
    #[inline(always)]
    pub const fn ccal(&self) -> &Ccal {
        &self.ccal
    }
    #[doc = "0x1c - Alarm A register"]
    #[inline(always)]
    pub const fn ala(&self) -> &Ala {
        &self.ala
    }
    #[doc = "0x20 - Alarm B register"]
    #[inline(always)]
    pub const fn alb(&self) -> &Alb {
        &self.alb
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
    #[doc = "0x48 - alarm B sub second register"]
    #[inline(always)]
    pub const fn albsbs(&self) -> &Albsbs {
        &self.albsbs
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
    #[doc = "0x64 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr6dt(&self) -> &Bpr6dt {
        &self.bpr6dt
    }
    #[doc = "0x68 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr7dt(&self) -> &Bpr7dt {
        &self.bpr7dt
    }
    #[doc = "0x6c - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr8dt(&self) -> &Bpr8dt {
        &self.bpr8dt
    }
    #[doc = "0x70 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr9dt(&self) -> &Bpr9dt {
        &self.bpr9dt
    }
    #[doc = "0x74 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr10dt(&self) -> &Bpr10dt {
        &self.bpr10dt
    }
    #[doc = "0x78 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr11dt(&self) -> &Bpr11dt {
        &self.bpr11dt
    }
    #[doc = "0x7c - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr12dt(&self) -> &Bpr12dt {
        &self.bpr12dt
    }
    #[doc = "0x80 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr13dt(&self) -> &Bpr13dt {
        &self.bpr13dt
    }
    #[doc = "0x84 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr14dt(&self) -> &Bpr14dt {
        &self.bpr14dt
    }
    #[doc = "0x88 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr15dt(&self) -> &Bpr15dt {
        &self.bpr15dt
    }
    #[doc = "0x8c - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr16dt(&self) -> &Bpr16dt {
        &self.bpr16dt
    }
    #[doc = "0x90 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr17dt(&self) -> &Bpr17dt {
        &self.bpr17dt
    }
    #[doc = "0x94 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr18dt(&self) -> &Bpr18dt {
        &self.bpr18dt
    }
    #[doc = "0x98 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr19dt(&self) -> &Bpr19dt {
        &self.bpr19dt
    }
    #[doc = "0x9c - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr20dt(&self) -> &Bpr20dt {
        &self.bpr20dt
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
#[doc = "WAT (rw) register accessor: Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wat`]
module"]
#[doc(alias = "WAT")]
pub type Wat = crate::Reg<wat::WatSpec>;
#[doc = "Wakeup timer register"]
pub mod wat;
#[doc = "CCAL (rw) register accessor: Calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccal`]
module"]
#[doc(alias = "CCAL")]
pub type Ccal = crate::Reg<ccal::CcalSpec>;
#[doc = "Calibration register"]
pub mod ccal;
#[doc = "ALA (rw) register accessor: Alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ala::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ala::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ala`]
module"]
#[doc(alias = "ALA")]
pub type Ala = crate::Reg<ala::AlaSpec>;
#[doc = "Alarm A register"]
pub mod ala;
#[doc = "ALB (rw) register accessor: Alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alb`]
module"]
#[doc(alias = "ALB")]
pub type Alb = crate::Reg<alb::AlbSpec>;
#[doc = "Alarm B register"]
pub mod alb;
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
#[doc = "ALBSBS (rw) register accessor: alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`albsbs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`albsbs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@albsbs`]
module"]
#[doc(alias = "ALBSBS")]
pub type Albsbs = crate::Reg<albsbs::AlbsbsSpec>;
#[doc = "alarm B sub second register"]
pub mod albsbs;
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
#[doc = "BPR6DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr6dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr6dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr6dt`]
module"]
#[doc(alias = "BPR6DT")]
pub type Bpr6dt = crate::Reg<bpr6dt::Bpr6dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr6dt;
#[doc = "BPR7DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr7dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr7dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr7dt`]
module"]
#[doc(alias = "BPR7DT")]
pub type Bpr7dt = crate::Reg<bpr7dt::Bpr7dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr7dt;
#[doc = "BPR8DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr8dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr8dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr8dt`]
module"]
#[doc(alias = "BPR8DT")]
pub type Bpr8dt = crate::Reg<bpr8dt::Bpr8dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr8dt;
#[doc = "BPR9DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr9dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr9dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr9dt`]
module"]
#[doc(alias = "BPR9DT")]
pub type Bpr9dt = crate::Reg<bpr9dt::Bpr9dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr9dt;
#[doc = "BPR10DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr10dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr10dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr10dt`]
module"]
#[doc(alias = "BPR10DT")]
pub type Bpr10dt = crate::Reg<bpr10dt::Bpr10dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr10dt;
#[doc = "BPR11DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr11dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr11dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr11dt`]
module"]
#[doc(alias = "BPR11DT")]
pub type Bpr11dt = crate::Reg<bpr11dt::Bpr11dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr11dt;
#[doc = "BPR12DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr12dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr12dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr12dt`]
module"]
#[doc(alias = "BPR12DT")]
pub type Bpr12dt = crate::Reg<bpr12dt::Bpr12dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr12dt;
#[doc = "BPR13DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr13dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr13dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr13dt`]
module"]
#[doc(alias = "BPR13DT")]
pub type Bpr13dt = crate::Reg<bpr13dt::Bpr13dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr13dt;
#[doc = "BPR14DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr14dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr14dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr14dt`]
module"]
#[doc(alias = "BPR14DT")]
pub type Bpr14dt = crate::Reg<bpr14dt::Bpr14dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr14dt;
#[doc = "BPR15DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr15dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr15dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr15dt`]
module"]
#[doc(alias = "BPR15DT")]
pub type Bpr15dt = crate::Reg<bpr15dt::Bpr15dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr15dt;
#[doc = "BPR16DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr16dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr16dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr16dt`]
module"]
#[doc(alias = "BPR16DT")]
pub type Bpr16dt = crate::Reg<bpr16dt::Bpr16dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr16dt;
#[doc = "BPR17DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr17dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr17dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr17dt`]
module"]
#[doc(alias = "BPR17DT")]
pub type Bpr17dt = crate::Reg<bpr17dt::Bpr17dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr17dt;
#[doc = "BPR18DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr18dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr18dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr18dt`]
module"]
#[doc(alias = "BPR18DT")]
pub type Bpr18dt = crate::Reg<bpr18dt::Bpr18dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr18dt;
#[doc = "BPR19DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr19dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr19dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr19dt`]
module"]
#[doc(alias = "BPR19DT")]
pub type Bpr19dt = crate::Reg<bpr19dt::Bpr19dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr19dt;
#[doc = "BPR20DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr20dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr20dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpr20dt`]
module"]
#[doc(alias = "BPR20DT")]
pub type Bpr20dt = crate::Reg<bpr20dt::Bpr20dtSpec>;
#[doc = "Battery powered domain register"]
pub mod bpr20dt;
