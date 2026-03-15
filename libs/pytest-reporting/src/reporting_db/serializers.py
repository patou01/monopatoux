from rest_framework import serializers
from .models import TestRun, TestResult

class TestResultSerializer(serializers.ModelSerializer):
    class Meta:
        model = TestResult
        fields = '__all__'

class TestRunSerializer(serializers.ModelSerializer):
    results = TestResultSerializer(many=True, read_only=True)

    class Meta:
        model = TestRun
        fields = '__all__'
