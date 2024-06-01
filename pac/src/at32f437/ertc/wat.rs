#[doc = "Register `WAT` reader"]
pub type R = crate::R<WatSpec>;
#[doc = "Register `WAT` writer"]
pub type W = crate::W<WatSpec>;
#[doc = "Field `VAL` reader - Wakeup timer reload value"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Wakeup timer reload value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Wakeup timer reload value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAT").field("val", &self.val()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup timer reload value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<WatSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WatSpec;
impl crate::RegisterSpec for WatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wat::R`](R) reader structure"]
impl crate::Readable for WatSpec {}
#[doc = "`write(|w| ..)` method takes [`wat::W`](W) writer structure"]
impl crate::Writable for WatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAT to value 0xffff"]
impl crate::Resettable for WatSpec {
    const RESET_VALUE: u32 = 0xffff;
}
