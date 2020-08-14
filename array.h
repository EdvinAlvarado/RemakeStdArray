#include <cstddef>  //size_t
#include "reverse_iterator.h"
#include <stdexcept> // out_of_range

// Shitty take on c++ Array
template<class T, size_t N>
class array {
		T arr[N];
	public:
		array(T Carr[]) {
			for(int i = 0; i < N; i++) {arr[i] = Carr[i];}
		}
		array() {}
		// ~array() {delete arr;}
		// array() {
		// 	for(int i = 0; i < N; i++) {arr[i] = 0;}
		// }
		T at(int pos) {
			if (pos < 0 || pos >= N) {throw std::out_of_range("Index is out of range.");} 
			return arr[pos];		
		}
		T& operator[](int i) {return arr[i];}
		T front() {return arr[0];}
		T back() {return arr[N-1];}
		T* data() {return arr;}
		T* begin() {return &arr[0];}
		const T* cbegin() {return &arr[0];}
		T* end() {return &arr[N];}		
		const T* cend() {return &arr[N];}
		reverse_iterator<T> rbegin() {return &arr[N-1];}
		const reverse_iterator<T> crbegin() {return &arr[N-1];}
		reverse_iterator<T> rend() {return &arr[-1];}		
		const reverse_iterator<T> crend() {return &arr[-1];}
		bool empty() {return N<=0;} // Different implementation
		size_t size() {return N;}
		void fill(T n) {
			for(int i = 0; i < N; i++) {arr[i] = n;}
		}
		// array class works already.
		void operator=(T a[N]) {
			for (int i = 0; i < N; i++) {arr[i] = a[i];}
		}
		void operator+=(T n) {
			for (int i = 0; i < N; i++) {arr[i] += n;}
		}
		void operator+=(array<T, N> a) {
			for (int i = 0; i < N; i++) {arr[i] += a[i];}
		}
		void operator-=(T n) {
			for (int i = 0; i < N; i++) {arr[i] -= n;}
		}
		void operator-=(array<T, N> a) {
			for (int i = 0; i < N; i++) {arr[i] -= a[i];}
		}
		void operator*=(T n) {
			for (int i = 0; i < N; i++) {arr[i] *= n;}
		}
		void operator/=(T n) {
			for (int i = 0; i < N; i++) {arr[i] /= n;}
		}
		void operator%=(T n) {
			for (int i = 0; i < N; i++) {arr[i] %= n;}
		}
		bool operator==(T a[N]) {
			bool b = true;
			for (int i = 0; i < N; i++) {b *= arr[i] == a[i];}
			return b;
		}
		bool operator==(array<T, N> a) {
			bool b = true;
			for (int i = 0; i < N; i++) {b *= arr[i] == a[i];}
			return b;
		}
		bool operator!=(T a[N]) {
			bool b = true;
			for (int i = 0; i < N; i++) {b *= arr[i] != a[i];}
			return b;
		}
		bool operator!=(array<T, N> a) {
			bool b = true;
			for (int i = 0; i < N; i++) {b *= arr[i] != a[i];}
			return b;
		}
};

template<class T, size_t N>
T sum(array<T, N> arr) {
	T n = 0;
	for(int i = 0; i < arr.size(); i++) {
		n += arr[i];
	}
	return n;
}
