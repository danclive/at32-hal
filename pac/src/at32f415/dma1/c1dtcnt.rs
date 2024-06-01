#[doc = "Register `C1DTCNT` reader"]
pub type R = crate::R<C1dtcntSpec>;
#[doc = "Register `C1DTCNT` writer"]
pub type W = crate::W<C1dtcntSpec>;
#[doc = "Field `CNT` reader - Number of data to transfer"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data to transfer"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1DTCNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<C1dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 1 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1dtcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1dtcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1dtcntSpec;
impl crate::RegisterSpec for C1dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1dtcnt::R`](R) reader structure"]
impl crate::Readable for C1dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`c1dtcnt::W`](W) writer structure"]
impl crate::Writable for C1dtcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1DTCNT to value 0"]
impl crate::Resettable for C1dtcntSpec {
    const RESET_VALUE: u32 = 0;
}
