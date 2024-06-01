#[doc = "Register `RSP3` reader"]
pub type R = crate::R<Rsp3Spec>;
#[doc = "Field `CARDSTS2` reader - CARDSTATUS3"]
pub type Cardsts2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS3"]
    #[inline(always)]
    pub fn cardsts2(&self) -> Cardsts2R {
        Cardsts2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP3")
            .field("cardsts2", &self.cardsts2())
            .finish()
    }
}
#[doc = "Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp3Spec;
impl crate::RegisterSpec for Rsp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp3::R`](R) reader structure"]
impl crate::Readable for Rsp3Spec {}
#[doc = "`reset()` method sets RSP3 to value 0"]
impl crate::Resettable for Rsp3Spec {
    const RESET_VALUE: u32 = 0;
}
