#[doc = "Register `DT33` reader"]
pub type R = crate::R<Dt33Spec>;
#[doc = "Register `DT33` writer"]
pub type W = crate::W<Dt33Spec>;
#[doc = "Field `DT33` reader - BPR data33"]
pub type Dt33R = crate::FieldReader<u16>;
#[doc = "Field `DT33` writer - BPR data33"]
pub type Dt33W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data33"]
    #[inline(always)]
    pub fn dt33(&self) -> Dt33R {
        Dt33R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT33").field("dt33", &self.dt33()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data33"]
    #[inline(always)]
    #[must_use]
    pub fn dt33(&mut self) -> Dt33W<Dt33Spec> {
        Dt33W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt33Spec;
impl crate::RegisterSpec for Dt33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt33::R`](R) reader structure"]
impl crate::Readable for Dt33Spec {}
#[doc = "`write(|w| ..)` method takes [`dt33::W`](W) writer structure"]
impl crate::Writable for Dt33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT33 to value 0"]
impl crate::Resettable for Dt33Spec {
    const RESET_VALUE: u32 = 0;
}
