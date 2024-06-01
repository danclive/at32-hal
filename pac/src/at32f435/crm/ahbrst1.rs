#[doc = "Register `AHBRST1` reader"]
pub type R = crate::R<Ahbrst1Spec>;
#[doc = "Register `AHBRST1` writer"]
pub type W = crate::W<Ahbrst1Spec>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub type GpioerstR = crate::BitReader;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub type GpiogrstR = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub type GpiogrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub type GpiohrstR = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub type GpiohrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDMARST` reader - EDMA reset"]
pub type EdmarstR = crate::BitReader;
#[doc = "Field `EDMARST` writer - EDMA reset"]
pub type EdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type Dma2rstR = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRST` reader - EMAC reset"]
pub type EmacrstR = crate::BitReader;
#[doc = "Field `EMACRST` writer - EMAC reset"]
pub type EmacrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS2RST` reader - OTGFS2 interface reset"]
pub type Otgfs2rstR = crate::BitReader;
#[doc = "Field `OTGFS2RST` writer - OTGFS2 interface reset"]
pub type Otgfs2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GpiogrstR {
        GpiogrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GpiohrstR {
        GpiohrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA reset"]
    #[inline(always)]
    pub fn edmarst(&self) -> EdmarstR {
        EdmarstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC reset"]
    #[inline(always)]
    pub fn emacrst(&self) -> EmacrstR {
        EmacrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 interface reset"]
    #[inline(always)]
    pub fn otgfs2rst(&self) -> Otgfs2rstR {
        Otgfs2rstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST1")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("crcrst", &self.crcrst())
            .field("edmarst", &self.edmarst())
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .field("emacrst", &self.emacrst())
            .field("otgfs2rst", &self.otgfs2rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GpioarstW<Ahbrst1Spec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GpiobrstW<Ahbrst1Spec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GpiocrstW<Ahbrst1Spec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GpiodrstW<Ahbrst1Spec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GpioerstW<Ahbrst1Spec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GpiofrstW<Ahbrst1Spec> {
        GpiofrstW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GpiogrstW<Ahbrst1Spec> {
        GpiogrstW::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GpiohrstW<Ahbrst1Spec> {
        GpiohrstW::new(self, 7)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<Ahbrst1Spec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 21 - EDMA reset"]
    #[inline(always)]
    #[must_use]
    pub fn edmarst(&mut self) -> EdmarstW<Ahbrst1Spec> {
        EdmarstW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> Dma1rstW<Ahbrst1Spec> {
        Dma1rstW::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> Dma2rstW<Ahbrst1Spec> {
        Dma2rstW::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn emacrst(&mut self) -> EmacrstW<Ahbrst1Spec> {
        EmacrstW::new(self, 25)
    }
    #[doc = "Bit 29 - OTGFS2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2rst(&mut self) -> Otgfs2rstW<Ahbrst1Spec> {
        Otgfs2rstW::new(self, 29)
    }
}
#[doc = "AHB peripheral reset register1 (CRM_AHBRST1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbrst1Spec;
impl crate::RegisterSpec for Ahbrst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst1::R`](R) reader structure"]
impl crate::Readable for Ahbrst1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst1::W`](W) writer structure"]
impl crate::Writable for Ahbrst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST1 to value 0"]
impl crate::Resettable for Ahbrst1Spec {
    const RESET_VALUE: u32 = 0;
}
