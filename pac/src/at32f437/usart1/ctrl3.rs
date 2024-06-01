#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<Ctrl3Spec>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<Ctrl3Spec>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDAEN` reader - IrDA enable"]
pub type IrdaenR = crate::BitReader;
#[doc = "Field `IRDAEN` writer - IrDA enable"]
pub type IrdaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDALP` reader - IrDA low-power mode"]
pub type IrdalpR = crate::BitReader;
#[doc = "Field `IRDALP` writer - IrDA low-power mode"]
pub type IrdalpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SlbenR = crate::BitReader;
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SlbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNACKEN` reader - Smartcard NACK enable"]
pub type ScnackenR = crate::BitReader;
#[doc = "Field `SCNACKEN` writer - Smartcard NACK enable"]
pub type ScnackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMEN` reader - Smartcard mode enable"]
pub type ScmenR = crate::BitReader;
#[doc = "Field `SCMEN` writer - Smartcard mode enable"]
pub type ScmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - DMA receiver enable"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receiver enable"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATEN` reader - DMA transmitter enable"]
pub type DmatenR = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA transmitter enable"]
pub type DmatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCFIEN` reader - CTSCF interrupt enable"]
pub type CtscfienR = crate::BitReader;
#[doc = "Field `CTSCFIEN` writer - CTSCF interrupt enable"]
pub type CtscfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485EN` reader - RS485 enable"]
pub type Rs485enR = crate::BitReader;
#[doc = "Field `RS485EN` writer - RS485 enable"]
pub type Rs485enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - DE polarity selection"]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - DE polarity selection"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    pub fn irdaen(&self) -> IrdaenR {
        IrdaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    pub fn irdalp(&self) -> IrdalpR {
        IrdalpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&self) -> SlbenR {
        SlbenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn scnacken(&self) -> ScnackenR {
        ScnackenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scmen(&self) -> ScmenR {
        ScmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DmatenR {
        DmatenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    pub fn ctscfien(&self) -> CtscfienR {
        CtscfienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - RS485 enable"]
    #[inline(always)]
    pub fn rs485en(&self) -> Rs485enR {
        Rs485enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DE polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL3")
            .field("dep", &self.dep())
            .field("rs485en", &self.rs485en())
            .field("ctscfien", &self.ctscfien())
            .field("ctsen", &self.ctsen())
            .field("rtsen", &self.rtsen())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .field("scmen", &self.scmen())
            .field("scnacken", &self.scnacken())
            .field("slben", &self.slben())
            .field("irdalp", &self.irdalp())
            .field("irdaen", &self.irdaen())
            .field("errien", &self.errien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ErrienW<Ctrl3Spec> {
        ErrienW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    #[must_use]
    pub fn irdaen(&mut self) -> IrdaenW<Ctrl3Spec> {
        IrdaenW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn irdalp(&mut self) -> IrdalpW<Ctrl3Spec> {
        IrdalpW::new(self, 2)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SlbenW<Ctrl3Spec> {
        SlbenW::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn scnacken(&mut self) -> ScnackenW<Ctrl3Spec> {
        ScnackenW::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scmen(&mut self) -> ScmenW<Ctrl3Spec> {
        ScmenW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DmarenW<Ctrl3Spec> {
        DmarenW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DmatenW<Ctrl3Spec> {
        DmatenW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<Ctrl3Spec> {
        RtsenW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<Ctrl3Spec> {
        CtsenW::new(self, 9)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctscfien(&mut self) -> CtscfienW<Ctrl3Spec> {
        CtscfienW::new(self, 10)
    }
    #[doc = "Bit 14 - RS485 enable"]
    #[inline(always)]
    #[must_use]
    pub fn rs485en(&mut self) -> Rs485enW<Ctrl3Spec> {
        Rs485enW::new(self, 14)
    }
    #[doc = "Bit 15 - DE polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DepW<Ctrl3Spec> {
        DepW::new(self, 15)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl3Spec;
impl crate::RegisterSpec for Ctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for Ctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for Ctrl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for Ctrl3Spec {
    const RESET_VALUE: u32 = 0;
}
