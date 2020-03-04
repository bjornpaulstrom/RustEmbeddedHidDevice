# ! [ doc = "Universal synchronous asynchronous receiver transmitter" ]# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Usart2 {
    # [ doc = "0x00 - Status register" ]
    pub sr: Sr,
    # [ doc = "0x04 - Data register" ]
    pub dr: Dr,
    # [ doc = "0x08 - Baud rate register" ]
    pub brr: Brr,
    # [ doc = "0x0c - Control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x10 - Control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x14 - Control register 3" ]
    pub cr3: Cr3,
    # [ doc = "0x18 - Guard time and prescaler register" ]
    pub gtpr: Gtpr,
}

# [ doc = "Status register" ]
# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Status register" ]
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
    # [ doc = "Possible values of the field `cts`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum CtsR {
        # [ doc = "0: No change occurred on the nCTS status line" ]
        NoChange,
        # [ doc = "1: A change occurred on the nCTS status line" ]
        Change,
    }
    impl CtsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                CtsR::NoChange => 0,
                CtsR::Change => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> CtsR {
            match bits {
                0 => CtsR::NoChange,
                1 => CtsR::Change,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NoChange`" ]
        pub fn is_no_change(&self) -> bool {
            *self == CtsR::NoChange
        }
        # [ doc = "Check if the value of the field is `Change`" ]
        pub fn is_change(&self) -> bool {
            *self == CtsR::Change
        }
    }
    # [ doc = "Possible values of the field `lbd`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LbdR {
        # [ doc = "0: LIN Break not detected" ]
        NotDetected,
        # [ doc = "1: LIN break detected" ]
        BreakDetected,
    }
    impl LbdR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                LbdR::NotDetected => 0,
                LbdR::BreakDetected => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> LbdR {
            match bits {
                0 => LbdR::NotDetected,
                1 => LbdR::BreakDetected,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotDetected`" ]
        pub fn is_not_detected(&self) -> bool {
            *self == LbdR::NotDetected
        }
        # [ doc = "Check if the value of the field is `BreakDetected`" ]
        pub fn is_break_detected(&self) -> bool {
            *self == LbdR::BreakDetected
        }
    }
    # [ doc = "Possible values of the field `txe`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TxeR {
        # [ doc = "0: Data is not transferred to the shift register" ]
        NotDetected,
        # [ doc = "1: Data is transferred to the shift register" ]
        BreakDetected,
    }
    impl TxeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                TxeR::NotDetected => 0,
                TxeR::BreakDetected => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> TxeR {
            match bits {
                0 => TxeR::NotDetected,
                1 => TxeR::BreakDetected,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotDetected`" ]
        pub fn is_not_detected(&self) -> bool {
            *self == TxeR::NotDetected
        }
        # [ doc = "Check if the value of the field is `BreakDetected`" ]
        pub fn is_break_detected(&self) -> bool {
            *self == TxeR::BreakDetected
        }
    }
    # [ doc = "Possible values of the field `tc`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum TcR {
        # [ doc = "0: Transmission is not complete" ]
        NotComplete,
        # [ doc = "1: Transmission is complete" ]
        Complete,
    }
    impl TcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                TcR::NotComplete => 0,
                TcR::Complete => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> TcR {
            match bits {
                0 => TcR::NotComplete,
                1 => TcR::Complete,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotComplete`" ]
        pub fn is_not_complete(&self) -> bool {
            *self == TcR::NotComplete
        }
        # [ doc = "Check if the value of the field is `Complete`" ]
        pub fn is_complete(&self) -> bool {
            *self == TcR::Complete
        }
    }
    # [ doc = "Possible values of the field `rxne`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum RxneR {
        # [ doc = "0: Data is not received" ]
        NotReceived,
        # [ doc = "1: Received data is ready to be read" ]
        DataReady,
    }
    impl RxneR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                RxneR::NotReceived => 0,
                RxneR::DataReady => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> RxneR {
            match bits {
                0 => RxneR::NotReceived,
                1 => RxneR::DataReady,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotReceived`" ]
        pub fn is_not_received(&self) -> bool {
            *self == RxneR::NotReceived
        }
        # [ doc = "Check if the value of the field is `DataReady`" ]
        pub fn is_data_ready(&self) -> bool {
            *self == RxneR::DataReady
        }
    }
    # [ doc = "Possible values of the field `idle`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum IdleR {
        # [ doc = "0: No Idle Line is detected" ]
        NotDetected,
        # [ doc = "1: Idle Line is detected" ]
        Detected,
    }
    impl IdleR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                IdleR::NotDetected => 0,
                IdleR::Detected => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> IdleR {
            match bits {
                0 => IdleR::NotDetected,
                1 => IdleR::Detected,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotDetected`" ]
        pub fn is_not_detected(&self) -> bool {
            *self == IdleR::NotDetected
        }
        # [ doc = "Check if the value of the field is `Detected`" ]
        pub fn is_detected(&self) -> bool {
            *self == IdleR::Detected
        }
    }
    # [ doc = "Possible values of the field `ore`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OreR {
        # [ doc = "0: No Overrun error" ]
        NoOverrun,
        # [ doc = "1: Overrun error is detected" ]
        OverrunDetected,
    }
    impl OreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                OreR::NoOverrun => 0,
                OreR::OverrunDetected => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> OreR {
            match bits {
                0 => OreR::NoOverrun,
                1 => OreR::OverrunDetected,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NoOverrun`" ]
        pub fn is_no_overrun(&self) -> bool {
            *self == OreR::NoOverrun
        }
        # [ doc = "Check if the value of the field is `OverrunDetected`" ]
        pub fn is_overrun_detected(&self) -> bool {
            *self == OreR::OverrunDetected
        }
    }
    # [ doc = "Possible values of the field `nf`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum NfR {
        # [ doc = "0: No noise is detected" ]
        NotDetected,
        # [ doc = "1: Noise is detected" ]
        NoiseDetected,
    }
    impl NfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                NfR::NotDetected => 0,
                NfR::NoiseDetected => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> NfR {
            match bits {
                0 => NfR::NotDetected,
                1 => NfR::NoiseDetected,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotDetected`" ]
        pub fn is_not_detected(&self) -> bool {
            *self == NfR::NotDetected
        }
        # [ doc = "Check if the value of the field is `NoiseDetected`" ]
        pub fn is_noise_detected(&self) -> bool {
            *self == NfR::NoiseDetected
        }
    }
    # [ doc = "Possible values of the field `fe`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum FeR {
        # [ doc = "0: No Framing error is detected" ]
        NotDetected,
        # [ doc = "1: Framing error or break character is detected" ]
        FramingErrorDetected,
    }
    impl FeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                FeR::NotDetected => 0,
                FeR::FramingErrorDetected => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> FeR {
            match bits {
                0 => FeR::NotDetected,
                1 => FeR::FramingErrorDetected,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NotDetected`" ]
        pub fn is_not_detected(&self) -> bool {
            *self == FeR::NotDetected
        }
        # [ doc = "Check if the value of the field is `FramingErrorDetected`" ]
        pub fn is_framing_error_detected(&self) -> bool {
            *self == FeR::FramingErrorDetected
        }
    }
    # [ doc = "Possible values of the field `pe`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PeR {
        # [ doc = "0: No parity error" ]
        NoParityError,
        # [ doc = "1: Parity error" ]
        ParityError,
    }
    impl PeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                PeR::NoParityError => 0,
                PeR::ParityError => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> PeR {
            match bits {
                0 => PeR::NoParityError,
                1 => PeR::ParityError,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NoParityError`" ]
        pub fn is_no_parity_error(&self) -> bool {
            *self == PeR::NoParityError
        }
        # [ doc = "Check if the value of the field is `ParityError`" ]
        pub fn is_parity_error(&self) -> bool {
            *self == PeR::ParityError
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CtsW<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `cts`" ]
    pub enum CtsW {
        # [ doc = "0: No change occurred on the nCTS status line" ]
        NoChange,
        # [ doc = "1: A change occurred on the nCTS status line" ]
        Change,
    }
    impl CtsW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                CtsW::NoChange => 0,
                CtsW::Change => 1,
            }
        }
    }
    impl<'a> _CtsW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: CtsW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No change occurred on the nCTS status line" ]
        pub fn no_change(self) -> &'a mut W {
            self.variant(CtsW::NoChange)
        }
        # [ doc = "1: A change occurred on the nCTS status line" ]
        pub fn change(self) -> &'a mut W {
            self.variant(CtsW::Change)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LbdW<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `lbd`" ]
    pub enum LbdW {
        # [ doc = "0: LIN Break not detected" ]
        NotDetected,
        # [ doc = "1: LIN break detected" ]
        BreakDetected,
    }
    impl LbdW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                LbdW::NotDetected => 0,
                LbdW::BreakDetected => 1,
            }
        }
    }
    impl<'a> _LbdW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LbdW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: LIN Break not detected" ]
        pub fn not_detected(self) -> &'a mut W {
            self.variant(LbdW::NotDetected)
        }
        # [ doc = "1: LIN break detected" ]
        pub fn break_detected(self) -> &'a mut W {
            self.variant(LbdW::BreakDetected)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TcW<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `tc`" ]
    pub enum TcW {
        # [ doc = "0: Transmission is not complete" ]
        NotComplete,
        # [ doc = "1: Transmission is complete" ]
        Complete,
    }
    impl TcW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                TcW::NotComplete => 0,
                TcW::Complete => 1,
            }
        }
    }
    impl<'a> _TcW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: TcW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Transmission is not complete" ]
        pub fn not_complete(self) -> &'a mut W {
            self.variant(TcW::NotComplete)
        }
        # [ doc = "1: Transmission is complete" ]
        pub fn complete(self) -> &'a mut W {
            self.variant(TcW::Complete)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RxneW<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `rxne`" ]
    pub enum RxneW {
        # [ doc = "0: Data is not received" ]
        NotReceived,
        # [ doc = "1: Received data is ready to be read" ]
        DataReady,
    }
    impl RxneW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                RxneW::NotReceived => 0,
                RxneW::DataReady => 1,
            }
        }
    }
    impl<'a> _RxneW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: RxneW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Data is not received" ]
        pub fn not_received(self) -> &'a mut W {
            self.variant(RxneW::NotReceived)
        }
        # [ doc = "1: Received data is ready to be read" ]
        pub fn data_ready(self) -> &'a mut W {
            self.variant(RxneW::DataReady)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
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
        fn _cts(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - CTS flag" ]
        pub fn cts(&self) -> CtsR {
            CtsR::_from(self._cts())
        }
        fn _lbd(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - LIN break detection flag" ]
        pub fn lbd(&self) -> LbdR {
            LbdR::_from(self._lbd())
        }
        fn _txe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Transmit data register empty" ]
        pub fn txe(&self) -> TxeR {
            TxeR::_from(self._txe())
        }
        fn _tc(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transmission complete" ]
        pub fn tc(&self) -> TcR {
            TcR::_from(self._tc())
        }
        fn _rxne(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Read data register not empty" ]
        pub fn rxne(&self) -> RxneR {
            RxneR::_from(self._rxne())
        }
        fn _idle(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - IDLE line detected" ]
        pub fn idle(&self) -> IdleR {
            IdleR::_from(self._idle())
        }
        fn _ore(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Overrun error" ]
        pub fn ore(&self) -> OreR {
            OreR::_from(self._ore())
        }
        fn _nf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Noise detected flag" ]
        pub fn nf(&self) -> NfR {
            NfR::_from(self._nf())
        }
        fn _fe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Framing error" ]
        pub fn fe(&self) -> FeR {
            FeR::_from(self._fe())
        }
        fn _pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Parity error" ]
        pub fn pe(&self) -> PeR {
            PeR::_from(self._pe())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 12582912 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 9 - CTS flag" ]
        pub fn cts(&mut self) -> _CtsW {
            _CtsW { register: self }
        }
        # [ doc = "Bit 8 - LIN break detection flag" ]
        pub fn lbd(&mut self) -> _LbdW {
            _LbdW { register: self }
        }
        # [ doc = "Bit 6 - Transmission complete" ]
        pub fn tc(&mut self) -> _TcW {
            _TcW { register: self }
        }
        # [ doc = "Bit 5 - Read data register not empty" ]
        pub fn rxne(&mut self) -> _RxneW {
            _RxneW { register: self }
        }
    }
}

# [ doc = "Data register" ]
# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Data register" ]
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
            const MASK: u16 = 511;
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
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:8 - Data value" ]
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
        # [ doc = "Bits 0:8 - Data value" ]
        pub fn dr(&mut self) -> _DrW {
            _DrW { register: self }
        }
    }
}

# [ doc = "Baud rate register" ]
# [ repr ( C ) ]
pub struct Brr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Baud rate register" ]
pub mod brr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Brr {
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
    # [ doc = "Value of the field DIV_Mantissa" ]
    pub struct DivMantissaR {
        bits: u16,
    }
    impl DivMantissaR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field DIV_Fraction" ]
    pub struct DivFractionR {
        bits: u8,
    }
    impl DivFractionR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DivMantissaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DivMantissaW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DivFractionW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DivFractionW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
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
        fn _div_mantissa(&self) -> u16 {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 4:15 - mantissa of USARTDIV" ]
        pub fn div_mantissa(&self) -> DivMantissaR {
            DivMantissaR { bits: self._div_mantissa() }
        }
        fn _div_fraction(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - fraction of USARTDIV" ]
        pub fn div_fraction(&self) -> DivFractionR {
            DivFractionR { bits: self._div_fraction() }
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
        # [ doc = "Bits 4:15 - mantissa of USARTDIV" ]
        pub fn div_mantissa(&mut self) -> _DivMantissaW {
            _DivMantissaW { register: self }
        }
        # [ doc = "Bits 0:3 - fraction of USARTDIV" ]
        pub fn div_fraction(&mut self) -> _DivFractionW {
            _DivFractionW { register: self }
        }
    }
}

# [ doc = "Control register 1" ]
# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control register 1" ]
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
    # [ doc = "Value of the field OVER8" ]
    pub struct Over8R {
        bits: u8,
    }
    impl Over8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UE" ]
    pub struct UeR {
        bits: u8,
    }
    impl UeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field M" ]
    pub struct MR {
        bits: u8,
    }
    impl MR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WAKE" ]
    pub struct WakeR {
        bits: u8,
    }
    impl WakeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PCE" ]
    pub struct PceR {
        bits: u8,
    }
    impl PceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PS" ]
    pub struct PsR {
        bits: u8,
    }
    impl PsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PEIE" ]
    pub struct PeieR {
        bits: u8,
    }
    impl PeieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
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
    # [ doc = "Value of the field TCIE" ]
    pub struct TcieR {
        bits: u8,
    }
    impl TcieR {
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
    # [ doc = "Value of the field IDLEIE" ]
    pub struct IdleieR {
        bits: u8,
    }
    impl IdleieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TE" ]
    pub struct TeR {
        bits: u8,
    }
    impl TeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RE" ]
    pub struct ReR {
        bits: u8,
    }
    impl ReR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RWU" ]
    pub struct RwuR {
        bits: u8,
    }
    impl RwuR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SBK" ]
    pub struct SbkR {
        bits: u8,
    }
    impl SbkR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Over8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Over8W<'a> {
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
    pub struct _UeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UeW<'a> {
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
    pub struct _MW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MW<'a> {
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
    pub struct _WakeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WakeW<'a> {
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
    pub struct _PceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PceW<'a> {
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
    pub struct _PsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PsW<'a> {
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
    pub struct _PeieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PeieW<'a> {
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
    pub struct _TcieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TcieW<'a> {
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
    pub struct _RxneieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxneieW<'a> {
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
    pub struct _IdleieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IdleieW<'a> {
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
    pub struct _TeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TeW<'a> {
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
    pub struct _ReW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ReW<'a> {
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
    pub struct _RwuW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RwuW<'a> {
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
    pub struct _SbkW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SbkW<'a> {
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
        fn _over8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Oversampling mode" ]
        pub fn over8(&self) -> Over8R {
            Over8R { bits: self._over8() }
        }
        fn _ue(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - USART enable" ]
        pub fn ue(&self) -> UeR {
            UeR { bits: self._ue() }
        }
        fn _m(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Word length" ]
        pub fn m(&self) -> MR {
            MR { bits: self._m() }
        }
        fn _wake(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Wakeup method" ]
        pub fn wake(&self) -> WakeR {
            WakeR { bits: self._wake() }
        }
        fn _pce(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Parity control enable" ]
        pub fn pce(&self) -> PceR {
            PceR { bits: self._pce() }
        }
        fn _ps(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Parity selection" ]
        pub fn ps(&self) -> PsR {
            PsR { bits: self._ps() }
        }
        fn _peie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - PE interrupt enable" ]
        pub fn peie(&self) -> PeieR {
            PeieR { bits: self._peie() }
        }
        fn _txeie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - TXE interrupt enable" ]
        pub fn txeie(&self) -> TxeieR {
            TxeieR { bits: self._txeie() }
        }
        fn _tcie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transmission complete interrupt enable" ]
        pub fn tcie(&self) -> TcieR {
            TcieR { bits: self._tcie() }
        }
        fn _rxneie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - RXNE interrupt enable" ]
        pub fn rxneie(&self) -> RxneieR {
            RxneieR { bits: self._rxneie() }
        }
        fn _idleie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - IDLE interrupt enable" ]
        pub fn idleie(&self) -> IdleieR {
            IdleieR { bits: self._idleie() }
        }
        fn _te(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Transmitter enable" ]
        pub fn te(&self) -> TeR {
            TeR { bits: self._te() }
        }
        fn _re(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Receiver enable" ]
        pub fn re(&self) -> ReR {
            ReR { bits: self._re() }
        }
        fn _rwu(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Receiver wakeup" ]
        pub fn rwu(&self) -> RwuR {
            RwuR { bits: self._rwu() }
        }
        fn _sbk(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Send break" ]
        pub fn sbk(&self) -> SbkR {
            SbkR { bits: self._sbk() }
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
        # [ doc = "Bit 15 - Oversampling mode" ]
        pub fn over8(&mut self) -> _Over8W {
            _Over8W { register: self }
        }
        # [ doc = "Bit 13 - USART enable" ]
        pub fn ue(&mut self) -> _UeW {
            _UeW { register: self }
        }
        # [ doc = "Bit 12 - Word length" ]
        pub fn m(&mut self) -> _MW {
            _MW { register: self }
        }
        # [ doc = "Bit 11 - Wakeup method" ]
        pub fn wake(&mut self) -> _WakeW {
            _WakeW { register: self }
        }
        # [ doc = "Bit 10 - Parity control enable" ]
        pub fn pce(&mut self) -> _PceW {
            _PceW { register: self }
        }
        # [ doc = "Bit 9 - Parity selection" ]
        pub fn ps(&mut self) -> _PsW {
            _PsW { register: self }
        }
        # [ doc = "Bit 8 - PE interrupt enable" ]
        pub fn peie(&mut self) -> _PeieW {
            _PeieW { register: self }
        }
        # [ doc = "Bit 7 - TXE interrupt enable" ]
        pub fn txeie(&mut self) -> _TxeieW {
            _TxeieW { register: self }
        }
        # [ doc = "Bit 6 - Transmission complete interrupt enable" ]
        pub fn tcie(&mut self) -> _TcieW {
            _TcieW { register: self }
        }
        # [ doc = "Bit 5 - RXNE interrupt enable" ]
        pub fn rxneie(&mut self) -> _RxneieW {
            _RxneieW { register: self }
        }
        # [ doc = "Bit 4 - IDLE interrupt enable" ]
        pub fn idleie(&mut self) -> _IdleieW {
            _IdleieW { register: self }
        }
        # [ doc = "Bit 3 - Transmitter enable" ]
        pub fn te(&mut self) -> _TeW {
            _TeW { register: self }
        }
        # [ doc = "Bit 2 - Receiver enable" ]
        pub fn re(&mut self) -> _ReW {
            _ReW { register: self }
        }
        # [ doc = "Bit 1 - Receiver wakeup" ]
        pub fn rwu(&mut self) -> _RwuW {
            _RwuW { register: self }
        }
        # [ doc = "Bit 0 - Send break" ]
        pub fn sbk(&mut self) -> _SbkW {
            _SbkW { register: self }
        }
    }
}

# [ doc = "Control register 2" ]
# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control register 2" ]
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
    # [ doc = "Value of the field LINEN" ]
    pub struct LinenR {
        bits: u8,
    }
    impl LinenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field STOP" ]
    pub struct StopR {
        bits: u8,
    }
    impl StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CLKEN" ]
    pub struct ClkenR {
        bits: u8,
    }
    impl ClkenR {
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
    # [ doc = "Value of the field LBCL" ]
    pub struct LbclR {
        bits: u8,
    }
    impl LbclR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBDIE" ]
    pub struct LbdieR {
        bits: u8,
    }
    impl LbdieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBDL" ]
    pub struct LbdlR {
        bits: u8,
    }
    impl LbdlR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADD" ]
    pub struct AddR {
        bits: u8,
    }
    impl AddR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LinenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LinenW<'a> {
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
    pub struct _StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _StopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ClkenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ClkenW<'a> {
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
    pub struct _CpolW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CpolW<'a> {
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
    pub struct _CphaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CphaW<'a> {
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
    pub struct _LbclW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbclW<'a> {
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
    pub struct _LbdieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbdieW<'a> {
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
    pub struct _LbdlW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbdlW<'a> {
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
    pub struct _AddW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AddW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
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
        fn _linen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - LIN mode enable" ]
        pub fn linen(&self) -> LinenR {
            LinenR { bits: self._linen() }
        }
        fn _stop(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - STOP bits" ]
        pub fn stop(&self) -> StopR {
            StopR { bits: self._stop() }
        }
        fn _clken(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Clock enable" ]
        pub fn clken(&self) -> ClkenR {
            ClkenR { bits: self._clken() }
        }
        fn _cpol(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Clock polarity" ]
        pub fn cpol(&self) -> CpolR {
            CpolR { bits: self._cpol() }
        }
        fn _cpha(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Clock phase" ]
        pub fn cpha(&self) -> CphaR {
            CphaR { bits: self._cpha() }
        }
        fn _lbcl(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Last bit clock pulse" ]
        pub fn lbcl(&self) -> LbclR {
            LbclR { bits: self._lbcl() }
        }
        fn _lbdie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - LIN break detection interrupt enable" ]
        pub fn lbdie(&self) -> LbdieR {
            LbdieR { bits: self._lbdie() }
        }
        fn _lbdl(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - lin break detection length" ]
        pub fn lbdl(&self) -> LbdlR {
            LbdlR { bits: self._lbdl() }
        }
        fn _add(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - Address of the USART node" ]
        pub fn add(&self) -> AddR {
            AddR { bits: self._add() }
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
        # [ doc = "Bit 14 - LIN mode enable" ]
        pub fn linen(&mut self) -> _LinenW {
            _LinenW { register: self }
        }
        # [ doc = "Bits 12:13 - STOP bits" ]
        pub fn stop(&mut self) -> _StopW {
            _StopW { register: self }
        }
        # [ doc = "Bit 11 - Clock enable" ]
        pub fn clken(&mut self) -> _ClkenW {
            _ClkenW { register: self }
        }
        # [ doc = "Bit 10 - Clock polarity" ]
        pub fn cpol(&mut self) -> _CpolW {
            _CpolW { register: self }
        }
        # [ doc = "Bit 9 - Clock phase" ]
        pub fn cpha(&mut self) -> _CphaW {
            _CphaW { register: self }
        }
        # [ doc = "Bit 8 - Last bit clock pulse" ]
        pub fn lbcl(&mut self) -> _LbclW {
            _LbclW { register: self }
        }
        # [ doc = "Bit 6 - LIN break detection interrupt enable" ]
        pub fn lbdie(&mut self) -> _LbdieW {
            _LbdieW { register: self }
        }
        # [ doc = "Bit 5 - lin break detection length" ]
        pub fn lbdl(&mut self) -> _LbdlW {
            _LbdlW { register: self }
        }
        # [ doc = "Bits 0:3 - Address of the USART node" ]
        pub fn add(&mut self) -> _AddW {
            _AddW { register: self }
        }
    }
}

# [ doc = "Control register 3" ]
# [ repr ( C ) ]
pub struct Cr3 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control register 3" ]
pub mod cr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr3 {
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
    # [ doc = "Value of the field ONEBIT" ]
    pub struct OnebitR {
        bits: u8,
    }
    impl OnebitR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTSIE" ]
    pub struct CtsieR {
        bits: u8,
    }
    impl CtsieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTSE" ]
    pub struct CtseR {
        bits: u8,
    }
    impl CtseR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTSE" ]
    pub struct RtseR {
        bits: u8,
    }
    impl RtseR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMAT" ]
    pub struct DmatR {
        bits: u8,
    }
    impl DmatR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMAR" ]
    pub struct DmarR {
        bits: u8,
    }
    impl DmarR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SCEN" ]
    pub struct ScenR {
        bits: u8,
    }
    impl ScenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NACK" ]
    pub struct NackR {
        bits: u8,
    }
    impl NackR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HDSEL" ]
    pub struct HdselR {
        bits: u8,
    }
    impl HdselR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IRLP" ]
    pub struct IrlpR {
        bits: u8,
    }
    impl IrlpR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IREN" ]
    pub struct IrenR {
        bits: u8,
    }
    impl IrenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field EIE" ]
    pub struct EieR {
        bits: u8,
    }
    impl EieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OnebitW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OnebitW<'a> {
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
    pub struct _CtsieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CtsieW<'a> {
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
    pub struct _CtseW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CtseW<'a> {
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
    pub struct _RtseW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtseW<'a> {
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
    pub struct _DmatW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DmatW<'a> {
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
    pub struct _DmarW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DmarW<'a> {
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
    pub struct _ScenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ScenW<'a> {
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
    pub struct _NackW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NackW<'a> {
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
    pub struct _HdselW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HdselW<'a> {
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
    pub struct _IrlpW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IrlpW<'a> {
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
    pub struct _IrenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IrenW<'a> {
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
    pub struct _EieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EieW<'a> {
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
        fn _onebit(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - One sample bit method enable" ]
        pub fn onebit(&self) -> OnebitR {
            OnebitR { bits: self._onebit() }
        }
        fn _ctsie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - CTS interrupt enable" ]
        pub fn ctsie(&self) -> CtsieR {
            CtsieR { bits: self._ctsie() }
        }
        fn _ctse(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - CTS enable" ]
        pub fn ctse(&self) -> CtseR {
            CtseR { bits: self._ctse() }
        }
        fn _rtse(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - RTS enable" ]
        pub fn rtse(&self) -> RtseR {
            RtseR { bits: self._rtse() }
        }
        fn _dmat(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - DMA enable transmitter" ]
        pub fn dmat(&self) -> DmatR {
            DmatR { bits: self._dmat() }
        }
        fn _dmar(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - DMA enable receiver" ]
        pub fn dmar(&self) -> DmarR {
            DmarR { bits: self._dmar() }
        }
        fn _scen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Smartcard mode enable" ]
        pub fn scen(&self) -> ScenR {
            ScenR { bits: self._scen() }
        }
        fn _nack(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Smartcard NACK enable" ]
        pub fn nack(&self) -> NackR {
            NackR { bits: self._nack() }
        }
        fn _hdsel(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Half-duplex selection" ]
        pub fn hdsel(&self) -> HdselR {
            HdselR { bits: self._hdsel() }
        }
        fn _irlp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - IrDA low-power" ]
        pub fn irlp(&self) -> IrlpR {
            IrlpR { bits: self._irlp() }
        }
        fn _iren(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - IrDA mode enable" ]
        pub fn iren(&self) -> IrenR {
            IrenR { bits: self._iren() }
        }
        fn _eie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Error interrupt enable" ]
        pub fn eie(&self) -> EieR {
            EieR { bits: self._eie() }
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
        # [ doc = "Bit 11 - One sample bit method enable" ]
        pub fn onebit(&mut self) -> _OnebitW {
            _OnebitW { register: self }
        }
        # [ doc = "Bit 10 - CTS interrupt enable" ]
        pub fn ctsie(&mut self) -> _CtsieW {
            _CtsieW { register: self }
        }
        # [ doc = "Bit 9 - CTS enable" ]
        pub fn ctse(&mut self) -> _CtseW {
            _CtseW { register: self }
        }
        # [ doc = "Bit 8 - RTS enable" ]
        pub fn rtse(&mut self) -> _RtseW {
            _RtseW { register: self }
        }
        # [ doc = "Bit 7 - DMA enable transmitter" ]
        pub fn dmat(&mut self) -> _DmatW {
            _DmatW { register: self }
        }
        # [ doc = "Bit 6 - DMA enable receiver" ]
        pub fn dmar(&mut self) -> _DmarW {
            _DmarW { register: self }
        }
        # [ doc = "Bit 5 - Smartcard mode enable" ]
        pub fn scen(&mut self) -> _ScenW {
            _ScenW { register: self }
        }
        # [ doc = "Bit 4 - Smartcard NACK enable" ]
        pub fn nack(&mut self) -> _NackW {
            _NackW { register: self }
        }
        # [ doc = "Bit 3 - Half-duplex selection" ]
        pub fn hdsel(&mut self) -> _HdselW {
            _HdselW { register: self }
        }
        # [ doc = "Bit 2 - IrDA low-power" ]
        pub fn irlp(&mut self) -> _IrlpW {
            _IrlpW { register: self }
        }
        # [ doc = "Bit 1 - IrDA mode enable" ]
        pub fn iren(&mut self) -> _IrenW {
            _IrenW { register: self }
        }
        # [ doc = "Bit 0 - Error interrupt enable" ]
        pub fn eie(&mut self) -> _EieW {
            _EieW { register: self }
        }
    }
}

# [ doc = "Guard time and prescaler register" ]
# [ repr ( C ) ]
pub struct Gtpr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Guard time and prescaler register" ]
pub mod gtpr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Gtpr {
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
    # [ doc = "Value of the field GT" ]
    pub struct GtR {
        bits: u8,
    }
    impl GtR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PSC" ]
    pub struct PscR {
        bits: u8,
    }
    impl PscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _GtW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GtW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PscW<'a> {
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
        fn _gt(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:15 - Guard time value" ]
        pub fn gt(&self) -> GtR {
            GtR { bits: self._gt() }
        }
        fn _psc(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - Prescaler value" ]
        pub fn psc(&self) -> PscR {
            PscR { bits: self._psc() }
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
        # [ doc = "Bits 8:15 - Guard time value" ]
        pub fn gt(&mut self) -> _GtW {
            _GtW { register: self }
        }
        # [ doc = "Bits 0:7 - Prescaler value" ]
        pub fn psc(&mut self) -> _PscW {
            _PscW { register: self }
        }
    }
}
