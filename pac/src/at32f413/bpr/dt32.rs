#[doc = "Register `DT32` reader"]
pub type R = crate::R<Dt32Spec>;
#[doc = "Register `DT32` writer"]
pub type W = crate::W<Dt32Spec>;
#[doc = "Field `DT32` reader - BPR data32"]
pub type Dt32R = crate::FieldReader<u16>;
#[doc = "Field `DT32` writer - BPR data32"]
pub type Dt32W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data32"]
    #[inline(always)]
    pub fn dt32(&self) -> Dt32R {
        Dt32R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT32").field("dt32", &self.dt32()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data32"]
    #[inline(always)]
    #[must_use]
    pub fn dt32(&mut self) -> Dt32W<Dt32Spec> {
        Dt32W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt32Spec;
impl crate::RegisterSpec for Dt32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt32::R`](R) reader structure"]
impl crate::Readable for Dt32Spec {}
#[doc = "`write(|w| ..)` method takes [`dt32::W`](W) writer structure"]
impl crate::Writable for Dt32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT32 to value 0"]
impl crate::Resettable for Dt32Spec {
    const RESET_VALUE: u32 = 0;
}
