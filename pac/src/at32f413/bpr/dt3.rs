#[doc = "Register `DT3` reader"]
pub type R = crate::R<Dt3Spec>;
#[doc = "Register `DT3` writer"]
pub type W = crate::W<Dt3Spec>;
#[doc = "Field `DT3` reader - BPR data3"]
pub type Dt3R = crate::FieldReader<u16>;
#[doc = "Field `DT3` writer - BPR data3"]
pub type Dt3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data3"]
    #[inline(always)]
    pub fn dt3(&self) -> Dt3R {
        Dt3R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT3").field("dt3", &self.dt3()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data3"]
    #[inline(always)]
    #[must_use]
    pub fn dt3(&mut self) -> Dt3W<Dt3Spec> {
        Dt3W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt3Spec;
impl crate::RegisterSpec for Dt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt3::R`](R) reader structure"]
impl crate::Readable for Dt3Spec {}
#[doc = "`write(|w| ..)` method takes [`dt3::W`](W) writer structure"]
impl crate::Writable for Dt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT3 to value 0"]
impl crate::Resettable for Dt3Spec {
    const RESET_VALUE: u32 = 0;
}
