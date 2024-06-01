#[doc = "Register `DTXFSTS7` reader"]
pub type R = crate::R<Dtxfsts7Spec>;
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
        f.debug_struct("DTXFSTS7")
            .field("ineptxfsav", &self.ineptxfsav())
            .finish()
    }
}
#[doc = "OTGFS device IN endpoint-7 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts7Spec;
impl crate::RegisterSpec for Dtxfsts7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts7::R`](R) reader structure"]
impl crate::Readable for Dtxfsts7Spec {}
#[doc = "`reset()` method sets DTXFSTS7 to value 0"]
impl crate::Resettable for Dtxfsts7Spec {
    const RESET_VALUE: u32 = 0;
}
