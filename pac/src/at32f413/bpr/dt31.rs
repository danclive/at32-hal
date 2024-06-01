#[doc = "Register `DT31` reader"]
pub type R = crate::R<Dt31Spec>;
#[doc = "Register `DT31` writer"]
pub type W = crate::W<Dt31Spec>;
#[doc = "Field `DT31` reader - BPR data31"]
pub type Dt31R = crate::FieldReader<u16>;
#[doc = "Field `DT31` writer - BPR data31"]
pub type Dt31W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data31"]
    #[inline(always)]
    pub fn dt31(&self) -> Dt31R {
        Dt31R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT31").field("dt31", &self.dt31()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data31"]
    #[inline(always)]
    #[must_use]
    pub fn dt31(&mut self) -> Dt31W<Dt31Spec> {
        Dt31W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt31Spec;
impl crate::RegisterSpec for Dt31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt31::R`](R) reader structure"]
impl crate::Readable for Dt31Spec {}
#[doc = "`write(|w| ..)` method takes [`dt31::W`](W) writer structure"]
impl crate::Writable for Dt31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT31 to value 0"]
impl crate::Resettable for Dt31Spec {
    const RESET_VALUE: u32 = 0;
}
