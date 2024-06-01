#[doc = "Register `MACIMR` reader"]
pub type R = crate::R<MacimrSpec>;
#[doc = "Register `MACIMR` writer"]
pub type W = crate::W<MacimrSpec>;
#[doc = "Field `PIM` reader - PMT interrupt mask"]
pub type PimR = crate::BitReader;
#[doc = "Field `PIM` writer - PMT interrupt mask"]
pub type PimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM` reader - Timestamp interrupt mask"]
pub type TimR = crate::BitReader;
#[doc = "Field `TIM` writer - Timestamp interrupt mask"]
pub type TimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pim(&self) -> PimR {
        PimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    pub fn tim(&self) -> TimR {
        TimR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACIMR")
            .field("pim", &self.pim())
            .field("tim", &self.tim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pim(&mut self) -> PimW<MacimrSpec> {
        PimW::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tim(&mut self) -> TimW<MacimrSpec> {
        TimW::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacimrSpec;
impl crate::RegisterSpec for MacimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macimr::R`](R) reader structure"]
impl crate::Readable for MacimrSpec {}
#[doc = "`write(|w| ..)` method takes [`macimr::W`](W) writer structure"]
impl crate::Writable for MacimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MacimrSpec {
    const RESET_VALUE: u32 = 0;
}
