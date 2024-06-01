#[doc = "Register `AHBLPEN1` reader"]
pub type R = crate::R<Ahblpen1Spec>;
#[doc = "Register `AHBLPEN1` writer"]
pub type W = crate::W<Ahblpen1Spec>;
#[doc = "Field `GPIOALPEN` reader - IO A clock enable during sleep mode"]
pub type GpioalpenR = crate::BitReader;
#[doc = "Field `GPIOALPEN` writer - IO A clock enable during sleep mode"]
pub type GpioalpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBLPEN` reader - IO B clock enable during sleep mode"]
pub type GpioblpenR = crate::BitReader;
#[doc = "Field `GPIOBLPEN` writer - IO B clock enable during sleep mode"]
pub type GpioblpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCLPEN` reader - IO C clock enable during sleep mode"]
pub type GpioclpenR = crate::BitReader;
#[doc = "Field `GPIOCLPEN` writer - IO C clock enable during sleep mode"]
pub type GpioclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODLPEN` reader - IO D clock enable during sleep mode"]
pub type GpiodlpenR = crate::BitReader;
#[doc = "Field `GPIODLPEN` writer - IO D clock enable during sleep mode"]
pub type GpiodlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOELPEN` reader - IO E clock enable during sleep mode"]
pub type GpioelpenR = crate::BitReader;
#[doc = "Field `GPIOELPEN` writer - IO E clock enable during sleep mode"]
pub type GpioelpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFLPEN` reader - IO F clock enable during sleep mode"]
pub type GpioflpenR = crate::BitReader;
#[doc = "Field `GPIOFLPEN` writer - IO F clock enable during sleep mode"]
pub type GpioflpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGLPEN` reader - IO G clock enable during sleep mode"]
pub type GpioglpenR = crate::BitReader;
#[doc = "Field `GPIOGLPEN` writer - IO G clock enable during sleep mode"]
pub type GpioglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHLPEN` reader - IO H clock enable during sleep mode"]
pub type GpiohlpenR = crate::BitReader;
#[doc = "Field `GPIOHLPEN` writer - IO H clock enable during sleep mode"]
pub type GpiohlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during sleep mode"]
pub type CrclpenR = crate::BitReader;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during sleep mode"]
pub type CrclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHLPEN` reader - Flash clock enable during sleep mode"]
pub type FlashlpenR = crate::BitReader;
#[doc = "Field `FLASHLPEN` writer - Flash clock enable during sleep mode"]
pub type FlashlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 clock enable during sleep mode"]
pub type Sram1lpenR = crate::BitReader;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 clock enable during sleep mode"]
pub type Sram1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 clock enable during sleep mode"]
pub type Sram2lpenR = crate::BitReader;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 clock enable during sleep mode"]
pub type Sram2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDMALPEN` reader - EDMA clock enable during sleep mode"]
pub type EdmalpenR = crate::BitReader;
#[doc = "Field `EDMALPEN` writer - EDMA clock enable during sleep mode"]
pub type EdmalpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during sleep mode"]
pub type Dma1lpenR = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during sleep mode"]
pub type Dma1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during sleep mode"]
pub type Dma2lpenR = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during sleep mode"]
pub type Dma2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACLPEN` reader - EMAC clock enable during sleep mode"]
pub type EmaclpenR = crate::BitReader;
#[doc = "Field `EMACLPEN` writer - EMAC clock enable during sleep mode"]
pub type EmaclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACTXLPEN` reader - EMAC Tx clock enable during sleep mode"]
pub type EmactxlpenR = crate::BitReader;
#[doc = "Field `EMACTXLPEN` writer - EMAC Tx clock enable during sleep mode"]
pub type EmactxlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRXLPEN` reader - EMAC Rx clock enable during sleep mode"]
pub type EmacrxlpenR = crate::BitReader;
#[doc = "Field `EMACRXLPEN` writer - EMAC Rx clock enable during sleep mode"]
pub type EmacrxlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACPTPLPEN` reader - EMAC PTP clock enable during sleep mode"]
pub type EmacptplpenR = crate::BitReader;
#[doc = "Field `EMACPTPLPEN` writer - EMAC PTP clock enable during sleep mode"]
pub type EmacptplpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS2LPEN` reader - OTGFS2 clock enable during sleep mode"]
pub type Otgfs2lpenR = crate::BitReader;
#[doc = "Field `OTGFS2LPEN` writer - OTGFS2 clock enable during sleep mode"]
pub type Otgfs2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GpioalpenR {
        GpioalpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GpioblpenR {
        GpioblpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GpioclpenR {
        GpioclpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GpiodlpenR {
        GpiodlpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GpioelpenR {
        GpioelpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GpioflpenR {
        GpioflpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GpioglpenR {
        GpioglpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GpiohlpenR {
        GpiohlpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CrclpenR {
        CrclpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    pub fn flashlpen(&self) -> FlashlpenR {
        FlashlpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> Sram1lpenR {
        Sram1lpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> Sram2lpenR {
        Sram2lpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    pub fn edmalpen(&self) -> EdmalpenR {
        EdmalpenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> Dma1lpenR {
        Dma1lpenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> Dma2lpenR {
        Dma2lpenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn emaclpen(&self) -> EmaclpenR {
        EmaclpenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emactxlpen(&self) -> EmactxlpenR {
        EmactxlpenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacrxlpen(&self) -> EmacrxlpenR {
        EmacrxlpenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacptplpen(&self) -> EmacptplpenR {
        EmacptplpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs2lpen(&self) -> Otgfs2lpenR {
        Otgfs2lpenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN1")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("crclpen", &self.crclpen())
            .field("flashlpen", &self.flashlpen())
            .field("sram1lpen", &self.sram1lpen())
            .field("sram2lpen", &self.sram2lpen())
            .field("edmalpen", &self.edmalpen())
            .field("dma1lpen", &self.dma1lpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("emaclpen", &self.emaclpen())
            .field("emactxlpen", &self.emactxlpen())
            .field("emacrxlpen", &self.emacrxlpen())
            .field("emacptplpen", &self.emacptplpen())
            .field("otgfs2lpen", &self.otgfs2lpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GpioalpenW<Ahblpen1Spec> {
        GpioalpenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GpioblpenW<Ahblpen1Spec> {
        GpioblpenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GpioclpenW<Ahblpen1Spec> {
        GpioclpenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GpiodlpenW<Ahblpen1Spec> {
        GpiodlpenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GpioelpenW<Ahblpen1Spec> {
        GpioelpenW::new(self, 4)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GpioflpenW<Ahblpen1Spec> {
        GpioflpenW::new(self, 5)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GpioglpenW<Ahblpen1Spec> {
        GpioglpenW::new(self, 6)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GpiohlpenW<Ahblpen1Spec> {
        GpiohlpenW::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CrclpenW<Ahblpen1Spec> {
        CrclpenW::new(self, 12)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashlpen(&mut self) -> FlashlpenW<Ahblpen1Spec> {
        FlashlpenW::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> Sram1lpenW<Ahblpen1Spec> {
        Sram1lpenW::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> Sram2lpenW<Ahblpen1Spec> {
        Sram2lpenW::new(self, 17)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn edmalpen(&mut self) -> EdmalpenW<Ahblpen1Spec> {
        EdmalpenW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> Dma1lpenW<Ahblpen1Spec> {
        Dma1lpenW::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> Dma2lpenW<Ahblpen1Spec> {
        Dma2lpenW::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emaclpen(&mut self) -> EmaclpenW<Ahblpen1Spec> {
        EmaclpenW::new(self, 25)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emactxlpen(&mut self) -> EmactxlpenW<Ahblpen1Spec> {
        EmactxlpenW::new(self, 26)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emacrxlpen(&mut self) -> EmacrxlpenW<Ahblpen1Spec> {
        EmacrxlpenW::new(self, 27)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emacptplpen(&mut self) -> EmacptplpenW<Ahblpen1Spec> {
        EmacptplpenW::new(self, 28)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2lpen(&mut self) -> Otgfs2lpenW<Ahblpen1Spec> {
        Otgfs2lpenW::new(self, 29)
    }
}
#[doc = "AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahblpen1Spec;
impl crate::RegisterSpec for Ahblpen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen1::R`](R) reader structure"]
impl crate::Readable for Ahblpen1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahblpen1::W`](W) writer structure"]
impl crate::Writable for Ahblpen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLPEN1 to value 0x3e63_90ff"]
impl crate::Resettable for Ahblpen1Spec {
    const RESET_VALUE: u32 = 0x3e63_90ff;
}
