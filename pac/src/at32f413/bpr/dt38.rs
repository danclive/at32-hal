#[doc = "Register `DT38` reader"]
pub type R = crate::R<Dt38Spec>;
#[doc = "Register `DT38` writer"]
pub type W = crate::W<Dt38Spec>;
#[doc = "Field `DT38` reader - BPR data38"]
pub type Dt38R = crate::FieldReader<u16>;
#[doc = "Field `DT38` writer - BPR data38"]
pub type Dt38W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data38"]
    #[inline(always)]
    pub fn dt38(&self) -> Dt38R {
        Dt38R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT38").field("dt38", &self.dt38()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data38"]
    #[inline(always)]
    #[must_use]
    pub fn dt38(&mut self) -> Dt38W<Dt38Spec> {
        Dt38W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt38Spec;
impl crate::RegisterSpec for Dt38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt38::R`](R) reader structure"]
impl crate::Readable for Dt38Spec {}
#[doc = "`write(|w| ..)` method takes [`dt38::W`](W) writer structure"]
impl crate::Writable for Dt38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT38 to value 0"]
impl crate::Resettable for Dt38Spec {
    const RESET_VALUE: u32 = 0;
}
