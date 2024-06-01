#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `WATCLK` reader - Wakeup timer clock selection"]
pub type WatclkR = crate::FieldReader;
#[doc = "Field `WATCLK` writer - Wakeup timer clock selection"]
pub type WatclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDG` reader - Timestamp trigger edge"]
pub type TsedgR = crate::BitReader;
#[doc = "Field `TSEDG` writer - Timestamp trigger edge"]
pub type TsedgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCDEN` reader - Reference clock detection enable"]
pub type RcdenR = crate::BitReader;
#[doc = "Field `RCDEN` writer - Reference clock detection enable"]
pub type RcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREN` reader - Date/time register direct read enable"]
pub type DrenR = crate::BitReader;
#[doc = "Field `DREN` writer - Date/time register direct read enable"]
pub type DrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HM` reader - Hour mode"]
pub type HmR = crate::BitReader;
#[doc = "Field `HM` writer - Hour mode"]
pub type HmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCALEN` reader - Coarse calibration enable"]
pub type CcalenR = crate::BitReader;
#[doc = "Field `CCALEN` writer - Coarse calibration enable"]
pub type CcalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALAEN` reader - Alarm A enable"]
pub type AlaenR = crate::BitReader;
#[doc = "Field `ALAEN` writer - Alarm A enable"]
pub type AlaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALBEN` reader - Alarm B enable"]
pub type AlbenR = crate::BitReader;
#[doc = "Field `ALBEN` writer - Alarm B enable"]
pub type AlbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATEN` reader - Wakeup timer enable"]
pub type WatenR = crate::BitReader;
#[doc = "Field `WATEN` writer - Wakeup timer enable"]
pub type WatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - Timestamp enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - Timestamp enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALAIEN` reader - Alarm A interrupt enable"]
pub type AlaienR = crate::BitReader;
#[doc = "Field `ALAIEN` writer - Alarm A interrupt enable"]
pub type AlaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALBIEN` reader - Alarm B interrupt enable"]
pub type AlbienR = crate::BitReader;
#[doc = "Field `ALBIEN` writer - Alarm B interrupt enable"]
pub type AlbienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATIEN` reader - Wakeup timer interrupt enable"]
pub type WatienR = crate::BitReader;
#[doc = "Field `WATIEN` writer - Wakeup timer interrupt enable"]
pub type WatienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIEN` reader - Timestamp interrupt enable"]
pub type TsienR = crate::BitReader;
#[doc = "Field `TSIEN` writer - Timestamp interrupt enable"]
pub type TsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` reader - Add 1 hour"]
pub type Add1hR = crate::BitReader;
#[doc = "Field `ADD1H` writer - Add 1 hour"]
pub type Add1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC1H` reader - Decrease 1 hour"]
pub type Dec1hR = crate::BitReader;
#[doc = "Field `DEC1H` writer - Decrease 1 hour"]
pub type Dec1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPR` reader - Battery power domain data register"]
pub type BprR = crate::BitReader;
#[doc = "Field `BPR` writer - Battery power domain data register"]
pub type BprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOSEL` reader - Calibration output selection"]
pub type CaloselR = crate::BitReader;
#[doc = "Field `CALOSEL` writer - Calibration output selection"]
pub type CaloselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTP` reader - Output polarity"]
pub type OutpR = crate::BitReader;
#[doc = "Field `OUTP` writer - Output polarity"]
pub type OutpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSEL` reader - Output source selection"]
pub type OutselR = crate::FieldReader;
#[doc = "Field `OUTSEL` writer - Output source selection"]
pub type OutselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CALOEN` reader - Calibration output enable"]
pub type CaloenR = crate::BitReader;
#[doc = "Field `CALOEN` writer - Calibration output enable"]
pub type CaloenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wakeup timer clock selection"]
    #[inline(always)]
    pub fn watclk(&self) -> WatclkR {
        WatclkR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    pub fn tsedg(&self) -> TsedgR {
        TsedgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    pub fn rcden(&self) -> RcdenR {
        RcdenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    pub fn dren(&self) -> DrenR {
        DrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    pub fn hm(&self) -> HmR {
        HmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Coarse calibration enable"]
    #[inline(always)]
    pub fn ccalen(&self) -> CcalenR {
        CcalenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alaen(&self) -> AlaenR {
        AlaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alben(&self) -> AlbenR {
        AlbenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn waten(&self) -> WatenR {
        WatenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alaien(&self) -> AlaienR {
        AlaienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn albien(&self) -> AlbienR {
        AlbienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn watien(&self) -> WatienR {
        WatienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TsienR {
        TsienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    pub fn add1h(&self) -> Add1hR {
        Add1hR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    pub fn dec1h(&self) -> Dec1hR {
        Dec1hR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    pub fn bpr(&self) -> BprR {
        BprR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn calosel(&self) -> CaloselR {
        CaloselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn outp(&self) -> OutpR {
        OutpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OutselR {
        OutselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn caloen(&self) -> CaloenR {
        CaloenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("caloen", &self.caloen())
            .field("outsel", &self.outsel())
            .field("outp", &self.outp())
            .field("calosel", &self.calosel())
            .field("bpr", &self.bpr())
            .field("dec1h", &self.dec1h())
            .field("add1h", &self.add1h())
            .field("tsien", &self.tsien())
            .field("watien", &self.watien())
            .field("albien", &self.albien())
            .field("alaien", &self.alaien())
            .field("tsen", &self.tsen())
            .field("waten", &self.waten())
            .field("alben", &self.alben())
            .field("alaen", &self.alaen())
            .field("ccalen", &self.ccalen())
            .field("hm", &self.hm())
            .field("dren", &self.dren())
            .field("rcden", &self.rcden())
            .field("tsedg", &self.tsedg())
            .field("watclk", &self.watclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup timer clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn watclk(&mut self) -> WatclkW<CtrlSpec> {
        WatclkW::new(self, 0)
    }
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedg(&mut self) -> TsedgW<CtrlSpec> {
        TsedgW::new(self, 3)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcden(&mut self) -> RcdenW<CtrlSpec> {
        RcdenW::new(self, 4)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    #[must_use]
    pub fn dren(&mut self) -> DrenW<CtrlSpec> {
        DrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HmW<CtrlSpec> {
        HmW::new(self, 6)
    }
    #[doc = "Bit 7 - Coarse calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccalen(&mut self) -> CcalenW<CtrlSpec> {
        CcalenW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaen(&mut self) -> AlaenW<CtrlSpec> {
        AlaenW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alben(&mut self) -> AlbenW<CtrlSpec> {
        AlbenW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn waten(&mut self) -> WatenW<CtrlSpec> {
        WatenW::new(self, 10)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<CtrlSpec> {
        TsenW::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaien(&mut self) -> AlaienW<CtrlSpec> {
        AlaienW::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn albien(&mut self) -> AlbienW<CtrlSpec> {
        AlbienW::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn watien(&mut self) -> WatienW<CtrlSpec> {
        WatienW::new(self, 14)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TsienW<CtrlSpec> {
        TsienW::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> Add1hW<CtrlSpec> {
        Add1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn dec1h(&mut self) -> Dec1hW<CtrlSpec> {
        Dec1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    #[must_use]
    pub fn bpr(&mut self) -> BprW<CtrlSpec> {
        BprW::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn calosel(&mut self) -> CaloselW<CtrlSpec> {
        CaloselW::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn outp(&mut self) -> OutpW<CtrlSpec> {
        OutpW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OutselW<CtrlSpec> {
        OutselW::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn caloen(&mut self) -> CaloenW<CtrlSpec> {
        CaloenW::new(self, 23)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
