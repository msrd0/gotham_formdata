initSidebarItems({"struct":[["BorrowBag","`BorrowBag` allows the storage of any value using `add(T)`, and returns a `Handle` which can be used to borrow the value back later. As the `BorrowBag` is add-only, `Handle` values remain valid for the lifetime of the `BorrowBag`."],["Handle","A value which can be used with the `BorrowBag` to borrow the element which was added."]],"trait":[["Append","Describes the result of appending `T` to the borrow-bag. This is useful in specifying the return type when creating/modifying a `BorrowBag` in a function."],["Lookup","Allows borrowing a value of type `T` from the implementing type. This can be used to constrain a `Handle` argument to ensure it can be used with the corresponding `BorrowBag`."]]});