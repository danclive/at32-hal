#[doc = "Register `CRC_CTRL` writer"]
pub type W = crate::W<CrcCtrlSpec>;
#[doc = "Field `CRC_SS` writer - CRC start sector"]
pub type CrcSsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CRC_SN` writer - CRC sector numbler"]
pub type CrcSnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CRC_STRT` writer - CRC start"]
pub type CrcStrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CrcCtrlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:6 - CRC start sector"]
    #[inline(always)]
    #[must_use]
    pub fn crc_ss(&mut self) -> CrcSsW<CrcCtrlSpec> {
        CrcSsW::new(self, 0)
    }
    #[doc = "Bits 7:13 - CRC sector numbler"]
    #[inline(always)]
    #[must_use]
    pub fn crc_sn(&mut self) -> CrcSnW<CrcCtrlSpec> {
        CrcSnW::new(self, 7)
    }
    #[doc = "Bit 14 - CRC start"]
    #[inline(always)]
    #[must_use]
    pub fn crc_strt(&mut self) -> CrcStrtW<CrcCtrlSpec> {
        CrcStrtW::new(self, 14)
    }
}
#[doc = "Flash CRC controler register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtrlSpec;
impl crate::RegisterSpec for CrcCtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl::W`](W) writer structure"]
impl crate::Writable for CrcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CTRL to value 0"]
impl crate::Resettable for CrcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
