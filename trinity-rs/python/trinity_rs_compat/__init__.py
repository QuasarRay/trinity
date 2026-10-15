"""Development compatibility shim for the Bevy/PyO3 Trinity reimplementation.

This package intentionally does not replace Carbon's existing ``trinity`` module in-place.
Consumers can import it explicitly while compatibility is being proven, or an embedding host can
re-export selected classes into the existing Carbon namespace.
"""

from _trinity_rs import ALResult, InvalidArgument, Runtime, Tr2BitmapDimensions

__all__ = ["ALResult", "InvalidArgument", "Runtime", "Tr2BitmapDimensions"]
