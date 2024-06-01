#[doc = "Register `DIEPINT1` reader"]
pub type R = crate::R<Diepint1Spec>;
#[doc = "Register `DIEPINT1` writer"]
pub type W = crate::W<Diepint1Spec>;
#[doc = "Field `XFERC` reader - Transfer completed interrupt"]
pub type XfercR = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed interrupt"]
pub type XfercW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISD` reader - Endpoint disabled interrupt"]
pub type EptdisdR = crate::BitReader;
#[doc = "Field `EPTDISD` writer - Endpoint disabled interrupt"]
pub type EptdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Timeout condition"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout condition"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMP` reader - IN token received when TxFIFO is empty"]
pub type IntkntxfempR = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - IN token received when TxFIFO is empty"]
pub type IntkntxfempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPTNAK` reader - IN endpoint NAK effective"]
pub type IneptnakR = crate::BitReader;
#[doc = "Field `INEPTNAK` writer - IN endpoint NAK effective"]
pub type IneptnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO empty"]
pub type TxfempR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xferc(&self) -> XfercR {
        XfercR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn eptdisd(&self) -> EptdisdR {
        EptdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> IntkntxfempR {
        IntkntxfempR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn ineptnak(&self) -> IneptnakR {
        IneptnakR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfemp(&self) -> TxfempR {
        TxfempR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT1")
            .field("xferc", &self.xferc())
            .field("eptdisd", &self.eptdisd())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("ineptnak", &self.ineptnak())
            .field("txfemp", &self.txfemp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XfercW<Diepint1Spec> {
        XfercW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EptdisdW<Diepint1Spec> {
        EptdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<Diepint1Spec> {
        TimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> IntkntxfempW<Diepint1Spec> {
        IntkntxfempW::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnak(&mut self) -> IneptnakW<Diepint1Spec> {
        IneptnakW::new(self, 6)
    }
}
#[doc = "OTGFS device IN endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepint1Spec;
impl crate::RegisterSpec for Diepint1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint1::R`](R) reader structure"]
impl crate::Readable for Diepint1Spec {}
#[doc = "`write(|w| ..)` method takes [`diepint1::W`](W) writer structure"]
impl crate::Writable for Diepint1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT1 to value 0x80"]
impl crate::Resettable for Diepint1Spec {
    const RESET_VALUE: u32 = 0x80;
}
