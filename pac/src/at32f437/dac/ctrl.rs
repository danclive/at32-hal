#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `D1EN` reader - DAC1 enable"]
pub type D1enR = crate::BitReader;
#[doc = "Field `D1EN` writer - DAC1 enable"]
pub type D1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1OBDIS` reader - DAC1 output buffer disable"]
pub type D1obdisR = crate::BitReader;
#[doc = "Field `D1OBDIS` writer - DAC1 output buffer disable"]
pub type D1obdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1TRGEN` reader - DAC1 trigger enable"]
pub type D1trgenR = crate::BitReader;
#[doc = "Field `D1TRGEN` writer - DAC1 trigger enable"]
pub type D1trgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1TRGSEL` reader - DAC1 trigger selection"]
pub type D1trgselR = crate::FieldReader;
#[doc = "Field `D1TRGSEL` writer - DAC1 trigger selection"]
pub type D1trgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D1NM` reader - DAC1 noise/triangle wave generation enable"]
pub type D1nmR = crate::FieldReader;
#[doc = "Field `D1NM` writer - DAC1 noise/triangle wave generation enable"]
pub type D1nmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D1NBSEL` reader - DAC1 mask/amplitude selector"]
pub type D1nbselR = crate::FieldReader;
#[doc = "Field `D1NBSEL` writer - DAC1 mask/amplitude selector"]
pub type D1nbselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D1DMAEN` reader - DAC1 DMA enable"]
pub type D1dmaenR = crate::BitReader;
#[doc = "Field `D1DMAEN` writer - DAC1 DMA enable"]
pub type D1dmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1DMAUDRIEN` reader - DAC1 DMA underrun interrupt enable"]
pub type D1dmaudrienR = crate::BitReader;
#[doc = "Field `D1DMAUDRIEN` writer - DAC1 DMA underrun interrupt enable"]
pub type D1dmaudrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2EN` reader - DAC2 enable"]
pub type D2enR = crate::BitReader;
#[doc = "Field `D2EN` writer - DAC2 enable"]
pub type D2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2OBDIS` reader - DAC2 output buffer disable"]
pub type D2obdisR = crate::BitReader;
#[doc = "Field `D2OBDIS` writer - DAC2 output buffer disable"]
pub type D2obdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2TRGEN` reader - DAC2 trigger enable"]
pub type D2trgenR = crate::BitReader;
#[doc = "Field `D2TRGEN` writer - DAC2 trigger enable"]
pub type D2trgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2TRGSEL` reader - DAC2 trigger selection"]
pub type D2trgselR = crate::FieldReader;
#[doc = "Field `D2TRGSEL` writer - DAC2 trigger selection"]
pub type D2trgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D2NM` reader - DAC2 noise/triangle wave generation enable"]
pub type D2nmR = crate::FieldReader;
#[doc = "Field `D2NM` writer - DAC2 noise/triangle wave generation enable"]
pub type D2nmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D2NBSEL` reader - DAC2 mask/amplitude selector"]
pub type D2nbselR = crate::FieldReader;
#[doc = "Field `D2NBSEL` writer - DAC2 mask/amplitude selector"]
pub type D2nbselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D2DMAEN` reader - DAC2 DMA enable"]
pub type D2dmaenR = crate::BitReader;
#[doc = "Field `D2DMAEN` writer - DAC2 DMA enable"]
pub type D2dmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2DMAUDRIEN` reader - DAC2 DMA underrun interrupt enable"]
pub type D2dmaudrienR = crate::BitReader;
#[doc = "Field `D2DMAUDRIEN` writer - DAC2 DMA underrun interrupt enable"]
pub type D2dmaudrienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC1 enable"]
    #[inline(always)]
    pub fn d1en(&self) -> D1enR {
        D1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC1 output buffer disable"]
    #[inline(always)]
    pub fn d1obdis(&self) -> D1obdisR {
        D1obdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn d1trgen(&self) -> D1trgenR {
        D1trgenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn d1trgsel(&self) -> D1trgselR {
        D1trgselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d1nm(&self) -> D1nmR {
        D1nmR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC1 mask/amplitude selector"]
    #[inline(always)]
    pub fn d1nbsel(&self) -> D1nbselR {
        D1nbselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn d1dmaen(&self) -> D1dmaenR {
        D1dmaenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC1 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn d1dmaudrien(&self) -> D1dmaudrienR {
        D1dmaudrienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC2 enable"]
    #[inline(always)]
    pub fn d2en(&self) -> D2enR {
        D2enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 output buffer disable"]
    #[inline(always)]
    pub fn d2obdis(&self) -> D2obdisR {
        D2obdisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC2 trigger enable"]
    #[inline(always)]
    pub fn d2trgen(&self) -> D2trgenR {
        D2trgenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC2 trigger selection"]
    #[inline(always)]
    pub fn d2trgsel(&self) -> D2trgselR {
        D2trgselR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d2nm(&self) -> D2nmR {
        D2nmR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC2 mask/amplitude selector"]
    #[inline(always)]
    pub fn d2nbsel(&self) -> D2nbselR {
        D2nbselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC2 DMA enable"]
    #[inline(always)]
    pub fn d2dmaen(&self) -> D2dmaenR {
        D2dmaenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn d2dmaudrien(&self) -> D2dmaudrienR {
        D2dmaudrienR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("d1en", &self.d1en())
            .field("d1obdis", &self.d1obdis())
            .field("d1trgen", &self.d1trgen())
            .field("d1trgsel", &self.d1trgsel())
            .field("d1nm", &self.d1nm())
            .field("d1nbsel", &self.d1nbsel())
            .field("d1dmaen", &self.d1dmaen())
            .field("d1dmaudrien", &self.d1dmaudrien())
            .field("d2en", &self.d2en())
            .field("d2obdis", &self.d2obdis())
            .field("d2trgen", &self.d2trgen())
            .field("d2trgsel", &self.d2trgsel())
            .field("d2nm", &self.d2nm())
            .field("d2nbsel", &self.d2nbsel())
            .field("d2dmaen", &self.d2dmaen())
            .field("d2dmaudrien", &self.d2dmaudrien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DAC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1en(&mut self) -> D1enW<CtrlSpec> {
        D1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC1 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn d1obdis(&mut self) -> D1obdisW<CtrlSpec> {
        D1obdisW::new(self, 1)
    }
    #[doc = "Bit 2 - DAC1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1trgen(&mut self) -> D1trgenW<CtrlSpec> {
        D1trgenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn d1trgsel(&mut self) -> D1trgselW<CtrlSpec> {
        D1trgselW::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1nm(&mut self) -> D1nmW<CtrlSpec> {
        D1nmW::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn d1nbsel(&mut self) -> D1nbselW<CtrlSpec> {
        D1nbselW::new(self, 8)
    }
    #[doc = "Bit 12 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1dmaen(&mut self) -> D1dmaenW<CtrlSpec> {
        D1dmaenW::new(self, 12)
    }
    #[doc = "Bit 13 - DAC1 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1dmaudrien(&mut self) -> D1dmaudrienW<CtrlSpec> {
        D1dmaudrienW::new(self, 13)
    }
    #[doc = "Bit 16 - DAC2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2en(&mut self) -> D2enW<CtrlSpec> {
        D2enW::new(self, 16)
    }
    #[doc = "Bit 17 - DAC2 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn d2obdis(&mut self) -> D2obdisW<CtrlSpec> {
        D2obdisW::new(self, 17)
    }
    #[doc = "Bit 18 - DAC2 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2trgen(&mut self) -> D2trgenW<CtrlSpec> {
        D2trgenW::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC2 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn d2trgsel(&mut self) -> D2trgselW<CtrlSpec> {
        D2trgselW::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC2 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2nm(&mut self) -> D2nmW<CtrlSpec> {
        D2nmW::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC2 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn d2nbsel(&mut self) -> D2nbselW<CtrlSpec> {
        D2nbselW::new(self, 24)
    }
    #[doc = "Bit 28 - DAC2 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2dmaen(&mut self) -> D2dmaenW<CtrlSpec> {
        D2dmaenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2dmaudrien(&mut self) -> D2dmaudrienW<CtrlSpec> {
        D2dmaudrienW::new(self, 29)
    }
}
#[doc = "Control register (DAC_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
