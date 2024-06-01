#[doc = "Register `TSDT` reader"]
pub type R = crate::R<TsdtSpec>;
#[doc = "Field `DU` reader - Date units"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DT` reader - Date tens"]
pub type DtR = crate::FieldReader;
#[doc = "Field `MU` reader - Month units"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MT` reader - Month tens"]
pub type MtR = crate::BitReader;
#[doc = "Field `WK` reader - Week"]
pub type WkR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    pub fn wk(&self) -> WkR {
        WkR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSDT")
            .field("wk", &self.wk())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .finish()
    }
}
#[doc = "timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsdtSpec;
impl crate::RegisterSpec for TsdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsdt::R`](R) reader structure"]
impl crate::Readable for TsdtSpec {}
#[doc = "`reset()` method sets TSDT to value 0"]
impl crate::Resettable for TsdtSpec {
    const RESET_VALUE: u32 = 0;
}
