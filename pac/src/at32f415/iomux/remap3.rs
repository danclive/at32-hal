#[doc = "Register `REMAP3` reader"]
pub type R = crate::R<Remap3Spec>;
#[doc = "Register `REMAP3` writer"]
pub type W = crate::W<Remap3Spec>;
#[doc = "Field `TMR9_GMUX` reader - TMR9 muxing"]
pub type Tmr9GmuxR = crate::FieldReader;
#[doc = "Field `TMR9_GMUX` writer - TMR9 muxing"]
pub type Tmr9GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR10_GMUX` reader - TMR10 muxing"]
pub type Tmr10GmuxR = crate::FieldReader;
#[doc = "Field `TMR10_GMUX` writer - TMR10 muxing"]
pub type Tmr10GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR11_GMUX` reader - TMR11 muxing"]
pub type Tmr11GmuxR = crate::FieldReader;
#[doc = "Field `TMR11_GMUX` writer - TMR11 muxing"]
pub type Tmr11GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_gmux(&self) -> Tmr9GmuxR {
        Tmr9GmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - TMR10 muxing"]
    #[inline(always)]
    pub fn tmr10_gmux(&self) -> Tmr10GmuxR {
        Tmr10GmuxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TMR11 muxing"]
    #[inline(always)]
    pub fn tmr11_gmux(&self) -> Tmr11GmuxR {
        Tmr11GmuxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP3")
            .field("tmr11_gmux", &self.tmr11_gmux())
            .field("tmr10_gmux", &self.tmr10_gmux())
            .field("tmr9_gmux", &self.tmr9_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_gmux(&mut self) -> Tmr9GmuxW<Remap3Spec> {
        Tmr9GmuxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - TMR10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_gmux(&mut self) -> Tmr10GmuxW<Remap3Spec> {
        Tmr10GmuxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - TMR11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_gmux(&mut self) -> Tmr11GmuxW<Remap3Spec> {
        Tmr11GmuxW::new(self, 8)
    }
}
#[doc = "IO MUX remap register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap3Spec;
impl crate::RegisterSpec for Remap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap3::R`](R) reader structure"]
impl crate::Readable for Remap3Spec {}
#[doc = "`write(|w| ..)` method takes [`remap3::W`](W) writer structure"]
impl crate::Writable for Remap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP3 to value 0"]
impl crate::Resettable for Remap3Spec {
    const RESET_VALUE: u32 = 0;
}
