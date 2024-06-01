#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IdenSpec>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IdenSpec>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OvfienR = crate::BitReader;
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OvfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OvfdenR = crate::BitReader;
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OvfdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OvfienR {
        OvfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&self) -> OvfdenR {
        OvfdenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEN")
            .field("ovfden", &self.ovfden())
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
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfden(&mut self) -> OvfdenW<IdenSpec> {
        OvfdenW::new(self, 8)
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
