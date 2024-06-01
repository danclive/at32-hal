#[doc = "Register `DMACTD` reader"]
pub type R = crate::R<DmactdSpec>;
#[doc = "Field `HTDAP` reader - Host transmit descriptor address pointer"]
pub type HtdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit descriptor address pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HtdapR {
        HtdapR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTD")
            .field("htdap", &self.htdap())
            .finish()
    }
}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactdSpec;
impl crate::RegisterSpec for DmactdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactd::R`](R) reader structure"]
impl crate::Readable for DmactdSpec {}
#[doc = "`reset()` method sets DMACTD to value 0"]
impl crate::Resettable for DmactdSpec {
    const RESET_VALUE: u32 = 0;
}
