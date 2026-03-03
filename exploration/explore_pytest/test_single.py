"""
Check fixture scope
"""

def test_cat(cat):
    print(cat)
    assert cat == "cat"
