#[doc = "Register `DIVH` writer"]
pub type W = crate::W<DivhSpec>;
#[doc = "Field `DIV` writer - RTC divider high"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl core::fmt::Debug for crate::generic::Reg<DivhSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC divider high"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<DivhSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivhSpec;
impl crate::RegisterSpec for DivhSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`divh::W`](W) writer structure"]
impl crate::Writable for DivhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DivhSpec {
    const RESET_VALUE: u32 = 0;
}
