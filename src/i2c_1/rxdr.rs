#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RxdrSpec>;
#[doc = "Field `RXDATA` reader - Буфер принятых данных"]
pub type RxdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Буфер принятых данных"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Регистр принятых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdrSpec;
impl crate::RegisterSpec for RxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RxdrSpec {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RxdrSpec {}
