import pytest

@pytest.fixture(name="cat")
def cat():
    return "cat"
