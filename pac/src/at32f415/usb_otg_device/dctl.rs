#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "Field `RWKUPSIG` reader - Remote wakeup signaling"]
pub type RwkupsigR = crate::BitReader;
#[doc = "Field `RWKUPSIG` writer - Remote wakeup signaling"]
pub type RwkupsigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTDISCON` reader - Soft disconnect"]
pub type SftdisconR = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - Soft disconnect"]
pub type SftdisconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNPINNAKSTS` reader - Global IN NAK status"]
pub type GnpinnakstsR = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK status"]
pub type GoutnakstsR = crate::BitReader;
#[doc = "Field `TSTCTL` reader - Test control"]
pub type TstctlR = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - Test control"]
pub type TstctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGNPINNAK` reader - Set global IN NAK"]
pub type SgnpinnakR = crate::BitReader;
#[doc = "Field `SGNPINNAK` writer - Set global IN NAK"]
pub type SgnpinnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPINNAK` reader - Clear global IN NAK"]
pub type CgnpinnakR = crate::BitReader;
#[doc = "Field `CGNPINNAK` writer - Clear global IN NAK"]
pub type CgnpinnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNAK` reader - Set global OUT NAK"]
pub type SgoutnakR = crate::BitReader;
#[doc = "Field `SGOUTNAK` writer - Set global OUT NAK"]
pub type SgoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNAK` reader - Clear global OUT NAK"]
pub type CgoutnakR = crate::BitReader;
#[doc = "Field `CGOUTNAK` writer - Clear global OUT NAK"]
pub type CgoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROPRGDNE` reader - Power-on programming done"]
pub type PwroprgdneR = crate::BitReader;
#[doc = "Field `PWROPRGDNE` writer - Power-on programming done"]
pub type PwroprgdneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwkupsig(&self) -> RwkupsigR {
        RwkupsigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SftdisconR {
        SftdisconR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GnpinnakstsR {
        GnpinnakstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GoutnakstsR {
        GoutnakstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TstctlR {
        TstctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&self) -> SgnpinnakR {
        SgnpinnakR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&self) -> CgnpinnakR {
        CgnpinnakR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&self) -> SgoutnakR {
        SgoutnakR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&self) -> CgoutnakR {
        CgoutnakR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn pwroprgdne(&self) -> PwroprgdneR {
        PwroprgdneR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rwkupsig", &self.rwkupsig())
            .field("sftdiscon", &self.sftdiscon())
            .field("gnpinnaksts", &self.gnpinnaksts())
            .field("goutnaksts", &self.goutnaksts())
            .field("tstctl", &self.tstctl())
            .field("sgnpinnak", &self.sgnpinnak())
            .field("cgnpinnak", &self.cgnpinnak())
            .field("sgoutnak", &self.sgoutnak())
            .field("cgoutnak", &self.cgoutnak())
            .field("pwroprgdne", &self.pwroprgdne())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rwkupsig(&mut self) -> RwkupsigW<DctlSpec> {
        RwkupsigW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sftdiscon(&mut self) -> SftdisconW<DctlSpec> {
        SftdisconW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TstctlW<DctlSpec> {
        TstctlW::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpinnak(&mut self) -> SgnpinnakW<DctlSpec> {
        SgnpinnakW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpinnak(&mut self) -> CgnpinnakW<DctlSpec> {
        CgnpinnakW::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SgoutnakW<DctlSpec> {
        SgoutnakW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CgoutnakW<DctlSpec> {
        CgoutnakW::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    #[must_use]
    pub fn pwroprgdne(&mut self) -> PwroprgdneW<DctlSpec> {
        PwroprgdneW::new(self, 11)
    }
}
#[doc = "OTGFS device control register (OTGFS_DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0;
}
