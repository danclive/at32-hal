#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlh: Ctrlh,
    ctrll: Ctrll,
    divh: Divh,
    divl: Divl,
    divcnth: Divcnth,
    divcntl: Divcntl,
    cnth: Cnth,
    cntl: Cntl,
    tah: Tah,
    tal: Tal,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    #[inline(always)]
    pub const fn ctrlh(&self) -> &Ctrlh {
        &self.ctrlh
    }
    #[doc = "0x04 - RTC Control Register Low"]
    #[inline(always)]
    pub const fn ctrll(&self) -> &Ctrll {
        &self.ctrll
    }
    #[doc = "0x08 - RTC Divider Register High"]
    #[inline(always)]
    pub const fn divh(&self) -> &Divh {
        &self.divh
    }
    #[doc = "0x0c - RTC Divider Register Low"]
    #[inline(always)]
    pub const fn divl(&self) -> &Divl {
        &self.divl
    }
    #[doc = "0x10 - RTC Divider Register High"]
    #[inline(always)]
    pub const fn divcnth(&self) -> &Divcnth {
        &self.divcnth
    }
    #[doc = "0x14 - RTC Divider Register Low"]
    #[inline(always)]
    pub const fn divcntl(&self) -> &Divcntl {
        &self.divcntl
    }
    #[doc = "0x18 - RTC Counter Register High"]
    #[inline(always)]
    pub const fn cnth(&self) -> &Cnth {
        &self.cnth
    }
    #[doc = "0x1c - RTC Counter Register Low"]
    #[inline(always)]
    pub const fn cntl(&self) -> &Cntl {
        &self.cntl
    }
    #[doc = "0x20 - RTC Alarm Register High"]
    #[inline(always)]
    pub const fn tah(&self) -> &Tah {
        &self.tah
    }
    #[doc = "0x24 - Time alarm register low"]
    #[inline(always)]
    pub const fn tal(&self) -> &Tal {
        &self.tal
    }
}
#[doc = "CTRLH (rw) register accessor: RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlh`]
module"]
#[doc(alias = "CTRLH")]
pub type Ctrlh = crate::Reg<ctrlh::CtrlhSpec>;
#[doc = "RTC Control Register High"]
pub mod ctrlh;
#[doc = "CTRLL (rw) register accessor: RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrll`]
module"]
#[doc(alias = "CTRLL")]
pub type Ctrll = crate::Reg<ctrll::CtrllSpec>;
#[doc = "RTC Control Register Low"]
pub mod ctrll;
#[doc = "DIVH (w) register accessor: RTC Divider Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`]
module"]
#[doc(alias = "DIVH")]
pub type Divh = crate::Reg<divh::DivhSpec>;
#[doc = "RTC Divider Register High"]
pub mod divh;
#[doc = "DIVL (w) register accessor: RTC Divider Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`]
module"]
#[doc(alias = "DIVL")]
pub type Divl = crate::Reg<divl::DivlSpec>;
#[doc = "RTC Divider Register Low"]
pub mod divl;
#[doc = "DIVCNTH (rw) register accessor: RTC Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcnth`]
module"]
#[doc(alias = "DIVCNTH")]
pub type Divcnth = crate::Reg<divcnth::DivcnthSpec>;
#[doc = "RTC Divider Register High"]
pub mod divcnth;
#[doc = "DIVCNTL (rw) register accessor: RTC Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcntl`]
module"]
#[doc(alias = "DIVCNTL")]
pub type Divcntl = crate::Reg<divcntl::DivcntlSpec>;
#[doc = "RTC Divider Register Low"]
pub mod divcntl;
#[doc = "CNTH (rw) register accessor: RTC Counter Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
#[doc(alias = "CNTH")]
pub type Cnth = crate::Reg<cnth::CnthSpec>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC Counter Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
#[doc(alias = "CNTL")]
pub type Cntl = crate::Reg<cntl::CntlSpec>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "TAH (w) register accessor: RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tah::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tah`]
module"]
#[doc(alias = "TAH")]
pub type Tah = crate::Reg<tah::TahSpec>;
#[doc = "RTC Alarm Register High"]
pub mod tah;
#[doc = "TAL (w) register accessor: Time alarm register low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tal`]
module"]
#[doc(alias = "TAL")]
pub type Tal = crate::Reg<tal::TalSpec>;
#[doc = "Time alarm register low"]
pub mod tal;
