#[doc = "Register `DTXFSTS4` reader"]
pub type R = crate::R<Dtxfsts4Spec>;
#[doc = "Field `INEPTXFSAV` reader - IN endpoint TxFIFO space available"]
pub type IneptxfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptxfsav(&self) -> IneptxfsavR {
        IneptxfsavR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS4")
            .field("ineptxfsav", &self.ineptxfsav())
            .finish()
    }
}
#[doc = "OTGFS device IN endpoint-4 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts4Spec;
impl crate::RegisterSpec for Dtxfsts4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts4::R`](R) reader structure"]
impl crate::Readable for Dtxfsts4Spec {}
#[doc = "`reset()` method sets DTXFSTS4 to value 0"]
impl crate::Resettable for Dtxfsts4Spec {
    const RESET_VALUE: u32 = 0;
}
