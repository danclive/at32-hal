#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type ClkphaR = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type ClkphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type ClkpolR = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTEN` reader - Master enable"]
pub type MstenR = crate::BitReader;
#[doc = "Field `MSTEN` writer - Master enable"]
pub type MstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIV2_0` reader - Master clock frequency division bit2-0"]
pub type Mdiv2_0R = crate::FieldReader;
#[doc = "Field `MDIV2_0` writer - Master clock frequency division bit2-0"]
pub type Mdiv2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTF` reader - LSB transmit first"]
pub type LtfR = crate::BitReader;
#[doc = "Field `LTF` writer - LSB transmit first"]
pub type LtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWCSIL` reader - Software CS internal level"]
pub type SwcsilR = crate::BitReader;
#[doc = "Field `SWCSIL` writer - Software CS internal level"]
pub type SwcsilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWCSEN` reader - Software CS enable"]
pub type SwcsenR = crate::BitReader;
#[doc = "Field `SWCSEN` writer - Software CS enable"]
pub type SwcsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORA` reader - Only receive active"]
pub type OraR = crate::BitReader;
#[doc = "Field `ORA` writer - Only receive active"]
pub type OraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBN` reader - frame bit num"]
pub type FbnR = crate::BitReader;
#[doc = "Field `FBN` writer - frame bit num"]
pub type FbnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTC` reader - Next transmission CRC"]
pub type NtcR = crate::BitReader;
#[doc = "Field `NTC` writer - Next transmission CRC"]
pub type NtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEN` reader - CRC calculation enable"]
pub type CcenR = crate::BitReader;
#[doc = "Field `CCEN` writer - CRC calculation enable"]
pub type CcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBTD` reader - Single line bidirectional half-duplex transmission direction"]
pub type SlbtdR = crate::BitReader;
#[doc = "Field `SLBTD` writer - Single line bidirectional half-duplex transmission direction"]
pub type SlbtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SlbenR = crate::BitReader;
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SlbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    pub fn msten(&self) -> MstenR {
        MstenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    pub fn mdiv2_0(&self) -> Mdiv2_0R {
        Mdiv2_0R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    pub fn ltf(&self) -> LtfR {
        LtfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    pub fn swcsil(&self) -> SwcsilR {
        SwcsilR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    pub fn swcsen(&self) -> SwcsenR {
        SwcsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    pub fn ora(&self) -> OraR {
        OraR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    pub fn fbn(&self) -> FbnR {
        FbnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    pub fn ntc(&self) -> NtcR {
        NtcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CcenR {
        CcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    pub fn slbtd(&self) -> SlbtdR {
        SlbtdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&self) -> SlbenR {
        SlbenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("slben", &self.slben())
            .field("slbtd", &self.slbtd())
            .field("ccen", &self.ccen())
            .field("ntc", &self.ntc())
            .field("fbn", &self.fbn())
            .field("ora", &self.ora())
            .field("swcsen", &self.swcsen())
            .field("swcsil", &self.swcsil())
            .field("ltf", &self.ltf())
            .field("spien", &self.spien())
            .field("mdiv2_0", &self.mdiv2_0())
            .field("msten", &self.msten())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> ClkphaW<Ctrl1Spec> {
        ClkphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> ClkpolW<Ctrl1Spec> {
        ClkpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    #[must_use]
    pub fn msten(&mut self) -> MstenW<Ctrl1Spec> {
        MstenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv2_0(&mut self) -> Mdiv2_0W<Ctrl1Spec> {
        Mdiv2_0W::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SpienW<Ctrl1Spec> {
        SpienW::new(self, 6)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    #[must_use]
    pub fn ltf(&mut self) -> LtfW<Ctrl1Spec> {
        LtfW::new(self, 7)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    #[must_use]
    pub fn swcsil(&mut self) -> SwcsilW<Ctrl1Spec> {
        SwcsilW::new(self, 8)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    #[must_use]
    pub fn swcsen(&mut self) -> SwcsenW<Ctrl1Spec> {
        SwcsenW::new(self, 9)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    #[must_use]
    pub fn ora(&mut self) -> OraW<Ctrl1Spec> {
        OraW::new(self, 10)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn fbn(&mut self) -> FbnW<Ctrl1Spec> {
        FbnW::new(self, 11)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NtcW<Ctrl1Spec> {
        NtcW::new(self, 12)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CcenW<Ctrl1Spec> {
        CcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    #[must_use]
    pub fn slbtd(&mut self) -> SlbtdW<Ctrl1Spec> {
        SlbtdW::new(self, 14)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SlbenW<Ctrl1Spec> {
        SlbenW::new(self, 15)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
