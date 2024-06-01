#[doc = "Register `WIN` reader"]
pub type R = crate::R<WinSpec>;
#[doc = "Register `WIN` writer"]
pub type W = crate::W<WinSpec>;
#[doc = "Field `WIN` reader - Window value"]
pub type WinR = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Window value"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN").field("win", &self.win()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<WinSpec> {
        WinW::new(self, 0)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinSpec;
impl crate::RegisterSpec for WinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win::R`](R) reader structure"]
impl crate::Readable for WinSpec {}
#[doc = "`write(|w| ..)` method takes [`win::W`](W) writer structure"]
impl crate::Writable for WinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN to value 0x0fff"]
impl crate::Resettable for WinSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
