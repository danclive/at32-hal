#[doc = "Register `DT4` reader"]
pub type R = crate::R<Dt4Spec>;
#[doc = "Register `DT4` writer"]
pub type W = crate::W<Dt4Spec>;
#[doc = "Field `DT4` reader - BPR data4"]
pub type Dt4R = crate::FieldReader<u16>;
#[doc = "Field `DT4` writer - BPR data4"]
pub type Dt4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data4"]
    #[inline(always)]
    pub fn dt4(&self) -> Dt4R {
        Dt4R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT4").field("dt4", &self.dt4()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data4"]
    #[inline(always)]
    #[must_use]
    pub fn dt4(&mut self) -> Dt4W<Dt4Spec> {
        Dt4W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt4Spec;
impl crate::RegisterSpec for Dt4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt4::R`](R) reader structure"]
impl crate::Readable for Dt4Spec {}
#[doc = "`write(|w| ..)` method takes [`dt4::W`](W) writer structure"]
impl crate::Writable for Dt4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT4 to value 0"]
impl crate::Resettable for Dt4Spec {
    const RESET_VALUE: u32 = 0;
}
