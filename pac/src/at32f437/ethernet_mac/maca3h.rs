#[doc = "Register `MACA3H` reader"]
pub type R = crate::R<Maca3hSpec>;
#[doc = "Register `MACA3H` writer"]
pub type W = crate::W<Maca3hSpec>;
#[doc = "Field `MA3H` reader - MAC address3 high"]
pub type Ma3hR = crate::FieldReader<u16>;
#[doc = "Field `MA3H` writer - MAC address3 high"]
pub type Ma3hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    pub fn ma3h(&self) -> Ma3hR {
        Ma3hR::new((self.bits & 0xffff) as u16)
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
        f.debug_struct("MACA3H")
            .field("ma3h", &self.ma3h())
            .field("mbc", &self.mbc())
            .field("sa", &self.sa())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma3h(&mut self) -> Ma3hW<Maca3hSpec> {
        Ma3hW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MbcW<Maca3hSpec> {
        MbcW::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<Maca3hSpec> {
        SaW::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AeW<Maca3hSpec> {
        AeW::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca3hSpec;
impl crate::RegisterSpec for Maca3hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3h::R`](R) reader structure"]
impl crate::Readable for Maca3hSpec {}
#[doc = "`write(|w| ..)` method takes [`maca3h::W`](W) writer structure"]
impl crate::Writable for Maca3hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA3H to value 0xffff"]
impl crate::Resettable for Maca3hSpec {
    const RESET_VALUE: u32 = 0xffff;
}
