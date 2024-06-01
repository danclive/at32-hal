#[doc = "Register `ISTS` reader"]
pub type R = crate::R<IstsSpec>;
#[doc = "Register `ISTS` writer"]
pub type W = crate::W<IstsSpec>;
#[doc = "Field `OVFIF` reader - Overflow interrupt flag"]
pub type OvfifR = crate::BitReader;
#[doc = "Field `OVFIF` writer - Overflow interrupt flag"]
pub type OvfifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IF` reader - Channel 1 interrupt flag"]
pub type C1ifR = crate::BitReader;
#[doc = "Field `C1IF` writer - Channel 1 interrupt flag"]
pub type C1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IF` reader - Channel 2 interrupt flag"]
pub type C2ifR = crate::BitReader;
#[doc = "Field `C2IF` writer - Channel 2 interrupt flag"]
pub type C2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TrgifR = crate::BitReader;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TrgifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1RF` reader - Channel 1 recapture flag"]
pub type C1rfR = crate::BitReader;
#[doc = "Field `C1RF` writer - Channel 1 recapture flag"]
pub type C1rfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2RF` reader - Channel 2 recapture flag"]
pub type C2rfR = crate::BitReader;
#[doc = "Field `C2RF` writer - Channel 2 recapture flag"]
pub type C2rfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OvfifR {
        OvfifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1ifR {
        C1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2ifR {
        C2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TrgifR {
        TrgifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&self) -> C1rfR {
        C1rfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&self) -> C2rfR {
        C2rfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("c2rf", &self.c2rf())
            .field("c1rf", &self.c1rf())
            .field("trgif", &self.trgif())
            .field("c2if", &self.c2if())
            .field("c1if", &self.c1if())
            .field("ovfif", &self.ovfif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovfif(&mut self) -> OvfifW<IstsSpec> {
        OvfifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1if(&mut self) -> C1ifW<IstsSpec> {
        C1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c2if(&mut self) -> C2ifW<IstsSpec> {
        C2ifW::new(self, 2)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TrgifW<IstsSpec> {
        TrgifW::new(self, 6)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1rf(&mut self) -> C1rfW<IstsSpec> {
        C1rfW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c2rf(&mut self) -> C2rfW<IstsSpec> {
        C2rfW::new(self, 10)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ists::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstsSpec;
impl crate::RegisterSpec for IstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for IstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ists::W`](W) writer structure"]
impl crate::Writable for IstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for IstsSpec {
    const RESET_VALUE: u32 = 0;
}
