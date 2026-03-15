from rest_framework import viewsets
from .models import TestRun, TestResult
from .serializers import TestRunSerializer, TestResultSerializer

class TestRunViewSet(viewsets.ModelViewSet):
    queryset = TestRun.objects.all().order_by('-timestamp')
    serializer_class = TestRunSerializer

class TestResultViewSet(viewsets.ModelViewSet):
    queryset = TestResult.objects.all()
    serializer_class = TestResultSerializer
