#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Field `ACC_HSICAL` reader - Internal high-speed auto clock calibration"]
pub type AccHsicalR = crate::FieldReader;
#[doc = "Field `ACC_HSITRIM` reader - Internal high-speed auto clock trimming"]
pub type AccHsitrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Internal high-speed auto clock calibration"]
    #[inline(always)]
    pub fn acc_hsical(&self) -> AccHsicalR {
        AccHsicalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Internal high-speed auto clock trimming"]
    #[inline(always)]
    pub fn acc_hsitrim(&self) -> AccHsitrimR {
        AccHsitrimR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("acc_hsical", &self.acc_hsical())
            .field("acc_hsitrim", &self.acc_hsitrim())
            .finish()
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`reset()` method sets CTRL2 to value 0x2080"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x2080;
}
