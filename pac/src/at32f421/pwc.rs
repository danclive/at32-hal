#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    ctrlsts: Ctrlsts,
    _reserved2: [u8; 0x18],
    ctrl2: Ctrl2,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register (PWC_CTRL)"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Power control and status register (PWC_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
    #[doc = "0x20 - Power control and status register2 (PWC_CTRL2)"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
}
#[doc = "CTRL (rw) register accessor: Power control register (PWC_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Power control register (PWC_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: Power control and status register (PWC_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
#[doc(alias = "CTRLSTS")]
pub type Ctrlsts = crate::Reg<ctrlsts::CtrlstsSpec>;
#[doc = "Power control and status register (PWC_CTRLSTS)"]
pub mod ctrlsts;
#[doc = "CTRL2 (rw) register accessor: Power control and status register2 (PWC_CTRL2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Power control and status register2 (PWC_CTRL2)"]
pub mod ctrl2;
