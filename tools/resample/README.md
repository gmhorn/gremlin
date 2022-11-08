# Resampling tool

Gremlin uses a single basis for all if its sampled radiometric data
([source](/src/spectrum/sampled.rs)). So to use external radiometric data, it
must first be resampled into the basis Gremlin uses.

PBRT has this same issue; they have implemented the math to perform that 
resampling themselves. That's a bit beyond the scope for this project, so
instead we use Numpy / Scipy, which already have extremely good mathematical
manipulation libraries.

Maybe one day Gremlin will be updated to be able to ingest raw tabular data and
resample it itself, but for now this works just fine.

## Data sources

* [`data/ciexyz31.csv`](./data/ciexyz31.csv) - CIE color matching curves at 1nm
  intervals, from <http://cvrl.ucl.ac.uk/cmfs.htm>
* <https://refractiveindex.info> - Public domain CC1.0 database of optical measurements.