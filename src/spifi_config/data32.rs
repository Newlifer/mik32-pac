#[doc = "Register `DATA32` reader"]
pub type R = crate::R<Data32Spec>;
#[doc = "Register `DATA32` writer"]
pub type W = crate::W<Data32Spec>;
#[doc = "Field `DATA` reader - Входные или выходные данные"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Входные или выходные данные"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Data32Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "SPIFI регистр данных, 32-битный доступ. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 \"Load access fault\").\n\nYou can [`read`](crate::Reg::read) this register and get [`data32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data32Spec;
impl crate::RegisterSpec for Data32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data32::R`](R) reader structure"]
impl crate::Readable for Data32Spec {}
#[doc = "`write(|w| ..)` method takes [`data32::W`](W) writer structure"]
impl crate::Writable for Data32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA32 to value 0"]
impl crate::Resettable for Data32Spec {}
