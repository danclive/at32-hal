#[doc = "Register `ALASBS` reader"]
pub type R = crate::R<AlasbsSpec>;
#[doc = "Register `ALASBS` writer"]
pub type W = crate::W<AlasbsSpec>;
#[doc = "Field `SBS` reader - Sub-seconds value"]
pub type SbsR = crate::FieldReader<u16>;
#[doc = "Field `SBS` writer - Sub-seconds value"]
pub type SbsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SBSMSK` reader - Sub-second mask"]
pub type SbsmskR = crate::FieldReader;
#[doc = "Field `SBSMSK` writer - Sub-second mask"]
pub type SbsmskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    pub fn sbsmsk(&self) -> SbsmskR {
        SbsmskR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALASBS")
            .field("sbsmsk", &self.sbsmsk())
            .field("sbs", &self.sbs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn sbs(&mut self) -> SbsW<AlasbsSpec> {
        SbsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    #[must_use]
    pub fn sbsmsk(&mut self) -> SbsmskW<AlasbsSpec> {
        SbsmskW::new(self, 24)
    }
}
#[doc = "alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alasbs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alasbs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlasbsSpec;
impl crate::RegisterSpec for AlasbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alasbs::R`](R) reader structure"]
impl crate::Readable for AlasbsSpec {}
#[doc = "`write(|w| ..)` method takes [`alasbs::W`](W) writer structure"]
impl crate::Writable for AlasbsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALASBS to value 0"]
impl crate::Resettable for AlasbsSpec {
    const RESET_VALUE: u32 = 0;
}
