#[doc = "Register `GRXSTSR_Host` reader"]
pub type R = crate::R<GrxstsrHostSpec>;
#[doc = "Field `CHNUM` reader - Channel number"]
pub type ChnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PktstsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Channel number"]
    #[inline(always)]
    pub fn chnum(&self) -> ChnumR {
        ChnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR_Host")
            .field("chnum", &self.chnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .finish()
    }
}
#[doc = "OTGFS Receive status debug read(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_host::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstsrHostSpec;
impl crate::RegisterSpec for GrxstsrHostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr_host::R`](R) reader structure"]
impl crate::Readable for GrxstsrHostSpec {}
#[doc = "`reset()` method sets GRXSTSR_Host to value 0"]
impl crate::Resettable for GrxstsrHostSpec {
    const RESET_VALUE: u32 = 0;
}
