#[doc = "Register `MMCRI` reader"]
pub type R = crate::R<MmcriSpec>;
#[doc = "Register `MMCRI` writer"]
pub type W = crate::W<MmcriSpec>;
#[doc = "Field `RFCE` reader - Received frames CRC error"]
pub type RfceR = crate::BitReader;
#[doc = "Field `RFCE` writer - Received frames CRC error"]
pub type RfceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAE` reader - Received frames alignment error"]
pub type RfaeR = crate::BitReader;
#[doc = "Field `RFAE` writer - Received frames alignment error"]
pub type RfaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUF` reader - Received good unicast frames"]
pub type RgufR = crate::BitReader;
#[doc = "Field `RGUF` writer - Received good unicast frames"]
pub type RgufW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frames CRC error"]
    #[inline(always)]
    pub fn rfce(&self) -> RfceR {
        RfceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error"]
    #[inline(always)]
    pub fn rfae(&self) -> RfaeR {
        RfaeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames"]
    #[inline(always)]
    pub fn rguf(&self) -> RgufR {
        RgufR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRI")
            .field("rfce", &self.rfce())
            .field("rfae", &self.rfae())
            .field("rguf", &self.rguf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Received frames CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RfceW<MmcriSpec> {
        RfceW::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn rfae(&mut self) -> RfaeW<MmcriSpec> {
        RfaeW::new(self, 6)
    }
    #[doc = "Bit 17 - Received good unicast frames"]
    #[inline(always)]
    #[must_use]
    pub fn rguf(&mut self) -> RgufW<MmcriSpec> {
        RgufW::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcriSpec;
impl crate::RegisterSpec for MmcriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcri::R`](R) reader structure"]
impl crate::Readable for MmcriSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcri::W`](W) writer structure"]
impl crate::Writable for MmcriSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCRI to value 0"]
impl crate::Resettable for MmcriSpec {
    const RESET_VALUE: u32 = 0;
}
