#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "Field `C1` reader - Compare 1"]
pub type C1R = crate::FieldReader<u16>;
#[doc = "Field `C1` writer - Compare 1"]
pub type C1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 1"]
    #[inline(always)]
    pub fn c1(&self) -> C1R {
        C1R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1").field("c1", &self.c1()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1W<C1Spec> {
        C1W::new(self, 0)
    }
}
#[doc = "compare value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1 to value 0x1f2c"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u32 = 0x1f2c;
}
