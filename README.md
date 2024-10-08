# Library Management System

## Overview

This project implements a **Library Management System** in Rust using **Test-Driven Development (TDD)** principles. The system supports adding books and checking out books, and it comes with a suite of unit tests to verify its functionality. The primary focus of the project is the testing process, which drives the development and ensures the correctness of the implemented methods.

## Features

- **Add a Book**: The system allows adding new books to the library's collection with a title and author.
- **Check Out a Book**: Users can check out books from the library, making them unavailable to others.
- **Basic Validation**: The system prevents adding books with empty titles or authors and avoids checking out unavailable books.

## Testing Process

The project follows a **Test-Driven Development (TDD)** approach, where tests are written **before** the actual implementation. Each new feature begins with writing test cases that describe the expected behavior. Once tests are defined, the code is implemented to pass those tests, ensuring that the system behaves as expected from the start.

### Step-by-Step Testing Process:

1. **Red Phase**: Write the tests before implementing the methods.
   - We wrote tests for both `add_book` and `check_out_book` methods.
   - Example tests include:
     - Adding a valid book to the library.
     - Checking out a book that exists and is available.
     - Checking out a non-existent or unavailable book.
   
2. **Green Phase**: Implement the code to pass the tests.
   - The code for the `add_book` and `check_out_book` methods was written to meet the expectations defined by the tests.
   - Edge cases such as adding a book with an empty title or checking out a book that's already checked out were also handled.
   
3. **Refactor Phase**: Clean up the code while ensuring the tests still pass.
   - After passing all tests, the code was refactored to improve readability and maintainability without altering functionality.

### Test Results

The initial test cases covered a variety of normal and edge scenarios. Here are some examples of scenarios that were tested:

- **Normal Scenarios**:
  - Successfully adding a book with valid title and author.
  - Successfully checking out an available book.
  
- **Edge Cases**:
  - Attempting to add a book with an empty title or author.
  - Attempting to check out a book that doesn’t exist.
  - Trying to check out a book that’s already checked out.

# Challenges Encountered

During the development process, a few challenges were encountered:

## Unused Variables and Fields:

- The `author` field in the `Book` struct was not used after adding books to the collection, which led to warnings. While these warnings were addressed by marking them for future use, it highlighted the need to integrate features such as searching by author.
- Similarly, the `user` parameter in the `check_out_book` method was initially unused. This indicated a gap in the system where the library might eventually need to track who checked out each book.

## Handling Non-Existent Books:

An additional layer of validation was required to handle attempts to check out books that were never added to the collection. This required revisiting the test cases and updating the `check_out_book` method to return appropriate feedback when books were not found.

## Refining Edge Case Tests:

Some tests, such as adding books with empty titles or checking out unavailable books, revealed areas where the system lacked robustness. Additional code was written to handle these edge cases, ensuring that the system could handle real-world scenarios gracefully.

---

# Lessons Learned

- **Test-Driven Development (TDD)** encourages writing cleaner, more reliable code. By writing the tests first, the development process became more structured, and potential issues were identified early.
- Writing comprehensive tests for **edge cases** is crucial. The edge cases helped us think about scenarios that may not be immediately obvious but could lead to bugs if not handled properly.
- Rust’s **strict compiler checks** (e.g., unused variables, dead code) greatly helped in maintaining code quality and avoiding common mistakes.

# How to Run the Project

To run this project locally, follow these steps:

## Step 1: Clone the Repository

First, clone the repository to your local machine and navigate to the project directory:

git clone https://github.com/your_username/library_management.git
cd library_management

## Step 2: build the project

BUild the project using cargo(Rust package manager)

## Step 3: Running the tests

Execute the unit test to see if everything is actually working 

run: cargo test