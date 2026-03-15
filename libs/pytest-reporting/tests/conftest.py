import pytest

@pytest.fixture(scope="session")
def report_results(request):
    return request.config.getoption("--report-results")
