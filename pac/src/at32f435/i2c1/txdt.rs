#[doc = "Register `TXDT` reader"]
pub type R = crate::R<TxdtSpec>;
#[doc = "Register `TXDT` writer"]
pub type W = crate::W<TxdtSpec>;
#[doc = "Field `DT` reader - Transmit data register"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Transmit data register"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data register"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDT").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<TxdtSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdtSpec;
impl crate::RegisterSpec for TxdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdt::R`](R) reader structure"]
impl crate::Readable for TxdtSpec {}
#[doc = "`write(|w| ..)` method takes [`txdt::W`](W) writer structure"]
impl crate::Writable for TxdtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDT to value 0"]
impl crate::Resettable for TxdtSpec {
    const RESET_VALUE: u32 = 0;
}
