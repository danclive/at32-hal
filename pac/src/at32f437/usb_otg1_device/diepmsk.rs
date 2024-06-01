#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DiepmskSpec>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DiepmskSpec>;
#[doc = "Field `XFERCMSK` reader - Transfer completed interrupt mask"]
pub type XfercmskR = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed interrupt mask"]
pub type XfercmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISMSK` reader - Endpoint disabled interrupt mask"]
pub type EptdismskR = crate::BitReader;
#[doc = "Field `EPTDISMSK` writer - Endpoint disabled interrupt mask"]
pub type EptdismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTMSK` reader - Timeout condition mask (Non-isochronous endpoints)"]
pub type TimeoutmskR = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - Timeout condition mask (Non-isochronous endpoints)"]
pub type TimeoutmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN token received when TxFIFO empty mask"]
pub type IntkntxfempmskR = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN token received when TxFIFO empty mask"]
pub type IntkntxfempmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPTMISMSK` reader - IN token received with EP mismatch mask"]
pub type IntkneptmismskR = crate::BitReader;
#[doc = "Field `INTKNEPTMISMSK` writer - IN token received with EP mismatch mask"]
pub type IntkneptmismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPTNAKMSK` reader - IN endpoint NAK effective mask"]
pub type IneptnakmskR = crate::BitReader;
#[doc = "Field `INEPTNAKMSK` writer - IN endpoint NAK effective mask"]
pub type IneptnakmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOUDRMSK` reader - FIFO underrun mask"]
pub type TxfifoudrmskR = crate::BitReader;
#[doc = "Field `TXFIFOUDRMSK` writer - FIFO underrun mask"]
pub type TxfifoudrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINMSK` reader - BNA interrupt mask"]
pub type BnainmskR = crate::BitReader;
#[doc = "Field `BNAINMSK` writer - BNA interrupt mask"]
pub type BnainmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfercmsk(&self) -> XfercmskR {
        XfercmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn eptdismsk(&self) -> EptdismskR {
        EptdismskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TimeoutmskR {
        TimeoutmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> IntkntxfempmskR {
        IntkntxfempmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn intkneptmismsk(&self) -> IntkneptmismskR {
        IntkneptmismskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn ineptnakmsk(&self) -> IneptnakmskR {
        IneptnakmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfifoudrmsk(&self) -> TxfifoudrmskR {
        TxfifoudrmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnainmsk(&self) -> BnainmskR {
        BnainmskR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("xfercmsk", &self.xfercmsk())
            .field("eptdismsk", &self.eptdismsk())
            .field("timeoutmsk", &self.timeoutmsk())
            .field("intkntxfempmsk", &self.intkntxfempmsk())
            .field("intkneptmismsk", &self.intkneptmismsk())
            .field("ineptnakmsk", &self.ineptnakmsk())
            .field("txfifoudrmsk", &self.txfifoudrmsk())
            .field("bnainmsk", &self.bnainmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XfercmskW<DiepmskSpec> {
        XfercmskW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eptdismsk(&mut self) -> EptdismskW<DiepmskSpec> {
        EptdismskW::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutmsk(&mut self) -> TimeoutmskW<DiepmskSpec> {
        TimeoutmskW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfempmsk(&mut self) -> IntkntxfempmskW<DiepmskSpec> {
        IntkntxfempmskW::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkneptmismsk(&mut self) -> IntkneptmismskW<DiepmskSpec> {
        IntkneptmismskW::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnakmsk(&mut self) -> IneptnakmskW<DiepmskSpec> {
        IneptnakmskW::new(self, 6)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoudrmsk(&mut self) -> TxfifoudrmskW<DiepmskSpec> {
        TxfifoudrmskW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnainmsk(&mut self) -> BnainmskW<DiepmskSpec> {
        BnainmskW::new(self, 9)
    }
}
#[doc = "OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepmskSpec;
impl crate::RegisterSpec for DiepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DiepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DiepmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DiepmskSpec {
    const RESET_VALUE: u32 = 0;
}
