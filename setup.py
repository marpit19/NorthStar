from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="northstar",
    version="0.1.0",
    packages=["northstar"],
    rust_extensions=[RustExtension(
        "northstar.northstar", binding=Binding.PyO3)],
    install_requires=[],
    setup_requires=["setuptools-rust>=1.5.2"],
    include_package_data=True,
    zip_safe=False,
)
