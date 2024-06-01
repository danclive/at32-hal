#[doc = "Register `DT18` reader"]
pub type R = crate::R<Dt18Spec>;
#[doc = "Register `DT18` writer"]
pub type W = crate::W<Dt18Spec>;
#[doc = "Field `DT18` reader - BPR data18"]
pub type Dt18R = crate::FieldReader<u16>;
#[doc = "Field `DT18` writer - BPR data18"]
pub type Dt18W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data18"]
    #[inline(always)]
    pub fn dt18(&self) -> Dt18R {
        Dt18R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT18").field("dt18", &self.dt18()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data18"]
    #[inline(always)]
    #[must_use]
    pub fn dt18(&mut self) -> Dt18W<Dt18Spec> {
        Dt18W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt18Spec;
impl crate::RegisterSpec for Dt18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt18::R`](R) reader structure"]
impl crate::Readable for Dt18Spec {}
#[doc = "`write(|w| ..)` method takes [`dt18::W`](W) writer structure"]
impl crate::Writable for Dt18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT18 to value 0"]
impl crate::Resettable for Dt18Spec {
    const RESET_VALUE: u32 = 0;
}
