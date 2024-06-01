#[doc = "Register `DT22` reader"]
pub type R = crate::R<Dt22Spec>;
#[doc = "Register `DT22` writer"]
pub type W = crate::W<Dt22Spec>;
#[doc = "Field `DT22` reader - BPR data22"]
pub type Dt22R = crate::FieldReader<u16>;
#[doc = "Field `DT22` writer - BPR data22"]
pub type Dt22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data22"]
    #[inline(always)]
    pub fn dt22(&self) -> Dt22R {
        Dt22R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT22").field("dt22", &self.dt22()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data22"]
    #[inline(always)]
    #[must_use]
    pub fn dt22(&mut self) -> Dt22W<Dt22Spec> {
        Dt22W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt22Spec;
impl crate::RegisterSpec for Dt22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt22::R`](R) reader structure"]
impl crate::Readable for Dt22Spec {}
#[doc = "`write(|w| ..)` method takes [`dt22::W`](W) writer structure"]
impl crate::Writable for Dt22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT22 to value 0"]
impl crate::Resettable for Dt22Spec {
    const RESET_VALUE: u32 = 0;
}
