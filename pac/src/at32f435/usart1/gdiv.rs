#[doc = "Register `GDIV` reader"]
pub type R = crate::R<GdivSpec>;
#[doc = "Register `GDIV` writer"]
pub type W = crate::W<GdivSpec>;
#[doc = "Field `ISDIV` reader - IrDA/smartcard division value"]
pub type IsdivR = crate::FieldReader;
#[doc = "Field `ISDIV` writer - IrDA/smartcard division value"]
pub type IsdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCGT` reader - Smart card guard time value"]
pub type ScgtR = crate::FieldReader;
#[doc = "Field `SCGT` writer - Smart card guard time value"]
pub type ScgtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    pub fn isdiv(&self) -> IsdivR {
        IsdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    pub fn scgt(&self) -> ScgtR {
        ScgtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDIV")
            .field("scgt", &self.scgt())
            .field("isdiv", &self.isdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    #[must_use]
    pub fn isdiv(&mut self) -> IsdivW<GdivSpec> {
        IsdivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    #[must_use]
    pub fn scgt(&mut self) -> ScgtW<GdivSpec> {
        ScgtW::new(self, 8)
    }
}
#[doc = "Guard time and division register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdivSpec;
impl crate::RegisterSpec for GdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdiv::R`](R) reader structure"]
impl crate::Readable for GdivSpec {}
#[doc = "`write(|w| ..)` method takes [`gdiv::W`](W) writer structure"]
impl crate::Writable for GdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDIV to value 0"]
impl crate::Resettable for GdivSpec {
    const RESET_VALUE: u32 = 0;
}
