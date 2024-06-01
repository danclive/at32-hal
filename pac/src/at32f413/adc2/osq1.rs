#[doc = "Register `OSQ1` reader"]
pub type R = crate::R<Osq1Spec>;
#[doc = "Register `OSQ1` writer"]
pub type W = crate::W<Osq1Spec>;
#[doc = "Field `OSN13` reader - Number of 13th conversion in ordinary sequence"]
pub type Osn13R = crate::FieldReader;
#[doc = "Field `OSN13` writer - Number of 13th conversion in ordinary sequence"]
pub type Osn13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN14` reader - Number of 14th conversion in ordinary sequence"]
pub type Osn14R = crate::FieldReader;
#[doc = "Field `OSN14` writer - Number of 14th conversion in ordinary sequence"]
pub type Osn14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN15` reader - Number of 15th conversion in ordinary sequence"]
pub type Osn15R = crate::FieldReader;
#[doc = "Field `OSN15` writer - Number of 15th conversion in ordinary sequence"]
pub type Osn15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN16` reader - Number of 16th conversion in ordinary sequence"]
pub type Osn16R = crate::FieldReader;
#[doc = "Field `OSN16` writer - Number of 16th conversion in ordinary sequence"]
pub type Osn16W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OCLEN` reader - Ordinary conversion sequence length"]
pub type OclenR = crate::FieldReader;
#[doc = "Field `OCLEN` writer - Ordinary conversion sequence length"]
pub type OclenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn13(&self) -> Osn13R {
        Osn13R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn14(&self) -> Osn14R {
        Osn14R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn15(&self) -> Osn15R {
        Osn15R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn16(&self) -> Osn16R {
        Osn16R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Ordinary conversion sequence length"]
    #[inline(always)]
    pub fn oclen(&self) -> OclenR {
        OclenR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ1")
            .field("oclen", &self.oclen())
            .field("osn16", &self.osn16())
            .field("osn15", &self.osn15())
            .field("osn14", &self.osn14())
            .field("osn13", &self.osn13())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn13(&mut self) -> Osn13W<Osq1Spec> {
        Osn13W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn14(&mut self) -> Osn14W<Osq1Spec> {
        Osn14W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn15(&mut self) -> Osn15W<Osq1Spec> {
        Osn15W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn16(&mut self) -> Osn16W<Osq1Spec> {
        Osn16W::new(self, 15)
    }
    #[doc = "Bits 20:23 - Ordinary conversion sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn oclen(&mut self) -> OclenW<Osq1Spec> {
        OclenW::new(self, 20)
    }
}
#[doc = "Ordinary sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osq1Spec;
impl crate::RegisterSpec for Osq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq1::R`](R) reader structure"]
impl crate::Readable for Osq1Spec {}
#[doc = "`write(|w| ..)` method takes [`osq1::W`](W) writer structure"]
impl crate::Writable for Osq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSQ1 to value 0"]
impl crate::Resettable for Osq1Spec {
    const RESET_VALUE: u32 = 0;
}
