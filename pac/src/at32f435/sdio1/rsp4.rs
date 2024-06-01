#[doc = "Register `RSP4` reader"]
pub type R = crate::R<Rsp4Spec>;
#[doc = "Field `CARDSTS2` reader - CARDSTATUS4"]
pub type Cardsts2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS4"]
    #[inline(always)]
    pub fn cardsts2(&self) -> Cardsts2R {
        Cardsts2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP4")
            .field("cardsts2", &self.cardsts2())
            .finish()
    }
}
#[doc = "Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp4Spec;
impl crate::RegisterSpec for Rsp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp4::R`](R) reader structure"]
impl crate::Readable for Rsp4Spec {}
#[doc = "`reset()` method sets RSP4 to value 0"]
impl crate::Resettable for Rsp4Spec {
    const RESET_VALUE: u32 = 0;
}
