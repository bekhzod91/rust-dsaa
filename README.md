# Rust-DSAA (Data structures and algorithms)

### Description

This repository contains implementations of data structures and algorithms in Rust step by step. The goal is to learn Rust and refresh my knowledge of data structures and algorithms.

### Data structures
  [x] [Stack](#stack-lifo) \
  [x] [Queue](#queue)\
  [x] [Doubly Linked List](#doublylinkendlist) \
  [ ] AVL Tree \
  [ ] BTree \
  [ ] Graph \
  [ ] Hashing \
  [ ] Heap \
  [x] [Bubble Sort](#bubble-sort) \
  [x] [Selection Sort](#selection-sort) \
  [x] [Insertion Sort](#insertion-sort) \
  [x] [Binary Search](#binary-search) \
  [ ] Fibonacci


## Stack (LIFO)
[src](src/stack.rs)\
Stack is a linear data structure which follows a particular order in which the operations are performed. The order may be LIFO(Last In First Out) or FILO(First In Last Out).

## Queue
[src](src/linkend_list.rs)\
Queue based on doubly linked list. Queue is a linear data structure which follows a particular order in which the operations are performed. The order is First In First Out (FIFO).

## DoublyLinkendList
[src](src/linkend_list.rs)\
DoublyLinkedList is a linear data structure in which each node has a pointer to the previous and the next node in the list.

## Bubble Sort
[src](src/bubble_sort.rs)\
Bubble Sort is the simplest sorting algorithm that works by repeatedly swapping the adjacent elements if they are in wrong order.

## Selection Sort
[src](src/selection_sort.rs)\
Selection sort is a sorting algorithm that selects the biggest element from an unsorted list in each iteration and places that element at the end of the sorted list.

## Insertion Sort
[src](src/insertion_sort.rs)\
Insertion sort is a sorting algorithm that places an unsorted element at its suitable place in each iteration. It has a sorted sublist and an unsorted sublist. Each iteration removes an element from the unsorted sublist, finds the location it belongs within the sorted sublist, and inserts it there. It repeats until no input elements remain.


## Binary Search
[src](src/binary_search.rs)\
Binary Search is a searching algorithm for finding an element's position in a sorted array. In this approach, the element is always searched in the middle of a portion of an array.

