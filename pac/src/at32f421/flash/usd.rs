#[doc = "Register `USD` reader"]
pub type R = crate::R<UsdSpec>;
#[doc = "Field `USDERR` reader - User system data error"]
pub type UsderrR = crate::BitReader;
#[doc = "Field `FAP` reader - FLASH access protection"]
pub type FapR = crate::BitReader;
#[doc = "Field `nWDT_ATO_EN` reader - WDT auto enable"]
pub type NWdtAtoEnR = crate::BitReader;
#[doc = "Field `nDEPSLP_RST` reader - Deepsleep reset"]
pub type NDepslpRstR = crate::BitReader;
#[doc = "Field `nSTDBY_RST` reader - Standby reset"]
pub type NStdbyRstR = crate::BitReader;
#[doc = "Field `nBOOT1` reader - boot1"]
pub type NBoot1R = crate::BitReader;
#[doc = "Field `USER_D0` reader - User data 0"]
pub type UserD0R = crate::FieldReader;
#[doc = "Field `USER_D1` reader - User data 1"]
pub type UserD1R = crate::FieldReader;
#[doc = "Field `FAP_HL` reader - FAP high level"]
pub type FapHlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - User system data error"]
    #[inline(always)]
    pub fn usderr(&self) -> UsderrR {
        UsderrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH access protection"]
    #[inline(always)]
    pub fn fap(&self) -> FapR {
        FapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT auto enable"]
    #[inline(always)]
    pub fn n_wdt_ato_en(&self) -> NWdtAtoEnR {
        NWdtAtoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Deepsleep reset"]
    #[inline(always)]
    pub fn n_depslp_rst(&self) -> NDepslpRstR {
        NDepslpRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Standby reset"]
    #[inline(always)]
    pub fn n_stdby_rst(&self) -> NStdbyRstR {
        NStdbyRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - boot1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBoot1R {
        NBoot1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:17 - User data 0"]
    #[inline(always)]
    pub fn user_d0(&self) -> UserD0R {
        UserD0R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - User data 1"]
    #[inline(always)]
    pub fn user_d1(&self) -> UserD1R {
        UserD1R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 26 - FAP high level"]
    #[inline(always)]
    pub fn fap_hl(&self) -> FapHlR {
        FapHlR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USD")
            .field("usderr", &self.usderr())
            .field("fap", &self.fap())
            .field("n_wdt_ato_en", &self.n_wdt_ato_en())
            .field("n_depslp_rst", &self.n_depslp_rst())
            .field("n_stdby_rst", &self.n_stdby_rst())
            .field("n_boot1", &self.n_boot1())
            .field("user_d0", &self.user_d0())
            .field("user_d1", &self.user_d1())
            .field("fap_hl", &self.fap_hl())
            .finish()
    }
}
#[doc = "User system data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsdSpec;
impl crate::RegisterSpec for UsdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usd::R`](R) reader structure"]
impl crate::Readable for UsdSpec {}
#[doc = "`reset()` method sets USD to value 0x03ff_fffc"]
impl crate::Resettable for UsdSpec {
    const RESET_VALUE: u32 = 0x03ff_fffc;
}
