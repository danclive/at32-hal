#[doc = "Register `CMDSTS` reader"]
pub type R = crate::R<CmdstsSpec>;
#[doc = "Field `CMDSTS` reader - Command complete status"]
pub type CmdstsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command complete status"]
    #[inline(always)]
    pub fn cmdsts(&self) -> CmdstsR {
        CmdstsR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDSTS")
            .field("cmdsts", &self.cmdsts())
            .finish()
    }
}
#[doc = "CMD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdstsSpec;
impl crate::RegisterSpec for CmdstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdsts::R`](R) reader structure"]
impl crate::Readable for CmdstsSpec {}
#[doc = "`reset()` method sets CMDSTS to value 0"]
impl crate::Resettable for CmdstsSpec {
    const RESET_VALUE: u32 = 0;
}
