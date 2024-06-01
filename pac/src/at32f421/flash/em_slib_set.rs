#[doc = "Register `EM_SLIB_SET` writer"]
pub type W = crate::W<EmSlibSetSpec>;
#[doc = "Field `EM_SLIB_SET` writer - Extension memory sLib setting"]
pub type EmSlibSetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM_SLIB_ISS_SET` writer - Extension memory sLib instruction start sector setting"]
pub type EmSlibIssSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<EmSlibSetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Extension memory sLib setting"]
    #[inline(always)]
    #[must_use]
    pub fn em_slib_set(&mut self) -> EmSlibSetW<EmSlibSetSpec> {
        EmSlibSetW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Extension memory sLib instruction start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn em_slib_iss_set(&mut self) -> EmSlibIssSetW<EmSlibSetSpec> {
        EmSlibIssSetW::new(self, 16)
    }
}
#[doc = "Extension momery slib set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em_slib_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmSlibSetSpec;
impl crate::RegisterSpec for EmSlibSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`em_slib_set::W`](W) writer structure"]
impl crate::Writable for EmSlibSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EM_SLIB_SET to value 0"]
impl crate::Resettable for EmSlibSetSpec {
    const RESET_VALUE: u32 = 0;
}
