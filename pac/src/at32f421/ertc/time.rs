#[doc = "Register `TIME` reader"]
pub type R = crate::R<TimeSpec>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TimeSpec>;
#[doc = "Field `SU` reader - Second units"]
pub type SuR = crate::FieldReader;
#[doc = "Field `SU` writer - Second units"]
pub type SuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens"]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MU` reader - Minute units"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MU` writer - Minute units"]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Minute tens"]
pub type MtR = crate::FieldReader;
#[doc = "Field `MT` writer - Minute tens"]
pub type MtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HU` reader - Hour units"]
pub type HuR = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units"]
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens"]
pub type HtR = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens"]
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMPM` reader - AM/PM notation"]
pub type AmpmR = crate::BitReader;
#[doc = "Field `AMPM` writer - AM/PM notation"]
pub type AmpmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME")
            .field("ampm", &self.ampm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SuW<TimeSpec> {
        SuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<TimeSpec> {
        StW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MuW<TimeSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MtW<TimeSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HuW<TimeSpec> {
        HuW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HtW<TimeSpec> {
        HtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AmpmW<TimeSpec> {
        AmpmW::new(self, 22)
    }
}
#[doc = "time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeSpec;
impl crate::RegisterSpec for TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TimeSpec {
    const RESET_VALUE: u32 = 0;
}
