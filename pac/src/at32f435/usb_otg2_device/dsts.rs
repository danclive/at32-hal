#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DstsSpec>;
#[doc = "Field `SUSPSTS` reader - Suspend status"]
pub type SuspstsR = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated speed"]
pub type EnumspdR = crate::FieldReader;
#[doc = "Field `ETICERR` reader - Erratic error"]
pub type EticerrR = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame number of the received SOF"]
pub type SoffnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SuspstsR {
        SuspstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> EnumspdR {
        EnumspdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic error"]
    #[inline(always)]
    pub fn eticerr(&self) -> EticerrR {
        EticerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SoffnR {
        SoffnR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("eticerr", &self.eticerr())
            .field("soffn", &self.soffn())
            .finish()
    }
}
#[doc = "OTGFS device status register (OTGFS_DSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstsSpec;
impl crate::RegisterSpec for DstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DstsSpec {}
#[doc = "`reset()` method sets DSTS to value 0x10"]
impl crate::Resettable for DstsSpec {
    const RESET_VALUE: u32 = 0x10;
}
