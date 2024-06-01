#[doc = "Register `DT13` reader"]
pub type R = crate::R<Dt13Spec>;
#[doc = "Register `DT13` writer"]
pub type W = crate::W<Dt13Spec>;
#[doc = "Field `DT13` reader - BPR data13"]
pub type Dt13R = crate::FieldReader<u16>;
#[doc = "Field `DT13` writer - BPR data13"]
pub type Dt13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data13"]
    #[inline(always)]
    pub fn dt13(&self) -> Dt13R {
        Dt13R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT13").field("dt13", &self.dt13()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data13"]
    #[inline(always)]
    #[must_use]
    pub fn dt13(&mut self) -> Dt13W<Dt13Spec> {
        Dt13W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt13Spec;
impl crate::RegisterSpec for Dt13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt13::R`](R) reader structure"]
impl crate::Readable for Dt13Spec {}
#[doc = "`write(|w| ..)` method takes [`dt13::W`](W) writer structure"]
impl crate::Writable for Dt13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT13 to value 0"]
impl crate::Resettable for Dt13Spec {
    const RESET_VALUE: u32 = 0;
}
