from setuptools import find_packages
from setuptools import setup

setup(
    name='whycode_interfaces',
    version='0.0.1',
    packages=find_packages(
        include=('whycode_interfaces', 'whycode_interfaces.*')),
)
