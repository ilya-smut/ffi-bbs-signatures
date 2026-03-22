from enum import IntEnum


class DocIntEnum(IntEnum):
    def __new__(cls, value, doc=None):
        self = int.__new__(cls, value)  # calling super().__new__(value) here would fail
        self._value_ = value
        if doc is not None:
            self.__doc__ = doc
        return self


class SignatureProofStatus(DocIntEnum):
    success = 0, "The proof is verified"
    bad_signature = 1, "The proof failed because the signature proof of knowledge failed"
    bad_hidden_signature = 2, "The proof failed because a hidden message was invalid when the proof was created"
    bad_revealed_message = 3, "The proof failed because a revealed message was invalid"
