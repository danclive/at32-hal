#[doc = "Register `HCCHAR13` reader"]
pub type R = crate::R<Hcchar13Spec>;
#[doc = "Register `HCCHAR13` writer"]
pub type W = crate::W<Hcchar13Spec>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPTNUM` reader - Endpoint number"]
pub type EptnumR = crate::FieldReader;
#[doc = "Field `EPTNUM` writer - Endpoint number"]
pub type EptnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPTDIR` reader - Endpoint direction"]
pub type EptdirR = crate::BitReader;
#[doc = "Field `EPTDIR` writer - Endpoint direction"]
pub type EptdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPDDEV` reader - Low-speed device"]
pub type LspddevR = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - Low-speed device"]
pub type LspddevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MC` reader - Multicount"]
pub type McR = crate::FieldReader;
#[doc = "Field `MC` writer - Multicount"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVADDR` reader - Device address"]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device address"]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type OddfrmR = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type OddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel disable"]
pub type ChdisR = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel disable"]
pub type ChdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel enable"]
pub type ChenaR = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel enable"]
pub type ChenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn eptnum(&self) -> EptnumR {
        EptnumR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn eptdir(&self) -> EptdirR {
        EptdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lspddev(&self) -> LspddevR {
        LspddevR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> OddfrmR {
        OddfrmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&self) -> ChdisR {
        ChdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&self) -> ChenaR {
        ChenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR13")
            .field("mps", &self.mps())
            .field("eptnum", &self.eptnum())
            .field("eptdir", &self.eptdir())
            .field("lspddev", &self.lspddev())
            .field("eptype", &self.eptype())
            .field("mc", &self.mc())
            .field("devaddr", &self.devaddr())
            .field("oddfrm", &self.oddfrm())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<Hcchar13Spec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn eptnum(&mut self) -> EptnumW<Hcchar13Spec> {
        EptnumW::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn eptdir(&mut self) -> EptdirW<Hcchar13Spec> {
        EptdirW::new(self, 15)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lspddev(&mut self) -> LspddevW<Hcchar13Spec> {
        LspddevW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<Hcchar13Spec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> McW<Hcchar13Spec> {
        McW::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DevaddrW<Hcchar13Spec> {
        DevaddrW::new(self, 22)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> OddfrmW<Hcchar13Spec> {
        OddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> ChdisW<Hcchar13Spec> {
        ChdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> ChenaW<Hcchar13Spec> {
        ChenaW::new(self, 31)
    }
}
#[doc = "OTGFS host channel-13 characteristics register (OTGFS_HCCHAR13)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcchar13Spec;
impl crate::RegisterSpec for Hcchar13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar13::R`](R) reader structure"]
impl crate::Readable for Hcchar13Spec {}
#[doc = "`write(|w| ..)` method takes [`hcchar13::W`](W) writer structure"]
impl crate::Writable for Hcchar13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCHAR13 to value 0"]
impl crate::Resettable for Hcchar13Spec {
    const RESET_VALUE: u32 = 0;
}
