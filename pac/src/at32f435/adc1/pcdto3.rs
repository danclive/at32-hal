#[doc = "Register `PCDTO3` reader"]
pub type R = crate::R<Pcdto3Spec>;
#[doc = "Register `PCDTO3` writer"]
pub type W = crate::W<Pcdto3Spec>;
#[doc = "Field `PCDTO3` reader - Data offset for Preempted channel 3"]
pub type Pcdto3R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO3` writer - Data offset for Preempted channel 3"]
pub type Pcdto3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 3"]
    #[inline(always)]
    pub fn pcdto3(&self) -> Pcdto3R {
        Pcdto3R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCDTO3")
            .field("pcdto3", &self.pcdto3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto3(&mut self) -> Pcdto3W<Pcdto3Spec> {
        Pcdto3W::new(self, 0)
    }
}
#[doc = "Preempted channel 3 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdto3Spec;
impl crate::RegisterSpec for Pcdto3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto3::R`](R) reader structure"]
impl crate::Readable for Pcdto3Spec {}
#[doc = "`write(|w| ..)` method takes [`pcdto3::W`](W) writer structure"]
impl crate::Writable for Pcdto3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCDTO3 to value 0"]
impl crate::Resettable for Pcdto3Spec {
    const RESET_VALUE: u32 = 0;
}
