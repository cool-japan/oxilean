## Differential verdict join: oxilean vs lean4lean

```
total distinct declarations joined: 1987

comparison buckets:
  AGREE (both verified or both rejected) : 1576
  DISAGREE (verified vs rejected)        : 0   <-- FINDINGS
  ONLY-ONE-SIDE (version/toolchain skew) : 228
  UNSUPPORTED-SKIPPED (one side declines): 183

oxilean per-verdict:
  verified    : 1631
  rejected    : 0
  unsupported : 183
  absent      : 173
lean4lean per-verdict:
  verified    : 1932
  rejected    : 0
  unsupported : 0
  absent      : 55
```

### Disagreements (findings)

No verified-vs-rejected disagreements. Every declaration checked by both sides received the same accept/reject verdict.
