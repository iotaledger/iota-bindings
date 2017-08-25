#!/usr/bin/env python
# coding=utf-8
# :bc: Not importing unicode_literals because in Python 2 distutils,
# some values are expected to be byte strings.
from __future__ import absolute_import, division, print_function

from codecs import StreamReader, open
from sys import version_info

from setuptools import find_packages, setup
from setuptools_rust import Binding, RustExtension

##
# Load long description for PyPi.
#with open('README.rst', 'r', 'utf-8') as f: # type: StreamReader
#  long_description = f.read()

##
# For compatibility with versions of pip < 9, we will determine
# dependencies at runtime.
# Maybe once Travis upgrades their containers to use a newer version,
# we'll switch to the newer syntax (:
install_dependencies = ['setuptools_rust']

unit_test_dependencies = []

setup(
    name='iota',
    description='iota.rs bindings for Python',
    url='https://github.com/iotaledger/iota-bindings/python',
    version='0.1.0',

    # long_description = long_description,
    rust_extensions=[
        RustExtension('iota._helloworld', 'Cargo.toml', binding=Binding.PyO3)
    ],
    include_package_data=True,
    install_requires=install_dependencies,
    license='MIT',
    packages=['iota'],
    # rust extensions are not zip safe, just like C-extensions.
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Rust',
        'Topic :: Software Development :: Libraries :: Python Modules',
    ],
    keywords='iota,tangle,iot,internet of things,api,library,cryptocurrency,'
    'balanced ternary,bindings,rust',
    author='Andreas C. Osowski',
    author_email='andreas+pypi@osowski.de',
    zip_safe=False)
