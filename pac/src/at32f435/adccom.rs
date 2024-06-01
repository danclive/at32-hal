#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csts: Csts,
    cctrl: Cctrl,
    codt: Codt,
}
impl RegisterBlock {
    #[doc = "0x00 - Common status register"]
    #[inline(always)]
    pub const fn csts(&self) -> &Csts {
        &self.csts
    }
    #[doc = "0x04 - Common control register"]
    #[inline(always)]
    pub const fn cctrl(&self) -> &Cctrl {
        &self.cctrl
    }
    #[doc = "0x08 - Common Ordinary data register"]
    #[inline(always)]
    pub const fn codt(&self) -> &Codt {
        &self.codt
    }
}
#[doc = "CSTS (r) register accessor: Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csts`]
module"]
#[doc(alias = "CSTS")]
pub type Csts = crate::Reg<csts::CstsSpec>;
#[doc = "Common status register"]
pub mod csts;
#[doc = "CCTRL (rw) register accessor: Common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrl`]
module"]
#[doc(alias = "CCTRL")]
pub type Cctrl = crate::Reg<cctrl::CctrlSpec>;
#[doc = "Common control register"]
pub mod cctrl;
#[doc = "CODT (r) register accessor: Common Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codt`]
module"]
#[doc(alias = "CODT")]
pub type Codt = crate::Reg<codt::CodtSpec>;
#[doc = "Common Ordinary data register"]
pub mod codt;
