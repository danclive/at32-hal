#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PcgcctlSpec>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PcgcctlSpec>;
#[doc = "Field `STOPPCLK` reader - Stop PHY clock"]
pub type StoppclkR = crate::BitReader;
#[doc = "Field `STOPPCLK` writer - Stop PHY clock"]
pub type StoppclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPENDM` reader - PHY Suspended"]
pub type SuspendmR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stoppclk(&self) -> StoppclkR {
        StoppclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn suspendm(&self) -> SuspendmR {
        SuspendmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stoppclk", &self.stoppclk())
            .field("suspendm", &self.suspendm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stoppclk(&mut self) -> StoppclkW<PcgcctlSpec> {
        StoppclkW::new(self, 0)
    }
}
#[doc = "OTGFS power and clock gating control register (OTGFS_PCGCCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcgcctlSpec;
impl crate::RegisterSpec for PcgcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PcgcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PcgcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PcgcctlSpec {
    const RESET_VALUE: u32 = 0;
}
