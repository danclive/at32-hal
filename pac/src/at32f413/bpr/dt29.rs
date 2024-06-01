#[doc = "Register `DT29` reader"]
pub type R = crate::R<Dt29Spec>;
#[doc = "Register `DT29` writer"]
pub type W = crate::W<Dt29Spec>;
#[doc = "Field `DT29` reader - BPR data29"]
pub type Dt29R = crate::FieldReader<u16>;
#[doc = "Field `DT29` writer - BPR data29"]
pub type Dt29W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data29"]
    #[inline(always)]
    pub fn dt29(&self) -> Dt29R {
        Dt29R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT29").field("dt29", &self.dt29()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data29"]
    #[inline(always)]
    #[must_use]
    pub fn dt29(&mut self) -> Dt29W<Dt29Spec> {
        Dt29W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt29Spec;
impl crate::RegisterSpec for Dt29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt29::R`](R) reader structure"]
impl crate::Readable for Dt29Spec {}
#[doc = "`write(|w| ..)` method takes [`dt29::W`](W) writer structure"]
impl crate::Writable for Dt29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT29 to value 0"]
impl crate::Resettable for Dt29Spec {
    const RESET_VALUE: u32 = 0;
}
