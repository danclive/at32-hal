#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
#[doc = "Field `OTGFS1RST` reader - OTGFS1 reset"]
pub type Otgfs1rstR = crate::BitReader;
#[doc = "Field `OTGFS1RST` writer - OTGFS1 reset"]
pub type Otgfs1rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1rst(&self) -> Otgfs1rstR {
        Otgfs1rstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST")
            .field("otgfs1rst", &self.otgfs1rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - OTGFS1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1rst(&mut self) -> Otgfs1rstW<AhbrstSpec> {
        Otgfs1rstW::new(self, 12)
    }
}
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AhbrstSpec {
    const RESET_VALUE: u32 = 0;
}
