# pytest-reporting

The goal of this project is to provide a comprehensive test reporting mechanism for `pytest`, enforcing test ownership and storing execution results in a centralized database.

## Architecture

The project is structured as follows:

- `src/pytest_reporting/`: Contains the `pytest` plugin implementation, including the `owner` marker logic.
- `src/reporting_db/`: A Django-based application defining the database models for `TestRun` and `TestResult`.
- `podman/`: Contains the database infrastructure configuration, including `docker-compose.yml` for PostgreSQL and a control script `db_ctl.sh`.
- `tests/`: Contains integration tests for the `pytest` plugin to ensure markers are correctly enforced.
- `requirements.txt`: Python package requirements for the library.
- `requirements.lock`: Pinned versions for all dependencies.
- `BUILD.bazel`: Main Bazel build file for the library and its dependencies.

## Components

The library consists of three main components:

- **Pytest Plugin**: Found in `src/pytest_reporting/`. This plugin adds a mandatory `owner` marker to `pytest` tests. Every test must be decorated with `@pytest.mark.owner("Name")`.
- **Reporting Database**: Found in `src/reporting_db/`. A Django-based database backend that defines models for `TestRun` and `TestResult` to store information about test execution.
- **Database Infrastructure**: Found in `podman/`. A `podman-compose` configuration for running a PostgreSQL database instance that the Django app interacts with.

## Usage

### Pytest Plugin

To use the plugin in a `pytest` run, ensure it's loaded:

```bash
pytest -p pytest_reporting.plugin ...
```

### Database Control

Use the provided control script to manage the PostgreSQL database:

```bash
# Start the database
bazel run //libs/pytest-reporting/podman:db_ctl -- up -d

# Stop the database
bazel run //libs/pytest-reporting/podman:db_ctl -- down
```

### Django Interface

To manage and view test results, you can use the Django management target:

```bash
# Start the database (if not already running)
bazel run //libs/pytest-reporting/podman:db_ctl -- up -d

# Run migrations (initial setup)
bazel run //libs/pytest-reporting/src/reporting_db:manage -- migrate

# Create a superuser to access the admin
bazel run //libs/pytest-reporting/src/reporting_db:manage -- createsuperuser

# Start the Django development server
bazel run //libs/pytest-reporting/src/reporting_db:manage -- runserver
```
(superuser = drooppi, pass 123)
Once the server is running, access the interface at `http://127.0.0.1:8000/admin/`.

## Build and Test

This library is fully integrated with Bazel:

```bash
# Build the entire package
bazel build //libs/pytest-reporting/...

# Run tests
bazel test //libs/pytest-reporting/tests:test_owner_plugin
```
