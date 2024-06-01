#[doc = "Register `DT15` reader"]
pub type R = crate::R<Dt15Spec>;
#[doc = "Register `DT15` writer"]
pub type W = crate::W<Dt15Spec>;
#[doc = "Field `DT15` reader - BPR data15"]
pub type Dt15R = crate::FieldReader<u16>;
#[doc = "Field `DT15` writer - BPR data15"]
pub type Dt15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data15"]
    #[inline(always)]
    pub fn dt15(&self) -> Dt15R {
        Dt15R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT15").field("dt15", &self.dt15()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data15"]
    #[inline(always)]
    #[must_use]
    pub fn dt15(&mut self) -> Dt15W<Dt15Spec> {
        Dt15W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt15Spec;
impl crate::RegisterSpec for Dt15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt15::R`](R) reader structure"]
impl crate::Readable for Dt15Spec {}
#[doc = "`write(|w| ..)` method takes [`dt15::W`](W) writer structure"]
impl crate::Writable for Dt15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT15 to value 0"]
impl crate::Resettable for Dt15Spec {
    const RESET_VALUE: u32 = 0;
}
