#[doc = "Register `CRC_ADDR` writer"]
pub type W = crate::W<CrcAddrSpec>;
#[doc = "Field `CRC_ADDR` writer - CRC address"]
pub type CrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CrcAddrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_addr(&mut self) -> CrcAddrW<CrcAddrSpec> {
        CrcAddrW::new(self, 0)
    }
}
#[doc = "Flash CRC data start address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcAddrSpec;
impl crate::RegisterSpec for CrcAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crc_addr::W`](W) writer structure"]
impl crate::Writable for CrcAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_ADDR to value 0"]
impl crate::Resettable for CrcAddrSpec {
    const RESET_VALUE: u32 = 0;
}
