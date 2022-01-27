#!/usr/bin/env python3

import pytest

from .network import setup_chainmain, setup_cronos


@pytest.fixture(scope="session")
def chainmain(tmp_path_factory):
    yield from setup_chainmain(tmp_path_factory.mktemp("chainmain"), 26800)


@pytest.fixture(scope="session")
def cronos(tmp_path_factory):
    yield from setup_cronos(tmp_path_factory.mktemp("cronos"), 26650)
