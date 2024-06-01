#[doc = "Register `DT27` reader"]
pub type R = crate::R<Dt27Spec>;
#[doc = "Register `DT27` writer"]
pub type W = crate::W<Dt27Spec>;
#[doc = "Field `DT27` reader - BPR data27"]
pub type Dt27R = crate::FieldReader<u16>;
#[doc = "Field `DT27` writer - BPR data27"]
pub type Dt27W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data27"]
    #[inline(always)]
    pub fn dt27(&self) -> Dt27R {
        Dt27R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT27").field("dt27", &self.dt27()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data27"]
    #[inline(always)]
    #[must_use]
    pub fn dt27(&mut self) -> Dt27W<Dt27Spec> {
        Dt27W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt27Spec;
impl crate::RegisterSpec for Dt27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt27::R`](R) reader structure"]
impl crate::Readable for Dt27Spec {}
#[doc = "`write(|w| ..)` method takes [`dt27::W`](W) writer structure"]
impl crate::Writable for Dt27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT27 to value 0"]
impl crate::Resettable for Dt27Spec {
    const RESET_VALUE: u32 = 0;
}
