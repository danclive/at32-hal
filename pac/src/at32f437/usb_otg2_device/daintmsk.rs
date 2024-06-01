#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DaintmskSpec>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DaintmskSpec>;
#[doc = "Field `INEPTMSK` reader - IN EP interrupt mask bits"]
pub type IneptmskR = crate::FieldReader<u16>;
#[doc = "Field `INEPTMSK` writer - IN EP interrupt mask bits"]
pub type IneptmskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUTEPTMSK` reader - OUT endpoint interrupt bits"]
pub type OuteptmskR = crate::FieldReader<u16>;
#[doc = "Field `OUTEPTMSK` writer - OUT endpoint interrupt bits"]
pub type OuteptmskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn ineptmsk(&self) -> IneptmskR {
        IneptmskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn outeptmsk(&self) -> OuteptmskR {
        OuteptmskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINTMSK")
            .field("ineptmsk", &self.ineptmsk())
            .field("outeptmsk", &self.outeptmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn ineptmsk(&mut self) -> IneptmskW<DaintmskSpec> {
        IneptmskW::new(self, 0)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn outeptmsk(&mut self) -> OuteptmskW<DaintmskSpec> {
        OuteptmskW::new(self, 16)
    }
}
#[doc = "OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintmskSpec;
impl crate::RegisterSpec for DaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DaintmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DaintmskSpec {
    const RESET_VALUE: u32 = 0;
}
