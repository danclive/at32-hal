#[doc = "Register `GRXFSIZ` reader"]
pub type R = crate::R<GrxfsizSpec>;
#[doc = "Register `GRXFSIZ` writer"]
pub type W = crate::W<GrxfsizSpec>;
#[doc = "Field `RXFDEP` reader - RxFIFO depth"]
pub type RxfdepR = crate::FieldReader<u16>;
#[doc = "Field `RXFDEP` writer - RxFIFO depth"]
pub type RxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfdep(&self) -> RxfdepR {
        RxfdepR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXFSIZ")
            .field("rxfdep", &self.rxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfdep(&mut self) -> RxfdepW<GrxfsizSpec> {
        RxfdepW::new(self, 0)
    }
}
#[doc = "OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxfsizSpec;
impl crate::RegisterSpec for GrxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfsiz::R`](R) reader structure"]
impl crate::Readable for GrxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure"]
impl crate::Writable for GrxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0200"]
impl crate::Resettable for GrxfsizSpec {
    const RESET_VALUE: u32 = 0x0200;
}
