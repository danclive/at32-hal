#[doc = "Register `PCDTO1` reader"]
pub type R = crate::R<Pcdto1Spec>;
#[doc = "Register `PCDTO1` writer"]
pub type W = crate::W<Pcdto1Spec>;
#[doc = "Field `PCDTO1` reader - Data offset for Preempted channel 1"]
pub type Pcdto1R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO1` writer - Data offset for Preempted channel 1"]
pub type Pcdto1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    pub fn pcdto1(&self) -> Pcdto1R {
        Pcdto1R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCDTO1")
            .field("pcdto1", &self.pcdto1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto1(&mut self) -> Pcdto1W<Pcdto1Spec> {
        Pcdto1W::new(self, 0)
    }
}
#[doc = "Preempted channel 1 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdto1Spec;
impl crate::RegisterSpec for Pcdto1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto1::R`](R) reader structure"]
impl crate::Readable for Pcdto1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcdto1::W`](W) writer structure"]
impl crate::Writable for Pcdto1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCDTO1 to value 0"]
impl crate::Resettable for Pcdto1Spec {
    const RESET_VALUE: u32 = 0;
}
