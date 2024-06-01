#[doc = "Register `PCDTO2` reader"]
pub type R = crate::R<Pcdto2Spec>;
#[doc = "Register `PCDTO2` writer"]
pub type W = crate::W<Pcdto2Spec>;
#[doc = "Field `PCDTO2` reader - Data offset for Preempted channel 2"]
pub type Pcdto2R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO2` writer - Data offset for Preempted channel 2"]
pub type Pcdto2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 2"]
    #[inline(always)]
    pub fn pcdto2(&self) -> Pcdto2R {
        Pcdto2R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCDTO2")
            .field("pcdto2", &self.pcdto2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto2(&mut self) -> Pcdto2W<Pcdto2Spec> {
        Pcdto2W::new(self, 0)
    }
}
#[doc = "Preempted channel 2 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdto2Spec;
impl crate::RegisterSpec for Pcdto2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto2::R`](R) reader structure"]
impl crate::Readable for Pcdto2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcdto2::W`](W) writer structure"]
impl crate::Writable for Pcdto2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCDTO2 to value 0"]
impl crate::Resettable for Pcdto2Spec {
    const RESET_VALUE: u32 = 0;
}
