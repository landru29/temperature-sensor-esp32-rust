use embedded_io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::Infallible;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::delay::BLOCK;

// The output interface: data is written to the UART through an Rc<RefCell<>>
// shared with the read loop in `main`
pub struct UartIo<'d> {
    pub driver: Rc<RefCell<UartDriver<'d>>>,
}

impl<'d> embedded_io::ErrorType for UartIo<'d> {
    type Error = Infallible;
}

impl<'d> Write for UartIo<'d> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let n = self.driver.borrow_mut().write(buf).unwrap();
        Ok(n)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // Wait until the hardware TX buffer is empty
        let _ = self.driver.borrow_mut().wait_tx_done(BLOCK);
        Ok(())
    }
}