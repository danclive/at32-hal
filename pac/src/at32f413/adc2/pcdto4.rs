#[doc = "Register `PCDTO4` reader"]
pub type R = crate::R<Pcdto4Spec>;
#[doc = "Register `PCDTO4` writer"]
pub type W = crate::W<Pcdto4Spec>;
#[doc = "Field `PCDTO4` reader - Data offset for Preempted channel 4"]
pub type Pcdto4R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO4` writer - Data offset for Preempted channel 4"]
pub type Pcdto4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 4"]
    #[inline(always)]
    pub fn pcdto4(&self) -> Pcdto4R {
        Pcdto4R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCDTO4")
            .field("pcdto4", &self.pcdto4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto4(&mut self) -> Pcdto4W<Pcdto4Spec> {
        Pcdto4W::new(self, 0)
    }
}
#[doc = "Preempted channel 4 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdto4Spec;
impl crate::RegisterSpec for Pcdto4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto4::R`](R) reader structure"]
impl crate::Readable for Pcdto4Spec {}
#[doc = "`write(|w| ..)` method takes [`pcdto4::W`](W) writer structure"]
impl crate::Writable for Pcdto4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCDTO4 to value 0"]
impl crate::Resettable for Pcdto4Spec {
    const RESET_VALUE: u32 = 0;
}
