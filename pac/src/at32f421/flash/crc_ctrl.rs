#[doc = "Register `CRC_CTRL` reader"]
pub type R = crate::R<CrcCtrlSpec>;
#[doc = "Register `CRC_CTRL` writer"]
pub type W = crate::W<CrcCtrlSpec>;
#[doc = "Field `CRC_SN` reader - CRC sector numbler"]
pub type CrcSnR = crate::FieldReader<u16>;
#[doc = "Field `CRC_SN` writer - CRC sector numbler"]
pub type CrcSnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CRC_STRT` writer - CRC start"]
pub type CrcStrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CRC sector numbler"]
    #[inline(always)]
    pub fn crc_sn(&self) -> CrcSnR {
        CrcSnR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_CTRL")
            .field("crc_sn", &self.crc_sn())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC sector numbler"]
    #[inline(always)]
    #[must_use]
    pub fn crc_sn(&mut self) -> CrcSnW<CrcCtrlSpec> {
        CrcSnW::new(self, 0)
    }
    #[doc = "Bit 16 - CRC start"]
    #[inline(always)]
    #[must_use]
    pub fn crc_strt(&mut self) -> CrcStrtW<CrcCtrlSpec> {
        CrcStrtW::new(self, 16)
    }
}
#[doc = "Flash CRC controll register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtrlSpec;
impl crate::RegisterSpec for CrcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctrl::R`](R) reader structure"]
impl crate::Readable for CrcCtrlSpec {}
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
