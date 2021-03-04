#[doc = "Register `HFCLK192MRUN` reader"]
pub struct R(crate::R<HFCLK192MRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLK192MRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HFCLK192MRUN_SPEC>> for R {
    fn from(reader: crate::R<HFCLK192MRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "HFCLK192MSTART task triggered or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Task not triggered"]
    NOTTRIGGERED = 0,
    #[doc = "1: Task triggered"]
    TRIGGERED = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - HFCLK192MSTART task triggered or not"]
pub struct STATUS_R(crate::FieldReader<bool, STATUS_A>);
impl STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NOTTRIGGERED,
            true => STATUS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTTRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        **self == STATUS_A::NOTTRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        **self == STATUS_A::TRIGGERED
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HFCLK192MSTART task triggered or not"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status indicating that HFCLK192MSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclk192mrun](index.html) module"]
pub struct HFCLK192MRUN_SPEC;
impl crate::RegisterSpec for HFCLK192MRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclk192mrun::R](R) reader structure"]
impl crate::Readable for HFCLK192MRUN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLK192MRUN to value 0"]
impl crate::Resettable for HFCLK192MRUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
