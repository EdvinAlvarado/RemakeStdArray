#ifndef _LINKEDLIST_
#define LINKEDLIST

#include <cstddef>
#include <iostream>
#include <stdexcept>
#include <type_traits>

// Link list node
template<class T>
class Node {
	public:
		T data;
		Node* next;
		Node() {}
		Node(T tData) {
			data = tData;
			next = nullptr;
		}
};

// Linked List == deque
template<class T>
class LinkedList {
		Node<T>* head;
		Node<T>* tail;
		size_t size;
	public:
		LinkedList() {head = new Node<T>(); size = 0;}
		LinkedList<T>(T tData) {head = new Node<T>(tData); size = 1; tail = head;}
		~LinkedList<T>() {
			clear();
		}	
		// Modifiers
		void push_front(T tData) {
			Node<T>* temp = new Node<T>(tData);
			temp->next = head;
			head = temp;
			size++;
		}
		void push_back(T tData) {
			if (head == nullptr) {push_front(tData); return;}
			Node<T>* new_node = new Node<T>(tData);
			tail->next = new_node;
			tail = new_node;
			size++;
		}
		void pop_front() {
			if (head == nullptr) {std::cerr << "vector is empty" << std::endl; return;}
			Node<T>* pop_node = head;
			head = head->next;
			delete pop_node;
			size--;
		}
		// TODO could make O(1) if have const prev_tail ptr
		 void pop_back() {
			if (head == nullptr) {std::cerr << "vector is empty" << std::endl; return;}
			Node<T>* before_tail = head;
			while (before_tail->next != tail) {before_tail = before_tail->next;}
			Node<T>* pop_node = tail;
			before_tail->next = nullptr;
			tail = before_tail;
			delete pop_node;
			size--;
		}
		void insert(size_t pos, T val) {
			Node<T>* prev_node = head;
			if (pos == 0) {push_front(val); return;}
			for (size_t i = 0; i < pos-1; i++) {
				if (prev_node == nullptr) {std::cerr << "out of bound position" << std::endl; return;}
				else {prev_node = prev_node->next;}
			}
			Node<T>* new_node = new Node<T>(val);
			new_node->next 	= prev_node->next;
			prev_node->next = new_node;
			size++;
		}
		void erase(size_t pos) {
			if (pos == 0) {pop_front(); return;}
			if (pos == size-1) {pop_back(); return;}
			Node<T>* erase_node = head;
			for (size_t i = 0; i < pos-2; i++) {erase_node = erase_node->next;}
			Node<T>* prev_erase_node = erase_node;
			erase_node = erase_node->next;
			prev_erase_node->next = erase_node->next;
			delete erase_node;
			size--;
		}
		void swap(size_t pos1, size_t pos2) {
			if (pos1 == pos2) {return;}
			Node<T>* pos1_node = head;
			Node<T>* pos2_node = head;
			if (pos1 == size-1) {Node<T>* pos1_node = tail;}
			else if (pos1 != 0){
				for (size_t i = 0; i < pos1; i++) {pos1_node = pos1_node->next;}
			}
			if (pos2 == size-1) {Node<T>* pos2_node = tail;}
			else if (pos2 != 0){
				for (size_t i = 0; i < pos2; i++) {pos2_node = pos2_node->next;}
			}
			T temp = pos1_node->data;
			pos1_node->data = pos2_node->data;
			pos2_node->data = temp;
		}
		void clear() {
			Node<T>* current_node = head;
			Node<T>* next_node = current_node->next;
			while (current_node->next != nullptr) {
				delete current_node;
				current_node = next_node;
				next_node = current_node->next;
				size--;
			}
			delete current_node;
			size--;
			head = tail;
		}
		// Utilities
		void print() {
			Node<T>* n = head;
			while (n != nullptr) {
				std::cout << n->data << " ";
				n = n->next;
			}
			std::cout << std::endl;
		}
		// Capacity
		size_t length() {return size;}
		void resize() {

		}
		bool empty() {
			return head == nullptr;
		}
		// Element Access
		T& front() {
			return head->data;
		}
		T& back() {
			return tail->data;
		}
		T& at(size_t pos) {
			if (pos == 0) {return head->data;}
			if (pos == size-1) {return tail->data;}
			Node<T>* at_node = head;
			if (pos < 0 || pos >= size) {throw std::out_of_range("out of bound.");}
			for (size_t i = 0; i < pos-1; i++) {at_node = at_node->next;}
			return at_node->data;
		}
		T& operator[](size_t pos) {
			if (pos == 0) {return head->data;}
			if (pos == size-1) {return tail->data;}
			Node<T>* at_node = head;
			for (size_t i = 0; i < pos-1; i++) {at_node = at_node->next;}
			return at_node->data;
		}

};

#endif
