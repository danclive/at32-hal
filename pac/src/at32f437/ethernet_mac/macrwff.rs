#[doc = "Register `MACRWFF` reader"]
pub type R = crate::R<MacrwffSpec>;
#[doc = "Register `MACRWFF` writer"]
pub type W = crate::W<MacrwffSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacrwffSpec;
impl crate::RegisterSpec for MacrwffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwff::R`](R) reader structure"]
impl crate::Readable for MacrwffSpec {}
#[doc = "`write(|w| ..)` method takes [`macrwff::W`](W) writer structure"]
impl crate::Writable for MacrwffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRWFF to value 0"]
impl crate::Resettable for MacrwffSpec {
    const RESET_VALUE: u32 = 0;
}
