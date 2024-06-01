#[doc = "Register `TMC0` reader"]
pub type R = crate::R<Tmc0Spec>;
#[doc = "Register `TMC0` writer"]
pub type W = crate::W<Tmc0Spec>;
#[doc = "Field `TMDTBL` reader - Transmit mailbox data byte length"]
pub type TmdtblR = crate::FieldReader;
#[doc = "Field `TMDTBL` writer - Transmit mailbox data byte length"]
pub type TmdtblW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMTSTEN` reader - Transmit mailbox time stamp transmit enable"]
pub type TmtstenR = crate::BitReader;
#[doc = "Field `TMTSTEN` writer - Transmit mailbox time stamp transmit enable"]
pub type TmtstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMTS` reader - Transmit mailbox time stamp"]
pub type TmtsR = crate::FieldReader<u16>;
#[doc = "Field `TMTS` writer - Transmit mailbox time stamp"]
pub type TmtsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    pub fn tmdtbl(&self) -> TmdtblR {
        TmdtblR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn tmtsten(&self) -> TmtstenR {
        TmtstenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    pub fn tmts(&self) -> TmtsR {
        TmtsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMC0")
            .field("tmts", &self.tmts())
            .field("tmtsten", &self.tmtsten())
            .field("tmdtbl", &self.tmdtbl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    #[must_use]
    pub fn tmdtbl(&mut self) -> TmdtblW<Tmc0Spec> {
        TmdtblW::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmtsten(&mut self) -> TmtstenW<Tmc0Spec> {
        TmtstenW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn tmts(&mut self) -> TmtsW<Tmc0Spec> {
        TmtsW::new(self, 16)
    }
}
#[doc = "Transmit mailbox 0 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmc0Spec;
impl crate::RegisterSpec for Tmc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmc0::R`](R) reader structure"]
impl crate::Readable for Tmc0Spec {}
#[doc = "`write(|w| ..)` method takes [`tmc0::W`](W) writer structure"]
impl crate::Writable for Tmc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMC0 to value 0"]
impl crate::Resettable for Tmc0Spec {
    const RESET_VALUE: u32 = 0;
}
