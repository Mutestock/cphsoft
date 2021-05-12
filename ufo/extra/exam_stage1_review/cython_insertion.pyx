# cython: language_level=3, boundscheck=False
from cpython cimport array
from libc.math cimport sqrt
import time

cpdef sort (int[:] arr, int t):
    cdef int i
    cdef int j
    cdef int tempVal
    for i in range (0, len(arr)):
        tempVal = arr[i]
        j = i - 1
        while j >= 0 and arr[j] > tempVal:
            arr[j + 1] = arr[j]
            j = j - 1
        arr [j + 1] = tempVal


cpdef check_method_time():
    cdef int n = 1000
    cdef int count = 10000
    cdef float st = 0.0
    cdef float sst = 0.0
    cdef float start
    cdef float stop
    cdef float ns
    cdef int i
    cdef int j
    cdef array.array arrs = array.array('i',[10, 22, 14, 4, 2, 50, 75, 49, 4])
    cdef int[:] arr = arrs

    for j in range ( n):
        start = time.perf_counter_ns ()
        for i in range ( count ):
            sort(arr, i)
        stop = time.perf_counter_ns ()
        ns = ( stop - start ) / count
        st += ns
        sst += ns * ns

    cdef float mean = st /n
    cdef float sdev = sqrt(( sst - mean * mean * n) /(( n -1) ))
    print ("{:.1f} ns +/ - {:.1f}ns".format(mean, sdev ))