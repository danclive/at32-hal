#[doc = "Register `TSTS` reader"]
pub type R = crate::R<TstsSpec>;
#[doc = "Register `TSTS` writer"]
pub type W = crate::W<TstsSpec>;
#[doc = "Field `TM0TCF` reader - Transmit mailbox 0 transmission complete flag"]
pub type Tm0tcfR = crate::BitReader;
#[doc = "Field `TM0TCF` writer - Transmit mailbox 0 transmission complete flag"]
pub type Tm0tcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM0TSF` reader - Transmit mailbox 0 transmission success flag"]
pub type Tm0tsfR = crate::BitReader;
#[doc = "Field `TM0TSF` writer - Transmit mailbox 0 transmission success flag"]
pub type Tm0tsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM0ALF` reader - Transmit mailbox 0 arbitration lost flag"]
pub type Tm0alfR = crate::BitReader;
#[doc = "Field `TM0ALF` writer - Transmit mailbox 0 arbitration lost flag"]
pub type Tm0alfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM0TEF` reader - Transmit mailbox 0 transmission error flag"]
pub type Tm0tefR = crate::BitReader;
#[doc = "Field `TM0TEF` writer - Transmit mailbox 0 transmission error flag"]
pub type Tm0tefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM0CT` reader - Transmit mailbox 0 cancel transmission"]
pub type Tm0ctR = crate::BitReader;
#[doc = "Field `TM0CT` writer - Transmit mailbox 0 cancel transmission"]
pub type Tm0ctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM1TCF` reader - Transmit mailbox 1 transmission complete flag"]
pub type Tm1tcfR = crate::BitReader;
#[doc = "Field `TM1TCF` writer - Transmit mailbox 1 transmission complete flag"]
pub type Tm1tcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM1TSF` reader - Transmit mailbox 1 transmission success flag"]
pub type Tm1tsfR = crate::BitReader;
#[doc = "Field `TM1TSF` writer - Transmit mailbox 1 transmission success flag"]
pub type Tm1tsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM1ALF` reader - Transmit mailbox 1 arbitration lost flag"]
pub type Tm1alfR = crate::BitReader;
#[doc = "Field `TM1ALF` writer - Transmit mailbox 1 arbitration lost flag"]
pub type Tm1alfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM1TEF` reader - Transmit mailbox 1 transmission error flag"]
pub type Tm1tefR = crate::BitReader;
#[doc = "Field `TM1TEF` writer - Transmit mailbox 1 transmission error flag"]
pub type Tm1tefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM1CT` reader - Transmit mailbox 1 cancel transmission"]
pub type Tm1ctR = crate::BitReader;
#[doc = "Field `TM1CT` writer - Transmit mailbox 1 cancel transmission"]
pub type Tm1ctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM2TCF` reader - transmit mailbox 2 transmission complete flag"]
pub type Tm2tcfR = crate::BitReader;
#[doc = "Field `TM2TCF` writer - transmit mailbox 2 transmission complete flag"]
pub type Tm2tcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM2TSF` reader - Transmit mailbox 2 transmission success flag"]
pub type Tm2tsfR = crate::BitReader;
#[doc = "Field `TM2TSF` writer - Transmit mailbox 2 transmission success flag"]
pub type Tm2tsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM2ALF` reader - Transmit mailbox 2 arbitration lost flag"]
pub type Tm2alfR = crate::BitReader;
#[doc = "Field `TM2ALF` writer - Transmit mailbox 2 arbitration lost flag"]
pub type Tm2alfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM2TEF` reader - Transmit mailbox 2 transmission error flag"]
pub type Tm2tefR = crate::BitReader;
#[doc = "Field `TM2TEF` writer - Transmit mailbox 2 transmission error flag"]
pub type Tm2tefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM2CT` reader - Transmit mailbox 2 cancel transmission"]
pub type Tm2ctR = crate::BitReader;
#[doc = "Field `TM2CT` writer - Transmit mailbox 2 cancel transmission"]
pub type Tm2ctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMNR` reader - Transmit Mailbox number record"]
pub type TmnrR = crate::FieldReader;
#[doc = "Field `TM0EF` reader - Transmit mailbox 0 empty flag"]
pub type Tm0efR = crate::BitReader;
#[doc = "Field `TM1EF` reader - Transmit mailbox 1 empty flag"]
pub type Tm1efR = crate::BitReader;
#[doc = "Field `TM2EF` reader - Transmit mailbox 2 empty flag"]
pub type Tm2efR = crate::BitReader;
#[doc = "Field `TM0LPF` reader - Transmit mailbox 0 lowest priority flag"]
pub type Tm0lpfR = crate::BitReader;
#[doc = "Field `TM1LPF` reader - Transmit mailbox 1 lowest priority flag"]
pub type Tm1lpfR = crate::BitReader;
#[doc = "Field `TM2LPF` reader - Transmit mailbox 2 lowest priority flag"]
pub type Tm2lpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    pub fn tm0tcf(&self) -> Tm0tcfR {
        Tm0tcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    pub fn tm0tsf(&self) -> Tm0tsfR {
        Tm0tsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    pub fn tm0alf(&self) -> Tm0alfR {
        Tm0alfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    pub fn tm0tef(&self) -> Tm0tefR {
        Tm0tefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    pub fn tm0ct(&self) -> Tm0ctR {
        Tm0ctR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    pub fn tm1tcf(&self) -> Tm1tcfR {
        Tm1tcfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    pub fn tm1tsf(&self) -> Tm1tsfR {
        Tm1tsfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    pub fn tm1alf(&self) -> Tm1alfR {
        Tm1alfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    pub fn tm1tef(&self) -> Tm1tefR {
        Tm1tefR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    pub fn tm1ct(&self) -> Tm1ctR {
        Tm1ctR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    pub fn tm2tcf(&self) -> Tm2tcfR {
        Tm2tcfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    pub fn tm2tsf(&self) -> Tm2tsfR {
        Tm2tsfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    pub fn tm2alf(&self) -> Tm2alfR {
        Tm2alfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    pub fn tm2tef(&self) -> Tm2tefR {
        Tm2tefR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    pub fn tm2ct(&self) -> Tm2ctR {
        Tm2ctR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transmit Mailbox number record"]
    #[inline(always)]
    pub fn tmnr(&self) -> TmnrR {
        TmnrR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty flag"]
    #[inline(always)]
    pub fn tm0ef(&self) -> Tm0efR {
        Tm0efR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty flag"]
    #[inline(always)]
    pub fn tm1ef(&self) -> Tm1efR {
        Tm1efR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty flag"]
    #[inline(always)]
    pub fn tm2ef(&self) -> Tm2efR {
        Tm2efR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 lowest priority flag"]
    #[inline(always)]
    pub fn tm0lpf(&self) -> Tm0lpfR {
        Tm0lpfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 lowest priority flag"]
    #[inline(always)]
    pub fn tm1lpf(&self) -> Tm1lpfR {
        Tm1lpfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit mailbox 2 lowest priority flag"]
    #[inline(always)]
    pub fn tm2lpf(&self) -> Tm2lpfR {
        Tm2lpfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTS")
            .field("tm2lpf", &self.tm2lpf())
            .field("tm1lpf", &self.tm1lpf())
            .field("tm0lpf", &self.tm0lpf())
            .field("tm2ef", &self.tm2ef())
            .field("tm1ef", &self.tm1ef())
            .field("tm0ef", &self.tm0ef())
            .field("tmnr", &self.tmnr())
            .field("tm2ct", &self.tm2ct())
            .field("tm2tef", &self.tm2tef())
            .field("tm2alf", &self.tm2alf())
            .field("tm2tsf", &self.tm2tsf())
            .field("tm2tcf", &self.tm2tcf())
            .field("tm1ct", &self.tm1ct())
            .field("tm1tef", &self.tm1tef())
            .field("tm1alf", &self.tm1alf())
            .field("tm1tsf", &self.tm1tsf())
            .field("tm1tcf", &self.tm1tcf())
            .field("tm0ct", &self.tm0ct())
            .field("tm0tef", &self.tm0tef())
            .field("tm0alf", &self.tm0alf())
            .field("tm0tsf", &self.tm0tsf())
            .field("tm0tcf", &self.tm0tcf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tcf(&mut self) -> Tm0tcfW<TstsSpec> {
        Tm0tcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tsf(&mut self) -> Tm0tsfW<TstsSpec> {
        Tm0tsfW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0alf(&mut self) -> Tm0alfW<TstsSpec> {
        Tm0alfW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tef(&mut self) -> Tm0tefW<TstsSpec> {
        Tm0tefW::new(self, 3)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm0ct(&mut self) -> Tm0ctW<TstsSpec> {
        Tm0ctW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tcf(&mut self) -> Tm1tcfW<TstsSpec> {
        Tm1tcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tsf(&mut self) -> Tm1tsfW<TstsSpec> {
        Tm1tsfW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1alf(&mut self) -> Tm1alfW<TstsSpec> {
        Tm1alfW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tef(&mut self) -> Tm1tefW<TstsSpec> {
        Tm1tefW::new(self, 11)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm1ct(&mut self) -> Tm1ctW<TstsSpec> {
        Tm1ctW::new(self, 15)
    }
    #[doc = "Bit 16 - transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tcf(&mut self) -> Tm2tcfW<TstsSpec> {
        Tm2tcfW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tsf(&mut self) -> Tm2tsfW<TstsSpec> {
        Tm2tsfW::new(self, 17)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2alf(&mut self) -> Tm2alfW<TstsSpec> {
        Tm2alfW::new(self, 18)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tef(&mut self) -> Tm2tefW<TstsSpec> {
        Tm2tefW::new(self, 19)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm2ct(&mut self) -> Tm2ctW<TstsSpec> {
        Tm2ctW::new(self, 23)
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstsSpec;
impl crate::RegisterSpec for TstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsts::R`](R) reader structure"]
impl crate::Readable for TstsSpec {}
#[doc = "`write(|w| ..)` method takes [`tsts::W`](W) writer structure"]
impl crate::Writable for TstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSTS to value 0x1c00_0000"]
impl crate::Resettable for TstsSpec {
    const RESET_VALUE: u32 = 0x1c00_0000;
}
