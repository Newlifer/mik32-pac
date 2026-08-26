#[doc = "Register `DATA16` reader"]
pub type R = crate::R<Data16Spec>;
#[doc = "Register `DATA16` writer"]
pub type W = crate::W<Data16Spec>;
#[doc = "Field `DATA` reader - Входные или выходные данные"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Входные или выходные данные"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Data16Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "SPIFI регистр данных, 16-битный доступ. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 \"Load access fault\").\n\nYou can [`read`](crate::Reg::read) this register and get [`data16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data16Spec;
impl crate::RegisterSpec for Data16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data16::R`](R) reader structure"]
impl crate::Readable for Data16Spec {}
#[doc = "`write(|w| ..)` method takes [`data16::W`](W) writer structure"]
impl crate::Writable for Data16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA16 to value 0"]
impl crate::Resettable for Data16Spec {}
