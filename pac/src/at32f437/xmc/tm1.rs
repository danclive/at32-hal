#[doc = "Register `TM1` reader"]
pub type R = crate::R<Tm1Spec>;
#[doc = "Register `TM1` writer"]
pub type W = crate::W<Tm1Spec>;
#[doc = "Field `TMRD` reader - Mode register program to active delay"]
pub type TmrdR = crate::FieldReader;
#[doc = "Field `TMRD` writer - Mode register program to active delay"]
pub type TmrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSR` reader - Exit Self-refresh to active delay"]
pub type TxsrR = crate::FieldReader;
#[doc = "Field `TXSR` writer - Exit Self-refresh to active delay"]
pub type TxsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRAS` reader - Self refresh time"]
pub type TrasR = crate::FieldReader;
#[doc = "Field `TRAS` writer - Self refresh time"]
pub type TrasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRC` reader - Refresh to active delay"]
pub type TrcR = crate::FieldReader;
#[doc = "Field `TRC` writer - Refresh to active delay"]
pub type TrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TWR` reader - Write Recovery delay"]
pub type TwrR = crate::FieldReader;
#[doc = "Field `TWR` writer - Write Recovery delay"]
pub type TwrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRP` reader - Precharge to active delay"]
pub type TrpR = crate::FieldReader;
#[doc = "Field `TRP` writer - Precharge to active delay"]
pub type TrpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCD` reader - Row active to Read/Write delay"]
pub type TrcdR = crate::FieldReader;
#[doc = "Field `TRCD` writer - Row active to Read/Write delay"]
pub type TrcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Mode register program to active delay"]
    #[inline(always)]
    pub fn tmrd(&self) -> TmrdR {
        TmrdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh to active delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TxsrR {
        TxsrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&self) -> TrasR {
        TrasR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Refresh to active delay"]
    #[inline(always)]
    pub fn trc(&self) -> TrcR {
        TrcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write Recovery delay"]
    #[inline(always)]
    pub fn twr(&self) -> TwrR {
        TwrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Precharge to active delay"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row active to Read/Write delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TrcdR {
        TrcdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TM1")
            .field("tmrd", &self.tmrd())
            .field("txsr", &self.txsr())
            .field("tras", &self.tras())
            .field("trc", &self.trc())
            .field("twr", &self.twr())
            .field("trp", &self.trp())
            .field("trcd", &self.trcd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode register program to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd(&mut self) -> TmrdW<Tm1Spec> {
        TmrdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn txsr(&mut self) -> TxsrW<Tm1Spec> {
        TxsrW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TrasW<Tm1Spec> {
        TrasW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Refresh to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TrcW<Tm1Spec> {
        TrcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Write Recovery delay"]
    #[inline(always)]
    #[must_use]
    pub fn twr(&mut self) -> TwrW<Tm1Spec> {
        TwrW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Precharge to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TrpW<Tm1Spec> {
        TrpW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Row active to Read/Write delay"]
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TrcdW<Tm1Spec> {
        TrcdW::new(self, 24)
    }
}
#[doc = "SDRAM Timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tm1Spec;
impl crate::RegisterSpec for Tm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm1::R`](R) reader structure"]
impl crate::Readable for Tm1Spec {}
#[doc = "`write(|w| ..)` method takes [`tm1::W`](W) writer structure"]
impl crate::Writable for Tm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TM1 to value 0x0fff_ffff"]
impl crate::Resettable for Tm1Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
