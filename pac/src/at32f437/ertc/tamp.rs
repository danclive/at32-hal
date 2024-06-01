#[doc = "Register `TAMP` reader"]
pub type R = crate::R<TampSpec>;
#[doc = "Register `TAMP` writer"]
pub type W = crate::W<TampSpec>;
#[doc = "Field `TP1EN` reader - Tamper detection 1 enable"]
pub type Tp1enR = crate::BitReader;
#[doc = "Field `TP1EN` writer - Tamper detection 1 enable"]
pub type Tp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1EDG` reader - Tamper detection 1 valid edge"]
pub type Tp1edgR = crate::BitReader;
#[doc = "Field `TP1EDG` writer - Tamper detection 1 valid edge"]
pub type Tp1edgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIEN` reader - Tamper detection interrupt enable"]
pub type TpienR = crate::BitReader;
#[doc = "Field `TPIEN` writer - Tamper detection interrupt enable"]
pub type TpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP2EN` reader - Tamper detection 2 enable"]
pub type Tp2enR = crate::BitReader;
#[doc = "Field `TP2EN` writer - Tamper detection 2 enable"]
pub type Tp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP2EDG` reader - Tamper detection 2 valid edge"]
pub type Tp2edgR = crate::BitReader;
#[doc = "Field `TP2EDG` writer - Tamper detection 2 valid edge"]
pub type Tp2edgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPTSEN` reader - Tamper detection timestamp enable"]
pub type TptsenR = crate::BitReader;
#[doc = "Field `TPTSEN` writer - Tamper detection timestamp enable"]
pub type TptsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPFREQ` reader - Tamper detection frequency"]
pub type TpfreqR = crate::FieldReader;
#[doc = "Field `TPFREQ` writer - Tamper detection frequency"]
pub type TpfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TPFLT` reader - Tamper detection filter time"]
pub type TpfltR = crate::FieldReader;
#[doc = "Field `TPFLT` writer - Tamper detection filter time"]
pub type TpfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TPPR` reader - Tamper detection pre-charge time"]
pub type TpprR = crate::FieldReader;
#[doc = "Field `TPPR` writer - Tamper detection pre-charge time"]
pub type TpprW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TPPU` reader - Tamper detection pull-up"]
pub type TppuR = crate::BitReader;
#[doc = "Field `TPPU` writer - Tamper detection pull-up"]
pub type TppuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1PIN` reader - Tamper detection pin selection"]
pub type Tp1pinR = crate::BitReader;
#[doc = "Field `TP1PIN` writer - Tamper detection pin selection"]
pub type Tp1pinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSPIN` reader - Time stamp detection pin selection"]
pub type TspinR = crate::BitReader;
#[doc = "Field `TSPIN` writer - Time stamp detection pin selection"]
pub type TspinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTYPE` reader - Output type"]
pub type OuttypeR = crate::BitReader;
#[doc = "Field `OUTTYPE` writer - Output type"]
pub type OuttypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper detection 1 enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> Tp1enR {
        Tp1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection 1 valid edge"]
    #[inline(always)]
    pub fn tp1edg(&self) -> Tp1edgR {
        Tp1edgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TpienR {
        TpienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper detection 2 enable"]
    #[inline(always)]
    pub fn tp2en(&self) -> Tp2enR {
        Tp2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper detection 2 valid edge"]
    #[inline(always)]
    pub fn tp2edg(&self) -> Tp2edgR {
        Tp2edgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper detection timestamp enable"]
    #[inline(always)]
    pub fn tptsen(&self) -> TptsenR {
        TptsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper detection frequency"]
    #[inline(always)]
    pub fn tpfreq(&self) -> TpfreqR {
        TpfreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper detection filter time"]
    #[inline(always)]
    pub fn tpflt(&self) -> TpfltR {
        TpfltR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper detection pre-charge time"]
    #[inline(always)]
    pub fn tppr(&self) -> TpprR {
        TpprR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Tamper detection pull-up"]
    #[inline(always)]
    pub fn tppu(&self) -> TppuR {
        TppuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper detection pin selection"]
    #[inline(always)]
    pub fn tp1pin(&self) -> Tp1pinR {
        Tp1pinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Time stamp detection pin selection"]
    #[inline(always)]
    pub fn tspin(&self) -> TspinR {
        TspinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output type"]
    #[inline(always)]
    pub fn outtype(&self) -> OuttypeR {
        OuttypeR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP")
            .field("outtype", &self.outtype())
            .field("tspin", &self.tspin())
            .field("tp1pin", &self.tp1pin())
            .field("tppu", &self.tppu())
            .field("tppr", &self.tppr())
            .field("tpflt", &self.tpflt())
            .field("tpfreq", &self.tpfreq())
            .field("tptsen", &self.tptsen())
            .field("tp2edg", &self.tp2edg())
            .field("tp2en", &self.tp2en())
            .field("tpien", &self.tpien())
            .field("tp1edg", &self.tp1edg())
            .field("tp1en", &self.tp1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp1en(&mut self) -> Tp1enW<TampSpec> {
        Tp1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection 1 valid edge"]
    #[inline(always)]
    #[must_use]
    pub fn tp1edg(&mut self) -> Tp1edgW<TampSpec> {
        Tp1edgW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TpienW<TampSpec> {
        TpienW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper detection 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp2en(&mut self) -> Tp2enW<TampSpec> {
        Tp2enW::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper detection 2 valid edge"]
    #[inline(always)]
    #[must_use]
    pub fn tp2edg(&mut self) -> Tp2edgW<TampSpec> {
        Tp2edgW::new(self, 4)
    }
    #[doc = "Bit 7 - Tamper detection timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tptsen(&mut self) -> TptsenW<TampSpec> {
        TptsenW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper detection frequency"]
    #[inline(always)]
    #[must_use]
    pub fn tpfreq(&mut self) -> TpfreqW<TampSpec> {
        TpfreqW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Tamper detection filter time"]
    #[inline(always)]
    #[must_use]
    pub fn tpflt(&mut self) -> TpfltW<TampSpec> {
        TpfltW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Tamper detection pre-charge time"]
    #[inline(always)]
    #[must_use]
    pub fn tppr(&mut self) -> TpprW<TampSpec> {
        TpprW::new(self, 13)
    }
    #[doc = "Bit 15 - Tamper detection pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn tppu(&mut self) -> TppuW<TampSpec> {
        TppuW::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper detection pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn tp1pin(&mut self) -> Tp1pinW<TampSpec> {
        Tp1pinW::new(self, 16)
    }
    #[doc = "Bit 17 - Time stamp detection pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn tspin(&mut self) -> TspinW<TampSpec> {
        TspinW::new(self, 17)
    }
    #[doc = "Bit 18 - Output type"]
    #[inline(always)]
    #[must_use]
    pub fn outtype(&mut self) -> OuttypeW<TampSpec> {
        OuttypeW::new(self, 18)
    }
}
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampSpec;
impl crate::RegisterSpec for TampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp::R`](R) reader structure"]
impl crate::Readable for TampSpec {}
#[doc = "`write(|w| ..)` method takes [`tamp::W`](W) writer structure"]
impl crate::Writable for TampSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP to value 0"]
impl crate::Resettable for TampSpec {
    const RESET_VALUE: u32 = 0;
}
