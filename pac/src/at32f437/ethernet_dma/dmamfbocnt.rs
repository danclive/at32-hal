#[doc = "Register `DMAMFBOCNT` reader"]
pub type R = crate::R<DmamfbocntSpec>;
#[doc = "Field `MFC` reader - Missed frames control"]
pub type MfcR = crate::FieldReader<u16>;
#[doc = "Field `OBMFC` reader - Overflow bit for missed frame counter"]
pub type ObmfcR = crate::BitReader;
#[doc = "Field `OFC` reader - Overflow frame counter"]
pub type OfcR = crate::FieldReader<u16>;
#[doc = "Field `OBFOC` reader - Overflow bit for FIFO overflow counter"]
pub type ObfocR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Missed frames control"]
    #[inline(always)]
    pub fn mfc(&self) -> MfcR {
        MfcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn obmfc(&self) -> ObmfcR {
        ObmfcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow frame counter"]
    #[inline(always)]
    pub fn ofc(&self) -> OfcR {
        OfcR::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn obfoc(&self) -> ObfocR {
        ObfocR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMFBOCNT")
            .field("mfc", &self.mfc())
            .field("obmfc", &self.obmfc())
            .field("ofc", &self.ofc())
            .field("obfoc", &self.obfoc())
            .finish()
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamfbocntSpec;
impl crate::RegisterSpec for DmamfbocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamfbocnt::R`](R) reader structure"]
impl crate::Readable for DmamfbocntSpec {}
#[doc = "`reset()` method sets DMAMFBOCNT to value 0"]
impl crate::Resettable for DmamfbocntSpec {
    const RESET_VALUE: u32 = 0;
}
