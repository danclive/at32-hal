#[doc = "Register `BPDC` reader"]
pub type R = crate::R<BpdcSpec>;
#[doc = "Register `BPDC` writer"]
pub type W = crate::W<BpdcSpec>;
#[doc = "Field `LEXTEN` reader - Low speed external crystal enable"]
pub type LextenR = crate::BitReader;
#[doc = "Field `LEXTEN` writer - Low speed external crystal enable"]
pub type LextenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEXTSTBL` reader - Low speed external crystal ready"]
pub type LextstblR = crate::BitReader;
#[doc = "Field `LEXTBYPS` reader - Low speed external crystal bypass"]
pub type LextbypsR = crate::BitReader;
#[doc = "Field `LEXTBYPS` writer - Low speed external crystal bypass"]
pub type LextbypsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL` reader - RTC clock selection"]
pub type RtcselR = crate::FieldReader;
#[doc = "Field `RTCSEL` writer - RTC clock selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPDRST` reader - Battery powered domain software reset"]
pub type BpdrstR = crate::BitReader;
#[doc = "Field `BPDRST` writer - Battery powered domain software reset"]
pub type BpdrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    pub fn lexten(&self) -> LextenR {
        LextenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed external crystal ready"]
    #[inline(always)]
    pub fn lextstbl(&self) -> LextstblR {
        LextstblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    pub fn lextbyps(&self) -> LextbypsR {
        LextbypsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    pub fn bpdrst(&self) -> BpdrstR {
        BpdrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPDC")
            .field("lexten", &self.lexten())
            .field("lextstbl", &self.lextstbl())
            .field("lextbyps", &self.lextbyps())
            .field("rtcsel", &self.rtcsel())
            .field("rtcen", &self.rtcen())
            .field("bpdrst", &self.bpdrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    #[must_use]
    pub fn lexten(&mut self) -> LextenW<BpdcSpec> {
        LextenW::new(self, 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lextbyps(&mut self) -> LextbypsW<BpdcSpec> {
        LextbypsW::new(self, 2)
    }
    #[doc = "Bits 8:9 - RTC clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RtcselW<BpdcSpec> {
        RtcselW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RtcenW<BpdcSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bpdrst(&mut self) -> BpdrstW<BpdcSpec> {
        BpdrstW::new(self, 16)
    }
}
#[doc = "Battery powered domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpdcSpec;
impl crate::RegisterSpec for BpdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpdc::R`](R) reader structure"]
impl crate::Readable for BpdcSpec {}
#[doc = "`write(|w| ..)` method takes [`bpdc::W`](W) writer structure"]
impl crate::Writable for BpdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPDC to value 0"]
impl crate::Resettable for BpdcSpec {
    const RESET_VALUE: u32 = 0;
}
