from setuptools import setup
from setuptools_rust import Binding, RustExtension

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="northstar-data",
    version="0.1.1",
    author="Arpit Mohapatra",
    author_email="arpit.mohapatra19@gmail.com",
    description="A blazingly fast data processing library implemented in Rust with Python bindings",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/marpit19/NorthStar",
    packages=["northstar"],
    rust_extensions=[RustExtension(
        "northstar.northstar", binding=Binding.PyO3)],
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Rust",
    ],
    python_requires=">=3.7",
    install_requires=[],
    setup_requires=["setuptools-rust>=1.5.2"],
    include_package_data=True,
    zip_safe=False,
)
