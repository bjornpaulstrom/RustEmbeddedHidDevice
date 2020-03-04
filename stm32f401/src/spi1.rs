# ! [ doc = "Serial peripheral interface" ]# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Spi1 {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x08 - status register" ]
    pub sr: Sr,
    # [ doc = "0x0c - data register" ]
    pub dr: Dr,
    # [ doc = "0x10 - CRC polynomial register" ]
    pub crcpr: Crcpr,
    # [ doc = "0x14 - RX CRC register" ]
    pub rxcrcr: Rxcrcr,
    # [ doc = "0x18 - TX CRC register" ]
    pub txcrcr: Txcrcr,
    # [ doc = "0x1c - I2S configuration register" ]
    pub i2scfgr: I2scfgr,
    # [ doc = "0x20 - I2S prescaler register" ]
    pub i2spr: I2spr,
}

# [ doc = "control register 1" ]
# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "control register 1" ]
pub mod cr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr1 {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field BIDIMODE" ]
    pub struct BidimodeR {
        bits: u8,
    }
    impl BidimodeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BIDIOE" ]
    pub struct BidioeR {
        bits: u8,
    }
    impl BidioeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCEN" ]
    pub struct CrcenR {
        bits: u8,
    }
    impl CrcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCNEXT" ]
    pub struct CrcnextR {
        bits: u8,
    }
    impl CrcnextR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DFF" ]
    pub struct DffR {
        bits: u8,
    }
    impl DffR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXONLY" ]
    pub struct RxonlyR {
        bits: u8,
    }
    impl RxonlyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SSM" ]
    pub struct SsmR {
        bits: u8,
    }
    impl SsmR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SSI" ]
    pub struct SsiR {
        bits: u8,
    }
    impl SsiR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSBFIRST" ]
    pub struct LsbfirstR {
        bits: u8,
    }
    impl LsbfirstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPE" ]
    pub struct SpeR {
        bits: u8,
    }
    impl SpeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BR" ]
    pub struct BrR {
        bits: u8,
    }
    impl BrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MSTR" ]
    pub struct MstrR {
        bits: u8,
    }
    impl MstrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CPOL" ]
    pub struct CpolR {
        bits: u8,
    }
    impl CpolR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CPHA" ]
    pub struct CphaR {
        bits: u8,
    }
    impl CphaR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BidimodeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BidimodeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BidioeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BidioeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CrcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CrcnextW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcnextW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DffW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DffW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RxonlyW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxonlyW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SsmW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SsmW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SsiW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SsiW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LsbfirstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsbfirstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SpeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SpeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BrW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BrW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MstrW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MstrW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CpolW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CpolW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CphaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CphaW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _bidimode(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Bidirectional data mode enable" ]
        pub fn bidimode(&self) -> BidimodeR {
            BidimodeR { bits: self._bidimode() }
        }
        fn _bidioe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Output enable in bidirectional mode" ]
        pub fn bidioe(&self) -> BidioeR {
            BidioeR { bits: self._bidioe() }
        }
        fn _crcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Hardware CRC calculation enable" ]
        pub fn crcen(&self) -> CrcenR {
            CrcenR { bits: self._crcen() }
        }
        fn _crcnext(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - CRC transfer next" ]
        pub fn crcnext(&self) -> CrcnextR {
            CrcnextR { bits: self._crcnext() }
        }
        fn _dff(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Data frame format" ]
        pub fn dff(&self) -> DffR {
            DffR { bits: self._dff() }
        }
        fn _rxonly(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Receive only" ]
        pub fn rxonly(&self) -> RxonlyR {
            RxonlyR { bits: self._rxonly() }
        }
        fn _ssm(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Software slave management" ]
        pub fn ssm(&self) -> SsmR {
            SsmR { bits: self._ssm() }
        }
        fn _ssi(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Internal slave select" ]
        pub fn ssi(&self) -> SsiR {
            SsiR { bits: self._ssi() }
        }
        fn _lsbfirst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Frame format" ]
        pub fn lsbfirst(&self) -> LsbfirstR {
            LsbfirstR { bits: self._lsbfirst() }
        }
        fn _spe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - SPI enable" ]
        pub fn spe(&self) -> SpeR {
            SpeR { bits: self._spe() }
        }
        fn _br(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 3:5 - Baud rate control" ]
        pub fn br(&self) -> BrR {
            BrR { bits: self._br() }
        }
        fn _mstr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Master selection" ]
        pub fn mstr(&self) -> MstrR {
            MstrR { bits: self._mstr() }
        }
        fn _cpol(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Clock polarity" ]
        pub fn cpol(&self) -> CpolR {
            CpolR { bits: self._cpol() }
        }
        fn _cpha(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Clock phase" ]
        pub fn cpha(&self) -> CphaR {
            CphaR { bits: self._cpha() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 15 - Bidirectional data mode enable" ]
        pub fn bidimode(&mut self) -> _BidimodeW {
            _BidimodeW { register: self }
        }
        # [ doc = "Bit 14 - Output enable in bidirectional mode" ]
        pub fn bidioe(&mut self) -> _BidioeW {
            _BidioeW { register: self }
        }
        # [ doc = "Bit 13 - Hardware CRC calculation enable" ]
        pub fn crcen(&mut self) -> _CrcenW {
            _CrcenW { register: self }
        }
        # [ doc = "Bit 12 - CRC transfer next" ]
        pub fn crcnext(&mut self) -> _CrcnextW {
            _CrcnextW { register: self }
        }
        # [ doc = "Bit 11 - Data frame format" ]
        pub fn dff(&mut self) -> _DffW {
            _DffW { register: self }
        }
        # [ doc = "Bit 10 - Receive only" ]
        pub fn rxonly(&mut self) -> _RxonlyW {
            _RxonlyW { register: self }
        }
        # [ doc = "Bit 9 - Software slave management" ]
        pub fn ssm(&mut self) -> _SsmW {
            _SsmW { register: self }
        }
        # [ doc = "Bit 8 - Internal slave select" ]
        pub fn ssi(&mut self) -> _SsiW {
            _SsiW { register: self }
        }
        # [ doc = "Bit 7 - Frame format" ]
        pub fn lsbfirst(&mut self) -> _LsbfirstW {
            _LsbfirstW { register: self }
        }
        # [ doc = "Bit 6 - SPI enable" ]
        pub fn spe(&mut self) -> _SpeW {
            _SpeW { register: self }
        }
        # [ doc = "Bits 3:5 - Baud rate control" ]
        pub fn br(&mut self) -> _BrW {
            _BrW { register: self }
        }
        # [ doc = "Bit 2 - Master selection" ]
        pub fn mstr(&mut self) -> _MstrW {
            _MstrW { register: self }
        }
        # [ doc = "Bit 1 - Clock polarity" ]
        pub fn cpol(&mut self) -> _CpolW {
            _CpolW { register: self }
        }
        # [ doc = "Bit 0 - Clock phase" ]
        pub fn cpha(&mut self) -> _CphaW {
            _CphaW { register: self }
        }
    }
}

# [ doc = "control register 2" ]
# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "control register 2" ]
pub mod cr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr2 {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field TXEIE" ]
    pub struct TxeieR {
        bits: u8,
    }
    impl TxeieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXNEIE" ]
    pub struct RxneieR {
        bits: u8,
    }
    impl RxneieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ERRIE" ]
    pub struct ErrieR {
        bits: u8,
    }
    impl ErrieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field FRF" ]
    pub struct FrfR {
        bits: u8,
    }
    impl FrfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SSOE" ]
    pub struct SsoeR {
        bits: u8,
    }
    impl SsoeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TXDMAEN" ]
    pub struct TxdmaenR {
        bits: u8,
    }
    impl TxdmaenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXDMAEN" ]
    pub struct RxdmaenR {
        bits: u8,
    }
    impl RxdmaenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TxeieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxeieW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RxneieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxneieW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ErrieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ErrieW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FrfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FrfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SsoeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SsoeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TxdmaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxdmaenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RxdmaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxdmaenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _txeie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Tx buffer empty interrupt enable" ]
        pub fn txeie(&self) -> TxeieR {
            TxeieR { bits: self._txeie() }
        }
        fn _rxneie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - RX buffer not empty interrupt enable" ]
        pub fn rxneie(&self) -> RxneieR {
            RxneieR { bits: self._rxneie() }
        }
        fn _errie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Error interrupt enable" ]
        pub fn errie(&self) -> ErrieR {
            ErrieR { bits: self._errie() }
        }
        fn _frf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Frame format" ]
        pub fn frf(&self) -> FrfR {
            FrfR { bits: self._frf() }
        }
        fn _ssoe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - SS output enable" ]
        pub fn ssoe(&self) -> SsoeR {
            SsoeR { bits: self._ssoe() }
        }
        fn _txdmaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Tx buffer DMA enable" ]
        pub fn txdmaen(&self) -> TxdmaenR {
            TxdmaenR { bits: self._txdmaen() }
        }
        fn _rxdmaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Rx buffer DMA enable" ]
        pub fn rxdmaen(&self) -> RxdmaenR {
            RxdmaenR { bits: self._rxdmaen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 7 - Tx buffer empty interrupt enable" ]
        pub fn txeie(&mut self) -> _TxeieW {
            _TxeieW { register: self }
        }
        # [ doc = "Bit 6 - RX buffer not empty interrupt enable" ]
        pub fn rxneie(&mut self) -> _RxneieW {
            _RxneieW { register: self }
        }
        # [ doc = "Bit 5 - Error interrupt enable" ]
        pub fn errie(&mut self) -> _ErrieW {
            _ErrieW { register: self }
        }
        # [ doc = "Bit 4 - Frame format" ]
        pub fn frf(&mut self) -> _FrfW {
            _FrfW { register: self }
        }
        # [ doc = "Bit 2 - SS output enable" ]
        pub fn ssoe(&mut self) -> _SsoeW {
            _SsoeW { register: self }
        }
        # [ doc = "Bit 1 - Tx buffer DMA enable" ]
        pub fn txdmaen(&mut self) -> _TxdmaenW {
            _TxdmaenW { register: self }
        }
        # [ doc = "Bit 0 - Rx buffer DMA enable" ]
        pub fn rxdmaen(&mut self) -> _RxdmaenW {
            _RxdmaenW { register: self }
        }
    }
}

# [ doc = "status register" ]
# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "status register" ]
pub mod sr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Sr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field TIFRFE" ]
    pub struct TifrfeR {
        bits: u8,
    }
    impl TifrfeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BSY" ]
    pub struct BsyR {
        bits: u8,
    }
    impl BsyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OVR" ]
    pub struct OvrR {
        bits: u8,
    }
    impl OvrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODF" ]
    pub struct ModfR {
        bits: u8,
    }
    impl ModfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCERR" ]
    pub struct CrcerrR {
        bits: u8,
    }
    impl CrcerrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UDR" ]
    pub struct UdrR {
        bits: u8,
    }
    impl UdrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CHSIDE" ]
    pub struct ChsideR {
        bits: u8,
    }
    impl ChsideR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TXE" ]
    pub struct TxeR {
        bits: u8,
    }
    impl TxeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXNE" ]
    pub struct RxneR {
        bits: u8,
    }
    impl RxneR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CrcerrW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcerrW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _tifrfe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - TI frame format error" ]
        pub fn tifrfe(&self) -> TifrfeR {
            TifrfeR { bits: self._tifrfe() }
        }
        fn _bsy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Busy flag" ]
        pub fn bsy(&self) -> BsyR {
            BsyR { bits: self._bsy() }
        }
        fn _ovr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Overrun flag" ]
        pub fn ovr(&self) -> OvrR {
            OvrR { bits: self._ovr() }
        }
        fn _modf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Mode fault" ]
        pub fn modf(&self) -> ModfR {
            ModfR { bits: self._modf() }
        }
        fn _crcerr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - CRC error flag" ]
        pub fn crcerr(&self) -> CrcerrR {
            CrcerrR { bits: self._crcerr() }
        }
        fn _udr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Underrun flag" ]
        pub fn udr(&self) -> UdrR {
            UdrR { bits: self._udr() }
        }
        fn _chside(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Channel side" ]
        pub fn chside(&self) -> ChsideR {
            ChsideR { bits: self._chside() }
        }
        fn _txe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Transmit buffer empty" ]
        pub fn txe(&self) -> TxeR {
            TxeR { bits: self._txe() }
        }
        fn _rxne(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Receive buffer not empty" ]
        pub fn rxne(&self) -> RxneR {
            RxneR { bits: self._rxne() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 2 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 4 - CRC error flag" ]
        pub fn crcerr(&mut self) -> _CrcerrW {
            _CrcerrW { register: self }
        }
    }
}

# [ doc = "data register" ]
# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "data register" ]
pub mod dr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Dr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field DR" ]
    pub struct DrR {
        bits: u16,
    }
    impl DrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DrW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DrW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _dr(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Data register" ]
        pub fn dr(&self) -> DrR {
            DrR { bits: self._dr() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - Data register" ]
        pub fn dr(&mut self) -> _DrW {
            _DrW { register: self }
        }
    }
}

# [ doc = "CRC polynomial register" ]
# [ repr ( C ) ]
pub struct Crcpr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "CRC polynomial register" ]
pub mod crcpr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Crcpr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field CRCPOLY" ]
    pub struct CrcpolyR {
        bits: u16,
    }
    impl CrcpolyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CrcpolyW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcpolyW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _crcpoly(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - CRC polynomial register" ]
        pub fn crcpoly(&self) -> CrcpolyR {
            CrcpolyR { bits: self._crcpoly() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 7 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:15 - CRC polynomial register" ]
        pub fn crcpoly(&mut self) -> _CrcpolyW {
            _CrcpolyW { register: self }
        }
    }
}

# [ doc = "RX CRC register" ]
# [ repr ( C ) ]
pub struct Rxcrcr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "RX CRC register" ]
pub mod rxcrcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Rxcrcr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field RxCRC" ]
    pub struct RxCrcR {
        bits: u16,
    }
    impl RxCrcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _rx_crc(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Rx CRC register" ]
        pub fn rx_crc(&self) -> RxCrcR {
            RxCrcR { bits: self._rx_crc() }
        }
    }
}

# [ doc = "TX CRC register" ]
# [ repr ( C ) ]
pub struct Txcrcr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "TX CRC register" ]
pub mod txcrcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Txcrcr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field TxCRC" ]
    pub struct TxCrcR {
        bits: u16,
    }
    impl TxCrcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _tx_crc(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Tx CRC register" ]
        pub fn tx_crc(&self) -> TxCrcR {
            TxCrcR { bits: self._tx_crc() }
        }
    }
}

# [ doc = "I2S configuration register" ]
# [ repr ( C ) ]
pub struct I2scfgr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "I2S configuration register" ]
pub mod i2scfgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::I2scfgr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field I2SMOD" ]
    pub struct I2smodR {
        bits: u8,
    }
    impl I2smodR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2SE" ]
    pub struct I2seR {
        bits: u8,
    }
    impl I2seR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2SCFG" ]
    pub struct I2scfgR {
        bits: u8,
    }
    impl I2scfgR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PCMSYNC" ]
    pub struct PcmsyncR {
        bits: u8,
    }
    impl PcmsyncR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2SSTD" ]
    pub struct I2sstdR {
        bits: u8,
    }
    impl I2sstdR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CKPOL" ]
    pub struct CkpolR {
        bits: u8,
    }
    impl CkpolR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DATLEN" ]
    pub struct DatlenR {
        bits: u8,
    }
    impl DatlenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CHLEN" ]
    pub struct ChlenR {
        bits: u8,
    }
    impl ChlenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2smodW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2smodW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2seW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2seW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2scfgW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2scfgW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PcmsyncW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PcmsyncW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2sstdW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2sstdW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CkpolW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CkpolW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DatlenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DatlenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ChlenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ChlenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _i2smod(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - I2S mode selection" ]
        pub fn i2smod(&self) -> I2smodR {
            I2smodR { bits: self._i2smod() }
        }
        fn _i2se(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - I2S Enable" ]
        pub fn i2se(&self) -> I2seR {
            I2seR { bits: self._i2se() }
        }
        fn _i2scfg(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - I2S configuration mode" ]
        pub fn i2scfg(&self) -> I2scfgR {
            I2scfgR { bits: self._i2scfg() }
        }
        fn _pcmsync(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - PCM frame synchronization" ]
        pub fn pcmsync(&self) -> PcmsyncR {
            PcmsyncR { bits: self._pcmsync() }
        }
        fn _i2sstd(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - I2S standard selection" ]
        pub fn i2sstd(&self) -> I2sstdR {
            I2sstdR { bits: self._i2sstd() }
        }
        fn _ckpol(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Steady state clock polarity" ]
        pub fn ckpol(&self) -> CkpolR {
            CkpolR { bits: self._ckpol() }
        }
        fn _datlen(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 1:2 - Data length to be transferred" ]
        pub fn datlen(&self) -> DatlenR {
            DatlenR { bits: self._datlen() }
        }
        fn _chlen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Channel length (number of bits per audio channel)" ]
        pub fn chlen(&self) -> ChlenR {
            ChlenR { bits: self._chlen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 11 - I2S mode selection" ]
        pub fn i2smod(&mut self) -> _I2smodW {
            _I2smodW { register: self }
        }
        # [ doc = "Bit 10 - I2S Enable" ]
        pub fn i2se(&mut self) -> _I2seW {
            _I2seW { register: self }
        }
        # [ doc = "Bits 8:9 - I2S configuration mode" ]
        pub fn i2scfg(&mut self) -> _I2scfgW {
            _I2scfgW { register: self }
        }
        # [ doc = "Bit 7 - PCM frame synchronization" ]
        pub fn pcmsync(&mut self) -> _PcmsyncW {
            _PcmsyncW { register: self }
        }
        # [ doc = "Bits 4:5 - I2S standard selection" ]
        pub fn i2sstd(&mut self) -> _I2sstdW {
            _I2sstdW { register: self }
        }
        # [ doc = "Bit 3 - Steady state clock polarity" ]
        pub fn ckpol(&mut self) -> _CkpolW {
            _CkpolW { register: self }
        }
        # [ doc = "Bits 1:2 - Data length to be transferred" ]
        pub fn datlen(&mut self) -> _DatlenW {
            _DatlenW { register: self }
        }
        # [ doc = "Bit 0 - Channel length (number of bits per audio channel)" ]
        pub fn chlen(&mut self) -> _ChlenW {
            _ChlenW { register: self }
        }
    }
}

# [ doc = "I2S prescaler register" ]
# [ repr ( C ) ]
pub struct I2spr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "I2S prescaler register" ]
pub mod i2spr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::I2spr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field MCKOE" ]
    pub struct MckoeR {
        bits: u8,
    }
    impl MckoeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODD" ]
    pub struct OddR {
        bits: u8,
    }
    impl OddR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2SDIV" ]
    pub struct I2sdivR {
        bits: u8,
    }
    impl I2sdivR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MckoeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MckoeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OddW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OddW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2sdivW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2sdivW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _mckoe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Master clock output enable" ]
        pub fn mckoe(&self) -> MckoeR {
            MckoeR { bits: self._mckoe() }
        }
        fn _odd(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Odd factor for the prescaler" ]
        pub fn odd(&self) -> OddR {
            OddR { bits: self._odd() }
        }
        fn _i2sdiv(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - I2S Linear prescaler" ]
        pub fn i2sdiv(&self) -> I2sdivR {
            I2sdivR { bits: self._i2sdiv() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 10 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 9 - Master clock output enable" ]
        pub fn mckoe(&mut self) -> _MckoeW {
            _MckoeW { register: self }
        }
        # [ doc = "Bit 8 - Odd factor for the prescaler" ]
        pub fn odd(&mut self) -> _OddW {
            _OddW { register: self }
        }
        # [ doc = "Bits 0:7 - I2S Linear prescaler" ]
        pub fn i2sdiv(&mut self) -> _I2sdivW {
            _I2sdivW { register: self }
        }
    }
}
