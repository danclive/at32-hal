#[doc = "Register `ALA` reader"]
pub type R = crate::R<AlaSpec>;
#[doc = "Register `ALA` writer"]
pub type W = crate::W<AlaSpec>;
#[doc = "Field `SU` reader - Second units"]
pub type SuR = crate::FieldReader;
#[doc = "Field `SU` writer - Second units"]
pub type SuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens"]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASK1` reader - Seconds mask"]
pub type Mask1R = crate::BitReader;
#[doc = "Field `MASK1` writer - Seconds mask"]
pub type Mask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MU` reader - Minute units"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MU` writer - Minute units"]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Minute tens"]
pub type MtR = crate::FieldReader;
#[doc = "Field `MT` writer - Minute tens"]
pub type MtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASK2` reader - Minutes mask"]
pub type Mask2R = crate::BitReader;
#[doc = "Field `MASK2` writer - Minutes mask"]
pub type Mask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HU` reader - Hour units"]
pub type HuR = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units"]
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens"]
pub type HtR = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens"]
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMPM` reader - AM/PM"]
pub type AmpmR = crate::BitReader;
#[doc = "Field `AMPM` writer - AM/PM"]
pub type AmpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK3` reader - Hours mask"]
pub type Mask3R = crate::BitReader;
#[doc = "Field `MASK3` writer - Hours mask"]
pub type Mask3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Date units"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units"]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKSEL` reader - Date/week mode select"]
pub type WkselR = crate::BitReader;
#[doc = "Field `WKSEL` writer - Date/week mode select"]
pub type WkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK4` reader - Date/week mask"]
pub type Mask4R = crate::BitReader;
#[doc = "Field `MASK4` writer - Date/week mask"]
pub type Mask4W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 7 - Seconds mask"]
    #[inline(always)]
    pub fn mask1(&self) -> Mask1R {
        Mask1R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 15 - Minutes mask"]
    #[inline(always)]
    pub fn mask2(&self) -> Mask2R {
        Mask2R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 22 - AM/PM"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hours mask"]
    #[inline(always)]
    pub fn mask3(&self) -> Mask3R {
        Mask3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Date/week mode select"]
    #[inline(always)]
    pub fn wksel(&self) -> WkselR {
        WkselR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Date/week mask"]
    #[inline(always)]
    pub fn mask4(&self) -> Mask4R {
        Mask4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALA")
            .field("mask4", &self.mask4())
            .field("wksel", &self.wksel())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .field("mask3", &self.mask3())
            .field("ampm", &self.ampm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mask2", &self.mask2())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("mask1", &self.mask1())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SuW<AlaSpec> {
        SuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<AlaSpec> {
        StW::new(self, 4)
    }
    #[doc = "Bit 7 - Seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask1(&mut self) -> Mask1W<AlaSpec> {
        Mask1W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MuW<AlaSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MtW<AlaSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bit 15 - Minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask2(&mut self) -> Mask2W<AlaSpec> {
        Mask2W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HuW<AlaSpec> {
        HuW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HtW<AlaSpec> {
        HtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AmpmW<AlaSpec> {
        AmpmW::new(self, 22)
    }
    #[doc = "Bit 23 - Hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask3(&mut self) -> Mask3W<AlaSpec> {
        Mask3W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DuW<AlaSpec> {
        DuW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<AlaSpec> {
        DtW::new(self, 28)
    }
    #[doc = "Bit 30 - Date/week mode select"]
    #[inline(always)]
    #[must_use]
    pub fn wksel(&mut self) -> WkselW<AlaSpec> {
        WkselW::new(self, 30)
    }
    #[doc = "Bit 31 - Date/week mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask4(&mut self) -> Mask4W<AlaSpec> {
        Mask4W::new(self, 31)
    }
}
#[doc = "Alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ala::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ala::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlaSpec;
impl crate::RegisterSpec for AlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ala::R`](R) reader structure"]
impl crate::Readable for AlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ala::W`](W) writer structure"]
impl crate::Writable for AlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALA to value 0"]
impl crate::Resettable for AlaSpec {
    const RESET_VALUE: u32 = 0;
}
