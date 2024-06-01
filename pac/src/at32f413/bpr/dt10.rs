#[doc = "Register `DT10` reader"]
pub type R = crate::R<Dt10Spec>;
#[doc = "Register `DT10` writer"]
pub type W = crate::W<Dt10Spec>;
#[doc = "Field `DT10` reader - BPR data10"]
pub type Dt10R = crate::FieldReader<u16>;
#[doc = "Field `DT10` writer - BPR data10"]
pub type Dt10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data10"]
    #[inline(always)]
    pub fn dt10(&self) -> Dt10R {
        Dt10R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT10").field("dt10", &self.dt10()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data10"]
    #[inline(always)]
    #[must_use]
    pub fn dt10(&mut self) -> Dt10W<Dt10Spec> {
        Dt10W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt10Spec;
impl crate::RegisterSpec for Dt10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt10::R`](R) reader structure"]
impl crate::Readable for Dt10Spec {}
#[doc = "`write(|w| ..)` method takes [`dt10::W`](W) writer structure"]
impl crate::Writable for Dt10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT10 to value 0"]
impl crate::Resettable for Dt10Spec {
    const RESET_VALUE: u32 = 0;
}
