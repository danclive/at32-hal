#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    debug_idcode: DebugIdcode,
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - DEBUG IDCODE"]
    #[inline(always)]
    pub const fn debug_idcode(&self) -> &DebugIdcode {
        &self.debug_idcode
    }
    #[doc = "0x04 - MCUDBG_CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "DEBUG_IDCODE (r) register accessor: DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_idcode`]
module"]
#[doc(alias = "DEBUG_IDCODE")]
pub type DebugIdcode = crate::Reg<debug_idcode::DebugIdcodeSpec>;
#[doc = "DEBUG IDCODE"]
pub mod debug_idcode;
#[doc = "CTRL (rw) register accessor: MCUDBG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "MCUDBG_CTRL"]
pub mod ctrl;
