#[doc = "Register `AHBEN1` reader"]
pub type R = crate::R<Ahben1Spec>;
#[doc = "Register `AHBEN1` writer"]
pub type W = crate::W<Ahben1Spec>;
#[doc = "Field `GPIOAEN` reader - IO A clock enable"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - IO A clock enable"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - IO B clock enable"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - IO B clock enable"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - IO C clock enable"]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - IO C clock enable"]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - IO D clock enable"]
pub type GpiodenR = crate::BitReader;
#[doc = "Field `GPIODEN` writer - IO D clock enable"]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - IO E clock enable"]
pub type GpioeenR = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - IO E clock enable"]
pub type GpioeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - IO F clock enable"]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - IO F clock enable"]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - IO G clock enable"]
pub type GpiogenR = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - IO G clock enable"]
pub type GpiogenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - IO H clock enable"]
pub type GpiohenR = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - IO H clock enable"]
pub type GpiohenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDMAEN` reader - DMA1 clock enable"]
pub type EdmaenR = crate::BitReader;
#[doc = "Field `EDMAEN` writer - DMA1 clock enable"]
pub type EdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACEN` reader - EMAC clock enable"]
pub type EmacenR = crate::BitReader;
#[doc = "Field `EMACEN` writer - EMAC clock enable"]
pub type EmacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACTXEN` reader - EMAC Tx clock enable"]
pub type EmactxenR = crate::BitReader;
#[doc = "Field `EMACTXEN` writer - EMAC Tx clock enable"]
pub type EmactxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRXEN` reader - EMAC Rx clock enable"]
pub type EmacrxenR = crate::BitReader;
#[doc = "Field `EMACRXEN` writer - EMAC Rx clock enable"]
pub type EmacrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACPTPEN` reader - EMAC PTP clock enable"]
pub type EmacptpenR = crate::BitReader;
#[doc = "Field `EMACPTPEN` writer - EMAC PTP clock enable"]
pub type EmacptpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS2EN` reader - OTGFS2 clock enable"]
pub type Otgfs2enR = crate::BitReader;
#[doc = "Field `OTGFS2EN` writer - OTGFS2 clock enable"]
pub type Otgfs2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GpiogenR {
        GpiogenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GpiohenR {
        GpiohenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn edmaen(&self) -> EdmaenR {
        EdmaenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable"]
    #[inline(always)]
    pub fn emacen(&self) -> EmacenR {
        EmacenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable"]
    #[inline(always)]
    pub fn emactxen(&self) -> EmactxenR {
        EmactxenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable"]
    #[inline(always)]
    pub fn emacrxen(&self) -> EmacrxenR {
        EmacrxenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable"]
    #[inline(always)]
    pub fn emacptpen(&self) -> EmacptpenR {
        EmacptpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable"]
    #[inline(always)]
    pub fn otgfs2en(&self) -> Otgfs2enR {
        Otgfs2enR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN1")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("crcen", &self.crcen())
            .field("edmaen", &self.edmaen())
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("emacen", &self.emacen())
            .field("emactxen", &self.emactxen())
            .field("emacrxen", &self.emacrxen())
            .field("emacptpen", &self.emacptpen())
            .field("otgfs2en", &self.otgfs2en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GpioaenW<Ahben1Spec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GpiobenW<Ahben1Spec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GpiocenW<Ahben1Spec> {
        GpiocenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GpiodenW<Ahben1Spec> {
        GpiodenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GpioeenW<Ahben1Spec> {
        GpioeenW::new(self, 4)
    }
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GpiofenW<Ahben1Spec> {
        GpiofenW::new(self, 5)
    }
    #[doc = "Bit 6 - IO G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GpiogenW<Ahben1Spec> {
        GpiogenW::new(self, 6)
    }
    #[doc = "Bit 7 - IO H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GpiohenW<Ahben1Spec> {
        GpiohenW::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<Ahben1Spec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn edmaen(&mut self) -> EdmaenW<Ahben1Spec> {
        EdmaenW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> Dma1enW<Ahben1Spec> {
        Dma1enW::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> Dma2enW<Ahben1Spec> {
        Dma2enW::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacen(&mut self) -> EmacenW<Ahben1Spec> {
        EmacenW::new(self, 25)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emactxen(&mut self) -> EmactxenW<Ahben1Spec> {
        EmactxenW::new(self, 26)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacrxen(&mut self) -> EmacrxenW<Ahben1Spec> {
        EmacrxenW::new(self, 27)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacptpen(&mut self) -> EmacptpenW<Ahben1Spec> {
        EmacptpenW::new(self, 28)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2en(&mut self) -> Otgfs2enW<Ahben1Spec> {
        Otgfs2enW::new(self, 29)
    }
}
#[doc = "AHB Peripheral Clock enable register 1 (CRM_AHBEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahben1Spec;
impl crate::RegisterSpec for Ahben1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben1::R`](R) reader structure"]
impl crate::Readable for Ahben1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahben1::W`](W) writer structure"]
impl crate::Writable for Ahben1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN1 to value 0"]
impl crate::Resettable for Ahben1Spec {
    const RESET_VALUE: u32 = 0;
}
