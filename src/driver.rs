pub trait Driver {
    fn read_msg(&mut self) -> crate::error::Result<()>;
}

pub struct SerialDriver<D>
where
    D: std::io::Read + std::io::Write,
{
    device: D,
}

impl<D> SerialDriver<D>
where
    D: std::io::Read + std::io::Write,
{
    /// Create a new serial driver based on the given stream
    pub fn new(device: D) -> Self {
        SerialDriver { device }
    }

    /// Read a single byte from the stream and retries the amount of times as specified
    fn read_byte(&mut self, timeout: Option<usize>) -> crate::error::Result<u8> {
        // buffer to read the byte in
        let mut buffer = [0u8; 1];

        // request the byte read
        match self.device.read_exact(&mut buffer) {
            // on success return the byte
            Ok(_) => Ok(buffer[0]),
            // on error
            Err(e) => {
                // we check if there was a timeout
                if e.kind() == std::io::ErrorKind::TimedOut {
                    // when yes, we calculate the new timeout
                    if let Some(t) = timeout {
                        let new_timeout = t - 1;

                        // when there are still timeout time left, retry
                        if new_timeout > 0 {
                            return self.read_byte(Some(new_timeout));
                        }
                    }
                }

                // if an error occoured or no timeouts are left, stop trying
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestDeviceMode {
        Timeout(isize),
    }

    struct TestDevice {
        mode: TestDeviceMode,
    }

    impl TestDevice {
        fn new(mode: TestDeviceMode) -> Self {
            TestDevice { mode }
        }
    }

    impl std::io::Read for TestDevice {
        fn read(&mut self, inp: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
            match self.mode {
                // on timeout we always timeout and count one down
                // when we reach zero, we sent the value 0xFF
                TestDeviceMode::Timeout(t) => {
                    let t = t - 1;
                    self.mode = TestDeviceMode::Timeout(t);

                    if t == 0 {
                        inp[0] = 0xFF;
                        Ok(1)
                    } else {
                        Err(std::io::Error::from(std::io::ErrorKind::TimedOut))
                    }
                }
            }
        }
    }

    impl std::io::Write for TestDevice {
        fn write(&mut self, _inp: &[u8]) -> std::result::Result<usize, std::io::Error> {
            Ok(0)
        }

        fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[test]
    fn test_new() {
        let device = std::io::Cursor::new(Vec::new());
        SerialDriver::new(device);
    }

    #[test]
    fn test_timeout_read_byte() {
        // timeout error to compare against
        let timeout_error = crate::error::Error::new(
            crate::error::ErrorKind::Io(std::io::ErrorKind::TimedOut),
            "timed out",
        );

        // Test Device to generate data for the test
        let device = TestDevice::new(TestDeviceMode::Timeout(0));
        // generate a driver for the test device
        let mut driver = SerialDriver::new(device);

        // check if we can timeout
        assert_eq!(driver.read_byte(Some(1600)), Err(timeout_error));
    }

    #[test]
    fn test_delayed_read_byte() {
        // Test Device to generate data for the test
        let device = TestDevice::new(TestDeviceMode::Timeout(5));
        // generate a driver for the test device
        let mut driver = SerialDriver::new(device);

        // check if we can timeout
        assert_eq!(driver.read_byte(Some(16)), Ok(0xFF));
    }
}
