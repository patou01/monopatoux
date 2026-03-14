try:
    # This is for when the module is installed normally (e.g. as a wheel)
    from .comtrade import hello
except ImportError:
    import sys
    import os
    
    # Try importing libcomtrade (the name of the .so file)
    try:
        from . import libcomtrade as _core
        hello = _core.hello
    except ImportError:
        # Check if it is in rs folder (Bazel structure)
        rs_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "rs"))
        if rs_path not in sys.path:
            sys.path.append(rs_path)
        
        import libcomtrade as _core
        hello = _core.hello

__all__ = ["hello"]
