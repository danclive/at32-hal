#[doc = "Register `REMAP5` reader"]
pub type R = crate::R<Remap5Spec>;
#[doc = "Register `REMAP5` writer"]
pub type W = crate::W<Remap5Spec>;
#[doc = "Field `I2C1_GMUX` reader - I2C1 muxing"]
pub type I2c1GmuxR = crate::FieldReader;
#[doc = "Field `I2C1_GMUX` writer - I2C1 muxing"]
pub type I2c1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C2_GMUX` reader - I2C2 muxing"]
pub type I2c2GmuxR = crate::FieldReader;
#[doc = "Field `I2C2_GMUX` writer - I2C2 muxing"]
pub type I2c2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI1_GMUX` reader - SPI1 muxing"]
pub type Spi1GmuxR = crate::FieldReader;
#[doc = "Field `SPI1_GMUX` writer - SPI1 muxing"]
pub type Spi1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI2_GMUX` reader - SPI2 muxing"]
pub type Spi2GmuxR = crate::FieldReader;
#[doc = "Field `SPI2_GMUX` writer - SPI2 muxing"]
pub type Spi2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&self) -> I2c1GmuxR {
        I2c1GmuxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - I2C2 muxing"]
    #[inline(always)]
    pub fn i2c2_gmux(&self) -> I2c2GmuxR {
        I2c2GmuxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    pub fn spi1_gmux(&self) -> Spi1GmuxR {
        Spi1GmuxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    pub fn spi2_gmux(&self) -> Spi2GmuxR {
        Spi2GmuxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP5")
            .field("spi2_gmux", &self.spi2_gmux())
            .field("spi1_gmux", &self.spi1_gmux())
            .field("i2c2_gmux", &self.i2c2_gmux())
            .field("i2c1_gmux", &self.i2c1_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_gmux(&mut self) -> I2c1GmuxW<Remap5Spec> {
        I2c1GmuxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - I2C2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_gmux(&mut self) -> I2c2GmuxW<Remap5Spec> {
        I2c2GmuxW::new(self, 8)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_gmux(&mut self) -> Spi1GmuxW<Remap5Spec> {
        Spi1GmuxW::new(self, 16)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_gmux(&mut self) -> Spi2GmuxW<Remap5Spec> {
        Spi2GmuxW::new(self, 20)
    }
}
#[doc = "IO MUX remap register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap5Spec;
impl crate::RegisterSpec for Remap5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap5::R`](R) reader structure"]
impl crate::Readable for Remap5Spec {}
#[doc = "`write(|w| ..)` method takes [`remap5::W`](W) writer structure"]
impl crate::Writable for Remap5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP5 to value 0"]
impl crate::Resettable for Remap5Spec {
    const RESET_VALUE: u32 = 0;
}
