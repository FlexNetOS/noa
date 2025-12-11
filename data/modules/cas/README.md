# Module Content-Addressable Storage (CAS)

Blobs are stored under `data/modules/cas/{h0}{h1}/{h2}{h3}/{hash}` using the
SHA-256 hash of the content. This directory is created by the module CAS
service at runtime and can be pruned safely using reference counts in
`module_versions`.
