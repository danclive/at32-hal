#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `ERR` reader - error flag"]
pub type ErrR = crate::BitReader;
#[doc = "Field `BK1STS` reader - Bank 1 Status"]
pub type Bk1stsR = crate::FieldReader;
#[doc = "Field `BK2STS` reader - Bank 2 Status"]
pub type Bk2stsR = crate::FieldReader;
#[doc = "Field `BUSY` reader - Busy status"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - error flag"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Bank 1 Status"]
    #[inline(always)]
    pub fn bk1sts(&self) -> Bk1stsR {
        Bk1stsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Bank 2 Status"]
    #[inline(always)]
    pub fn bk2sts(&self) -> Bk2stsR {
        Bk2stsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("err", &self.err())
            .field("bk1sts", &self.bk1sts())
            .field("bk2sts", &self.bk2sts())
            .field("busy", &self.busy())
            .finish()
    }
}
#[doc = "SDRAM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
