#[doc = "Register `DIVR` reader"]
pub type R = crate::R<DivrSpec>;
#[doc = "Register `DIVR` writer"]
pub type W = crate::W<DivrSpec>;
#[doc = "Field `FDIV` reader - Flash divider"]
pub type FdivR = crate::FieldReader;
#[doc = "Field `FDIV` writer - Flash divider"]
pub type FdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDIV_STS` reader - Flash divider status"]
pub type FdivStsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Flash divider"]
    #[inline(always)]
    pub fn fdiv(&self) -> FdivR {
        FdivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash divider status"]
    #[inline(always)]
    pub fn fdiv_sts(&self) -> FdivStsR {
        FdivStsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVR")
            .field("fdiv", &self.fdiv())
            .field("fdiv_sts", &self.fdiv_sts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash divider"]
    #[inline(always)]
    #[must_use]
    pub fn fdiv(&mut self) -> FdivW<DivrSpec> {
        FdivW::new(self, 0)
    }
}
#[doc = "Flash divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivrSpec;
impl crate::RegisterSpec for DivrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divr::R`](R) reader structure"]
impl crate::Readable for DivrSpec {}
#[doc = "`write(|w| ..)` method takes [`divr::W`](W) writer structure"]
impl crate::Writable for DivrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVR to value 0x22"]
impl crate::Resettable for DivrSpec {
    const RESET_VALUE: u32 = 0x22;
}
