#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Field `NZW_BST` reader - Flash non-zero wait area boost"]
pub type NzwBstR = crate::BitReader;
#[doc = "Field `NZW_BST` writer - Flash non-zero wait area boost"]
pub type NzwBstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NZW_BST_STS` reader - Flash non-zero wait area boost status"]
pub type NzwBstStsR = crate::BitReader;
impl R {
    #[doc = "Bit 12 - Flash non-zero wait area boost"]
    #[inline(always)]
    pub fn nzw_bst(&self) -> NzwBstR {
        NzwBstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash non-zero wait area boost status"]
    #[inline(always)]
    pub fn nzw_bst_sts(&self) -> NzwBstStsR {
        NzwBstStsR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("nzw_bst_sts", &self.nzw_bst_sts())
            .field("nzw_bst", &self.nzw_bst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Flash non-zero wait area boost"]
    #[inline(always)]
    #[must_use]
    pub fn nzw_bst(&mut self) -> NzwBstW<PsrSpec> {
        NzwBstW::new(self, 12)
    }
}
#[doc = "Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0x0330"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x0330;
}
