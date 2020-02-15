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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let device = std::io::Cursor::new(Vec::new());
        SerialDriver::new(device);
    }
}
