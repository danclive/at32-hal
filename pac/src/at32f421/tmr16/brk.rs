#[doc = "Register `BRK` reader"]
pub type R = crate::R<BrkSpec>;
#[doc = "Register `BRK` writer"]
pub type W = crate::W<BrkSpec>;
#[doc = "Field `DTC` reader - Dead-time configuration"]
pub type DtcR = crate::FieldReader;
#[doc = "Field `DTC` writer - Dead-time configuration"]
pub type DtcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WPC` reader - Write protected configuration"]
pub type WpcR = crate::FieldReader;
#[doc = "Field `WPC` writer - Write protected configuration"]
pub type WpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FCSODIS` reader - Frozen channel status when holistic output disable"]
pub type FcsodisR = crate::BitReader;
#[doc = "Field `FCSODIS` writer - Frozen channel status when holistic output disable"]
pub type FcsodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSOEN` reader - Frozen channel status when holistic output enable"]
pub type FcsoenR = crate::BitReader;
#[doc = "Field `FCSOEN` writer - Frozen channel status when holistic output enable"]
pub type FcsoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKEN` reader - Brake enable"]
pub type BrkenR = crate::BitReader;
#[doc = "Field `BRKEN` writer - Brake enable"]
pub type BrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKV` reader - Brake input validity"]
pub type BrkvR = crate::BitReader;
#[doc = "Field `BRKV` writer - Brake input validity"]
pub type BrkvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOEN` reader - Automatic output enable"]
pub type AoenR = crate::BitReader;
#[doc = "Field `AOEN` writer - Automatic output enable"]
pub type AoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN` reader - Output enable"]
pub type OenR = crate::BitReader;
#[doc = "Field `OEN` writer - Output enable"]
pub type OenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    pub fn dtc(&self) -> DtcR {
        DtcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    pub fn wpc(&self) -> WpcR {
        WpcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    pub fn fcsodis(&self) -> FcsodisR {
        FcsodisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    pub fn fcsoen(&self) -> FcsoenR {
        FcsoenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    pub fn brken(&self) -> BrkenR {
        BrkenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    pub fn brkv(&self) -> BrkvR {
        BrkvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&self) -> AoenR {
        AoenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    pub fn oen(&self) -> OenR {
        OenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRK")
            .field("oen", &self.oen())
            .field("aoen", &self.aoen())
            .field("brkv", &self.brkv())
            .field("brken", &self.brken())
            .field("fcsoen", &self.fcsoen())
            .field("fcsodis", &self.fcsodis())
            .field("wpc", &self.wpc())
            .field("dtc", &self.dtc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dtc(&mut self) -> DtcW<BrkSpec> {
        DtcW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wpc(&mut self) -> WpcW<BrkSpec> {
        WpcW::new(self, 8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    #[must_use]
    pub fn fcsodis(&mut self) -> FcsodisW<BrkSpec> {
        FcsodisW::new(self, 10)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcsoen(&mut self) -> FcsoenW<BrkSpec> {
        FcsoenW::new(self, 11)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    #[must_use]
    pub fn brken(&mut self) -> BrkenW<BrkSpec> {
        BrkenW::new(self, 12)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    #[must_use]
    pub fn brkv(&mut self) -> BrkvW<BrkSpec> {
        BrkvW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoen(&mut self) -> AoenW<BrkSpec> {
        AoenW::new(self, 14)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OenW<BrkSpec> {
        OenW::new(self, 15)
    }
}
#[doc = "Brake register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrkSpec;
impl crate::RegisterSpec for BrkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brk::R`](R) reader structure"]
impl crate::Readable for BrkSpec {}
#[doc = "`write(|w| ..)` method takes [`brk::W`](W) writer structure"]
impl crate::Writable for BrkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRK to value 0"]
impl crate::Resettable for BrkSpec {
    const RESET_VALUE: u32 = 0;
}
