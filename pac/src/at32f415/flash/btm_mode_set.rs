#[doc = "Register `BTM_MODE_SET` writer"]
pub type W = crate::W<BtmModeSetSpec>;
#[doc = "Field `BTM_MODE_SET` writer - Boot memory mode setting"]
pub type BtmModeSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<BtmModeSetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Boot memory mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn btm_mode_set(&mut self) -> BtmModeSetW<BtmModeSetSpec> {
        BtmModeSetW::new(self, 0)
    }
}
#[doc = "Boot memory mode setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btm_mode_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtmModeSetSpec;
impl crate::RegisterSpec for BtmModeSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`btm_mode_set::W`](W) writer structure"]
impl crate::Writable for BtmModeSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTM_MODE_SET to value 0"]
impl crate::Resettable for BtmModeSetSpec {
    const RESET_VALUE: u32 = 0;
}
