#[doc = "Register `DT34` reader"]
pub type R = crate::R<Dt34Spec>;
#[doc = "Register `DT34` writer"]
pub type W = crate::W<Dt34Spec>;
#[doc = "Field `DT34` reader - BPR data34"]
pub type Dt34R = crate::FieldReader<u16>;
#[doc = "Field `DT34` writer - BPR data34"]
pub type Dt34W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data34"]
    #[inline(always)]
    pub fn dt34(&self) -> Dt34R {
        Dt34R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT34").field("dt34", &self.dt34()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data34"]
    #[inline(always)]
    #[must_use]
    pub fn dt34(&mut self) -> Dt34W<Dt34Spec> {
        Dt34W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt34Spec;
impl crate::RegisterSpec for Dt34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt34::R`](R) reader structure"]
impl crate::Readable for Dt34Spec {}
#[doc = "`write(|w| ..)` method takes [`dt34::W`](W) writer structure"]
impl crate::Writable for Dt34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT34 to value 0"]
impl crate::Resettable for Dt34Spec {
    const RESET_VALUE: u32 = 0;
}
