# Example

This is an example project that demonstrates, as well as lays out a template
of how to write a Rust library that is accessible from 
Python. 

See how the library is accessed in `__main__.py` as if it was a normal Python library.


You can use this command to build the package and install it in your virtual environment to test the build system.
```bash
pip3 install .
```
You can also use this command to install the package into your virtual environment while developing. 
```bash
maturin develop
```

The difference between these two is that the first requires everything to be setup in `Cargo.toml` and `pyproject.toml`,
while the second will work without the build system setup (while you are developing).