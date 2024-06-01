#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `DMAEN` reader - DMA handshake enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA handshake enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDIE` reader - Command complete interrupt enable"]
pub type CmdieR = crate::BitReader;
#[doc = "Field `CMDIE` writer - Command complete interrupt enable"]
pub type CmdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOTHOD` reader - TxFIFO thod"]
pub type TxfifothodR = crate::FieldReader;
#[doc = "Field `TXFIFOTHOD` writer - TxFIFO thod"]
pub type TxfifothodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXFIFOTHOD` reader - RxFIFO thod"]
pub type RxfifothodR = crate::FieldReader;
#[doc = "Field `RXFIFOTHOD` writer - RxFIFO thod"]
pub type RxfifothodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - DMA handshake enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command complete interrupt enable"]
    #[inline(always)]
    pub fn cmdie(&self) -> CmdieR {
        CmdieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - TxFIFO thod"]
    #[inline(always)]
    pub fn txfifothod(&self) -> TxfifothodR {
        TxfifothodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - RxFIFO thod"]
    #[inline(always)]
    pub fn rxfifothod(&self) -> RxfifothodR {
        RxfifothodR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("dmaen", &self.dmaen())
            .field("cmdie", &self.cmdie())
            .field("txfifothod", &self.txfifothod())
            .field("rxfifothod", &self.rxfifothod())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA handshake enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<Ctrl2Spec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Command complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdie(&mut self) -> CmdieW<Ctrl2Spec> {
        CmdieW::new(self, 1)
    }
    #[doc = "Bits 8:9 - TxFIFO thod"]
    #[inline(always)]
    #[must_use]
    pub fn txfifothod(&mut self) -> TxfifothodW<Ctrl2Spec> {
        TxfifothodW::new(self, 8)
    }
    #[doc = "Bits 12:13 - RxFIFO thod"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifothod(&mut self) -> RxfifothodW<Ctrl2Spec> {
        RxfifothodW::new(self, 12)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x01"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x01;
}
