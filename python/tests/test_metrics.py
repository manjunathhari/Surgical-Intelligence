import pytest

from si_eval.metrics import precision_at_k, recall_at_k


def test_recall_at_k():
    assert recall_at_k({"a", "b"}, ["b", "x", "a"], 3) == 1.0


def test_precision_at_k():
    assert precision_at_k({"a", "b"}, ["a", "x"], 2) == 0.5


def test_k_must_be_positive():
    with pytest.raises(ValueError):
        recall_at_k({"a"}, ["a"], 0)
