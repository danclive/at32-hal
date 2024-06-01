#[doc = "Register `MACMIIADDR` reader"]
pub type R = crate::R<MacmiiaddrSpec>;
#[doc = "Register `MACMIIADDR` writer"]
pub type W = crate::W<MacmiiaddrSpec>;
#[doc = "Field `MB` reader - MII busy"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - MII busy"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MW` reader - MII write"]
pub type MwR = crate::BitReader;
#[doc = "Field `MW` writer - MII write"]
pub type MwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - Clock range"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - Clock range"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MII` reader - MII register"]
pub type MiiR = crate::FieldReader;
#[doc = "Field `MII` writer - MII register"]
pub type MiiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PHY address"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MwR {
        MwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mii(&self) -> MiiR {
        MiiR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIADDR")
            .field("mb", &self.mb())
            .field("mw", &self.mw())
            .field("cr", &self.cr())
            .field("mii", &self.mii())
            .field("pa", &self.pa())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MbW<MacmiiaddrSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MwW<MacmiiaddrSpec> {
        MwW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<MacmiiaddrSpec> {
        CrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    #[must_use]
    pub fn mii(&mut self) -> MiiW<MacmiiaddrSpec> {
        MiiW::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PaW<MacmiiaddrSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmiiaddrSpec;
impl crate::RegisterSpec for MacmiiaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiiaddr::R`](R) reader structure"]
impl crate::Readable for MacmiiaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`macmiiaddr::W`](W) writer structure"]
impl crate::Writable for MacmiiaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMIIADDR to value 0"]
impl crate::Resettable for MacmiiaddrSpec {
    const RESET_VALUE: u32 = 0;
}
