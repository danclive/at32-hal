#[doc = "Register `S2DCTRL` reader"]
pub type R = crate::R<S2dctrlSpec>;
#[doc = "Register `S2DCTRL` writer"]
pub type W = crate::W<S2dctrlSpec>;
#[doc = "Field `S1_2DEN` reader - Stream 1 2D transfer enable"]
pub type S1_2denR = crate::BitReader;
#[doc = "Field `S1_2DEN` writer - Stream 1 2D transfer enable"]
pub type S1_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2_2DEN` reader - Stream 2 2D transfer enable"]
pub type S2_2denR = crate::BitReader;
#[doc = "Field `S2_2DEN` writer - Stream 2 2D transfer enable"]
pub type S2_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3_2DEN` reader - Stream 3 2D transfer enable"]
pub type S3_2denR = crate::BitReader;
#[doc = "Field `S3_2DEN` writer - Stream 3 2D transfer enable"]
pub type S3_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S4_2DEN` reader - Stream 4 2D transfer enable"]
pub type S4_2denR = crate::BitReader;
#[doc = "Field `S4_2DEN` writer - Stream 4 2D transfer enable"]
pub type S4_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S5_2DEN` reader - Stream 5 2D transfer enable"]
pub type S5_2denR = crate::BitReader;
#[doc = "Field `S5_2DEN` writer - Stream 5 2D transfer enable"]
pub type S5_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S6_2DEN` reader - Stream 6 2D transfer enable"]
pub type S6_2denR = crate::BitReader;
#[doc = "Field `S6_2DEN` writer - Stream 6 2D transfer enable"]
pub type S6_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S7_2DEN` reader - Stream 7 2D transfer enable"]
pub type S7_2denR = crate::BitReader;
#[doc = "Field `S7_2DEN` writer - Stream 7 2D transfer enable"]
pub type S7_2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S8_2DEN` reader - Stream 8 2D transfer enable"]
pub type S8_2denR = crate::BitReader;
#[doc = "Field `S8_2DEN` writer - Stream 8 2D transfer enable"]
pub type S8_2denW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 2D transfer enable"]
    #[inline(always)]
    pub fn s1_2den(&self) -> S1_2denR {
        S1_2denR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stream 2 2D transfer enable"]
    #[inline(always)]
    pub fn s2_2den(&self) -> S2_2denR {
        S2_2denR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 3 2D transfer enable"]
    #[inline(always)]
    pub fn s3_2den(&self) -> S3_2denR {
        S3_2denR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 4 2D transfer enable"]
    #[inline(always)]
    pub fn s4_2den(&self) -> S4_2denR {
        S4_2denR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 2D transfer enable"]
    #[inline(always)]
    pub fn s5_2den(&self) -> S5_2denR {
        S5_2denR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 6 2D transfer enable"]
    #[inline(always)]
    pub fn s6_2den(&self) -> S6_2denR {
        S6_2denR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 7 2D transfer enable"]
    #[inline(always)]
    pub fn s7_2den(&self) -> S7_2denR {
        S7_2denR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stream 8 2D transfer enable"]
    #[inline(always)]
    pub fn s8_2den(&self) -> S8_2denR {
        S8_2denR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2DCTRL")
            .field("s1_2den", &self.s1_2den())
            .field("s2_2den", &self.s2_2den())
            .field("s3_2den", &self.s3_2den())
            .field("s4_2den", &self.s4_2den())
            .field("s5_2den", &self.s5_2den())
            .field("s6_2den", &self.s6_2den())
            .field("s7_2den", &self.s7_2den())
            .field("s8_2den", &self.s8_2den())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1_2den(&mut self) -> S1_2denW<S2dctrlSpec> {
        S1_2denW::new(self, 0)
    }
    #[doc = "Bit 1 - Stream 2 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2_2den(&mut self) -> S2_2denW<S2dctrlSpec> {
        S2_2denW::new(self, 1)
    }
    #[doc = "Bit 2 - Stream 3 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3_2den(&mut self) -> S3_2denW<S2dctrlSpec> {
        S3_2denW::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 4 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s4_2den(&mut self) -> S4_2denW<S2dctrlSpec> {
        S4_2denW::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s5_2den(&mut self) -> S5_2denW<S2dctrlSpec> {
        S5_2denW::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 6 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s6_2den(&mut self) -> S6_2denW<S2dctrlSpec> {
        S6_2denW::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 7 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s7_2den(&mut self) -> S7_2denW<S2dctrlSpec> {
        S7_2denW::new(self, 6)
    }
    #[doc = "Bit 7 - Stream 8 2D transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn s8_2den(&mut self) -> S8_2denW<S2dctrlSpec> {
        S8_2denW::new(self, 7)
    }
}
#[doc = "EDMA 2D Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2dctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2dctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2dctrlSpec;
impl crate::RegisterSpec for S2dctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2dctrl::R`](R) reader structure"]
impl crate::Readable for S2dctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`s2dctrl::W`](W) writer structure"]
impl crate::Writable for S2dctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S2DCTRL to value 0"]
impl crate::Resettable for S2dctrlSpec {
    const RESET_VALUE: u32 = 0;
}
