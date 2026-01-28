use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead};
use std::io::Error;
use crate::core::shared_memory::MEMORY_TRACKER;
use crate::constants::Constants;

pub struct TunedStream<R> {
    reader: R,
    current_buffer_size: usize,
    max_buffer_size: usize,
    is_auto: bool,
    total_tracked_size: usize,
}

impl<R: AsyncRead + Unpin> TunedStream<R> {
    pub fn new(reader: R, initial_size: usize, max_size: usize, is_auto: bool) -> Self {
        Self {
            reader,
            current_buffer_size: initial_size,
            max_buffer_size: max_size,
            is_auto,
            total_tracked_size: initial_size,
        }
    }
}

impl<R: AsyncRead + Unpin> Stream for TunedStream<R> {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = vec![0u8; self.current_buffer_size];
        let mut read_buf = tokio::io::ReadBuf::new(&mut buf);
        
        match Pin::new(&mut self.reader).poll_read(cx, &mut read_buf) {
            Poll::Ready(Ok(())) => {
                let filled = read_buf.filled().len();
                if filled == 0 {
                    return Poll::Ready(None);
                }

                let this = self.get_mut();

                // Tier 1: Dynamic Auto-Sizing logic
                if this.is_auto && filled == this.current_buffer_size && this.current_buffer_size < this.max_buffer_size {
                    let old_size = this.current_buffer_size;
                    this.current_buffer_size = (this.current_buffer_size * 2).min(this.max_buffer_size);
                    
                    // Track additional memory
                    let growth = (this.current_buffer_size - old_size) as u64;
                    if MEMORY_TRACKER.read().unwrap().track_allocation(growth) {
                        this.total_tracked_size += growth as usize;
                    } else {
                        // If cap reached, stay at old size
                        this.current_buffer_size = old_size;
                    }
                } else if this.is_auto && filled < this.current_buffer_size / 2 && this.current_buffer_size > Constants::DEFAULT_AUTO_BUFFER_START {
                    // Shrink back if load is low
                    let old_size = this.current_buffer_size;
                    this.current_buffer_size = (this.current_buffer_size / 2).max(Constants::DEFAULT_AUTO_BUFFER_START);
                    
                    let shrink = (old_size - this.current_buffer_size) as u64;
                    MEMORY_TRACKER.read().unwrap().track_deallocation(shrink);
                    this.total_tracked_size -= shrink as usize;
                }

                Poll::Ready(Some(Ok(Bytes::copy_from_slice(read_buf.filled()))))
            }
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
        }
    }
}

impl<R> Drop for TunedStream<R> {
    fn drop(&mut self) {
        MEMORY_TRACKER.read().unwrap().track_deallocation(self.total_tracked_size as u64);
    }
}
