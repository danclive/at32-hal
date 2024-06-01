#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HptxstsSpec>;
#[doc = "Register `HPTXSTS` writer"]
pub type W = crate::W<HptxstsSpec>;
#[doc = "Field `PTXFSPCAVAIL` reader - Periodic transmit data FIFO space available"]
pub type PtxfspcavailR = crate::FieldReader<u16>;
#[doc = "Field `PTXFSPCAVAIL` writer - Periodic transmit data FIFO space available"]
pub type PtxfspcavailW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXQSPCAVAIL` reader - Periodic transmit request queue space available"]
pub type PtxqspcavailR = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - Top of the periodic transmit request queue"]
pub type PtxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PtxfspcavailR {
        PtxfspcavailR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PtxqspcavailR {
        PtxqspcavailR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PtxqtopR {
        PtxqtopR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfspcavail", &self.ptxfspcavail())
            .field("ptxqspcavail", &self.ptxqspcavail())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfspcavail(&mut self) -> PtxfspcavailW<HptxstsSpec> {
        PtxfspcavailW::new(self, 0)
    }
}
#[doc = "OTGFS_Host periodic transmit FIFO/queue status register (OTGFS_HPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxstsSpec;
impl crate::RegisterSpec for HptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HptxstsSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxsts::W`](W) writer structure"]
impl crate::Writable for HptxstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0100;
}
