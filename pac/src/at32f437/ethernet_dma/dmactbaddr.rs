#[doc = "Register `DMACTBADDR` reader"]
pub type R = crate::R<DmactbaddrSpec>;
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HtbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HtbapR {
        HtbapR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTBADDR")
            .field("htbap", &self.htbap())
            .finish()
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactbaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactbaddrSpec;
impl crate::RegisterSpec for DmactbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactbaddr::R`](R) reader structure"]
impl crate::Readable for DmactbaddrSpec {}
#[doc = "`reset()` method sets DMACTBADDR to value 0"]
impl crate::Resettable for DmactbaddrSpec {
    const RESET_VALUE: u32 = 0;
}
