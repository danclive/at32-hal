#[doc = "Register `DT26` reader"]
pub type R = crate::R<Dt26Spec>;
#[doc = "Register `DT26` writer"]
pub type W = crate::W<Dt26Spec>;
#[doc = "Field `DT26` reader - BPR data26"]
pub type Dt26R = crate::FieldReader<u16>;
#[doc = "Field `DT26` writer - BPR data26"]
pub type Dt26W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data26"]
    #[inline(always)]
    pub fn dt26(&self) -> Dt26R {
        Dt26R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT26").field("dt26", &self.dt26()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data26"]
    #[inline(always)]
    #[must_use]
    pub fn dt26(&mut self) -> Dt26W<Dt26Spec> {
        Dt26W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt26Spec;
impl crate::RegisterSpec for Dt26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt26::R`](R) reader structure"]
impl crate::Readable for Dt26Spec {}
#[doc = "`write(|w| ..)` method takes [`dt26::W`](W) writer structure"]
impl crate::Writable for Dt26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT26 to value 0"]
impl crate::Resettable for Dt26Spec {
    const RESET_VALUE: u32 = 0;
}
