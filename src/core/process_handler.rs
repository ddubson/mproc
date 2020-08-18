use crate::core::process_output_handler::ProcessOutputHandler;
use duct::ReaderHandle;

pub struct ProcessHandler {
    pub reader_handle: ReaderHandle,
    pub output_handler: ProcessOutputHandler,
}
