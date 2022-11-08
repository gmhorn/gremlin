import numpy as np
from scipy.interpolate import interp1d
from scipy import integrate

np.set_printoptions(linewidth=90)

wavelength_min = 380.
wavelength_max = 780.
wavelength_step = 5.

cmf = np.loadtxt('data/ciexyz31.csv', delimiter=",").T
cie_x = interp1d(cmf[0], cmf[1])
cie_y = interp1d(cmf[0], cmf[2])
cie_z = interp1d(cmf[0], cmf[3])

wavelengths = np.arange(wavelength_min, wavelength_max, wavelength_step)
# print(repr(cie_x(wavelengths)))
# print(repr(cie_y(wavelengths)))
# print(repr(cie_z(wavelengths)))

print(integrate.trapezoid(cie_y(wavelengths), wavelengths))
