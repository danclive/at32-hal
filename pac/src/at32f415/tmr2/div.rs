#[doc = "Register `DIV` reader"]
pub type R = crate::R<DivSpec>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DivSpec>;
#[doc = "Field `DIV` reader - Divider value"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Divider value"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Divider value"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV").field("div", &self.div()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Divider value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<DivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Divider value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DivSpec {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DivSpec {
    const RESET_VALUE: u32 = 0;
}
