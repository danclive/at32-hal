#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Field `SPEED` reader - I2C bus speed config"]
pub type SpeedR = crate::FieldReader<u16>;
#[doc = "Field `SPEED` writer - I2C bus speed config"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DUTYMODE` reader - Fast mode duty cycle"]
pub type DutymodeR = crate::BitReader;
#[doc = "Field `DUTYMODE` writer - Fast mode duty cycle"]
pub type DutymodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEEDMODE` reader - Speed mode selection"]
pub type SpeedmodeR = crate::BitReader;
#[doc = "Field `SPEEDMODE` writer - Speed mode selection"]
pub type SpeedmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - I2C bus speed config"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn dutymode(&self) -> DutymodeR {
        DutymodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Speed mode selection"]
    #[inline(always)]
    pub fn speedmode(&self) -> SpeedmodeR {
        SpeedmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("speedmode", &self.speedmode())
            .field("dutymode", &self.dutymode())
            .field("speed", &self.speed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C bus speed config"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<ClkctrlSpec> {
        SpeedW::new(self, 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dutymode(&mut self) -> DutymodeW<ClkctrlSpec> {
        DutymodeW::new(self, 14)
    }
    #[doc = "Bit 15 - Speed mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn speedmode(&mut self) -> SpeedmodeW<ClkctrlSpec> {
        SpeedmodeW::new(self, 15)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
