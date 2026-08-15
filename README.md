# axiom-capabilities

An explicit capability model with no ambient authority. Tokens bind a subject, resource, action, epoch and attenuation
chain.

> **Maturity:** research prototype v0.1. The default verifier proves properties by exhaustive evaluation over an
> explicitly finite input domain. A VALID receipt is therefore a theorem about that bounded model, not a claim of
> unbounded program correctness.


The v0.1 CLI demonstrates issue -> attenuate -> check. Cryptographic signatures are intentionally **not faked** in this
prototype; the next research milestone is Ed25519/threshold authorization plus proof-bound capabilities.
