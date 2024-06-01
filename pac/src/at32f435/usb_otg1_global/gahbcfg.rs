#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GahbcfgSpec>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GahbcfgSpec>;
#[doc = "Field `GLBINTMSK` reader - Global interrupt mask"]
pub type GlbintmskR = crate::BitReader;
#[doc = "Field `GLBINTMSK` writer - Global interrupt mask"]
pub type GlbintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPLVL` reader - Non-Periodic TxFIFO empty level"]
pub type NptxfemplvlR = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - Non-Periodic TxFIFO empty level"]
pub type NptxfemplvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPLVL` reader - Periodic TxFIFO empty level"]
pub type PtxfemplvlR = crate::BitReader;
#[doc = "Field `PTXFEMPLVL` writer - Periodic TxFIFO empty level"]
pub type PtxfemplvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn glbintmsk(&self) -> GlbintmskR {
        GlbintmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NptxfemplvlR {
        NptxfemplvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PtxfemplvlR {
        PtxfemplvlR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("glbintmsk", &self.glbintmsk())
            .field("nptxfemplvl", &self.nptxfemplvl())
            .field("ptxfemplvl", &self.ptxfemplvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn glbintmsk(&mut self) -> GlbintmskW<GahbcfgSpec> {
        GlbintmskW::new(self, 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfemplvl(&mut self) -> NptxfemplvlW<GahbcfgSpec> {
        NptxfemplvlW::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfemplvl(&mut self) -> PtxfemplvlW<GahbcfgSpec> {
        PtxfemplvlW::new(self, 8)
    }
}
#[doc = "OTGFS AHB configuration register (OTGFS_GAHBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcfgSpec;
impl crate::RegisterSpec for GahbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GahbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GahbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GahbcfgSpec {
    const RESET_VALUE: u32 = 0;
}
