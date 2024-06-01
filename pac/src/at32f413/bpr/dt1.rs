#[doc = "Register `DT1` reader"]
pub type R = crate::R<Dt1Spec>;
#[doc = "Register `DT1` writer"]
pub type W = crate::W<Dt1Spec>;
#[doc = "Field `DT1` reader - BPR data1"]
pub type Dt1R = crate::FieldReader<u16>;
#[doc = "Field `DT1` writer - BPR data1"]
pub type Dt1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data1"]
    #[inline(always)]
    pub fn dt1(&self) -> Dt1R {
        Dt1R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT1").field("dt1", &self.dt1()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1(&mut self) -> Dt1W<Dt1Spec> {
        Dt1W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt1Spec;
impl crate::RegisterSpec for Dt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt1::R`](R) reader structure"]
impl crate::Readable for Dt1Spec {}
#[doc = "`write(|w| ..)` method takes [`dt1::W`](W) writer structure"]
impl crate::Writable for Dt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT1 to value 0"]
impl crate::Resettable for Dt1Spec {
    const RESET_VALUE: u32 = 0;
}
