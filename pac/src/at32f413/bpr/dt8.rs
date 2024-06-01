#[doc = "Register `DT8` reader"]
pub type R = crate::R<Dt8Spec>;
#[doc = "Register `DT8` writer"]
pub type W = crate::W<Dt8Spec>;
#[doc = "Field `DT8` reader - BPR data8"]
pub type Dt8R = crate::FieldReader<u16>;
#[doc = "Field `DT8` writer - BPR data8"]
pub type Dt8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data8"]
    #[inline(always)]
    pub fn dt8(&self) -> Dt8R {
        Dt8R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT8").field("dt8", &self.dt8()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data8"]
    #[inline(always)]
    #[must_use]
    pub fn dt8(&mut self) -> Dt8W<Dt8Spec> {
        Dt8W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt8Spec;
impl crate::RegisterSpec for Dt8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt8::R`](R) reader structure"]
impl crate::Readable for Dt8Spec {}
#[doc = "`write(|w| ..)` method takes [`dt8::W`](W) writer structure"]
impl crate::Writable for Dt8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT8 to value 0"]
impl crate::Resettable for Dt8Spec {
    const RESET_VALUE: u32 = 0;
}
