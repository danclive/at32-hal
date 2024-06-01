#[doc = "Register `SLIB_SET_RANGE0` writer"]
pub type W = crate::W<SlibSetRange0Spec>;
#[doc = "Field `SLIB_SS_SET` writer - sLib start sector setting"]
pub type SlibSsSetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SLIB_ES_SET` writer - sLib end sector setting"]
pub type SlibEsSetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<SlibSetRange0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - sLib start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ss_set(&mut self) -> SlibSsSetW<SlibSetRange0Spec> {
        SlibSsSetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - sLib end sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_es_set(&mut self) -> SlibEsSetW<SlibSetRange0Spec> {
        SlibEsSetW::new(self, 16)
    }
}
#[doc = "Configure sLib range register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSetRange0Spec;
impl crate::RegisterSpec for SlibSetRange0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_range0::W`](W) writer structure"]
impl crate::Writable for SlibSetRange0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE0 to value 0"]
impl crate::Resettable for SlibSetRange0Spec {
    const RESET_VALUE: u32 = 0;
}
