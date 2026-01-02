use anyhow::Result;
use futures::{Stream, StreamExt, TryStreamExt};
use std::pin::Pin;
use tokio::sync::mpsc;

pub struct StreamingCompressor {
    algorithm: String,
    chunk_size: usize,
    buffer_size: usize,
}

impl StreamingCompressor {
    pub fn new(algorithm: String) -> Self {
        Self {
            algorithm,
            chunk_size: 64 * 1024,    // 64KB chunks
            buffer_size: 1024 * 1024, // 1MB buffer
        }
    }

    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn compress_stream(
        &self,
        input_stream: impl Stream<Item = Vec<u8>> + Send + 'static,
    ) -> impl Stream<Item = Result<Vec<u8>>> {
        let (tx, rx) = mpsc::channel(100);
        let algorithm = self.algorithm.clone();
        let chunk_size = self.chunk_size;

        tokio::spawn(async move {
            let mut compressor = match algorithm.as_str() {
                "zstd" => StreamingAlgorithm::Zstd(StreamingZstd::new()),
                "lz4" => StreamingAlgorithm::Lz4(StreamingLz4::new()),
                _ => {
                    let _ = tx
                        .send(Err(anyhow::anyhow!(
                            "Unsupported streaming algorithm: {}",
                            algorithm
                        )))
                        .await;
                    return;
                }
            };

            let mut buffer = Vec::new();

            futures::pin_mut!(input_stream);

            while let Some(chunk) = input_stream.next().await {
                buffer.extend_from_slice(&chunk);

                if buffer.len() >= chunk_size {
                    match compressor.compress_chunk(&buffer) {
                        Ok(compressed) => {
                            if let Err(_) = tx.send(Ok(compressed)).await {
                                break;
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            break;
                        }
                    }
                    buffer.clear();
                }
            }

            // Compress any remaining data
            if !buffer.is_empty() {
                match compressor.finalize() {
                    Ok(final_chunk) => {
                        if !final_chunk.is_empty() {
                            let _ = tx.send(Ok(final_chunk)).await;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                    }
                }
            }
        });

        tokio_stream::wrappers::ReceiverStream::new(rx)
    }

    pub fn decompress_stream(
        &self,
        input_stream: impl Stream<Item = Result<Vec<u8>>> + Send + 'static,
    ) -> impl Stream<Item = Result<Vec<u8>>> {
        let (tx, rx) = mpsc::channel(100);
        let algorithm = self.algorithm.clone();

        tokio::spawn(async move {
            let mut decompressor = match algorithm.as_str() {
                "zstd" => StreamingAlgorithm::Zstd(StreamingZstd::new()),
                "lz4" => StreamingAlgorithm::Lz4(StreamingLz4::new()),
                _ => {
                    let _ = tx
                        .send(Err(anyhow::anyhow!(
                            "Unsupported streaming algorithm: {}",
                            algorithm
                        )))
                        .await;
                    return;
                }
            };

            futures::pin_mut!(input_stream);

            while let Some(result) = input_stream.next().await {
                match result {
                    Ok(chunk) => match decompressor.decompress_chunk(&chunk) {
                        Ok(decompressed) => {
                            if !decompressed.is_empty() {
                                if let Err(_) = tx.send(Ok(decompressed)).await {
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            break;
                        }
                    },
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        break;
                    }
                }
            }

            // Finalize decompression
            match decompressor.finalize() {
                Ok(final_chunk) => {
                    if !final_chunk.is_empty() {
                        let _ = tx.send(Ok(final_chunk)).await;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                }
            }
        });

        tokio_stream::wrappers::ReceiverStream::new(rx)
    }
}

enum StreamingAlgorithm {
    Zstd(StreamingZstd),
    Lz4(StreamingLz4),
}

impl StreamingAlgorithm {
    fn compress_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            StreamingAlgorithm::Zstd(zstd) => zstd.compress_chunk(data),
            StreamingAlgorithm::Lz4(lz4) => lz4.compress_chunk(data),
        }
    }

    fn decompress_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        match self {
            StreamingAlgorithm::Zstd(zstd) => zstd.decompress_chunk(data),
            StreamingAlgorithm::Lz4(lz4) => lz4.decompress_chunk(data),
        }
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        match self {
            StreamingAlgorithm::Zstd(zstd) => zstd.finalize(),
            StreamingAlgorithm::Lz4(lz4) => lz4.finalize(),
        }
    }
}

struct StreamingZstd {
    encoder: Option<zstd::stream::write::Encoder<'static, Vec<u8>>>,
    decoder:
        Option<zstd::stream::read::Decoder<'static, std::io::BufReader<std::io::Cursor<Vec<u8>>>>>,
}

impl StreamingZstd {
    fn new() -> Self {
        Self {
            encoder: None,
            decoder: None,
        }
    }

    fn compress_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        if self.encoder.is_none() {
            let encoder = zstd::stream::write::Encoder::new(Vec::new(), 3)?;
            self.encoder = Some(encoder);
        }

        if let Some(ref mut encoder) = self.encoder {
            use std::io::Write;
            encoder.write_all(data)?;

            // Flush to get compressed data
            let mut buffer = Vec::new();
            std::mem::swap(&mut buffer, encoder.get_mut());
            Ok(buffer)
        } else {
            Err(anyhow::anyhow!("Encoder not initialized"))
        }
    }

    fn decompress_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        if self.decoder.is_none() {
            let cursor = std::io::Cursor::new(Vec::new());
            let decoder = zstd::stream::read::Decoder::new(cursor)?;
            self.decoder = Some(decoder);
        }

        if let Some(ref mut decoder) = self.decoder {
            // Create a new cursor with the data
            let mut cursor = std::io::Cursor::new(data.to_vec());
            let mut decompressed = Vec::new();
            std::io::Read::read_to_end(&mut cursor, &mut decompressed)?;
            Ok(decompressed)
        } else {
            Err(anyhow::anyhow!("Decoder not initialized"))
        }
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        if let Some(encoder) = self.encoder.take() {
            use std::io::Write;
            let result = encoder.finish()?;
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
}

struct StreamingLz4 {
    buffer: Vec<u8>,
}

impl StreamingLz4 {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn compress_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.buffer.extend_from_slice(data);

        if self.buffer.len() >= 64 * 1024 {
            // 64KB chunks
            let compressed = lz4_flex::compress_prepend_size(&self.buffer);
            self.buffer.clear();
            Ok(compressed)
        } else {
            Ok(Vec::new()) // Buffer not full yet
        }
    }

    fn decompress_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        lz4_flex::decompress_size_prepended(data)
            .map_err(|e| anyhow::anyhow!("LZ4 decompression failed: {:?}", e))
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        if !self.buffer.is_empty() {
            let compressed = lz4_flex::compress_prepend_size(&self.buffer);
            self.buffer.clear();
            Ok(compressed)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_streaming_compression() {
        let test_data = vec![
            b"This is the first chunk of data. ".to_vec(),
            b"This is the second chunk of data. ".to_vec(),
            b"This is the third chunk of data.".to_vec(),
        ];

        let compressor = StreamingCompressor::new("zstd".to_string()).with_chunk_size(32); // Small chunk size for testing

        let input_stream = stream::iter(test_data.clone());
        let compressed_stream = compressor.compress_stream(input_stream);

        let compressed_chunks: Vec<Result<Vec<u8>>> = compressed_stream.collect().await;

        // At least some chunks should be produced
        assert!(!compressed_chunks.is_empty());

        // Test decompression
        let decompressed_stream = compressor.decompress_stream(stream::iter(compressed_chunks));
        let decompressed_chunks: Vec<Result<Vec<u8>>> = decompressed_stream.collect().await;

        let decompressed_data: Vec<u8> = decompressed_chunks
            .into_iter()
            .filter_map(|result| result.ok())
            .flatten()
            .collect();

        let original_data: Vec<u8> = test_data.into_iter().flatten().collect();
        assert_eq!(decompressed_data, original_data);
    }

    #[tokio::test]
    async fn test_lz4_streaming() {
        let test_data = vec![
            b"Chunk 1 ".to_vec(),
            b"Chunk 2 ".to_vec(),
            b"Chunk 3".to_vec(),
        ];

        let compressor = StreamingCompressor::new("lz4".to_string());
        let input_stream = stream::iter(test_data.clone());
        let compressed_stream = compressor.compress_stream(input_stream);

        let compressed_chunks: Vec<Result<Vec<u8>>> = compressed_stream.collect().await;

        // Verify we get compressed data
        assert!(!compressed_chunks.is_empty());

        // Test decompression
        let decompressed_stream = compressor.decompress_stream(stream::iter(compressed_chunks));
        let decompressed_chunks: Vec<Result<Vec<u8>>> = decompressed_stream.collect().await;

        let decompressed_data: Vec<u8> = decompressed_chunks
            .into_iter()
            .filter_map(|result| result.ok())
            .flatten()
            .collect();

        let original_data: Vec<u8> = test_data.into_iter().flatten().collect();
        assert_eq!(decompressed_data, original_data);
    }
}
