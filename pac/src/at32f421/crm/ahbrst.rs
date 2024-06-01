#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
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
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiofrst", &self.gpiofrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 17 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GpioarstW<AhbrstSpec> {
        GpioarstW::new(self, 17)
    }
    #[doc = "Bit 18 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GpiobrstW<AhbrstSpec> {
        GpiobrstW::new(self, 18)
    }
    #[doc = "Bit 19 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GpiocrstW<AhbrstSpec> {
        GpiocrstW::new(self, 19)
    }
    #[doc = "Bit 22 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GpiofrstW<AhbrstSpec> {
        GpiofrstW::new(self, 22)
    }
}
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AhbrstSpec {
    const RESET_VALUE: u32 = 0;
}
