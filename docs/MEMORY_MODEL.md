# Memory model

## Assertion

An assertion represents one candidate unit of knowledge in a normalized subject-predicate-object form, alongside a readable statement.

Each assertion carries:
- epistemic status: observed, reported, inferred, or hypothesis;
- optional confidence, explicitly not assumed to be calibrated;
- retention class: durable, episodic, or temporary;
- lifecycle status: active, superseded, retracted, or disputed;
- provenance and observation time;
- optional validity interval and tags.

## Events

Changes are recorded as events. The initial implementation records insertion and status changes. Later versions should append richer events for amendment, corroboration, conflict detection, and resolution.

## Important distinction

A registry case, a surgeon's reflection about a case, a general technical principle, and a published evidence claim are different source/knowledge types. They should not be flattened into one undifferentiated text memory. Future schema revisions should retain this distinction explicitly.
