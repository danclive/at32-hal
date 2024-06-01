#[doc = "Register `SLIB_MISC_STS` reader"]
pub type R = crate::R<SlibMiscStsSpec>;
#[doc = "Register `SLIB_MISC_STS` writer"]
pub type W = crate::W<SlibMiscStsSpec>;
#[doc = "Field `SLIB_PWD_ERR` reader - sLib password error"]
pub type SlibPwdErrR = crate::BitReader;
#[doc = "Field `SLIB_PWD_OK` reader - sLib password ok"]
pub type SlibPwdOkR = crate::BitReader;
#[doc = "Field `SLIB_ULKF` reader - sLib unlock flag"]
pub type SlibUlkfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sLib password error"]
    #[inline(always)]
    pub fn slib_pwd_err(&self) -> SlibPwdErrR {
        SlibPwdErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sLib password ok"]
    #[inline(always)]
    pub fn slib_pwd_ok(&self) -> SlibPwdOkR {
        SlibPwdOkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sLib unlock flag"]
    #[inline(always)]
    pub fn slib_ulkf(&self) -> SlibUlkfR {
        SlibUlkfR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_MISC_STS")
            .field("slib_pwd_err", &self.slib_pwd_err())
            .field("slib_pwd_ok", &self.slib_pwd_ok())
            .field("slib_ulkf", &self.slib_ulkf())
            .finish()
    }
}
impl W {}
#[doc = "sLib misc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_misc_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_misc_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibMiscStsSpec;
impl crate::RegisterSpec for SlibMiscStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_misc_sts::R`](R) reader structure"]
impl crate::Readable for SlibMiscStsSpec {}
#[doc = "`write(|w| ..)` method takes [`slib_misc_sts::W`](W) writer structure"]
impl crate::Writable for SlibMiscStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_MISC_STS to value 0x0100_0000"]
impl crate::Resettable for SlibMiscStsSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
