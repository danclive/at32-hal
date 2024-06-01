#[doc = "Register `DT42` reader"]
pub type R = crate::R<Dt42Spec>;
#[doc = "Register `DT42` writer"]
pub type W = crate::W<Dt42Spec>;
#[doc = "Field `DT42` reader - BPR data42"]
pub type Dt42R = crate::FieldReader<u16>;
#[doc = "Field `DT42` writer - BPR data42"]
pub type Dt42W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data42"]
    #[inline(always)]
    pub fn dt42(&self) -> Dt42R {
        Dt42R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT42").field("dt42", &self.dt42()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data42"]
    #[inline(always)]
    #[must_use]
    pub fn dt42(&mut self) -> Dt42W<Dt42Spec> {
        Dt42W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt42Spec;
impl crate::RegisterSpec for Dt42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt42::R`](R) reader structure"]
impl crate::Readable for Dt42Spec {}
#[doc = "`write(|w| ..)` method takes [`dt42::W`](W) writer structure"]
impl crate::Writable for Dt42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT42 to value 0"]
impl crate::Resettable for Dt42Spec {
    const RESET_VALUE: u32 = 0;
}
