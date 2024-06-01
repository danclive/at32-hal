#[doc = "Register `STS1` reader"]
pub type R = crate::R<Sts1Spec>;
#[doc = "Register `STS1` writer"]
pub type W = crate::W<Sts1Spec>;
#[doc = "Field `STARTF` reader - Start bit (Master mode)"]
pub type StartfR = crate::BitReader;
#[doc = "Field `ADDR7F` reader - Address sent (master mode)/matched (slave mode)"]
pub type Addr7fR = crate::BitReader;
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TdcR = crate::BitReader;
#[doc = "Field `ADDRHF` reader - address header match (Master mode)"]
pub type AddrhfR = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type StopfR = crate::BitReader;
#[doc = "Field `RDBF` reader - Receive data buffer full (receivers)"]
pub type RdbfR = crate::BitReader;
#[doc = "Field `TDBE` reader - Transmit data buffer empty (transmitters)"]
pub type TdbeR = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus error"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus error"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOST` reader - Arbitration lost (master mode)"]
pub type ArlostR = crate::BitReader;
#[doc = "Field `ARLOST` writer - Arbitration lost (master mode)"]
pub type ArlostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAIL` reader - Acknowledge failure"]
pub type AckfailR = crate::BitReader;
#[doc = "Field `ACKFAIL` writer - Acknowledge failure"]
pub type AckfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUF` reader - Overflow or underflow"]
pub type OufR = crate::BitReader;
#[doc = "Field `OUF` writer - Overflow or underflow"]
pub type OufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` reader - PEC receive error"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `PECERR` writer - PEC receive error"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUT` reader - Timeout error"]
pub type TmoutR = crate::BitReader;
#[doc = "Field `TMOUT` writer - Timeout error"]
pub type TmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTF` reader - SMBus alert"]
pub type AlertfR = crate::BitReader;
#[doc = "Field `ALERTF` writer - SMBus alert"]
pub type AlertfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr7f(&self) -> Addr7fR {
        Addr7fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - address header match (Master mode)"]
    #[inline(always)]
    pub fn addrhf(&self) -> AddrhfR {
        AddrhfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive data buffer full (receivers)"]
    #[inline(always)]
    pub fn rdbf(&self) -> RdbfR {
        RdbfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty (transmitters)"]
    #[inline(always)]
    pub fn tdbe(&self) -> TdbeR {
        TdbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlost(&self) -> ArlostR {
        ArlostR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn ackfail(&self) -> AckfailR {
        AckfailR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overflow or underflow"]
    #[inline(always)]
    pub fn ouf(&self) -> OufR {
        OufR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC receive error"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout error"]
    #[inline(always)]
    pub fn tmout(&self) -> TmoutR {
        TmoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn alertf(&self) -> AlertfR {
        AlertfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS1")
            .field("alertf", &self.alertf())
            .field("tmout", &self.tmout())
            .field("pecerr", &self.pecerr())
            .field("ouf", &self.ouf())
            .field("ackfail", &self.ackfail())
            .field("arlost", &self.arlost())
            .field("buserr", &self.buserr())
            .field("tdbe", &self.tdbe())
            .field("rdbf", &self.rdbf())
            .field("stopf", &self.stopf())
            .field("addrhf", &self.addrhf())
            .field("tdc", &self.tdc())
            .field("addr7f", &self.addr7f())
            .field("startf", &self.startf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BuserrW<Sts1Spec> {
        BuserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn arlost(&mut self) -> ArlostW<Sts1Spec> {
        ArlostW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    #[must_use]
    pub fn ackfail(&mut self) -> AckfailW<Sts1Spec> {
        AckfailW::new(self, 10)
    }
    #[doc = "Bit 11 - Overflow or underflow"]
    #[inline(always)]
    #[must_use]
    pub fn ouf(&mut self) -> OufW<Sts1Spec> {
        OufW::new(self, 11)
    }
    #[doc = "Bit 12 - PEC receive error"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PecerrW<Sts1Spec> {
        PecerrW::new(self, 12)
    }
    #[doc = "Bit 14 - Timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn tmout(&mut self) -> TmoutW<Sts1Spec> {
        TmoutW::new(self, 14)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn alertf(&mut self) -> AlertfW<Sts1Spec> {
        AlertfW::new(self, 15)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts1Spec;
impl crate::RegisterSpec for Sts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for Sts1Spec {}
#[doc = "`write(|w| ..)` method takes [`sts1::W`](W) writer structure"]
impl crate::Writable for Sts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for Sts1Spec {
    const RESET_VALUE: u32 = 0;
}
