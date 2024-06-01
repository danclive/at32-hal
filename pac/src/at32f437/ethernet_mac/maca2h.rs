#[doc = "Register `MACA2H` reader"]
pub type R = crate::R<Maca2hSpec>;
#[doc = "Register `MACA2H` writer"]
pub type W = crate::W<Maca2hSpec>;
#[doc = "Field `MA2H` reader - MAC address 2 high"]
pub type Ma2hR = crate::FieldReader<u16>;
#[doc = "Field `MA2H` writer - MAC address 2 high"]
pub type Ma2hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MbcR = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MbcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source address"]
pub type SaR = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address enable"]
pub type AeR = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC address 2 high"]
    #[inline(always)]
    pub fn ma2h(&self) -> Ma2hR {
        Ma2hR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MbcR {
        MbcR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA2H")
            .field("ma2h", &self.ma2h())
            .field("mbc", &self.mbc())
            .field("sa", &self.sa())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address 2 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma2h(&mut self) -> Ma2hW<Maca2hSpec> {
        Ma2hW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MbcW<Maca2hSpec> {
        MbcW::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<Maca2hSpec> {
        SaW::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AeW<Maca2hSpec> {
        AeW::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca2hSpec;
impl crate::RegisterSpec for Maca2hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2h::R`](R) reader structure"]
impl crate::Readable for Maca2hSpec {}
#[doc = "`write(|w| ..)` method takes [`maca2h::W`](W) writer structure"]
impl crate::Writable for Maca2hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA2H to value 0x50"]
impl crate::Resettable for Maca2hSpec {
    const RESET_VALUE: u32 = 0x50;
}
