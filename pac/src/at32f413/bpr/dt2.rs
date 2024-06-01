#[doc = "Register `DT2` reader"]
pub type R = crate::R<Dt2Spec>;
#[doc = "Register `DT2` writer"]
pub type W = crate::W<Dt2Spec>;
#[doc = "Field `DT2` reader - BPR data2"]
pub type Dt2R = crate::FieldReader<u16>;
#[doc = "Field `DT2` writer - BPR data2"]
pub type Dt2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data2"]
    #[inline(always)]
    pub fn dt2(&self) -> Dt2R {
        Dt2R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT2").field("dt2", &self.dt2()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data2"]
    #[inline(always)]
    #[must_use]
    pub fn dt2(&mut self) -> Dt2W<Dt2Spec> {
        Dt2W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt2Spec;
impl crate::RegisterSpec for Dt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt2::R`](R) reader structure"]
impl crate::Readable for Dt2Spec {}
#[doc = "`write(|w| ..)` method takes [`dt2::W`](W) writer structure"]
impl crate::Writable for Dt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT2 to value 0"]
impl crate::Resettable for Dt2Spec {
    const RESET_VALUE: u32 = 0;
}
