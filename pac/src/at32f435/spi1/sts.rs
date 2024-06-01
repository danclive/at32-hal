#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RdbfR = crate::BitReader;
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TdbeR = crate::BitReader;
#[doc = "Field `ACS` reader - Audio channel state"]
pub type AcsR = crate::BitReader;
#[doc = "Field `TUERR` reader - Transmitter underload error"]
pub type TuerrR = crate::BitReader;
#[doc = "Field `CCERR` reader - CRC calculation error"]
pub type CcerrR = crate::BitReader;
#[doc = "Field `CCERR` writer - CRC calculation error"]
pub type CcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMERR` reader - Master mode error"]
pub type MmerrR = crate::BitReader;
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type RoerrR = crate::BitReader;
#[doc = "Field `BF` reader - Busy flag"]
pub type BfR = crate::BitReader;
#[doc = "Field `CSPAS` reader - CS pulse abnormal setting fiag"]
pub type CspasR = crate::BitReader;
#[doc = "Field `CSPAS` writer - CS pulse abnormal setting fiag"]
pub type CspasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&self) -> RdbfR {
        RdbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tdbe(&self) -> TdbeR {
        TdbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Audio channel state"]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter underload error"]
    #[inline(always)]
    pub fn tuerr(&self) -> TuerrR {
        TuerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    pub fn ccerr(&self) -> CcerrR {
        CcerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master mode error"]
    #[inline(always)]
    pub fn mmerr(&self) -> MmerrR {
        MmerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receiver overflow error"]
    #[inline(always)]
    pub fn roerr(&self) -> RoerrR {
        RoerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bf(&self) -> BfR {
        BfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CS pulse abnormal setting fiag"]
    #[inline(always)]
    pub fn cspas(&self) -> CspasR {
        CspasR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("cspas", &self.cspas())
            .field("bf", &self.bf())
            .field("roerr", &self.roerr())
            .field("mmerr", &self.mmerr())
            .field("ccerr", &self.ccerr())
            .field("tuerr", &self.tuerr())
            .field("acs", &self.acs())
            .field("tdbe", &self.tdbe())
            .field("rdbf", &self.rdbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    #[must_use]
    pub fn ccerr(&mut self) -> CcerrW<StsSpec> {
        CcerrW::new(self, 4)
    }
    #[doc = "Bit 8 - CS pulse abnormal setting fiag"]
    #[inline(always)]
    #[must_use]
    pub fn cspas(&mut self) -> CspasW<StsSpec> {
        CspasW::new(self, 8)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0x02"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0x02;
}
