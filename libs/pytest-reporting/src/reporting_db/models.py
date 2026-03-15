from django.db import models

class TestRun(models.Model):
    timestamp = models.DateTimeField(auto_now_add=True)
    pytest_args = models.TextField()

class TestResult(models.Model):
    test_run = models.ForeignKey(TestRun, on_delete=models.CASCADE, related_name="results")
    nodeid = models.CharField(max_length=512)
    owner = models.CharField(max_length=128)
    outcome = models.CharField(max_length=32)
    duration = models.FloatField()
    exception = models.TextField(blank=True, null=True)
