#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd_w0: CmdW0,
    cmd_w1: CmdW1,
    cmd_w2: CmdW2,
    cmd_w3: CmdW3,
    ctrl: Ctrl,
    _reserved5: [u8; 0x04],
    fifosts: Fifosts,
    _reserved6: [u8; 0x04],
    ctrl2: Ctrl2,
    cmdsts: Cmdsts,
    rsts: Rsts,
    fsize: Fsize,
    xip_cmd_w0: XipCmdW0,
    xip_cmd_w1: XipCmdW1,
    xip_cmd_w2: XipCmdW2,
    xip_cmd_w3: XipCmdW3,
    _reserved14: [u8; 0x10],
    rev: Rev,
    _reserved15: [u8; 0xac],
    dt: Dt,
}
impl RegisterBlock {
    #[doc = "0x00 - Command word 0"]
    #[inline(always)]
    pub const fn cmd_w0(&self) -> &CmdW0 {
        &self.cmd_w0
    }
    #[doc = "0x04 - Command word 1"]
    #[inline(always)]
    pub const fn cmd_w1(&self) -> &CmdW1 {
        &self.cmd_w1
    }
    #[doc = "0x08 - Command word 2"]
    #[inline(always)]
    pub const fn cmd_w2(&self) -> &CmdW2 {
        &self.cmd_w2
    }
    #[doc = "0x0c - Command word 3"]
    #[inline(always)]
    pub const fn cmd_w3(&self) -> &CmdW3 {
        &self.cmd_w3
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x18 - FIFO Status register"]
    #[inline(always)]
    pub const fn fifosts(&self) -> &Fifosts {
        &self.fifosts
    }
    #[doc = "0x20 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x24 - CMD status register"]
    #[inline(always)]
    pub const fn cmdsts(&self) -> &Cmdsts {
        &self.cmdsts
    }
    #[doc = "0x28 - SPI read status register"]
    #[inline(always)]
    pub const fn rsts(&self) -> &Rsts {
        &self.rsts
    }
    #[doc = "0x2c - SPI flash size"]
    #[inline(always)]
    pub const fn fsize(&self) -> &Fsize {
        &self.fsize
    }
    #[doc = "0x30 - XIP command word 0"]
    #[inline(always)]
    pub const fn xip_cmd_w0(&self) -> &XipCmdW0 {
        &self.xip_cmd_w0
    }
    #[doc = "0x34 - XIP command word 1"]
    #[inline(always)]
    pub const fn xip_cmd_w1(&self) -> &XipCmdW1 {
        &self.xip_cmd_w1
    }
    #[doc = "0x38 - XIP command word 2"]
    #[inline(always)]
    pub const fn xip_cmd_w2(&self) -> &XipCmdW2 {
        &self.xip_cmd_w2
    }
    #[doc = "0x3c - XIP command word 3"]
    #[inline(always)]
    pub const fn xip_cmd_w3(&self) -> &XipCmdW3 {
        &self.xip_cmd_w3
    }
    #[doc = "0x50 - Revision"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x100 - 32/16/8 bit data port register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
}
#[doc = "CMD_W0 (rw) register accessor: Command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w0`]
module"]
#[doc(alias = "CMD_W0")]
pub type CmdW0 = crate::Reg<cmd_w0::CmdW0Spec>;
#[doc = "Command word 0"]
pub mod cmd_w0;
#[doc = "CMD_W1 (rw) register accessor: Command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w1`]
module"]
#[doc(alias = "CMD_W1")]
pub type CmdW1 = crate::Reg<cmd_w1::CmdW1Spec>;
#[doc = "Command word 1"]
pub mod cmd_w1;
#[doc = "CMD_W2 (rw) register accessor: Command word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w2`]
module"]
#[doc(alias = "CMD_W2")]
pub type CmdW2 = crate::Reg<cmd_w2::CmdW2Spec>;
#[doc = "Command word 2"]
pub mod cmd_w2;
#[doc = "CMD_W3 (rw) register accessor: Command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w3`]
module"]
#[doc(alias = "CMD_W3")]
pub type CmdW3 = crate::Reg<cmd_w3::CmdW3Spec>;
#[doc = "Command word 3"]
pub mod cmd_w3;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "FIFOSTS (r) register accessor: FIFO Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifosts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifosts`]
module"]
#[doc(alias = "FIFOSTS")]
pub type Fifosts = crate::Reg<fifosts::FifostsSpec>;
#[doc = "FIFO Status register"]
pub mod fifosts;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "CMDSTS (r) register accessor: CMD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdsts`]
module"]
#[doc(alias = "CMDSTS")]
pub type Cmdsts = crate::Reg<cmdsts::CmdstsSpec>;
#[doc = "CMD status register"]
pub mod cmdsts;
#[doc = "RSTS (r) register accessor: SPI read status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsts`]
module"]
#[doc(alias = "RSTS")]
pub type Rsts = crate::Reg<rsts::RstsSpec>;
#[doc = "SPI read status register"]
pub mod rsts;
#[doc = "FSIZE (rw) register accessor: SPI flash size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsize`]
module"]
#[doc(alias = "FSIZE")]
pub type Fsize = crate::Reg<fsize::FsizeSpec>;
#[doc = "SPI flash size"]
pub mod fsize;
#[doc = "XIP_CMD_W0 (rw) register accessor: XIP command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w0`]
module"]
#[doc(alias = "XIP_CMD_W0")]
pub type XipCmdW0 = crate::Reg<xip_cmd_w0::XipCmdW0Spec>;
#[doc = "XIP command word 0"]
pub mod xip_cmd_w0;
#[doc = "XIP_CMD_W1 (rw) register accessor: XIP command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w1`]
module"]
#[doc(alias = "XIP_CMD_W1")]
pub type XipCmdW1 = crate::Reg<xip_cmd_w1::XipCmdW1Spec>;
#[doc = "XIP command word 1"]
pub mod xip_cmd_w1;
#[doc = "XIP_CMD_W2 (rw) register accessor: XIP command word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w2`]
module"]
#[doc(alias = "XIP_CMD_W2")]
pub type XipCmdW2 = crate::Reg<xip_cmd_w2::XipCmdW2Spec>;
#[doc = "XIP command word 2"]
pub mod xip_cmd_w2;
#[doc = "XIP_CMD_W3 (rw) register accessor: XIP command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w3`]
module"]
#[doc(alias = "XIP_CMD_W3")]
pub type XipCmdW3 = crate::Reg<xip_cmd_w3::XipCmdW3Spec>;
#[doc = "XIP command word 3"]
pub mod xip_cmd_w3;
#[doc = "REV (rw) register accessor: Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`]
module"]
#[doc(alias = "REV")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "Revision"]
pub mod rev;
#[doc = "DT (rw) register accessor: 32/16/8 bit data port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "32/16/8 bit data port register"]
pub mod dt;
