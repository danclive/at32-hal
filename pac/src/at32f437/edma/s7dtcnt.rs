#[doc = "Register `S7DTCNT` reader"]
pub type R = crate::R<S7dtcntSpec>;
#[doc = "Register `S7DTCNT` writer"]
pub type W = crate::W<S7dtcntSpec>;
#[doc = "Field `CNT` reader - Number of data items to transfer"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data items to transfer"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S7DTCNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<S7dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "stream 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7dtcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7dtcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7dtcntSpec;
impl crate::RegisterSpec for S7dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7dtcnt::R`](R) reader structure"]
impl crate::Readable for S7dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`s7dtcnt::W`](W) writer structure"]
impl crate::Writable for S7dtcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S7DTCNT to value 0"]
impl crate::Resettable for S7dtcntSpec {
    const RESET_VALUE: u32 = 0;
}
