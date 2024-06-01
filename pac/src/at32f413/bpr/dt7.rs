#[doc = "Register `DT7` reader"]
pub type R = crate::R<Dt7Spec>;
#[doc = "Register `DT7` writer"]
pub type W = crate::W<Dt7Spec>;
#[doc = "Field `DT7` reader - BPR data7"]
pub type Dt7R = crate::FieldReader<u16>;
#[doc = "Field `DT7` writer - BPR data7"]
pub type Dt7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data7"]
    #[inline(always)]
    pub fn dt7(&self) -> Dt7R {
        Dt7R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT7").field("dt7", &self.dt7()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data7"]
    #[inline(always)]
    #[must_use]
    pub fn dt7(&mut self) -> Dt7W<Dt7Spec> {
        Dt7W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt7Spec;
impl crate::RegisterSpec for Dt7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt7::R`](R) reader structure"]
impl crate::Readable for Dt7Spec {}
#[doc = "`write(|w| ..)` method takes [`dt7::W`](W) writer structure"]
impl crate::Writable for Dt7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT7 to value 0"]
impl crate::Resettable for Dt7Spec {
    const RESET_VALUE: u32 = 0;
}
