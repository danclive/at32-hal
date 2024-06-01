#[doc = "Register `MMCTFMSCC` reader"]
pub type R = crate::R<MmctfmsccSpec>;
#[doc = "Field `TGFMSCC` reader - Transmitted good frame more single collision counter"]
pub type TgfmsccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frame more single collision counter"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TgfmsccR {
        TgfmsccR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTFMSCC")
            .field("tgfmscc", &self.tgfmscc())
            .finish()
    }
}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfmscc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctfmsccSpec;
impl crate::RegisterSpec for MmctfmsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctfmscc::R`](R) reader structure"]
impl crate::Readable for MmctfmsccSpec {}
#[doc = "`reset()` method sets MMCTFMSCC to value 0"]
impl crate::Resettable for MmctfmsccSpec {
    const RESET_VALUE: u32 = 0;
}
