import pytest

def pytest_configure(config):
    """Add 'owner' marker to pytest configuration."""
    config.addinivalue_line(
        "markers", "owner(name): define the owner of a test."
    )

def pytest_runtest_setup(item):
    """Ensure that each test has an 'owner' marker."""
    owner_marker = item.get_closest_marker("owner")
    if owner_marker is None:
        pytest.fail(f"Test {item.nodeid} is missing an 'owner' marker.")
    elif not owner_marker.args:
        pytest.fail(f"Test {item.nodeid} 'owner' marker is missing arguments.")
