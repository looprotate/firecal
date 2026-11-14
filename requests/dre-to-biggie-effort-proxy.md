# Effort proxy: prerequisites before revisiting (#28)

The issue is resolved by its documented-drop acceptance path. The optional
effort feature is dropped from this demo; this request records prerequisites
for a future revisit, not an instruction to change the contract now.

`crates/firecal-ingest/effort-proxy.md` contains the research and candidate
equation. Detections-only FIRMS CSVs do not supply an observation denominator.
Daytime detections and first/last hotspot dates cannot substitute for pass
counts, usable observation masks or actual instrument availability.

FCT1 accepts exactly three planes. If this work is reopened, Biggie must own a
separate contract change covering units and scaling, unknown versus zero,
format versioning and reader/writer/query/wasm/UI compatibility. Dre has not
changed `firecal-types`, FCT1/FCA1, sigma, exported statistics or consumer APIs.

Before implementation, provide:

- Versioned satellite availability and latitude/date-specific pass estimates,
  with a documented cloud/quality/coverage policy and source product choice.
- Independent reference data and a stated validation tolerance across latitude
  bands and months; the index must remain explicitly an opportunity proxy.
- A lead-approved contract and coordinated consumers. Unknown exposure cannot
  silently become a zero value or a denominator for normalizing fire counts.

Future acceptance tests should include full and partial operational months,
leap February, exact latitude-band boundaries, absent inputs versus measured
zero, deterministic order, serialization round trips and native/wasm parity.
The note's hypothetical arithmetic example is not scientific validation.

No new dependency or runtime implementation is requested for the present demo.
