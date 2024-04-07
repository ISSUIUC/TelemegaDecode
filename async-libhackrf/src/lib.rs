//! HackRF One API.
//!
//! To get started take a look at [`HackRfOne::new`].
#![warn(missing_docs)]

use std::fmt::{Debug, Formatter};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use nusb::transfer::{Completion, Control, ControlType, Direction, Recipient, RequestBuffer};
use smol::{block_on, channel};
use smol::future::FutureExt;


/// HackRF USB vendor ID.
const HACKRF_USB_VID: u16 = 0x1D50;
/// HackRF One USB product ID.
const HACKRF_ONE_USB_PID: u16 = 0x6089;

#[allow(dead_code)]
#[repr(u8)]
enum Request {
    SetTransceiverMode = 1,
    Max2837Write = 2,
    Max2837Read = 3,
    Si5351CWrite = 4,
    Si5351CRead = 5,
    SampleRateSet = 6,
    BasebandFilterBandwidthSet = 7,
    Rffc5071Write = 8,
    Rffc5071Read = 9,
    SpiflashErase = 10,
    SpiflashWrite = 11,
    SpiflashRead = 12,
    BoardIdRead = 14,
    VersionStringRead = 15,
    SetFreq = 16,
    AmpEnable = 17,
    BoardPartidSerialnoRead = 18,
    SetLnaGain = 19,
    SetVgaGain = 20,
    SetTxvgaGain = 21,
    AntennaEnable = 23,
    SetFreqExplicit = 24,
    UsbWcidVendorReq = 25,
    InitSweep = 26,
    OperacakeGetBoards = 27,
    OperacakeSetPorts = 28,
    SetHwSyncMode = 29,
    Reset = 30,
    OperacakeSetRanges = 31,
    ClkoutEnable = 32,
    SpiflashStatus = 33,
    SpiflashClearStatus = 34,
    OperacakeGpioTest = 35,
    CpldChecksum = 36,
    UiEnable = 37,
}

impl From<Request> for u8 {
    fn from(r: Request) -> Self {
        r as u8
    }
}

#[allow(dead_code)]
#[repr(u8)]
enum TranscieverMode {
    Off = 0,
    Receive = 1,
    Transmit = 2,
    Ss = 3,
    CpldUpdate = 4,
    RxSweep = 5,
}

impl From<TranscieverMode> for u8 {
    fn from(tm: TranscieverMode) -> Self {
        tm as u8
    }
}

impl From<TranscieverMode> for u16 {
    fn from(tm: TranscieverMode) -> Self {
        tm as u16
    }
}

/// HackRF One errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// USB error.
    Transfer(nusb::transfer::TransferError),
    /// Failed to transfer all bytes in a control transfer.
    CtrlTransfer {
        /// Control transfer direction.
        dir: Direction,
        /// Actual amount of bytes transferred.
        actual: usize,
        /// Excepted number of bytes transferred.
        expected: usize,
    },
    /// An API call is not supported by your hardware.
    ///
    /// Try updating the firmware on your device.
    Version {
        /// Current device version.
        device: Version,
        /// Minimum version required.
        min: Version,
    },
    /// A provided argument was out of range.
    Argument,
}

// impl From<nusb::Error> for Error {
//     fn from(e: nusb::Error) -> Self {
//         Error::Usb(e)
//     }
// }

impl From<nusb::transfer::TransferError> for Error {
    fn from(e: nusb::transfer::TransferError) -> Self {
        Error::Transfer(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Version(pub u8, pub u8, pub u8);

impl Version {
    fn from_bcd(bcd: u16) -> Version {
        bcd.into()
    }

    pub fn major(&self) -> u8 { self.0 }
    pub fn minor(&self) -> u8 { self.1 }
    pub fn sub_minor(&self) -> u8 { self.2 }
}

impl From<u16> for Version {
    fn from(value: u16) -> Self {
        Version(((value & 0xFF00) >> 8) as u8, ((value & 0x00F0) >> 4) as u8, (value & 0x000F) as u8)
    }
}

/// Typestate for RX mode.
pub struct RxMode(mpsc::Receiver<Completion<Vec<u8>>>, channel::Sender<()>, thread::JoinHandle<()>);

impl Debug for RxMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RxMode")
            .finish_non_exhaustive()
    }
}

/// Typestate for an unknown mode.
#[derive(Debug)]
pub struct UnknownMode;

/// HackRF One software defined radio.
pub struct HackRfOne<MODE> {
    dh: nusb::Device,
    interface: nusb::Interface,
    desc: nusb::DeviceInfo,
    #[allow(dead_code)]
    mode: MODE,
    to: Duration,
}

impl HackRfOne<UnknownMode> {
    /// Open a new HackRF One.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// ```
    pub fn new() -> Option<HackRfOne<UnknownMode>> {
        let devices = nusb::list_devices().ok()?;

        for device in devices {
            if device.vendor_id() == HACKRF_USB_VID && device.product_id() == HACKRF_ONE_USB_PID {
                match device.open() {
                    Ok(handle) => {
                        return Some(HackRfOne {
                            interface: handle.claim_interface(0).expect("Could not claim interface 0???"),
                            dh: handle,
                            desc: device,
                            mode: UnknownMode,
                            to: Duration::from_secs(1),
                        })
                    }
                    Err(_) => continue,
                }
            }
        }

        None
    }
}

impl<MODE> HackRfOne<MODE> {
    fn read_control<const N: usize>(
        &self,
        request: Request,
        value: u16,
        index: u16,
    ) -> Result<[u8; N], Error> {
        let mut buf: [u8; N] = [0; N];

        let n: usize = self.interface.control_in_blocking(Control {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: request.into(),
            value, index
        }, &mut buf, self.to)?;
        if n != buf.len() {
            Err(Error::CtrlTransfer {
                dir: Direction::In,
                actual: n,
                expected: buf.len(),
            })
        } else {
            Ok(buf)
        }
    }

    fn write_control(
        &mut self,
        request: Request,
        value: u16,
        index: u16,
        buf: &[u8],
    ) -> Result<(), Error> {
        let n: usize = self.interface.control_out_blocking(Control {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: request.into(),
            value, index
        }, buf, self.to)?;
        if n != buf.len() {
            Err(Error::CtrlTransfer {
                dir: Direction::Out,
                actual: n,
                expected: buf.len(),
            })
        } else {
            Ok(())
        }
    }

    fn check_api_version(&self, min: Version) -> Result<(), Error> {
        fn version_to_u32(v: Version) -> u32 {
            ((v.major() as u32) << 16) | ((v.minor() as u32) << 8) | (v.sub_minor() as u32)
        }

        let v: Version = self.device_version();
        let v_cmp: u32 = version_to_u32(v);
        let min_cmp: u32 = version_to_u32(min);

        if v_cmp >= min_cmp {
            Ok(())
        } else {
            Err(Error::Version { device: v, min })
        }
    }

    /// Get the device version from the USB descriptor.
    ///
    /// The HackRF C API calls the equivalent of this function
    /// `hackrf_usb_api_version_read`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{Version, HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// assert_eq!(radio.device_version(), Version(1, 0, 4));
    /// ```
    pub fn device_version(&self) -> Version {
        self.desc.device_version().into()
    }

    /// Set the timeout for USB transfers.
    ///
    /// # Example
    ///
    /// Set a 100ms timeout.
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    /// use std::time::Duration;
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_timeout(Duration::from_millis(100))
    /// ```
    pub fn set_timeout(&mut self, duration: Duration) {
        self.to = duration;
    }

    /// Read the board ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// assert_eq!(radio.board_id()?, 0x02);
    /// ```
    pub fn board_id(&self) -> Result<u8, Error> {
        let data: [u8; 1] = self.read_control(Request::BoardIdRead, 0, 0)?;
        Ok(data[0])
    }

    /// Read the firmware version.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// assert_eq!(radio.version()?, "2021.03.1");
    /// ```
    pub fn version(&self) -> Result<String, Error> {
        let mut buf: [u8; 16] = [0; 16];
        let n: usize = self.interface.control_in_blocking(Control {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: Request::VersionStringRead.into(),
            value: 0, index: 0
        }, &mut buf, self.to)?;
        Ok(String::from_utf8_lossy(&buf[0..n]).into())
    }

    /// Set the center frequency.
    ///
    /// # Example
    ///
    /// Set the frequency to 915MHz.
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_freq(915_000_000)?;
    /// ```
    pub fn set_freq(&mut self, hz: u64) -> Result<(), Error> {
        let buf: [u8; 8] = freq_params(hz);
        self.write_control(Request::SetFreq, 0, 0, &buf)
    }

    /// Enable the RX/TX RF amplifier.
    ///
    /// In GNU radio this is used as the RF gain, where a value of 0 dB is off,
    /// and a value of 14 dB is on.
    ///
    /// # Example
    ///
    /// Disable the amplifier.
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_amp_enable(false)?;
    /// ```
    pub fn set_amp_enable(&mut self, en: bool) -> Result<(), Error> {
        self.write_control(Request::AmpEnable, en.into(), 0, &[])
    }

    /// Set the baseband filter bandwidth.
    ///
    /// This is automatically set when the sample rate is changed with
    /// [`set_sample_rate`].
    ///
    /// # Example
    ///
    /// Set the filter bandwidth to 70% of the sample rate.
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// const SAMPLE_HZ: u32 = 20_000_000;
    /// const SAMPLE_DIV: u32 = 2;
    /// const FILTER_BW: u32 = (0.7 * (SAMPLE_HZ as f32) / (SAMPLE_DIV as f32)) as u32;
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_sample_rate(SAMPLE_HZ, SAMPLE_DIV)?;
    /// radio.set_baseband_filter_bandwidth(FILTER_BW)?;
    /// ```
    ///
    /// [`set_sample_rate`]: crate::HackRfOne::set_sample_rate
    pub fn set_baseband_filter_bandwidth(&mut self, hz: u32) -> Result<(), Error> {
        self.write_control(
            Request::BasebandFilterBandwidthSet,
            (hz & 0xFFFF) as u16,
            (hz >> 16) as u16,
            &[],
        )
    }

    /// Set the sample rate.
    ///
    /// For anti-aliasing, the baseband filter bandwidth is automatically set to
    /// the widest available setting that is no more than 75% of the sample rate.
    /// This happens every time the sample rate is set.
    /// If you want to override the baseband filter selection, you must do so
    /// after setting the sample rate.
    ///
    /// Limits are 8MHz - 20MHz.
    /// Preferred rates are 8, 10, 12.5, 16, 20MHz due to less jitter.
    ///
    /// # Example
    ///
    /// Set the sample rate to 10 MHz.
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_sample_rate(20_000_000, 2)?;
    /// ```
    pub fn set_sample_rate(&mut self, hz: u32, div: u32) -> Result<(), Error> {
        let hz: u32 = hz.to_le();
        let div: u32 = div.to_le();
        let buf: [u8; 8] = [
            (hz & 0xFF) as u8,
            ((hz >> 8) & 0xFF) as u8,
            ((hz >> 16) & 0xFF) as u8,
            ((hz >> 24) & 0xFF) as u8,
            (div & 0xFF) as u8,
            ((div >> 8) & 0xFF) as u8,
            ((div >> 16) & 0xFF) as u8,
            ((div >> 24) & 0xFF) as u8,
        ];
        self.write_control(Request::SampleRateSet, 0, 0, &buf)?;
        self.set_baseband_filter_bandwidth((0.75 * (hz as f32) / (div as f32)) as u32)
    }

    /// Set the LNA (low noise amplifier) gain.
    ///
    /// Range 0 to 40dB in 8dB steps.
    ///
    /// This is also known as the IF gain.
    ///
    /// # Example
    ///
    /// Set the LNA gain to 16 dB (generally a reasonable gain to start with).
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_lna_gain(16)?;
    /// ```
    pub fn set_lna_gain(&mut self, gain: u16) -> Result<(), Error> {
        if gain > 40 {
            Err(Error::Argument)
        } else {
            let buf: [u8; 1] = self.read_control(Request::SetLnaGain, 0, gain & !0x07)?;
            if buf[0] == 0 {
                Err(Error::Argument)
            } else {
                Ok(())
            }
        }
    }

    /// Set the VGA (variable gain amplifier) gain.
    ///
    /// Range 0 to 62dB in 2dB steps.
    ///
    /// This is also known as the baseband (BB) gain.
    ///
    /// # Example
    ///
    /// Set the VGA gain to 16 dB (generally a reasonable gain to start with).
    ///
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// radio.set_vga_gain(16)?;
    /// ```
    pub fn set_vga_gain(&mut self, gain: u16) -> Result<(), Error> {
        if gain > 62 {
            Err(Error::Argument)
        } else {
            let buf: [u8; 1] = self.read_control(Request::SetVgaGain, 0, gain & !0b1)?;
            if buf[0] == 0 {
                Err(Error::Argument)
            } else {
                Ok(())
            }
        }
    }

    /// Set the transmit VGA gain.
    ///
    /// Range 0 to 47dB in 1db steps.
    pub fn set_txvga_gain(&mut self, gain: u16) -> Result<(), Error> {
        if gain > 47 {
            Err(Error::Argument)
        } else {
            let buf: [u8; 1] = self.read_control(Request::SetTxvgaGain, 0, gain)?;
            if buf[0] == 0 {
                Err(Error::Argument)
            } else {
                Ok(())
            }
        }
    }

    /// Antenna power port control.
    ///
    /// The source docs are a little lacking in terms of explanations here.
    pub fn set_antenna_enable(&mut self, value: u8) -> Result<(), Error> {
        self.write_control(Request::AntennaEnable, value.into(), 0, &[])
    }

    /// CLKOUT enable.
    ///
    /// The source docs are a little lacking in terms of explanations here.
    pub fn set_clkout_enable(&mut self, en: bool) -> Result<(), Error> {
        self.check_api_version(Version::from_bcd(0x0103))?;
        self.write_control(Request::ClkoutEnable, en.into(), 0, &[])
    }

    /// Reset the HackRF radio.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// let mut radio: HackRfOne<UnknownMode> = radio.reset()?;
    /// ```
    pub fn reset(mut self) -> Result<HackRfOne<UnknownMode>, Error> {
        self.check_api_version(Version::from_bcd(0x0102))?;
        self.write_control(Request::Reset, 0, 0, &[])?;
        Ok(HackRfOne {
            dh: self.dh,
            desc: self.desc,
            interface: self.interface,
            mode: UnknownMode,
            to: self.to,
        })
    }

    fn set_transceiver_mode(&mut self, mode: TranscieverMode) -> Result<(), Error> {
        self.write_control(Request::SetTransceiverMode, mode.into(), 0, &[])
    }

    /// Change the radio mode to RX.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, RxMode, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// let mut radio: HackRfOne<RxMode> = radio.into_rx_mode()?;
    /// ```
    pub fn into_rx_mode(mut self) -> Result<HackRfOne<RxMode>, Error> {
        self.set_transceiver_mode(TranscieverMode::Receive)?;
        let mut queue = self.interface.bulk_in_queue(0x81);
        for _ in 0..4 {
            queue.submit(RequestBuffer::new(262144));
        }
        let (tx, rx) = mpsc::channel();
        let (cancel_tx, cancel_rx) = channel::bounded::<()>(1);
        let handle = thread::spawn(move || {
            loop {
                let res = block_on(async {
                    cancel_rx.recv().await.unwrap();
                    None
                }.or(async {
                    Some(queue.next_complete().await)
                }));

                let Some(res) = res else { return; };

                match tx.send(res) {
                    Ok(_) => {
                        queue.submit(RequestBuffer::new(262144));
                    },
                    Err(_) => return
                }
            }
        });
        Ok(HackRfOne {
            dh: self.dh,
            desc: self.desc,
            interface: self.interface,
            mode: RxMode(rx, cancel_tx, handle),
            to: self.to,
        })
    }
}

impl HackRfOne<RxMode> {
    /// Receive data from the radio.
    ///
    /// The data format is pairs of signed 8-bit IQ. Use the [`iq_to_cplx_i8`] or
    /// [`iq_to_cplx_f32`] helpers to convert the
    /// data to a more manageable format.
    #[cfg_attr(not(feature = "num-complex"), allow(rustdoc::broken_intra_doc_links))]
    pub fn rx(&mut self) -> Result<Vec<u8>, Error> {
        let stuff = self.mode.0.recv().unwrap();
        stuff.into_result().map_err(|e| e.into())
    }

    /// Stop receiving.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_libhackrf::{HackRfOne, RxMode, UnknownMode};
    ///
    /// let mut radio: HackRfOne<UnknownMode> = HackRfOne::new().unwrap();
    /// let mut radio: HackRfOne<RxMode> = radio.into_rx_mode()?;
    /// radio.stop_rx()?;
    /// ```
    pub fn stop_rx(mut self) -> Result<HackRfOne<UnknownMode>, Error> {
        self.set_transceiver_mode(TranscieverMode::Off)?;
        let RxMode(rx, cancel_tx, handle) = self.mode;
        cancel_tx.send_blocking(()).unwrap();
        drop(rx);
        handle.join().unwrap();
        Ok(HackRfOne {
            dh: self.dh,
            desc: self.desc,
            interface: self.interface,
            mode: UnknownMode,
            to: self.to,
        })
    }
}

// Helper for set_freq
fn freq_params(hz: u64) -> [u8; 8] {
    const MHZ: u64 = 1_000_000;

    let l_freq_mhz: u32 = u32::try_from(hz / MHZ).unwrap_or(u32::MAX).to_le();
    let l_freq_hz: u32 = u32::try_from(hz - u64::from(l_freq_mhz) * MHZ)
        .unwrap_or(u32::MAX)
        .to_le();

    [
        (l_freq_mhz & 0xFF) as u8,
        ((l_freq_mhz >> 8) & 0xFF) as u8,
        ((l_freq_mhz >> 16) & 0xFF) as u8,
        ((l_freq_mhz >> 24) & 0xFF) as u8,
        (l_freq_hz & 0xFF) as u8,
        ((l_freq_hz >> 8) & 0xFF) as u8,
        ((l_freq_hz >> 16) & 0xFF) as u8,
        ((l_freq_hz >> 24) & 0xFF) as u8,
    ]
}
