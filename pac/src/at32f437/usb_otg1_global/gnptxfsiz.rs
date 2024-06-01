#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GnptxfsizSpec>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GnptxfsizSpec>;
#[doc = "Field `NPTXFSTADDR` reader - Non-periodic Transmit RAM Start address"]
pub type NptxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSTADDR` writer - Non-periodic Transmit RAM Start address"]
pub type NptxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFDEP` reader - Non-periodic TxFIFO depth"]
pub type NptxfdepR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFDEP` writer - Non-periodic TxFIFO depth"]
pub type NptxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start address"]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NptxfstaddrR {
        NptxfstaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfdep(&self) -> NptxfdepR {
        NptxfdepR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXFSIZ")
            .field("nptxfstaddr", &self.nptxfstaddr())
            .field("nptxfdep", &self.nptxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start address"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfstaddr(&mut self) -> NptxfstaddrW<GnptxfsizSpec> {
        NptxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfdep(&mut self) -> NptxfdepW<GnptxfsizSpec> {
        NptxfdepW::new(self, 16)
    }
}
#[doc = "OTGFS non-periodic transmit FIFO size register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxfsizSpec;
impl crate::RegisterSpec for GnptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GnptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GnptxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0200"]
impl crate::Resettable for GnptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200;
}
