#[doc = "Register `WP` writer"]
pub type W = crate::W<WpSpec>;
#[doc = "Field `CMD` writer - Command register"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<WpSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<WpSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpSpec;
impl crate::RegisterSpec for WpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wp::W`](W) writer structure"]
impl crate::Writable for WpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WpSpec {
    const RESET_VALUE: u32 = 0;
}
