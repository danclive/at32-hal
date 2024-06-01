#[doc = "Register `MUXSEL` reader"]
pub type R = crate::R<MuxselSpec>;
#[doc = "Register `MUXSEL` writer"]
pub type W = crate::W<MuxselSpec>;
#[doc = "Field `TBL_SEL` reader - Multiplexer Table Select"]
pub type TblSelR = crate::BitReader;
#[doc = "Field `TBL_SEL` writer - Multiplexer Table Select"]
pub type TblSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&self) -> TblSelR {
        TblSelR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSEL")
            .field("tbl_sel", &self.tbl_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    #[must_use]
    pub fn tbl_sel(&mut self) -> TblSelW<MuxselSpec> {
        TblSelW::new(self, 0)
    }
}
#[doc = "EDMA MUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxselSpec;
impl crate::RegisterSpec for MuxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsel::R`](R) reader structure"]
impl crate::Readable for MuxselSpec {}
#[doc = "`write(|w| ..)` method takes [`muxsel::W`](W) writer structure"]
impl crate::Writable for MuxselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXSEL to value 0"]
impl crate::Resettable for MuxselSpec {
    const RESET_VALUE: u32 = 0;
}
