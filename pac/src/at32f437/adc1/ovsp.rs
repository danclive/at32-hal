#[doc = "Register `OVSP` reader"]
pub type R = crate::R<OvspSpec>;
#[doc = "Register `OVSP` writer"]
pub type W = crate::W<OvspSpec>;
#[doc = "Field `OOSEN` reader - Ordinary oversampling enable"]
pub type OosenR = crate::BitReader;
#[doc = "Field `OOSEN` writer - Ordinary oversampling enable"]
pub type OosenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSEN` reader - Preempted oversampling enable"]
pub type PosenR = crate::BitReader;
#[doc = "Field `POSEN` writer - Preempted oversampling enable"]
pub type PosenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSRSEL` reader - Oversampling ratio select"]
pub type OsrselR = crate::FieldReader;
#[doc = "Field `OSRSEL` writer - Oversampling ratio select"]
pub type OsrselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSSSEL` reader - Oversampling shift select"]
pub type OssselR = crate::FieldReader;
#[doc = "Field `OSSSEL` writer - Oversampling shift select"]
pub type OssselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OOSTREN` reader - Ordinary oversampling trigger mode enable"]
pub type OostrenR = crate::BitReader;
#[doc = "Field `OOSTREN` writer - Ordinary oversampling trigger mode enable"]
pub type OostrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOSRSEL` reader - Ordinary oversampling recovery mode select"]
pub type OosrselR = crate::BitReader;
#[doc = "Field `OOSRSEL` writer - Ordinary oversampling recovery mode select"]
pub type OosrselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ordinary oversampling enable"]
    #[inline(always)]
    pub fn oosen(&self) -> OosenR {
        OosenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Preempted oversampling enable"]
    #[inline(always)]
    pub fn posen(&self) -> PosenR {
        PosenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio select"]
    #[inline(always)]
    pub fn osrsel(&self) -> OsrselR {
        OsrselR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift select"]
    #[inline(always)]
    pub fn osssel(&self) -> OssselR {
        OssselR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Ordinary oversampling trigger mode enable"]
    #[inline(always)]
    pub fn oostren(&self) -> OostrenR {
        OostrenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ordinary oversampling recovery mode select"]
    #[inline(always)]
    pub fn oosrsel(&self) -> OosrselR {
        OosrselR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OVSP")
            .field("oosrsel", &self.oosrsel())
            .field("oostren", &self.oostren())
            .field("osssel", &self.osssel())
            .field("osrsel", &self.osrsel())
            .field("posen", &self.posen())
            .field("oosen", &self.oosen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Ordinary oversampling enable"]
    #[inline(always)]
    #[must_use]
    pub fn oosen(&mut self) -> OosenW<OvspSpec> {
        OosenW::new(self, 0)
    }
    #[doc = "Bit 1 - Preempted oversampling enable"]
    #[inline(always)]
    #[must_use]
    pub fn posen(&mut self) -> PosenW<OvspSpec> {
        PosenW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Oversampling ratio select"]
    #[inline(always)]
    #[must_use]
    pub fn osrsel(&mut self) -> OsrselW<OvspSpec> {
        OsrselW::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift select"]
    #[inline(always)]
    #[must_use]
    pub fn osssel(&mut self) -> OssselW<OvspSpec> {
        OssselW::new(self, 5)
    }
    #[doc = "Bit 9 - Ordinary oversampling trigger mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn oostren(&mut self) -> OostrenW<OvspSpec> {
        OostrenW::new(self, 9)
    }
    #[doc = "Bit 10 - Ordinary oversampling recovery mode select"]
    #[inline(always)]
    #[must_use]
    pub fn oosrsel(&mut self) -> OosrselW<OvspSpec> {
        OosrselW::new(self, 10)
    }
}
#[doc = "oversampling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovsp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvspSpec;
impl crate::RegisterSpec for OvspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsp::R`](R) reader structure"]
impl crate::Readable for OvspSpec {}
#[doc = "`write(|w| ..)` method takes [`ovsp::W`](W) writer structure"]
impl crate::Writable for OvspSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVSP to value 0"]
impl crate::Resettable for OvspSpec {
    const RESET_VALUE: u32 = 0;
}
