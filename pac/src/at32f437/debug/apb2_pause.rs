#[doc = "Register `APB2_PAUSE` reader"]
pub type R = crate::R<Apb2PauseSpec>;
#[doc = "Register `APB2_PAUSE` writer"]
pub type W = crate::W<Apb2PauseSpec>;
#[doc = "Field `TMR1_PAUSE` reader - TMR1_PAUSE"]
pub type Tmr1PauseR = crate::BitReader;
#[doc = "Field `TMR1_PAUSE` writer - TMR1_PAUSE"]
pub type Tmr1PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8_PAUSE` reader - TMR8_PAUSE"]
pub type Tmr8PauseR = crate::BitReader;
#[doc = "Field `TMR8_PAUSE` writer - TMR8_PAUSE"]
pub type Tmr8PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20_PAUSE` reader - TIM20_PAUSE"]
pub type Tmr20PauseR = crate::BitReader;
#[doc = "Field `TMR20_PAUSE` writer - TIM20_PAUSE"]
pub type Tmr20PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9_PAUSE` reader - TMR9_PAUSE"]
pub type Tmr9PauseR = crate::BitReader;
#[doc = "Field `TMR9_PAUSE` writer - TMR9_PAUSE"]
pub type Tmr9PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10_PAUSE` reader - TMR10_PAUSE"]
pub type Tmr10PauseR = crate::BitReader;
#[doc = "Field `TMR10_PAUSE` writer - TMR10_PAUSE"]
pub type Tmr10PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11_PAUSE` reader - TMR11_PAUSE"]
pub type Tmr11PauseR = crate::BitReader;
#[doc = "Field `TMR11_PAUSE` writer - TMR11_PAUSE"]
pub type Tmr11PauseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> Tmr1PauseR {
        Tmr1PauseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR8_PAUSE"]
    #[inline(always)]
    pub fn tmr8_pause(&self) -> Tmr8PauseR {
        Tmr8PauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM20_PAUSE"]
    #[inline(always)]
    pub fn tmr20_pause(&self) -> Tmr20PauseR {
        Tmr20PauseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR9_PAUSE"]
    #[inline(always)]
    pub fn tmr9_pause(&self) -> Tmr9PauseR {
        Tmr9PauseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TMR10_PAUSE"]
    #[inline(always)]
    pub fn tmr10_pause(&self) -> Tmr10PauseR {
        Tmr10PauseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TMR11_PAUSE"]
    #[inline(always)]
    pub fn tmr11_pause(&self) -> Tmr11PauseR {
        Tmr11PauseR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_PAUSE")
            .field("tmr1_pause", &self.tmr1_pause())
            .field("tmr8_pause", &self.tmr8_pause())
            .field("tmr20_pause", &self.tmr20_pause())
            .field("tmr9_pause", &self.tmr9_pause())
            .field("tmr10_pause", &self.tmr10_pause())
            .field("tmr11_pause", &self.tmr11_pause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TMR1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_pause(&mut self) -> Tmr1PauseW<Apb2PauseSpec> {
        Tmr1PauseW::new(self, 0)
    }
    #[doc = "Bit 1 - TMR8_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8_pause(&mut self) -> Tmr8PauseW<Apb2PauseSpec> {
        Tmr8PauseW::new(self, 1)
    }
    #[doc = "Bit 6 - TIM20_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20_pause(&mut self) -> Tmr20PauseW<Apb2PauseSpec> {
        Tmr20PauseW::new(self, 6)
    }
    #[doc = "Bit 16 - TMR9_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_pause(&mut self) -> Tmr9PauseW<Apb2PauseSpec> {
        Tmr9PauseW::new(self, 16)
    }
    #[doc = "Bit 17 - TMR10_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_pause(&mut self) -> Tmr10PauseW<Apb2PauseSpec> {
        Tmr10PauseW::new(self, 17)
    }
    #[doc = "Bit 18 - TMR11_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_pause(&mut self) -> Tmr11PauseW<Apb2PauseSpec> {
        Tmr11PauseW::new(self, 18)
    }
}
#[doc = "DEBUG APB2 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2_pause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2_pause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2PauseSpec;
impl crate::RegisterSpec for Apb2PauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2_pause::R`](R) reader structure"]
impl crate::Readable for Apb2PauseSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2_pause::W`](W) writer structure"]
impl crate::Writable for Apb2PauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2_PAUSE to value 0"]
impl crate::Resettable for Apb2PauseSpec {
    const RESET_VALUE: u32 = 0;
}
