from pprint import pprint

import numpy as np
from kalax import extract_features

x = np.linspace(0, 1, 100)
y = np.sin(2 * np.pi * 5 * x) + 0.5
data = [{"x": x, "y": y}]
features = extract_features(data)
pprint(features)

"""
output:
    [{'x': {'absolute_maximum': 1.0,
        'length': 100.0,
        'maximum': 1.0,
        'mean': 0.5,
        'median': 0.5,
        'minimum': 0.0,
        'root_mean_square': 0.5788063881962906,
        'standard_deviation': 0.29157646512850627,
        'sum_values': 50.0,
        'variance': 0.08501683501683503},
  'y': {'absolute_maximum': 1.4998741276738752,
        'length': 100.0,
        'maximum': 1.4998741276738752,
        'mean': 0.4999999999999998,
        'median': 0.4999999999999994,
        'minimum': -0.49987412767387507,
        'root_mean_square': 0.8631338250816034,
        'standard_deviation': 0.7035623639735146,
        'sum_values': 49.99999999999998,
        'variance': 0.4950000000000002}}]
"""
