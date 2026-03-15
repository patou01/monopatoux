
from django.contrib import admin
from django.urls import path, include
from rest_framework import routers
from src.reporting_db.views import TestRunViewSet, TestResultViewSet

router = routers.DefaultRouter()
router.register(r"testruns", TestRunViewSet)
router.register(r"testresults", TestResultViewSet)

urlpatterns = [
    path("admin/", admin.site.urls),
    path("api/", include(router.urls)),
]
