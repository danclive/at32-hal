#[doc = "Register `PTPTSSR` reader"]
pub type R = crate::R<PtptssrSpec>;
#[doc = "Field `TSO` reader - Timestamp second overflow"]
pub type TsoR = crate::BitReader;
#[doc = "Field `TTTR` reader - Timestamp target time reached"]
pub type TttrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timestamp second overflow"]
    #[inline(always)]
    pub fn tso(&self) -> TsoR {
        TsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp target time reached"]
    #[inline(always)]
    pub fn tttr(&self) -> TttrR {
        TttrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSSR")
            .field("tso", &self.tso())
            .field("tttr", &self.tttr())
            .finish()
    }
}
#[doc = "Ethernet PTP time stamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptssrSpec;
impl crate::RegisterSpec for PtptssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptssr::R`](R) reader structure"]
impl crate::Readable for PtptssrSpec {}
#[doc = "`reset()` method sets PTPTSSR to value 0"]
impl crate::Resettable for PtptssrSpec {
    const RESET_VALUE: u32 = 0;
}
