#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GotgctlSpec>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GotgctlSpec>;
#[doc = "Field `CONIDSTS` reader - Connector ID status"]
pub type ConidstsR = crate::BitReader;
#[doc = "Field `CURMOD` reader - Current Mode of Operation"]
pub type CurmodR = crate::BitReader;
impl R {
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn conidsts(&self) -> ConidstsR {
        ConidstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CurmodR {
        CurmodR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGCTL")
            .field("conidsts", &self.conidsts())
            .field("curmod", &self.curmod())
            .finish()
    }
}
impl W {}
#[doc = "OTGFS control and status register (OTGFS_GOTGCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgctlSpec;
impl crate::RegisterSpec for GotgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GotgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GotgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0800"]
impl crate::Resettable for GotgctlSpec {
    const RESET_VALUE: u32 = 0x0800;
}
