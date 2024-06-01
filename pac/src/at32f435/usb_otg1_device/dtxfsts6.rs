#[doc = "Register `DTXFSTS6` reader"]
pub type R = crate::R<Dtxfsts6Spec>;
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
        f.debug_struct("DTXFSTS6")
            .field("ineptxfsav", &self.ineptxfsav())
            .finish()
    }
}
#[doc = "OTGFS device IN endpoint-6 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts6Spec;
impl crate::RegisterSpec for Dtxfsts6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts6::R`](R) reader structure"]
impl crate::Readable for Dtxfsts6Spec {}
#[doc = "`reset()` method sets DTXFSTS6 to value 0"]
impl crate::Resettable for Dtxfsts6Spec {
    const RESET_VALUE: u32 = 0;
}
