#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `ALAWF` reader - Alarm A register allows write flag"]
pub type AlawfR = crate::BitReader;
#[doc = "Field `ALBWF` reader - Alarm B register allows write flag"]
pub type AlbwfR = crate::BitReader;
#[doc = "Field `WATWF` reader - Wakeup timer register allows write flag"]
pub type WatwfR = crate::BitReader;
#[doc = "Field `TADJF` reader - Time adjustment flag"]
pub type TadjfR = crate::BitReader;
#[doc = "Field `TADJF` writer - Time adjustment flag"]
pub type TadjfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Calendar initialization flag"]
pub type InitfR = crate::BitReader;
#[doc = "Field `UPDF` reader - Calendar update flag"]
pub type UpdfR = crate::BitReader;
#[doc = "Field `UPDF` writer - Calendar update flag"]
pub type UpdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMF` reader - Enter initialization mode flag"]
pub type ImfR = crate::BitReader;
#[doc = "Field `IMEN` reader - Initialization mode enable"]
pub type ImenR = crate::BitReader;
#[doc = "Field `IMEN` writer - Initialization mode enable"]
pub type ImenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALAF` reader - Alarm A flag"]
pub type AlafR = crate::BitReader;
#[doc = "Field `ALAF` writer - Alarm A flag"]
pub type AlafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALBF` reader - Alarm B flag"]
pub type AlbfR = crate::BitReader;
#[doc = "Field `ALBF` writer - Alarm B flag"]
pub type AlbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATF` reader - Wakeup timer flag"]
pub type WatfR = crate::BitReader;
#[doc = "Field `WATF` writer - Wakeup timer flag"]
pub type WatfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Timestamp flag"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Timestamp flag"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOF` reader - Timestamp overflow flag"]
pub type TsofR = crate::BitReader;
#[doc = "Field `TSOF` writer - Timestamp overflow flag"]
pub type TsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1F` reader - Tamper detection 1 flag"]
pub type Tp1fR = crate::BitReader;
#[doc = "Field `TP1F` writer - Tamper detection 1 flag"]
pub type Tp1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP2F` reader - Tamper detection 2 flag"]
pub type Tp2fR = crate::BitReader;
#[doc = "Field `TP2F` writer - Tamper detection 2 flag"]
pub type Tp2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALUPDF` reader - Calibration value update completed flag"]
pub type CalupdfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A register allows write flag"]
    #[inline(always)]
    pub fn alawf(&self) -> AlawfR {
        AlawfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B register allows write flag"]
    #[inline(always)]
    pub fn albwf(&self) -> AlbwfR {
        AlbwfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer register allows write flag"]
    #[inline(always)]
    pub fn watwf(&self) -> WatwfR {
        WatwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time adjustment flag"]
    #[inline(always)]
    pub fn tadjf(&self) -> TadjfR {
        TadjfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calendar update flag"]
    #[inline(always)]
    pub fn updf(&self) -> UpdfR {
        UpdfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enter initialization mode flag"]
    #[inline(always)]
    pub fn imf(&self) -> ImfR {
        ImfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode enable"]
    #[inline(always)]
    pub fn imen(&self) -> ImenR {
        ImenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alaf(&self) -> AlafR {
        AlafR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn albf(&self) -> AlbfR {
        AlbfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn watf(&self) -> WatfR {
        WatfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timestamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp overflow flag"]
    #[inline(always)]
    pub fn tsof(&self) -> TsofR {
        TsofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection 1 flag"]
    #[inline(always)]
    pub fn tp1f(&self) -> Tp1fR {
        Tp1fR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper detection 2 flag"]
    #[inline(always)]
    pub fn tp2f(&self) -> Tp2fR {
        Tp2fR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Calibration value update completed flag"]
    #[inline(always)]
    pub fn calupdf(&self) -> CalupdfR {
        CalupdfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("alawf", &self.alawf())
            .field("albwf", &self.albwf())
            .field("watwf", &self.watwf())
            .field("tadjf", &self.tadjf())
            .field("initf", &self.initf())
            .field("updf", &self.updf())
            .field("imf", &self.imf())
            .field("imen", &self.imen())
            .field("alaf", &self.alaf())
            .field("albf", &self.albf())
            .field("watf", &self.watf())
            .field("tsf", &self.tsf())
            .field("tsof", &self.tsof())
            .field("tp1f", &self.tp1f())
            .field("tp2f", &self.tp2f())
            .field("calupdf", &self.calupdf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Time adjustment flag"]
    #[inline(always)]
    #[must_use]
    pub fn tadjf(&mut self) -> TadjfW<StsSpec> {
        TadjfW::new(self, 3)
    }
    #[doc = "Bit 5 - Calendar update flag"]
    #[inline(always)]
    #[must_use]
    pub fn updf(&mut self) -> UpdfW<StsSpec> {
        UpdfW::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn imen(&mut self) -> ImenW<StsSpec> {
        ImenW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alaf(&mut self) -> AlafW<StsSpec> {
        AlafW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn albf(&mut self) -> AlbfW<StsSpec> {
        AlbfW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn watf(&mut self) -> WatfW<StsSpec> {
        WatfW::new(self, 10)
    }
    #[doc = "Bit 11 - Timestamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<StsSpec> {
        TsfW::new(self, 11)
    }
    #[doc = "Bit 12 - Timestamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsof(&mut self) -> TsofW<StsSpec> {
        TsofW::new(self, 12)
    }
    #[doc = "Bit 13 - Tamper detection 1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn tp1f(&mut self) -> Tp1fW<StsSpec> {
        Tp1fW::new(self, 13)
    }
    #[doc = "Bit 14 - Tamper detection 2 flag"]
    #[inline(always)]
    #[must_use]
    pub fn tp2f(&mut self) -> Tp2fW<StsSpec> {
        Tp2fW::new(self, 14)
    }
}
#[doc = "initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0x07"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0x07;
}
