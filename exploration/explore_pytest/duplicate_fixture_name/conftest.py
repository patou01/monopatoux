"""
Trying to understand how a fixture gets called and whether
a fixture can depend upon "itself" (another fixture of the same name)
"""
import pytest

@pytest.fixture(name="cat")
def cat(cat):
    return 2*cat
