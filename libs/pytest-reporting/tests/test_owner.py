import pytest

@pytest.mark.owner("Alice")
def test_with_owner():
    assert True

@pytest.mark.owner("Bob")
def test_with_another_owner():
    assert True
