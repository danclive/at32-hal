#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    ctrlsts: Ctrlsts,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Power control and status register"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
}
#[doc = "CTRL (rw) register accessor: Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Power control register"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: Power control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
#[doc(alias = "CTRLSTS")]
pub type Ctrlsts = crate::Reg<ctrlsts::CtrlstsSpec>;
#[doc = "Power control and status register"]
pub mod ctrlsts;
