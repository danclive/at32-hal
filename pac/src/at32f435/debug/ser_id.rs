#[doc = "Register `SER_ID` reader"]
pub type R = crate::R<SerIdSpec>;
#[doc = "Field `REV_ID` reader - version ID"]
pub type RevIdR = crate::FieldReader;
#[doc = "Field `SER_ID` reader - series ID"]
pub type SerIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - version ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - series ID"]
    #[inline(always)]
    pub fn ser_id(&self) -> SerIdR {
        SerIdR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SER_ID")
            .field("rev_id", &self.rev_id())
            .field("ser_id", &self.ser_id())
            .finish()
    }
}
#[doc = "SERIES ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerIdSpec;
impl crate::RegisterSpec for SerIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser_id::R`](R) reader structure"]
impl crate::Readable for SerIdSpec {}
#[doc = "`reset()` method sets SER_ID to value 0"]
impl crate::Resettable for SerIdSpec {
    const RESET_VALUE: u32 = 0;
}
