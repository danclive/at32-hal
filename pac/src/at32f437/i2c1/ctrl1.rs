#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIEN` reader - Transmit data interrupt enable"]
pub type TdienR = crate::BitReader;
#[doc = "Field `TDIEN` writer - Transmit data interrupt enable"]
pub type TdienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIEN` reader - Receive data interrupt enable"]
pub type RdienR = crate::BitReader;
#[doc = "Field `RDIEN` writer - Receive data interrupt enable"]
pub type RdienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRIEN` reader - Address match interrupt enable"]
pub type AddrienR = crate::BitReader;
#[doc = "Field `ADDRIEN` writer - Address match interrupt enable"]
pub type AddrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILIEN` reader - Acknowledge fail interrupt enable"]
pub type AckfailienR = crate::BitReader;
#[doc = "Field `ACKFAILIEN` writer - Acknowledge fail interrupt enable"]
pub type AckfailienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIEN` reader - Stop generation complete interrupt enable"]
pub type StopienR = crate::BitReader;
#[doc = "Field `STOPIEN` writer - Stop generation complete interrupt enable"]
pub type StopienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCIEN` reader - Transfer data complete interrupt enable"]
pub type TdcienR = crate::BitReader;
#[doc = "Field `TDCIEN` writer - Transfer data complete interrupt enable"]
pub type TdcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIEN` reader - Error interrupts enable"]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupts enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLT` reader - Digital filter value"]
pub type DfltR = crate::FieldReader;
#[doc = "Field `DFLT` writer - Digital filter value"]
pub type DfltW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMATEN` reader - DMA Transmit data request enable"]
pub type DmatenR = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA Transmit data request enable"]
pub type DmatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - DMA receive data request enable"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receive data request enable"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTRL` reader - Slave receiving data control"]
pub type SctrlR = crate::BitReader;
#[doc = "Field `SCTRL` writer - Slave receiving data control"]
pub type SctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRETCH` reader - Clock stretching mode"]
pub type StretchR = crate::BitReader;
#[doc = "Field `STRETCH` writer - Clock stretching mode"]
pub type StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCAEN` reader - General call address enable"]
pub type GcaenR = crate::BitReader;
#[doc = "Field `GCAEN` writer - General call address enable"]
pub type GcaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HADDREN` reader - SMBus host address enable"]
pub type HaddrenR = crate::BitReader;
#[doc = "Field `HADDREN` writer - SMBus host address enable"]
pub type HaddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDREN` reader - SMBus device default address enable"]
pub type DevaddrenR = crate::BitReader;
#[doc = "Field `DEVADDREN` writer - SMBus device default address enable"]
pub type DevaddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALERT` reader - SMBus alert enable / pin set"]
pub type SmbalertR = crate::BitReader;
#[doc = "Field `SMBALERT` writer - SMBus alert enable / pin set"]
pub type SmbalertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC calculation enable"]
pub type PecenR = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC calculation enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data interrupt enable"]
    #[inline(always)]
    pub fn tdien(&self) -> TdienR {
        TdienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data interrupt enable"]
    #[inline(always)]
    pub fn rdien(&self) -> RdienR {
        RdienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable"]
    #[inline(always)]
    pub fn addrien(&self) -> AddrienR {
        AddrienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge fail interrupt enable"]
    #[inline(always)]
    pub fn ackfailien(&self) -> AckfailienR {
        AckfailienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop generation complete interrupt enable"]
    #[inline(always)]
    pub fn stopien(&self) -> StopienR {
        StopienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer data complete interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&self) -> TdcienR {
        TdcienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital filter value"]
    #[inline(always)]
    pub fn dflt(&self) -> DfltR {
        DfltR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DMA Transmit data request enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DmatenR {
        DmatenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA receive data request enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave receiving data control"]
    #[inline(always)]
    pub fn sctrl(&self) -> SctrlR {
        SctrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&self) -> StretchR {
        StretchR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GcaenR {
        GcaenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus host address enable"]
    #[inline(always)]
    pub fn haddren(&self) -> HaddrenR {
        HaddrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn devaddren(&self) -> DevaddrenR {
        DevaddrenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable / pin set"]
    #[inline(always)]
    pub fn smbalert(&self) -> SmbalertR {
        SmbalertR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("i2cen", &self.i2cen())
            .field("tdien", &self.tdien())
            .field("rdien", &self.rdien())
            .field("addrien", &self.addrien())
            .field("ackfailien", &self.ackfailien())
            .field("stopien", &self.stopien())
            .field("tdcien", &self.tdcien())
            .field("errien", &self.errien())
            .field("dflt", &self.dflt())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .field("sctrl", &self.sctrl())
            .field("stretch", &self.stretch())
            .field("gcaen", &self.gcaen())
            .field("haddren", &self.haddren())
            .field("devaddren", &self.devaddren())
            .field("smbalert", &self.smbalert())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2cenW<Ctrl1Spec> {
        I2cenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit data interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdien(&mut self) -> TdienW<Ctrl1Spec> {
        TdienW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive data interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdien(&mut self) -> RdienW<Ctrl1Spec> {
        RdienW::new(self, 2)
    }
    #[doc = "Bit 3 - Address match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrien(&mut self) -> AddrienW<Ctrl1Spec> {
        AddrienW::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailien(&mut self) -> AckfailienW<Ctrl1Spec> {
        AckfailienW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop generation complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopien(&mut self) -> StopienW<Ctrl1Spec> {
        StopienW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer data complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcien(&mut self) -> TdcienW<Ctrl1Spec> {
        TdcienW::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ErrienW<Ctrl1Spec> {
        ErrienW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital filter value"]
    #[inline(always)]
    #[must_use]
    pub fn dflt(&mut self) -> DfltW<Ctrl1Spec> {
        DfltW::new(self, 8)
    }
    #[doc = "Bit 14 - DMA Transmit data request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DmatenW<Ctrl1Spec> {
        DmatenW::new(self, 14)
    }
    #[doc = "Bit 15 - DMA receive data request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DmarenW<Ctrl1Spec> {
        DmarenW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave receiving data control"]
    #[inline(always)]
    #[must_use]
    pub fn sctrl(&mut self) -> SctrlW<Ctrl1Spec> {
        SctrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching mode"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> StretchW<Ctrl1Spec> {
        StretchW::new(self, 17)
    }
    #[doc = "Bit 19 - General call address enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcaen(&mut self) -> GcaenW<Ctrl1Spec> {
        GcaenW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus host address enable"]
    #[inline(always)]
    #[must_use]
    pub fn haddren(&mut self) -> HaddrenW<Ctrl1Spec> {
        HaddrenW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    #[must_use]
    pub fn devaddren(&mut self) -> DevaddrenW<Ctrl1Spec> {
        DevaddrenW::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable / pin set"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SmbalertW<Ctrl1Spec> {
        SmbalertW::new(self, 22)
    }
    #[doc = "Bit 23 - PEC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PecenW<Ctrl1Spec> {
        PecenW::new(self, 23)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
