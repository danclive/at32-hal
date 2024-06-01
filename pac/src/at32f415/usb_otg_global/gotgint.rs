#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GotgintSpec>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GotgintSpec>;
#[doc = "Field `SESENDDET` reader - VBUS is deasserted"]
pub type SesenddetR = crate::BitReader;
#[doc = "Field `SESENDDET` writer - VBUS is deasserted"]
pub type SesenddetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - VBUS is deasserted"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SesenddetR {
        SesenddetR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sesenddet", &self.sesenddet())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - VBUS is deasserted"]
    #[inline(always)]
    #[must_use]
    pub fn sesenddet(&mut self) -> SesenddetW<GotgintSpec> {
        SesenddetW::new(self, 2)
    }
}
#[doc = "OTGFS interrupt register (OTGFS_GOTGINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgintSpec;
impl crate::RegisterSpec for GotgintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GotgintSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GotgintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GotgintSpec {
    const RESET_VALUE: u32 = 0;
}
