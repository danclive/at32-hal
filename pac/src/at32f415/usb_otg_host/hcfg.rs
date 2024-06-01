#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HcfgSpec>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HcfgSpec>;
#[doc = "Field `FSLSPCLKSEL` reader - FS/LS PHY clock select"]
pub type FslspclkselR = crate::FieldReader;
#[doc = "Field `FSLSPCLKSEL` writer - FS/LS PHY clock select"]
pub type FslspclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSSUPP` reader - FS- and LS-only support"]
pub type FslssuppR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FslspclkselR {
        FslspclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FslssuppR {
        FslssuppR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFG")
            .field("fslspclksel", &self.fslspclksel())
            .field("fslssupp", &self.fslssupp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclksel(&mut self) -> FslspclkselW<HcfgSpec> {
        FslspclkselW::new(self, 0)
    }
}
#[doc = "OTGFS host configuration register (OTGFS_HCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfgSpec;
impl crate::RegisterSpec for HcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0"]
impl crate::Resettable for HcfgSpec {
    const RESET_VALUE: u32 = 0;
}
