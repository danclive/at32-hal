#[doc = "Register `SLIB_STS1` reader"]
pub type R = crate::R<SlibSts1Spec>;
#[doc = "Field `SLIB_SS` reader - sLib start sector"]
pub type SlibSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_DAT_SS` reader - sLib data start sector"]
pub type SlibDatSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_ES` reader - sLib end sector"]
pub type SlibEsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    pub fn slib_ss(&self) -> SlibSsR {
        SlibSsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - sLib data start sector"]
    #[inline(always)]
    pub fn slib_dat_ss(&self) -> SlibDatSsR {
        SlibDatSsR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    pub fn slib_es(&self) -> SlibEsR {
        SlibEsR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS1")
            .field("slib_ss", &self.slib_ss())
            .field("slib_dat_ss", &self.slib_dat_ss())
            .field("slib_es", &self.slib_es())
            .finish()
    }
}
#[doc = "sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts1Spec;
impl crate::RegisterSpec for SlibSts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts1::R`](R) reader structure"]
impl crate::Readable for SlibSts1Spec {}
#[doc = "`reset()` method sets SLIB_STS1 to value 0xffff_ffff"]
impl crate::Resettable for SlibSts1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
