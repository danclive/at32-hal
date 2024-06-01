#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Field `DEVSPD` reader - Device speed"]
pub type DevspdR = crate::FieldReader;
#[doc = "Field `DEVSPD` writer - Device speed"]
pub type DevspdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZSTSOUTHSHK` reader - Non-zero-length status OUT handshake"]
pub type NzstsouthshkR = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - Non-zero-length status OUT handshake"]
pub type NzstsouthshkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Device address"]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device address"]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PERFRINT` reader - Periodic frame interval"]
pub type PerfrintR = crate::FieldReader;
#[doc = "Field `PERFRINT` writer - Periodic frame interval"]
pub type PerfrintW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DevspdR {
        DevspdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NzstsouthshkR {
        NzstsouthshkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PerfrintR {
        PerfrintR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field("devspd", &self.devspd())
            .field("nzstsouthshk", &self.nzstsouthshk())
            .field("devaddr", &self.devaddr())
            .field("perfrint", &self.perfrint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    #[must_use]
    pub fn devspd(&mut self) -> DevspdW<DcfgSpec> {
        DevspdW::new(self, 0)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzstsouthshk(&mut self) -> NzstsouthshkW<DcfgSpec> {
        NzstsouthshkW::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DevaddrW<DcfgSpec> {
        DevaddrW::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    #[must_use]
    pub fn perfrint(&mut self) -> PerfrintW<DcfgSpec> {
        PerfrintW::new(self, 11)
    }
}
#[doc = "OTGFS device configuration register (OTGFS_DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0220_0000"]
impl crate::Resettable for DcfgSpec {
    const RESET_VALUE: u32 = 0x0220_0000;
}
