#include <cstddef>  //size_t

#define arrsize(arr) (sizeof(arr) / sizeof(arr[0]))

template<class T>
T sum(T array[], size_t size) {
	T n = 0;
	for(int i = 0; i < size; i++) {
		n += array[i];
	}
	return n;
}