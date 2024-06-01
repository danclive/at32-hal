#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HptxfsizSpec>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HptxfsizSpec>;
#[doc = "Field `PTXFSTADDR` reader - Host periodic TxFIFO start address"]
pub type PtxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `PTXFSTADDR` writer - Host periodic TxFIFO start address"]
pub type PtxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFSIZE` reader - Host periodic TxFIFO depth"]
pub type PtxfsizeR = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZE` writer - Host periodic TxFIFO depth"]
pub type PtxfsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PtxfstaddrR {
        PtxfstaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PtxfsizeR {
        PtxfsizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXFSIZ")
            .field("ptxfstaddr", &self.ptxfstaddr())
            .field("ptxfsize", &self.ptxfsize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfstaddr(&mut self) -> PtxfstaddrW<HptxfsizSpec> {
        PtxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsize(&mut self) -> PtxfsizeW<HptxfsizSpec> {
        PtxfsizeW::new(self, 16)
    }
}
#[doc = "OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxfsizSpec;
impl crate::RegisterSpec for HptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HptxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for HptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0600;
}
