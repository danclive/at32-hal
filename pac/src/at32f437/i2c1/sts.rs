#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `TDBE` reader - Transmit data buffer empty flag"]
pub type TdbeR = crate::BitReader;
#[doc = "Field `TDBE` writer - Transmit data buffer empty flag"]
pub type TdbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIS` reader - Send interrupt status"]
pub type TdisR = crate::BitReader;
#[doc = "Field `TDIS` writer - Send interrupt status"]
pub type TdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBF` reader - Receive data buffer full flag"]
pub type RdbfR = crate::BitReader;
#[doc = "Field `ADDRF` reader - 0~7 bit address match flag"]
pub type AddrfR = crate::BitReader;
#[doc = "Field `ACKFAIL` reader - Acknowledge failure flag"]
pub type AckfailR = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop condition generation complete flag"]
pub type StopfR = crate::BitReader;
#[doc = "Field `TDC` reader - Transmit data complete flag"]
pub type TdcR = crate::BitReader;
#[doc = "Field `TCRLD` reader - Transmission is complete, waiting to load data"]
pub type TcrldR = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus error flag"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `ARLOST` reader - Arbitration lost flag"]
pub type ArlostR = crate::BitReader;
#[doc = "Field `OUF` reader - Overflow or underflow flag"]
pub type OufR = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC receive error flag"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `TMOUT` reader - SMBus timeout flag"]
pub type TmoutR = crate::BitReader;
#[doc = "Field `ALERTF` reader - SMBus alert flag"]
pub type AlertfR = crate::BitReader;
#[doc = "Field `BUSYF` reader - Bus busy"]
pub type BusyfR = crate::BitReader;
#[doc = "Field `SDIR` reader - Slave data transmit direction"]
pub type SdirR = crate::BitReader;
#[doc = "Field `ADDR` reader - Slave address matching value"]
pub type AddrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data buffer empty flag"]
    #[inline(always)]
    pub fn tdbe(&self) -> TdbeR {
        TdbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send interrupt status"]
    #[inline(always)]
    pub fn tdis(&self) -> TdisR {
        TdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data buffer full flag"]
    #[inline(always)]
    pub fn rdbf(&self) -> RdbfR {
        RdbfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0~7 bit address match flag"]
    #[inline(always)]
    pub fn addrf(&self) -> AddrfR {
        AddrfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge failure flag"]
    #[inline(always)]
    pub fn ackfail(&self) -> AckfailR {
        AckfailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop condition generation complete flag"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data complete flag"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission is complete, waiting to load data"]
    #[inline(always)]
    pub fn tcrld(&self) -> TcrldR {
        TcrldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost flag"]
    #[inline(always)]
    pub fn arlost(&self) -> ArlostR {
        ArlostR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overflow or underflow flag"]
    #[inline(always)]
    pub fn ouf(&self) -> OufR {
        OufR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC receive error flag"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SMBus timeout flag"]
    #[inline(always)]
    pub fn tmout(&self) -> TmoutR {
        TmoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert flag"]
    #[inline(always)]
    pub fn alertf(&self) -> AlertfR {
        AlertfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline(always)]
    pub fn busyf(&self) -> BusyfR {
        BusyfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave data transmit direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SdirR {
        SdirR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Slave address matching value"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("addr", &self.addr())
            .field("sdir", &self.sdir())
            .field("busyf", &self.busyf())
            .field("alertf", &self.alertf())
            .field("tmout", &self.tmout())
            .field("pecerr", &self.pecerr())
            .field("ouf", &self.ouf())
            .field("arlost", &self.arlost())
            .field("buserr", &self.buserr())
            .field("tcrld", &self.tcrld())
            .field("tdc", &self.tdc())
            .field("stopf", &self.stopf())
            .field("ackfail", &self.ackfail())
            .field("addrf", &self.addrf())
            .field("rdbf", &self.rdbf())
            .field("tdis", &self.tdis())
            .field("tdbe", &self.tdbe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data buffer empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn tdbe(&mut self) -> TdbeW<StsSpec> {
        TdbeW::new(self, 0)
    }
    #[doc = "Bit 1 - Send interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn tdis(&mut self) -> TdisW<StsSpec> {
        TdisW::new(self, 1)
    }
}
#[doc = "Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0x01"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0x01;
}
