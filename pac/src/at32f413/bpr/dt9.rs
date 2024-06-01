#[doc = "Register `DT9` reader"]
pub type R = crate::R<Dt9Spec>;
#[doc = "Register `DT9` writer"]
pub type W = crate::W<Dt9Spec>;
#[doc = "Field `DT9` reader - BPR data9"]
pub type Dt9R = crate::FieldReader<u16>;
#[doc = "Field `DT9` writer - BPR data9"]
pub type Dt9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data9"]
    #[inline(always)]
    pub fn dt9(&self) -> Dt9R {
        Dt9R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT9").field("dt9", &self.dt9()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data9"]
    #[inline(always)]
    #[must_use]
    pub fn dt9(&mut self) -> Dt9W<Dt9Spec> {
        Dt9W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt9Spec;
impl crate::RegisterSpec for Dt9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt9::R`](R) reader structure"]
impl crate::Readable for Dt9Spec {}
#[doc = "`write(|w| ..)` method takes [`dt9::W`](W) writer structure"]
impl crate::Writable for Dt9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT9 to value 0"]
impl crate::Resettable for Dt9Spec {
    const RESET_VALUE: u32 = 0;
}
