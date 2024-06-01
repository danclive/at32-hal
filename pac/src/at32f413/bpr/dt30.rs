#[doc = "Register `DT30` reader"]
pub type R = crate::R<Dt30Spec>;
#[doc = "Register `DT30` writer"]
pub type W = crate::W<Dt30Spec>;
#[doc = "Field `DT30` reader - BPR data30"]
pub type Dt30R = crate::FieldReader<u16>;
#[doc = "Field `DT30` writer - BPR data30"]
pub type Dt30W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data30"]
    #[inline(always)]
    pub fn dt30(&self) -> Dt30R {
        Dt30R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT30").field("dt30", &self.dt30()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data30"]
    #[inline(always)]
    #[must_use]
    pub fn dt30(&mut self) -> Dt30W<Dt30Spec> {
        Dt30W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt30Spec;
impl crate::RegisterSpec for Dt30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt30::R`](R) reader structure"]
impl crate::Readable for Dt30Spec {}
#[doc = "`write(|w| ..)` method takes [`dt30::W`](W) writer structure"]
impl crate::Writable for Dt30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT30 to value 0"]
impl crate::Resettable for Dt30Spec {
    const RESET_VALUE: u32 = 0;
}
