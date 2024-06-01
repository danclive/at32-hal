#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    ctrl: Ctrl,
    apb1_pause: Apb1Pause,
    apb2_pause: Apb2Pause,
    _reserved4: [u8; 0x10],
    ser_id: SerId,
}
impl RegisterBlock {
    #[doc = "0x00 - DEBUG IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - DEBUG CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - DEBUG APB1 PAUSE"]
    #[inline(always)]
    pub const fn apb1_pause(&self) -> &Apb1Pause {
        &self.apb1_pause
    }
    #[doc = "0x0c - DEBUG APB2 PAUSE"]
    #[inline(always)]
    pub const fn apb2_pause(&self) -> &Apb2Pause {
        &self.apb2_pause
    }
    #[doc = "0x20 - SERIES ID"]
    #[inline(always)]
    pub const fn ser_id(&self) -> &SerId {
        &self.ser_id
    }
}
#[doc = "IDCODE (r) register accessor: DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`]
module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "DEBUG IDCODE"]
pub mod idcode;
#[doc = "CTRL (rw) register accessor: DEBUG CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DEBUG CTRL"]
pub mod ctrl;
#[doc = "APB1_PAUSE (rw) register accessor: DEBUG APB1 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_pause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_pause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1_pause`]
module"]
#[doc(alias = "APB1_PAUSE")]
pub type Apb1Pause = crate::Reg<apb1_pause::Apb1PauseSpec>;
#[doc = "DEBUG APB1 PAUSE"]
pub mod apb1_pause;
#[doc = "APB2_PAUSE (rw) register accessor: DEBUG APB2 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2_pause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2_pause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2_pause`]
module"]
#[doc(alias = "APB2_PAUSE")]
pub type Apb2Pause = crate::Reg<apb2_pause::Apb2PauseSpec>;
#[doc = "DEBUG APB2 PAUSE"]
pub mod apb2_pause;
#[doc = "SER_ID (r) register accessor: SERIES ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser_id`]
module"]
#[doc(alias = "SER_ID")]
pub type SerId = crate::Reg<ser_id::SerIdSpec>;
#[doc = "SERIES ID"]
pub mod ser_id;
