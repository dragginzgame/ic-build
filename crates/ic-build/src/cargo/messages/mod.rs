//! Shared Cargo message decoding and exact package/target matching.
use super::observation::schema;
use std::io;
pub fn artifact_matches(
    artifact: &schema::Artifact,
    package: &str,
    name: &str,
    kind: &schema::TargetKind,
) -> bool {
    artifact.package_id.repr == package
        && artifact.target.name == name
        && artifact.target.kind.contains(kind)
}
pub struct MessageDecoder {
    pending: Vec<u8>,
    maximum: usize,
    allow_text: bool,
}
impl MessageDecoder {
    pub const fn new(maximum: usize, allow_text: bool) -> Self {
        Self {
            pending: Vec::new(),
            maximum,
            allow_text,
        }
    }
    pub fn feed(
        &mut self,
        bytes: &[u8],
        mut emit: impl FnMut(schema::Message) -> io::Result<()>,
    ) -> io::Result<()> {
        for fragment in bytes.split_inclusive(|byte| *byte == b'\n') {
            if fragment.len() > self.maximum.saturating_sub(self.pending.len()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Cargo JSON message exceeds caller allowance",
                ));
            }
            self.pending
                .try_reserve(fragment.len())
                .map_err(io::Error::other)?;
            self.pending.extend_from_slice(fragment);
            if self.pending.ends_with(b"\n") {
                emit(self.decode()?)?;
                self.pending.clear();
            }
        }
        Ok(())
    }
    pub fn finish(
        &mut self,
        mut emit: impl FnMut(schema::Message) -> io::Result<()>,
    ) -> io::Result<()> {
        if !self.pending.is_empty() {
            emit(self.decode()?)?;
            self.pending.clear();
        }
        Ok(())
    }
    fn decode(&self) -> io::Result<schema::Message> {
        match serde_json::from_slice(&self.pending) {
            Ok(message) => Ok(message),
            Err(_)
                if self.allow_text
                    && self
                        .pending
                        .iter()
                        .copied()
                        .find(|byte| !byte.is_ascii_whitespace())
                        != Some(b'{') =>
            {
                Ok(schema::Message::TextLine(
                    String::from_utf8_lossy(&self.pending).into_owned(),
                ))
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_partitions_crlf_and_eof_tails_keep_complete_records() {
        let bytes = b"{\"reason\":\"build-finished\",\"success\":true}\r\n{\"reason\":\"build-finished\",\"success\":false}";
        for width in 1..=bytes.len() {
            let mut decoder = MessageDecoder::new(bytes.len(), false);
            let mut successes = Vec::new();
            let mut receive = |message| {
                let schema::Message::BuildFinished(message) = message else {
                    panic!("build-finished record");
                };
                successes.push(message.success);
                Ok(())
            };
            for chunk in bytes.chunks(width) {
                decoder.feed(chunk, &mut receive).unwrap();
            }
            decoder.finish(&mut receive).unwrap();
            assert_eq!(successes, [true, false]);
        }
    }

    #[test]
    fn limits_malformed_json_text_policy_and_original_sink_errors() {
        let mut limited = MessageDecoder::new(3, false);
        assert!(limited.feed(b"1234", |_| Ok(())).is_err());
        for allow_text in [false, true] {
            let mut malformed = MessageDecoder::new(64, allow_text);
            assert!(malformed.feed(b"{invalid}\n", |_| Ok(())).is_err());
        }
        let mut strict = MessageDecoder::new(64, false);
        assert!(strict.feed(b"plain output\n", |_| Ok(())).is_err());
        let mut text = MessageDecoder::new(64, true);
        text.feed(b"plain output\n", |message| {
            assert!(matches!(message, schema::Message::TextLine(_)));
            Ok(())
        })
        .unwrap();
        let bytes = b"{\"reason\":\"build-finished\",\"success\":true}\n";
        let mut decoder = MessageDecoder::new(bytes.len(), false);
        let error = decoder
            .feed(bytes, |_| Err(io::Error::from_raw_os_error(28)))
            .unwrap_err();
        assert_eq!(error.raw_os_error(), Some(28));
    }
}
