#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `WIN` reader - Window value"]
pub type WinR = crate::FieldReader;
#[doc = "Field `WIN` writer - Window value"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIV` reader - Clock division value"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Clock division value"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RLDIEN` reader - Reload counter interrupt"]
pub type RldienR = crate::BitReader;
#[doc = "Field `RLDIEN` writer - Reload counter interrupt"]
pub type RldienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Clock division value"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Reload counter interrupt"]
    #[inline(always)]
    pub fn rldien(&self) -> RldienR {
        RldienR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("win", &self.win())
            .field("div", &self.div())
            .field("rldien", &self.rldien())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<CfgSpec> {
        WinW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Clock division value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<CfgSpec> {
        DivW::new(self, 7)
    }
    #[doc = "Bit 9 - Reload counter interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rldien(&mut self) -> RldienW<CfgSpec> {
        RldienW::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x7f;
}
