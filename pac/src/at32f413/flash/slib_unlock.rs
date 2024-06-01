#[doc = "Register `SLIB_UNLOCK` writer"]
pub type W = crate::W<SlibUnlockSpec>;
#[doc = "Field `SLIB_UKVAL` writer - sLib unlock key value"]
pub type SlibUkvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SlibUnlockSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib unlock key value"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ukval(&mut self) -> SlibUkvalW<SlibUnlockSpec> {
        SlibUkvalW::new(self, 0)
    }
}
#[doc = "sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibUnlockSpec;
impl crate::RegisterSpec for SlibUnlockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_unlock::W`](W) writer structure"]
impl crate::Writable for SlibUnlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_UNLOCK to value 0"]
impl crate::Resettable for SlibUnlockSpec {
    const RESET_VALUE: u32 = 0;
}
