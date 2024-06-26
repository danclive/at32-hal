#[doc = "Register `SLIB_STS1` reader"]
pub type R = crate::R<SlibSts1Spec>;
#[doc = "Register `SLIB_STS1` writer"]
pub type W = crate::W<SlibSts1Spec>;
#[doc = "Field `SLIB_SS` reader - sLib start sector"]
pub type SlibSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_ES` reader - sLib end sector"]
pub type SlibEsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - sLib start sector"]
    #[inline(always)]
    pub fn slib_ss(&self) -> SlibSsR {
        SlibSsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - sLib end sector"]
    #[inline(always)]
    pub fn slib_es(&self) -> SlibEsR {
        SlibEsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS1")
            .field("slib_ss", &self.slib_ss())
            .field("slib_es", &self.slib_es())
            .finish()
    }
}
impl W {}
#[doc = "sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts1Spec;
impl crate::RegisterSpec for SlibSts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts1::R`](R) reader structure"]
impl crate::Readable for SlibSts1Spec {}
#[doc = "`write(|w| ..)` method takes [`slib_sts1::W`](W) writer structure"]
impl crate::Writable for SlibSts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_STS1 to value 0xffff_ffff"]
impl crate::Resettable for SlibSts1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
