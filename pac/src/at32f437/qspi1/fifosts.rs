#[doc = "Register `FIFOSTS` reader"]
pub type R = crate::R<FifostsSpec>;
#[doc = "Field `TXFIFORDY` reader - TxFIFO ready status"]
pub type TxfifordyR = crate::BitReader;
#[doc = "Field `RXFIFORDY` reader - RxFIFO ready status"]
pub type RxfifordyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TxFIFO ready status"]
    #[inline(always)]
    pub fn txfifordy(&self) -> TxfifordyR {
        TxfifordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RxFIFO ready status"]
    #[inline(always)]
    pub fn rxfifordy(&self) -> RxfifordyR {
        RxfifordyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOSTS")
            .field("txfifordy", &self.txfifordy())
            .field("rxfifordy", &self.rxfifordy())
            .finish()
    }
}
#[doc = "FIFO Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifosts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifostsSpec;
impl crate::RegisterSpec for FifostsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifosts::R`](R) reader structure"]
impl crate::Readable for FifostsSpec {}
#[doc = "`reset()` method sets FIFOSTS to value 0x01"]
impl crate::Resettable for FifostsSpec {
    const RESET_VALUE: u32 = 0x01;
}
