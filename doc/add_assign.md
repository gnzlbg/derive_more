% What #[derive(AddAssign)] generates

This code is very similar to the code that is generated for `#[derive(Add)]`.
The difference is that it mutates the existing instance instead of creating a
new one.

# Tuple structs

When deriving for a tuple struct with two fields like this:

```rust
#[derive(AddAssign)]
struct MyInts(i32, i32)
```

Code like this will be generated:

```rust
impl ::std::ops::AddAssign for MyInts {
    fn add_assign(&mut self, rhs: MyInts) {
        self.0.add_assign(rhs.0);
        self.1.add_assign(rhs.1);
    }
}
```

The behaviour is similar with more or less fields.



# Regular structs

When deriving for a tuple struct with two fields like this:

```rust
#[derive(AddAssign)]
struct MyInts(i32, i32)
```

Code like this will be generated:

```rust
impl ::std::ops::AddAssign for MyInts {
    fn add_assign(&mut self, rhs: MyInts) {
        self.0.add_assign(rhs.0);
        self.1.add_assign(rhs.1);
    }
}
```

The behaviour is similar with more or less fields.


# Enums

Deriving `AddAssign` is not (yet) supported for enums.
This is mostly due to the fact that it is not trivial convert the `Add`
derivation code, because that returns a `Result<EnumType>` instead of an
`EnumType`.
Handling the case where it errors would be hard and maybe impossible.
