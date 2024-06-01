#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<Remap2Spec>;
#[doc = "Field `CMP_MUX` writer - CMP internal muxing"]
pub type CmpMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl core::fmt::Debug for crate::generic::Reg<Remap2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 26:27 - CMP internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_mux(&mut self) -> CmpMuxW<Remap2Spec> {
        CmpMuxW::new(self, 26)
    }
}
#[doc = "IO MUX remap register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap2Spec;
impl crate::RegisterSpec for Remap2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for Remap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for Remap2Spec {
    const RESET_VALUE: u32 = 0;
}
