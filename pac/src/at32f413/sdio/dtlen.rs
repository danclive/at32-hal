#[doc = "Register `DTLEN` reader"]
pub type R = crate::R<DtlenSpec>;
#[doc = "Register `DTLEN` writer"]
pub type W = crate::W<DtlenSpec>;
#[doc = "Field `DTLEN` reader - Data length value"]
pub type DtlenR = crate::FieldReader<u32>;
#[doc = "Field `DTLEN` writer - Data length value"]
pub type DtlenW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn dtlen(&self) -> DtlenR {
        DtlenR::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTLEN")
            .field("dtlen", &self.dtlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    #[must_use]
    pub fn dtlen(&mut self) -> DtlenW<DtlenSpec> {
        DtlenW::new(self, 0)
    }
}
#[doc = "Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtlenSpec;
impl crate::RegisterSpec for DtlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtlen::R`](R) reader structure"]
impl crate::Readable for DtlenSpec {}
#[doc = "`write(|w| ..)` method takes [`dtlen::W`](W) writer structure"]
impl crate::Writable for DtlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTLEN to value 0"]
impl crate::Resettable for DtlenSpec {
    const RESET_VALUE: u32 = 0;
}
