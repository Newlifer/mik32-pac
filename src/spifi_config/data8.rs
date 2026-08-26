#[doc = "Register `DATA8` reader"]
pub type R = crate::R<Data8Spec>;
#[doc = "Register `DATA8` writer"]
pub type W = crate::W<Data8Spec>;
#[doc = "Field `DATA` reader - Входные или выходные данные"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Входные или выходные данные"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Data8Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "SPIFI регистр данных, 8-битный доступ. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 \"Load access fault\").\n\nYou can [`read`](crate::Reg::read) this register and get [`data8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data8Spec;
impl crate::RegisterSpec for Data8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`data8::R`](R) reader structure"]
impl crate::Readable for Data8Spec {}
#[doc = "`write(|w| ..)` method takes [`data8::W`](W) writer structure"]
impl crate::Writable for Data8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA8 to value 0"]
impl crate::Resettable for Data8Spec {}
