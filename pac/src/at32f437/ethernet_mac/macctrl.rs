#[doc = "Register `MACCTRL` reader"]
pub type R = crate::R<MacctrlSpec>;
#[doc = "Register `MACCTRL` writer"]
pub type W = crate::W<MacctrlSpec>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - Deferral check"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BlR = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACS` reader - Automatic pad/CRC stripping"]
pub type AcsR = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic pad/CRC stripping"]
pub type AcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - Disable retry"]
pub type DrR = crate::BitReader;
#[doc = "Field `DR` writer - Disable retry"]
pub type DrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - IPv4 checksum offload"]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - IPv4 checksum offload"]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRO` reader - Disable receive own"]
pub type DroR = crate::BitReader;
#[doc = "Field `DRO` writer - Disable receive own"]
pub type DroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Fast EMAC speed"]
pub type FesR = crate::BitReader;
#[doc = "Field `FES` writer - Fast EMAC speed"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS` reader - Disable carrier sense"]
pub type DcsR = crate::BitReader;
#[doc = "Field `DCS` writer - Disable carrier sense"]
pub type DcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IfgR = crate::FieldReader;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JdR = crate::BitReader;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WdR = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable retry"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable receive own"]
    #[inline(always)]
    pub fn dro(&self) -> DroR {
        DroR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable carrier sense"]
    #[inline(always)]
    pub fn dcs(&self) -> DcsR {
        DcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCTRL")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("acs", &self.acs())
            .field("dr", &self.dr())
            .field("ipc", &self.ipc())
            .field("dm", &self.dm())
            .field("lm", &self.lm())
            .field("dro", &self.dro())
            .field("fes", &self.fes())
            .field("dcs", &self.dcs())
            .field("ifg", &self.ifg())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<MacctrlSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<MacctrlSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DcW<MacctrlSpec> {
        DcW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BlW<MacctrlSpec> {
        BlW::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> AcsW<MacctrlSpec> {
        AcsW::new(self, 7)
    }
    #[doc = "Bit 9 - Disable retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<MacctrlSpec> {
        DrW::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IpcW<MacctrlSpec> {
        IpcW::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DmW<MacctrlSpec> {
        DmW::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<MacctrlSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable receive own"]
    #[inline(always)]
    #[must_use]
    pub fn dro(&mut self) -> DroW<MacctrlSpec> {
        DroW::new(self, 13)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FesW<MacctrlSpec> {
        FesW::new(self, 14)
    }
    #[doc = "Bit 16 - Disable carrier sense"]
    #[inline(always)]
    #[must_use]
    pub fn dcs(&mut self) -> DcsW<MacctrlSpec> {
        DcsW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IfgW<MacctrlSpec> {
        IfgW::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JdW<MacctrlSpec> {
        JdW::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WdW<MacctrlSpec> {
        WdW::new(self, 23)
    }
}
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacctrlSpec;
impl crate::RegisterSpec for MacctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macctrl::R`](R) reader structure"]
impl crate::Readable for MacctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`macctrl::W`](W) writer structure"]
impl crate::Writable for MacctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACCTRL to value 0x8000"]
impl crate::Resettable for MacctrlSpec {
    const RESET_VALUE: u32 = 0x8000;
}
