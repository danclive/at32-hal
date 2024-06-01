#[doc = "Register `DMACRD` reader"]
pub type R = crate::R<DmacrdSpec>;
#[doc = "Field `HRDAP` reader - Host receive descriptor address pointer"]
pub type HrdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive descriptor address pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HrdapR {
        HrdapR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRD")
            .field("hrdap", &self.hrdap())
            .finish()
    }
}
#[doc = "Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrdSpec;
impl crate::RegisterSpec for DmacrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrd::R`](R) reader structure"]
impl crate::Readable for DmacrdSpec {}
#[doc = "`reset()` method sets DMACRD to value 0"]
impl crate::Resettable for DmacrdSpec {
    const RESET_VALUE: u32 = 0;
}
