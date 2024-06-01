#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IdenSpec>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IdenSpec>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OvfienR = crate::BitReader;
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OvfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IEN` reader - Channel 1 interrupt enable"]
pub type C1ienR = crate::BitReader;
#[doc = "Field `C1IEN` writer - Channel 1 interrupt enable"]
pub type C1ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IEN` reader - Channel 2 interrupt enable"]
pub type C2ienR = crate::BitReader;
#[doc = "Field `C2IEN` writer - Channel 2 interrupt enable"]
pub type C2ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALLIEN` reader - HALL interrupt enable"]
pub type HallienR = crate::BitReader;
#[doc = "Field `HALLIEN` writer - HALL interrupt enable"]
pub type HallienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIEN` reader - Trigger interrupt enable"]
pub type TienR = crate::BitReader;
#[doc = "Field `TIEN` writer - Trigger interrupt enable"]
pub type TienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIE` reader - Brake interrupt enable"]
pub type BrkieR = crate::BitReader;
#[doc = "Field `BRKIE` writer - Brake interrupt enable"]
pub type BrkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OvfdenR = crate::BitReader;
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OvfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1DEN` reader - Channel 1 DMA request enable"]
pub type C1denR = crate::BitReader;
#[doc = "Field `C1DEN` writer - Channel 1 DMA request enable"]
pub type C1denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2DEN` reader - Channel 2 DMA request enable"]
pub type C2denR = crate::BitReader;
#[doc = "Field `C2DEN` writer - Channel 2 DMA request enable"]
pub type C2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALLDE` reader - HALL DMA request enable"]
pub type HalldeR = crate::BitReader;
#[doc = "Field `HALLDE` writer - HALL DMA request enable"]
pub type HalldeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDEN` reader - Trigger DMA request enable"]
pub type TdenR = crate::BitReader;
#[doc = "Field `TDEN` writer - Trigger DMA request enable"]
pub type TdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OvfienR {
        OvfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&self) -> C1ienR {
        C1ienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    pub fn c2ien(&self) -> C2ienR {
        C2ienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    pub fn hallien(&self) -> HallienR {
        HallienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tien(&self) -> TienR {
        TienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Brake interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BrkieR {
        BrkieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&self) -> OvfdenR {
        OvfdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    pub fn c1den(&self) -> C1denR {
        C1denR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    pub fn c2den(&self) -> C2denR {
        C2denR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - HALL DMA request enable"]
    #[inline(always)]
    pub fn hallde(&self) -> HalldeR {
        HalldeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tden(&self) -> TdenR {
        TdenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEN")
            .field("tden", &self.tden())
            .field("hallde", &self.hallde())
            .field("c2den", &self.c2den())
            .field("c1den", &self.c1den())
            .field("ovfden", &self.ovfden())
            .field("brkie", &self.brkie())
            .field("tien", &self.tien())
            .field("hallien", &self.hallien())
            .field("c2ien", &self.c2ien())
            .field("c1ien", &self.c1ien())
            .field("ovfien", &self.ovfien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfien(&mut self) -> OvfienW<IdenSpec> {
        OvfienW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1ien(&mut self) -> C1ienW<IdenSpec> {
        C1ienW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2ien(&mut self) -> C2ienW<IdenSpec> {
        C2ienW::new(self, 2)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallien(&mut self) -> HallienW<IdenSpec> {
        HallienW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TienW<IdenSpec> {
        TienW::new(self, 6)
    }
    #[doc = "Bit 7 - Brake interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BrkieW<IdenSpec> {
        BrkieW::new(self, 7)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfden(&mut self) -> OvfdenW<IdenSpec> {
        OvfdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1den(&mut self) -> C1denW<IdenSpec> {
        C1denW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2den(&mut self) -> C2denW<IdenSpec> {
        C2denW::new(self, 10)
    }
    #[doc = "Bit 13 - HALL DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallde(&mut self) -> HalldeW<IdenSpec> {
        HalldeW::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tden(&mut self) -> TdenW<IdenSpec> {
        TdenW::new(self, 14)
    }
}
#[doc = "Interrupt/DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdenSpec;
impl crate::RegisterSpec for IdenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iden::R`](R) reader structure"]
impl crate::Readable for IdenSpec {}
#[doc = "`write(|w| ..)` method takes [`iden::W`](W) writer structure"]
impl crate::Writable for IdenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IdenSpec {
    const RESET_VALUE: u32 = 0;
}
