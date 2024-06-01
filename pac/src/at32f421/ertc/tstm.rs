#[doc = "Register `TSTM` reader"]
pub type R = crate::R<TstmSpec>;
#[doc = "Field `SU` reader - Second units"]
pub type SuR = crate::FieldReader;
#[doc = "Field `ST` reader - Second tens"]
pub type StR = crate::FieldReader;
#[doc = "Field `MU` reader - Minute units"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MT` reader - Minute tens"]
pub type MtR = crate::FieldReader;
#[doc = "Field `HU` reader - Hour units"]
pub type HuR = crate::FieldReader;
#[doc = "Field `HT` reader - Hour tens"]
pub type HtR = crate::FieldReader;
#[doc = "Field `AMPM` reader - AMPM"]
pub type AmpmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AMPM"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTM")
            .field("ampm", &self.ampm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
#[doc = "time stamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstmSpec;
impl crate::RegisterSpec for TstmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstm::R`](R) reader structure"]
impl crate::Readable for TstmSpec {}
#[doc = "`reset()` method sets TSTM to value 0"]
impl crate::Resettable for TstmSpec {
    const RESET_VALUE: u32 = 0;
}
