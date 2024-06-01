#[doc = "Register `CODT` reader"]
pub type R = crate::R<CodtSpec>;
#[doc = "Field `CODTL` reader - Ordinary conversion low halfword data for master slave mode"]
pub type CodtlR = crate::FieldReader<u16>;
#[doc = "Field `CODTH` reader - Ordinary conversion high halfword data for master slave mode"]
pub type CodthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Ordinary conversion low halfword data for master slave mode"]
    #[inline(always)]
    pub fn codtl(&self) -> CodtlR {
        CodtlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Ordinary conversion high halfword data for master slave mode"]
    #[inline(always)]
    pub fn codth(&self) -> CodthR {
        CodthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODT")
            .field("codth", &self.codth())
            .field("codtl", &self.codtl())
            .finish()
    }
}
#[doc = "Common Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodtSpec;
impl crate::RegisterSpec for CodtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codt::R`](R) reader structure"]
impl crate::Readable for CodtSpec {}
#[doc = "`reset()` method sets CODT to value 0"]
impl crate::Resettable for CodtSpec {
    const RESET_VALUE: u32 = 0;
}
