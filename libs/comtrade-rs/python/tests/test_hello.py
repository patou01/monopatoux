from comtrade import hello

def test_hello():
    result = hello()
    assert result == "Hello from COMTRADE core!"

if __name__ == "__main__":
    test_hello()
    print("Test passed!")
