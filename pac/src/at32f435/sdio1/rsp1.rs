#[doc = "Register `RSP1` reader"]
pub type R = crate::R<Rsp1Spec>;
#[doc = "Field `CARDSTS1` reader - CARDSTATUS1"]
pub type Cardsts1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardsts1(&self) -> Cardsts1R {
        Cardsts1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP1")
            .field("cardsts1", &self.cardsts1())
            .finish()
    }
}
#[doc = "Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp1Spec;
impl crate::RegisterSpec for Rsp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp1::R`](R) reader structure"]
impl crate::Readable for Rsp1Spec {}
#[doc = "`reset()` method sets RSP1 to value 0"]
impl crate::Resettable for Rsp1Spec {
    const RESET_VALUE: u32 = 0;
}
