from setuptools import find_packages
from setuptools import setup

setup(
    name='error_msg',
    version='0.0.0',
    packages=find_packages(
        include=('error_msg', 'error_msg.*')),
)
