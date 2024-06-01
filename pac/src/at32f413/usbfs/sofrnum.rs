#[doc = "Register `SOFRNUM` reader"]
pub type R = crate::R<SofrnumSpec>;
#[doc = "Register `SOFRNUM` writer"]
pub type W = crate::W<SofrnumSpec>;
#[doc = "Field `SOFNUM` reader - Start of frame number"]
pub type SofnumR = crate::FieldReader<u16>;
#[doc = "Field `SOFNUM` writer - Start of frame number"]
pub type SofnumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `LSOFNUM` reader - Lost start of frame number"]
pub type LsofnumR = crate::FieldReader;
#[doc = "Field `LSOFNUM` writer - Lost start of frame number"]
pub type LsofnumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLCK` reader - Connect locked"]
pub type ClckR = crate::BitReader;
#[doc = "Field `CLCK` writer - Connect locked"]
pub type ClckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMSTS` reader - DM status"]
pub type DmstsR = crate::BitReader;
#[doc = "Field `DMSTS` writer - DM status"]
pub type DmstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPSTS` reader - DP status"]
pub type DpstsR = crate::BitReader;
#[doc = "Field `DPSTS` writer - DP status"]
pub type DpstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Start of frame number"]
    #[inline(always)]
    pub fn sofnum(&self) -> SofnumR {
        SofnumR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost start of frame number"]
    #[inline(always)]
    pub fn lsofnum(&self) -> LsofnumR {
        LsofnumR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Connect locked"]
    #[inline(always)]
    pub fn clck(&self) -> ClckR {
        ClckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DM status"]
    #[inline(always)]
    pub fn dmsts(&self) -> DmstsR {
        DmstsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DP status"]
    #[inline(always)]
    pub fn dpsts(&self) -> DpstsR {
        DpstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOFRNUM")
            .field("sofnum", &self.sofnum())
            .field("lsofnum", &self.lsofnum())
            .field("clck", &self.clck())
            .field("dmsts", &self.dmsts())
            .field("dpsts", &self.dpsts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Start of frame number"]
    #[inline(always)]
    #[must_use]
    pub fn sofnum(&mut self) -> SofnumW<SofrnumSpec> {
        SofnumW::new(self, 0)
    }
    #[doc = "Bits 11:12 - Lost start of frame number"]
    #[inline(always)]
    #[must_use]
    pub fn lsofnum(&mut self) -> LsofnumW<SofrnumSpec> {
        LsofnumW::new(self, 11)
    }
    #[doc = "Bit 13 - Connect locked"]
    #[inline(always)]
    #[must_use]
    pub fn clck(&mut self) -> ClckW<SofrnumSpec> {
        ClckW::new(self, 13)
    }
    #[doc = "Bit 14 - DM status"]
    #[inline(always)]
    #[must_use]
    pub fn dmsts(&mut self) -> DmstsW<SofrnumSpec> {
        DmstsW::new(self, 14)
    }
    #[doc = "Bit 15 - DP status"]
    #[inline(always)]
    #[must_use]
    pub fn dpsts(&mut self) -> DpstsW<SofrnumSpec> {
        DpstsW::new(self, 15)
    }
}
#[doc = "frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sofrnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sofrnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SofrnumSpec;
impl crate::RegisterSpec for SofrnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sofrnum::R`](R) reader structure"]
impl crate::Readable for SofrnumSpec {}
#[doc = "`write(|w| ..)` method takes [`sofrnum::W`](W) writer structure"]
impl crate::Writable for SofrnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFRNUM to value 0"]
impl crate::Resettable for SofrnumSpec {
    const RESET_VALUE: u32 = 0;
}
