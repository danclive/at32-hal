#[doc = "Register `SLIB_STS0` reader"]
pub type R = crate::R<SlibSts0Spec>;
#[doc = "Register `SLIB_STS0` writer"]
pub type W = crate::W<SlibSts0Spec>;
#[doc = "Field `SLIB_ENF` reader - sLib enabled flag"]
pub type SlibEnfR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - sLib enabled flag"]
    #[inline(always)]
    pub fn slib_enf(&self) -> SlibEnfR {
        SlibEnfR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS0")
            .field("slib_enf", &self.slib_enf())
            .finish()
    }
}
impl W {}
#[doc = "sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts0Spec;
impl crate::RegisterSpec for SlibSts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts0::R`](R) reader structure"]
impl crate::Readable for SlibSts0Spec {}
#[doc = "`write(|w| ..)` method takes [`slib_sts0::W`](W) writer structure"]
impl crate::Writable for SlibSts0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_STS0 to value 0"]
impl crate::Resettable for SlibSts0Spec {
    const RESET_VALUE: u32 = 0;
}
