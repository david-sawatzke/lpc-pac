#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BUSY0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BSYR {
    bits: u32,
}
impl BSYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:17 - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. 0 = not busy. 1 = busy."]
    #[inline]
    pub fn bsy(&self) -> BSYR {
        let bits = {
            const MASK: u32 = 262143;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        BSYR { bits }
    }
}
