#[doc = "Register `SYNCEN` reader"]
pub type R = crate::R<SyncenSpec>;
#[doc = "Register `SYNCEN` writer"]
pub type W = crate::W<SyncenSpec>;
#[doc = "Field `S1SYNC` reader - Stream 1 sync enable"]
pub type S1syncR = crate::BitReader;
#[doc = "Field `S1SYNC` writer - Stream 1 sync enable"]
pub type S1syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2SYNC` reader - Stream 2 sync enable"]
pub type S2syncR = crate::BitReader;
#[doc = "Field `S2SYNC` writer - Stream 2 sync enable"]
pub type S2syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3SYNC` reader - Stream 3 sync enable"]
pub type S3syncR = crate::BitReader;
#[doc = "Field `S3SYNC` writer - Stream 3 sync enable"]
pub type S3syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S4SYNC` reader - Stream 4 sync enable"]
pub type S4syncR = crate::BitReader;
#[doc = "Field `S4SYNC` writer - Stream 4 sync enable"]
pub type S4syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S5SYNC` reader - Stream 5 sync enable"]
pub type S5syncR = crate::BitReader;
#[doc = "Field `S5SYNC` writer - Stream 5 sync enable"]
pub type S5syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S6SYNC` reader - Stream 6 sync enable"]
pub type S6syncR = crate::BitReader;
#[doc = "Field `S6SYNC` writer - Stream 6 sync enable"]
pub type S6syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S7SYNC` reader - Stream 7 sync enable"]
pub type S7syncR = crate::BitReader;
#[doc = "Field `S7SYNC` writer - Stream 7 sync enable"]
pub type S7syncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S8SYNC` reader - Stream 8 sync enable"]
pub type S8syncR = crate::BitReader;
#[doc = "Field `S8SYNC` writer - Stream 8 sync enable"]
pub type S8syncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 sync enable"]
    #[inline(always)]
    pub fn s1sync(&self) -> S1syncR {
        S1syncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stream 2 sync enable"]
    #[inline(always)]
    pub fn s2sync(&self) -> S2syncR {
        S2syncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 3 sync enable"]
    #[inline(always)]
    pub fn s3sync(&self) -> S3syncR {
        S3syncR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 4 sync enable"]
    #[inline(always)]
    pub fn s4sync(&self) -> S4syncR {
        S4syncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 sync enable"]
    #[inline(always)]
    pub fn s5sync(&self) -> S5syncR {
        S5syncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 6 sync enable"]
    #[inline(always)]
    pub fn s6sync(&self) -> S6syncR {
        S6syncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 7 sync enable"]
    #[inline(always)]
    pub fn s7sync(&self) -> S7syncR {
        S7syncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stream 8 sync enable"]
    #[inline(always)]
    pub fn s8sync(&self) -> S8syncR {
        S8syncR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCEN")
            .field("s1sync", &self.s1sync())
            .field("s2sync", &self.s2sync())
            .field("s3sync", &self.s3sync())
            .field("s4sync", &self.s4sync())
            .field("s5sync", &self.s5sync())
            .field("s6sync", &self.s6sync())
            .field("s7sync", &self.s7sync())
            .field("s8sync", &self.s8sync())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1sync(&mut self) -> S1syncW<SyncenSpec> {
        S1syncW::new(self, 0)
    }
    #[doc = "Bit 1 - Stream 2 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2sync(&mut self) -> S2syncW<SyncenSpec> {
        S2syncW::new(self, 1)
    }
    #[doc = "Bit 2 - Stream 3 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3sync(&mut self) -> S3syncW<SyncenSpec> {
        S3syncW::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 4 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s4sync(&mut self) -> S4syncW<SyncenSpec> {
        S4syncW::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s5sync(&mut self) -> S5syncW<SyncenSpec> {
        S5syncW::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 6 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s6sync(&mut self) -> S6syncW<SyncenSpec> {
        S6syncW::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 7 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s7sync(&mut self) -> S7syncW<SyncenSpec> {
        S7syncW::new(self, 6)
    }
    #[doc = "Bit 7 - Stream 8 sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn s8sync(&mut self) -> S8syncW<SyncenSpec> {
        S8syncW::new(self, 7)
    }
}
#[doc = "Sync Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncenSpec;
impl crate::RegisterSpec for SyncenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncen::R`](R) reader structure"]
impl crate::Readable for SyncenSpec {}
#[doc = "`write(|w| ..)` method takes [`syncen::W`](W) writer structure"]
impl crate::Writable for SyncenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNCEN to value 0"]
impl crate::Resettable for SyncenSpec {
    const RESET_VALUE: u32 = 0;
}
