#[doc = "Register `PTPPPSCR` reader"]
pub type R = crate::R<PtpppscrSpec>;
#[doc = "Field `POFC` reader - PPS Output frequency control"]
pub type PofcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PPS Output frequency control"]
    #[inline(always)]
    pub fn pofc(&self) -> PofcR {
        PofcR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPPPSCR")
            .field("pofc", &self.pofc())
            .finish()
    }
}
#[doc = "Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpppscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpppscrSpec;
impl crate::RegisterSpec for PtpppscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpppscr::R`](R) reader structure"]
impl crate::Readable for PtpppscrSpec {}
#[doc = "`reset()` method sets PTPPPSCR to value 0"]
impl crate::Resettable for PtpppscrSpec {
    const RESET_VALUE: u32 = 0;
}
