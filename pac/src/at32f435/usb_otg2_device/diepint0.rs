#[doc = "Register `DIEPINT0` reader"]
pub type R = crate::R<Diepint0Spec>;
#[doc = "Register `DIEPINT0` writer"]
pub type W = crate::W<Diepint0Spec>;
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
        f.debug_struct("DIEPINT0")
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
    pub fn xferc(&mut self) -> XfercW<Diepint0Spec> {
        XfercW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EptdisdW<Diepint0Spec> {
        EptdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<Diepint0Spec> {
        TimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> IntkntxfempW<Diepint0Spec> {
        IntkntxfempW::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnak(&mut self) -> IneptnakW<Diepint0Spec> {
        IneptnakW::new(self, 6)
    }
}
#[doc = "OTGFS device IN endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepint0Spec;
impl crate::RegisterSpec for Diepint0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint0::R`](R) reader structure"]
impl crate::Readable for Diepint0Spec {}
#[doc = "`write(|w| ..)` method takes [`diepint0::W`](W) writer structure"]
impl crate::Writable for Diepint0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT0 to value 0x80"]
impl crate::Resettable for Diepint0Spec {
    const RESET_VALUE: u32 = 0x80;
}
