#[doc = "Register `LLCTRL` reader"]
pub type R = crate::R<LlctrlSpec>;
#[doc = "Register `LLCTRL` writer"]
pub type W = crate::W<LlctrlSpec>;
#[doc = "Field `S1LLEN` reader - Stream 1 link list enable"]
pub type S1llenR = crate::BitReader;
#[doc = "Field `S1LLEN` writer - Stream 1 link list enable"]
pub type S1llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2LLEN` reader - Stream 2 link list enable"]
pub type S2llenR = crate::BitReader;
#[doc = "Field `S2LLEN` writer - Stream 2 link list enable"]
pub type S2llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3LLEN` reader - Stream 3 link list enable"]
pub type S3llenR = crate::BitReader;
#[doc = "Field `S3LLEN` writer - Stream 3 link list enable"]
pub type S3llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S4LLEN` reader - Stream 4 link list enable"]
pub type S4llenR = crate::BitReader;
#[doc = "Field `S4LLEN` writer - Stream 4 link list enable"]
pub type S4llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S5LLEN` reader - Stream 5 link list enable"]
pub type S5llenR = crate::BitReader;
#[doc = "Field `S5LLEN` writer - Stream 5 link list enable"]
pub type S5llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S6LLEN` reader - Stream 6 link list enable"]
pub type S6llenR = crate::BitReader;
#[doc = "Field `S6LLEN` writer - Stream 6 link list enable"]
pub type S6llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S7LLEN` reader - Stream 7 link list enable"]
pub type S7llenR = crate::BitReader;
#[doc = "Field `S7LLEN` writer - Stream 7 link list enable"]
pub type S7llenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S8LLEN` reader - Stream 8 link list enable"]
pub type S8llenR = crate::BitReader;
#[doc = "Field `S8LLEN` writer - Stream 8 link list enable"]
pub type S8llenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 link list enable"]
    #[inline(always)]
    pub fn s1llen(&self) -> S1llenR {
        S1llenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stream 2 link list enable"]
    #[inline(always)]
    pub fn s2llen(&self) -> S2llenR {
        S2llenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 3 link list enable"]
    #[inline(always)]
    pub fn s3llen(&self) -> S3llenR {
        S3llenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 4 link list enable"]
    #[inline(always)]
    pub fn s4llen(&self) -> S4llenR {
        S4llenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 link list enable"]
    #[inline(always)]
    pub fn s5llen(&self) -> S5llenR {
        S5llenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 6 link list enable"]
    #[inline(always)]
    pub fn s6llen(&self) -> S6llenR {
        S6llenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 7 link list enable"]
    #[inline(always)]
    pub fn s7llen(&self) -> S7llenR {
        S7llenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stream 8 link list enable"]
    #[inline(always)]
    pub fn s8llen(&self) -> S8llenR {
        S8llenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLCTRL")
            .field("s1llen", &self.s1llen())
            .field("s2llen", &self.s2llen())
            .field("s3llen", &self.s3llen())
            .field("s4llen", &self.s4llen())
            .field("s5llen", &self.s5llen())
            .field("s6llen", &self.s6llen())
            .field("s7llen", &self.s7llen())
            .field("s8llen", &self.s8llen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1llen(&mut self) -> S1llenW<LlctrlSpec> {
        S1llenW::new(self, 0)
    }
    #[doc = "Bit 1 - Stream 2 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2llen(&mut self) -> S2llenW<LlctrlSpec> {
        S2llenW::new(self, 1)
    }
    #[doc = "Bit 2 - Stream 3 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3llen(&mut self) -> S3llenW<LlctrlSpec> {
        S3llenW::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 4 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s4llen(&mut self) -> S4llenW<LlctrlSpec> {
        S4llenW::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s5llen(&mut self) -> S5llenW<LlctrlSpec> {
        S5llenW::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 6 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s6llen(&mut self) -> S6llenW<LlctrlSpec> {
        S6llenW::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 7 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s7llen(&mut self) -> S7llenW<LlctrlSpec> {
        S7llenW::new(self, 6)
    }
    #[doc = "Bit 7 - Stream 8 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s8llen(&mut self) -> S8llenW<LlctrlSpec> {
        S8llenW::new(self, 7)
    }
}
#[doc = "DMA Link List Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LlctrlSpec;
impl crate::RegisterSpec for LlctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llctrl::R`](R) reader structure"]
impl crate::Readable for LlctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`llctrl::W`](W) writer structure"]
impl crate::Writable for LlctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLCTRL to value 0"]
impl crate::Resettable for LlctrlSpec {
    const RESET_VALUE: u32 = 0;
}
