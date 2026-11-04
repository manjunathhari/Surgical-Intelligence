from __future__ import annotations

from collections.abc import Iterable


def recall_at_k(relevant_ids: set[str], retrieved_ids: Iterable[str], k: int) -> float:
    """Fraction of known relevant items found in the first k results."""
    if k <= 0:
        raise ValueError("k must be positive")
    if not relevant_ids:
        return 1.0
    retrieved = list(retrieved_ids)[:k]
    return len(relevant_ids.intersection(retrieved)) / len(relevant_ids)


def precision_at_k(relevant_ids: set[str], retrieved_ids: Iterable[str], k: int) -> float:
    """Fraction of the first k retrieved items that are relevant."""
    if k <= 0:
        raise ValueError("k must be positive")
    retrieved = list(retrieved_ids)[:k]
    if not retrieved:
        return 0.0
    return len(relevant_ids.intersection(retrieved)) / len(retrieved)
