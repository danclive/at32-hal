#[doc = "Register `DT23` reader"]
pub type R = crate::R<Dt23Spec>;
#[doc = "Register `DT23` writer"]
pub type W = crate::W<Dt23Spec>;
#[doc = "Field `DT23` reader - BPR data23"]
pub type Dt23R = crate::FieldReader<u16>;
#[doc = "Field `DT23` writer - BPR data23"]
pub type Dt23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data23"]
    #[inline(always)]
    pub fn dt23(&self) -> Dt23R {
        Dt23R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT23").field("dt23", &self.dt23()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data23"]
    #[inline(always)]
    #[must_use]
    pub fn dt23(&mut self) -> Dt23W<Dt23Spec> {
        Dt23W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt23Spec;
impl crate::RegisterSpec for Dt23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt23::R`](R) reader structure"]
impl crate::Readable for Dt23Spec {}
#[doc = "`write(|w| ..)` method takes [`dt23::W`](W) writer structure"]
impl crate::Writable for Dt23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT23 to value 0"]
impl crate::Resettable for Dt23Spec {
    const RESET_VALUE: u32 = 0;
}
