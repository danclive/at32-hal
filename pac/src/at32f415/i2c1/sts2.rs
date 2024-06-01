#[doc = "Register `STS2` reader"]
pub type R = crate::R<Sts2Spec>;
#[doc = "Field `TRMODE` reader - Transmission mode"]
pub type TrmodeR = crate::BitReader;
#[doc = "Field `BUSYF` reader - Bus busy"]
pub type BusyfR = crate::BitReader;
#[doc = "Field `DIRF` reader - Transmission direction"]
pub type DirfR = crate::BitReader;
#[doc = "Field `GCADDRF` reader - General call address reception"]
pub type GcaddrfR = crate::BitReader;
#[doc = "Field `DEVADDRF` reader - SMBus device address receiving"]
pub type DevaddrfR = crate::BitReader;
#[doc = "Field `HOSTADDRF` reader - SMBus host address receiving"]
pub type HostaddrfR = crate::BitReader;
#[doc = "Field `ADDR2F` reader - Received address 2"]
pub type Addr2fR = crate::BitReader;
#[doc = "Field `PECVAL` reader - PEC value"]
pub type PecvalR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmission mode"]
    #[inline(always)]
    pub fn trmode(&self) -> TrmodeR {
        TrmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busyf(&self) -> BusyfR {
        BusyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission direction"]
    #[inline(always)]
    pub fn dirf(&self) -> DirfR {
        DirfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address reception"]
    #[inline(always)]
    pub fn gcaddrf(&self) -> GcaddrfR {
        GcaddrfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device address receiving"]
    #[inline(always)]
    pub fn devaddrf(&self) -> DevaddrfR {
        DevaddrfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host address receiving"]
    #[inline(always)]
    pub fn hostaddrf(&self) -> HostaddrfR {
        HostaddrfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received address 2"]
    #[inline(always)]
    pub fn addr2f(&self) -> Addr2fR {
        Addr2fR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PEC value"]
    #[inline(always)]
    pub fn pecval(&self) -> PecvalR {
        PecvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS2")
            .field("pecval", &self.pecval())
            .field("addr2f", &self.addr2f())
            .field("hostaddrf", &self.hostaddrf())
            .field("devaddrf", &self.devaddrf())
            .field("gcaddrf", &self.gcaddrf())
            .field("dirf", &self.dirf())
            .field("busyf", &self.busyf())
            .field("trmode", &self.trmode())
            .finish()
    }
}
#[doc = "Status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts2Spec;
impl crate::RegisterSpec for Sts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for Sts2Spec {}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for Sts2Spec {
    const RESET_VALUE: u32 = 0;
}
