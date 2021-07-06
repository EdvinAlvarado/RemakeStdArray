template<class T>
class reverse_iterator {
        T* rit;
    public:
        reverse_iterator(T* it) {
            rit = it;
        }
        T& operator++() {return *--rit;} // ++it
        T& operator--() {return *++rit;} // --it
        T operator++(int) {return *rit--;} // it++
        T operator--(int) {return *rit++;} // it--
        T& operator*() {return *rit;}
        T* operator&() {return rit;}
        bool operator!=(T* it) {return rit != it;}
        bool operator!=(reverse_iterator<T> it) {return rit != &it;}
        bool operator==(T* it) {return rit == it;}
        bool operator==(reverse_iterator<T> it) {return rit == &it;}
};
