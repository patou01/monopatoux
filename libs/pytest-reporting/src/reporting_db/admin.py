from django.contrib import admin
from .models import TestRun, TestResult

@admin.register(TestRun)
class TestRunAdmin(admin.ModelAdmin):
    list_display = ("id", "timestamp", "pytest_args")

@admin.register(TestResult)
class TestResultAdmin(admin.ModelAdmin):
    list_display = ("id", "test_run", "nodeid", "owner", "outcome", "duration")
    list_filter = ("outcome", "owner")
