#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcgcctl: Pcgcctl,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGFS power and clock gating control register (OTGFS_PCGCCTL)"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &Pcgcctl {
        &self.pcgcctl
    }
}
#[doc = "PCGCCTL (rw) register accessor: OTGFS power and clock gating control register (OTGFS_PCGCCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
#[doc(alias = "PCGCCTL")]
pub type Pcgcctl = crate::Reg<pcgcctl::PcgcctlSpec>;
#[doc = "OTGFS power and clock gating control register (OTGFS_PCGCCTL)"]
pub mod pcgcctl;
