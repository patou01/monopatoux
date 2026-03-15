import pytest
import requests

results = []
owner_map = {}

def pytest_configure(config):
    """Add 'owner' marker to pytest configuration."""
    config.addinivalue_line(
        "markers", "owner(name): define the owner of a test."
    )

def pytest_addoption(parser):
    parser.addoption(
        "--report-results", action="store_true", default=False,
        help="Report test results to the Django API after the run"
    )

def pytest_runtest_setup(item):
    owner_marker = item.get_closest_marker("owner")
    if owner_marker is None:
        pytest.fail(f"Test {item.nodeid} is missing an 'owner' marker.")
    elif not owner_marker.args:
        pytest.fail(f"Test {item.nodeid} 'owner' marker is missing arguments.")
    else:
        owner_map[item.nodeid] = owner_marker.args[0]

def pytest_runtest_logreport(report):
    if report.when == "call":
        owner = owner_map.get(getattr(report, "nodeid", None))
        results.append({
            "nodeid": getattr(report, "nodeid", None),
            "outcome": report.outcome,
            "duration": getattr(report, "duration", None),
            "owner": owner,
            "exception": str(report.longrepr) if report.failed else None,
        })

def pytest_sessionfinish(session, exitstatus):
    config = session.config
    if config.getoption("--report-results"):
        # Create a TestRun first
        test_run_data = {
            "pytest_args": " ".join(config.args),
        }
        test_run_id = None
        try:
            resp = requests.post(
                "http://127.0.0.1:8000/api/testruns/",
                json=test_run_data
            )
            if resp.status_code < 400:
                test_run_id = resp.json().get("id")
                print(f"Created TestRun with id {test_run_id}")
            else:
                print(f"Failed to create TestRun: {resp.text}")
        except Exception as e:
            print(f"Error creating TestRun: {e}")
        if not test_run_id:
            print("Cannot upload results without a TestRun.")
            return
        # Send results to Django API
        for result in results:
            result["test_run"] = test_run_id
            try:
                response = requests.post(
                    "http://127.0.0.1:8000/api/testresults/",
                    json=result
                )
                if response.status_code >= 400:
                    print(f"Failed to upload result: {response.text}")
                else:
                    print(f"Uploaded result for {result['nodeid']}.")
            except Exception as e:
                print(f"Error uploading result for {result['nodeid']}: {e}")
