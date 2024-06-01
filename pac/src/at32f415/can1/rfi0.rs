#[doc = "Register `RFI0` reader"]
pub type R = crate::R<Rfi0Spec>;
#[doc = "Field `RFFRI` reader - Receive FIFO frame type indication"]
pub type RffriR = crate::BitReader;
#[doc = "Field `RFIDI` reader - Receive FIFO identifier type indication"]
pub type RfidiR = crate::BitReader;
#[doc = "Field `RFEID` reader - Receive FIFO extended identifier"]
pub type RfeidR = crate::FieldReader<u32>;
#[doc = "Field `RFSID` reader - Receive FIFO standard identifier or receive FIFO extended identifier"]
pub type RfsidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - Receive FIFO frame type indication"]
    #[inline(always)]
    pub fn rffri(&self) -> RffriR {
        RffriR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO identifier type indication"]
    #[inline(always)]
    pub fn rfidi(&self) -> RfidiR {
        RfidiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Receive FIFO extended identifier"]
    #[inline(always)]
    pub fn rfeid(&self) -> RfeidR {
        RfeidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Receive FIFO standard identifier or receive FIFO extended identifier"]
    #[inline(always)]
    pub fn rfsid(&self) -> RfsidR {
        RfsidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFI0")
            .field("rfsid", &self.rfsid())
            .field("rfeid", &self.rfeid())
            .field("rfidi", &self.rfidi())
            .field("rffri", &self.rffri())
            .finish()
    }
}
#[doc = "Receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfi0Spec;
impl crate::RegisterSpec for Rfi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfi0::R`](R) reader structure"]
impl crate::Readable for Rfi0Spec {}
#[doc = "`reset()` method sets RFI0 to value 0"]
impl crate::Resettable for Rfi0Spec {
    const RESET_VALUE: u32 = 0;
}
