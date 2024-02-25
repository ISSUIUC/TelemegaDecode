import numpy as np
import matplotlib.pyplot as plt
arr = np.fromfile("log.txt", dtype=np.int8).astype(np.float32).view(np.complex64)
# arr = np.fromfile("log.dat", dtype=np.complex128)
arr = arr[0:20000000]
plt.plot(np.real(arr))
plt.plot(np.imag(arr))
plt.show()

# arr = np.fromfile("log.dat", dtype=np.int8)
# arr = arr[0:20_000_000]
# plt.plot(arr)
# plt.show()