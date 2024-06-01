#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AhbenSpec>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AhbenSpec>;
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - FLASH clock enable"]
pub type FlashenR = crate::BitReader;
#[doc = "Field `FLASHEN` writer - FLASH clock enable"]
pub type FlashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - I/O port C clock enable"]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - I/O port C clock enable"]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable"]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable"]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FlashenR {
        FlashenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN")
            .field("dmaen", &self.dmaen())
            .field("sramen", &self.sramen())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpiofen", &self.gpiofen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<AhbenSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SramenW<AhbenSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FlashenW<AhbenSpec> {
        FlashenW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<AhbenSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GpioaenW<AhbenSpec> {
        GpioaenW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GpiobenW<AhbenSpec> {
        GpiobenW::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GpiocenW<AhbenSpec> {
        GpiocenW::new(self, 19)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GpiofenW<AhbenSpec> {
        GpiofenW::new(self, 22)
    }
}
#[doc = "AHB Peripheral Clock enable register (CRM_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenSpec;
impl crate::RegisterSpec for AhbenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AhbenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AhbenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AhbenSpec {
    const RESET_VALUE: u32 = 0x14;
}
