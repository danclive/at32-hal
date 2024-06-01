#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `DMAREN` reader - DMA receive enable"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receive enable"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATEN` reader - DMA transmit enable"]
pub type DmatenR = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA transmit enable"]
pub type DmatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWCSOE` reader - Hardware CS output enable"]
pub type HwcsoeR = crate::BitReader;
#[doc = "Field `HWCSOE` writer - Hardware CS output enable"]
pub type HwcsoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBFIE` reader - Receive data buffer full interrupt enable"]
pub type RdbfieR = crate::BitReader;
#[doc = "Field `RDBFIE` writer - Receive data buffer full interrupt enable"]
pub type RdbfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBEIE` reader - Transmit data buffer empty interrupt enable"]
pub type TdbeieR = crate::BitReader;
#[doc = "Field `TDBEIE` writer - Transmit data buffer empty interrupt enable"]
pub type TdbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIV3` reader - Master clock frequency division bit3"]
pub type Mdiv3R = crate::BitReader;
#[doc = "Field `MDIV3` writer - Master clock frequency division bit3"]
pub type Mdiv3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DmatenR {
        DmatenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware CS output enable"]
    #[inline(always)]
    pub fn hwcsoe(&self) -> HwcsoeR {
        HwcsoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    pub fn rdbfie(&self) -> RdbfieR {
        RdbfieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tdbeie(&self) -> TdbeieR {
        TdbeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    pub fn mdiv3(&self) -> Mdiv3R {
        Mdiv3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("mdiv3", &self.mdiv3())
            .field("tdbeie", &self.tdbeie())
            .field("rdbfie", &self.rdbfie())
            .field("errie", &self.errie())
            .field("hwcsoe", &self.hwcsoe())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DmarenW<Ctrl2Spec> {
        DmarenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DmatenW<Ctrl2Spec> {
        DmatenW::new(self, 1)
    }
    #[doc = "Bit 2 - Hardware CS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwcsoe(&mut self) -> HwcsoeW<Ctrl2Spec> {
        HwcsoeW::new(self, 2)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctrl2Spec> {
        ErrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfie(&mut self) -> RdbfieW<Ctrl2Spec> {
        RdbfieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeie(&mut self) -> TdbeieW<Ctrl2Spec> {
        TdbeieW::new(self, 7)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv3(&mut self) -> Mdiv3W<Ctrl2Spec> {
        Mdiv3W::new(self, 8)
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
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
