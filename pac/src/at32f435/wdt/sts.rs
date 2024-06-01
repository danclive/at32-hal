#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `DIVF` reader - Division value update complete flag"]
pub type DivfR = crate::BitReader;
#[doc = "Field `RLDF` reader - Reload value update complete flag"]
pub type RldfR = crate::BitReader;
#[doc = "Field `WINF` reader - Window value update complete flag"]
pub type WinfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Division value update complete flag"]
    #[inline(always)]
    pub fn divf(&self) -> DivfR {
        DivfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload value update complete flag"]
    #[inline(always)]
    pub fn rldf(&self) -> RldfR {
        RldfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window value update complete flag"]
    #[inline(always)]
    pub fn winf(&self) -> WinfR {
        WinfR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("divf", &self.divf())
            .field("rldf", &self.rldf())
            .field("winf", &self.winf())
            .finish()
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
