# Biblical Source Storage

This directory holds curated Greek and Hebrew source texts used by the governance pipeline.

- Place raw text or structured files (TXT, JSONL, or USFM exports) in this folder.
- Prefix filenames with `grc_` for Greek or `hbo_` for Hebrew to aid language detection.
- Keep files UTF-8 encoded and include book/chapter metadata in the first line as JSON when possible.
- Do not commit licensed or proprietary texts; store hashes only if redistribution is restricted.

The ingestion pipeline writes `index.json` next to these sources to record checksums, detected language, and ingest timestamps. Ensure this directory remains writable by NOA so the audit trail for sacred-text decisions stays verifiable. 
