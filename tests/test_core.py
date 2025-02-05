import pytest
from pathlib import Path
from rustbam import get_depths

@pytest.fixture()
def bam_path():
    return str(Path(__file__).parent / "example.bam")

@pytest.fixture()
def chrom():
    return 'chr1'

@pytest.fixture()
def start():
    return 1000000

@pytest.fixture()
def end():
    return 1000020

def test_get_depths(bam_path, chrom, start, end):
    positions, depths = get_depths(bam_path, chrom, start, end)
    expected_positions = list(range(start, end+1))
    expected_depths = [
        51, 52, 44, 52, 53, 47, 51, 
        52, 49, 50, 49, 50, 50, 49, 
        50, 50, 46, 50, 48, 50, 44
    ]
    assert positions == expected_positions, expected_positions
    assert depths == expected_depths, expected_depths

def test_get_depths_multithread(bam_path, chrom, start, end):
    expected_positions = list(range(start, end+1))
    expected_depths = [
        51, 52, 44, 52, 53, 47, 51, 
        52, 49, 50, 49, 50, 50, 49, 
        50, 50, 46, 50, 48, 50, 44
    ]
    for num_threads in range(1, 48):
        positions, depths = get_depths(bam_path, chrom, start, end, 
                                       num_threads=num_threads)
        assert positions == expected_positions, expected_positions
        assert depths == expected_depths, expected_depths

def test_get_depths_maxdepth(bam_path, chrom, start, end):
    expected_positions = list(range(start, end+1))
    for max_depth in range(1, 10): # TODO: test for max_depth 0?
        expected_depths = [max_depth] * len(expected_positions)
        positions, depths = get_depths(bam_path, chrom, start, end, 
                                       max_depth=max_depth)
        assert positions == expected_positions, expected_positions
        assert depths == expected_depths, expected_depths

def test_get_depths_step(bam_path, chrom, start, end):
    original_positions = list(range(start, end+1))
    original_depths = [
        51, 52, 44, 52, 53, 47, 51, 
        52, 49, 50, 49, 50, 50, 49, 
        50, 50, 46, 50, 48, 50, 44
    ]
    for step in range(2, 100):
        step_ixs = range(0, len(original_positions), step)
        expected_positions = [original_positions[i] for i in step_ixs]
        expected_depths = [original_depths[i] for i in step_ixs]
        positions, depths = get_depths(bam_path, chrom, start, end, 
                                       step=step)
        assert positions == expected_positions, expected_positions
        assert depths == expected_depths, expected_depths

