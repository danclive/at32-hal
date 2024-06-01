#[doc = "Register `SLIB_STS2` reader"]
pub type R = crate::R<SlibSts2Spec>;
#[doc = "Register `SLIB_STS2` writer"]
pub type W = crate::W<SlibSts2Spec>;
#[doc = "Field `SLIB_INST_SS` reader - sLib instruction start sector"]
pub type SlibInstSsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - sLib instruction start sector"]
    #[inline(always)]
    pub fn slib_inst_ss(&self) -> SlibInstSsR {
        SlibInstSsR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS2")
            .field("slib_inst_ss", &self.slib_inst_ss())
            .finish()
    }
}
impl W {}
#[doc = "sLib status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts2Spec;
impl crate::RegisterSpec for SlibSts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts2::R`](R) reader structure"]
impl crate::Readable for SlibSts2Spec {}
#[doc = "`write(|w| ..)` method takes [`slib_sts2::W`](W) writer structure"]
impl crate::Writable for SlibSts2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_STS2 to value 0xffff"]
impl crate::Resettable for SlibSts2Spec {
    const RESET_VALUE: u32 = 0xffff;
}
