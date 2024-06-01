#[doc = "Register `CVAL` reader"]
pub type R = crate::R<CvalSpec>;
#[doc = "Register `CVAL` writer"]
pub type W = crate::W<CvalSpec>;
#[doc = "Field `CVAL` reader - Counter value"]
pub type CvalR = crate::FieldReader<u32>;
#[doc = "Field `CVAL` writer - Counter value"]
pub type CvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter value"]
    #[inline(always)]
    pub fn cval(&self) -> CvalR {
        CvalR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CVAL").field("cval", &self.cval()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cval(&mut self) -> CvalW<CvalSpec> {
        CvalW::new(self, 0)
    }
}
#[doc = "Counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvalSpec;
impl crate::RegisterSpec for CvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cval::R`](R) reader structure"]
impl crate::Readable for CvalSpec {}
#[doc = "`write(|w| ..)` method takes [`cval::W`](W) writer structure"]
impl crate::Writable for CvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CVAL to value 0"]
impl crate::Resettable for CvalSpec {
    const RESET_VALUE: u32 = 0;
}
