#[doc = "Register `REMAP4` reader"]
pub type R = crate::R<Remap4Spec>;
#[doc = "Register `REMAP4` writer"]
pub type W = crate::W<Remap4Spec>;
#[doc = "Field `TMR1_GMUX` reader - TMR1 muxing"]
pub type Tmr1GmuxR = crate::FieldReader;
#[doc = "Field `TMR1_GMUX` writer - TMR1 muxing"]
pub type Tmr1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR2_GMUX` reader - TMR2 muxing"]
pub type Tmr2GmuxR = crate::FieldReader;
#[doc = "Field `TMR2_GMUX` writer - TMR2 muxing"]
pub type Tmr2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMR2ITR1_GMUX` reader - TMR2 internal trigger 1 muxing"]
pub type Tmr2itr1GmuxR = crate::BitReader;
#[doc = "Field `TMR2ITR1_GMUX` writer - TMR2 internal trigger 1 muxing"]
pub type Tmr2itr1GmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3_GMUX` reader - TMR3 muxing"]
pub type Tmr3GmuxR = crate::FieldReader;
#[doc = "Field `TMR3_GMUX` writer - TMR3 muxing"]
pub type Tmr3GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR5_GMUX` reader - TMR5 muxing"]
pub type Tmr5GmuxR = crate::FieldReader;
#[doc = "Field `TMR5_GMUX` writer - TMR5 muxing"]
pub type Tmr5GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMR5CH4_GMUX` reader - TMR5 channel4 internal muxing"]
pub type Tmr5ch4GmuxR = crate::BitReader;
#[doc = "Field `TMR5CH4_GMUX` writer - TMR5 channel4 internal muxing"]
pub type Tmr5ch4GmuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_gmux(&self) -> Tmr1GmuxR {
        Tmr1GmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_gmux(&self) -> Tmr2GmuxR {
        Tmr2GmuxR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    pub fn tmr2itr1_gmux(&self) -> Tmr2itr1GmuxR {
        Tmr2itr1GmuxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_gmux(&self) -> Tmr3GmuxR {
        Tmr3GmuxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TMR5 muxing"]
    #[inline(always)]
    pub fn tmr5_gmux(&self) -> Tmr5GmuxR {
        Tmr5GmuxR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_gmux(&self) -> Tmr5ch4GmuxR {
        Tmr5ch4GmuxR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP4")
            .field("tmr1_gmux", &self.tmr1_gmux())
            .field("tmr2_gmux", &self.tmr2_gmux())
            .field("tmr2itr1_gmux", &self.tmr2itr1_gmux())
            .field("tmr3_gmux", &self.tmr3_gmux())
            .field("tmr5_gmux", &self.tmr5_gmux())
            .field("tmr5ch4_gmux", &self.tmr5ch4_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_gmux(&mut self) -> Tmr1GmuxW<Remap4Spec> {
        Tmr1GmuxW::new(self, 0)
    }
    #[doc = "Bits 4:6 - TMR2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_gmux(&mut self) -> Tmr2GmuxW<Remap4Spec> {
        Tmr2GmuxW::new(self, 4)
    }
    #[doc = "Bit 7 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2itr1_gmux(&mut self) -> Tmr2itr1GmuxW<Remap4Spec> {
        Tmr2itr1GmuxW::new(self, 7)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_gmux(&mut self) -> Tmr3GmuxW<Remap4Spec> {
        Tmr3GmuxW::new(self, 8)
    }
    #[doc = "Bits 16:18 - TMR5 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_gmux(&mut self) -> Tmr5GmuxW<Remap4Spec> {
        Tmr5GmuxW::new(self, 16)
    }
    #[doc = "Bit 19 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5ch4_gmux(&mut self) -> Tmr5ch4GmuxW<Remap4Spec> {
        Tmr5ch4GmuxW::new(self, 19)
    }
}
#[doc = "IO MUX remap register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap4Spec;
impl crate::RegisterSpec for Remap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap4::R`](R) reader structure"]
impl crate::Readable for Remap4Spec {}
#[doc = "`write(|w| ..)` method takes [`remap4::W`](W) writer structure"]
impl crate::Writable for Remap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP4 to value 0"]
impl crate::Resettable for Remap4Spec {
    const RESET_VALUE: u32 = 0;
}
