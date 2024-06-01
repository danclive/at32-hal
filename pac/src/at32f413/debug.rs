#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - DEBUG_IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - DEBUG_CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "IDCODE (r) register accessor: DEBUG_IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`]
module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "DEBUG_IDCODE"]
pub mod idcode;
#[doc = "CTRL (rw) register accessor: DEBUG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DEBUG_CTRL"]
pub mod ctrl;
