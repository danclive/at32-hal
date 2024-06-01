#[doc = "Register `ALBSBS` reader"]
pub type R = crate::R<AlbsbsSpec>;
#[doc = "Register `ALBSBS` writer"]
pub type W = crate::W<AlbsbsSpec>;
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
        f.debug_struct("ALBSBS")
            .field("sbsmsk", &self.sbsmsk())
            .field("sbs", &self.sbs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn sbs(&mut self) -> SbsW<AlbsbsSpec> {
        SbsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    #[must_use]
    pub fn sbsmsk(&mut self) -> SbsmskW<AlbsbsSpec> {
        SbsmskW::new(self, 24)
    }
}
#[doc = "alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`albsbs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`albsbs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlbsbsSpec;
impl crate::RegisterSpec for AlbsbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`albsbs::R`](R) reader structure"]
impl crate::Readable for AlbsbsSpec {}
#[doc = "`write(|w| ..)` method takes [`albsbs::W`](W) writer structure"]
impl crate::Writable for AlbsbsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALBSBS to value 0"]
impl crate::Resettable for AlbsbsSpec {
    const RESET_VALUE: u32 = 0;
}
