% What #[derive(Into)] generates

This derive creates the the exact oposite of [`#[derive(From)]`](from.html).
Instead of allowing you to create a new instance of the struct from the values
it should contain, it allows you to extract the values from the struct.
One thing to note is that this derive doesn't actually generate an
implementation for the `Into` trait.
Instead it derives `From` for the values contained in the struct and thus has an
indirect implementation of `Into` as recommended by the
[docs](https://doc.rust-lang.org/core/convert/trait.Into.html).

# Tuple structs

When deriving `Into` for a tuple struct with a single field (i.e. a newtype) like this:

```rust
#[derive(Into)]
struct MyInt(i32)
```

Code like this will be generated:

```rust
impl ::std::convert::From<MyInt> for (i32) {
    fn from(original: MyInt) -> (i32) {
        (original.0)
    }
}
```

The behaviour is a bit different when deriving for a struct with multiple
fields, since it returns a tuple. For instance when deriving for a tuple struct
with two fields like this:

```rust
#[derive(Into)]
struct MyInts(i32, i32)
```

Code like this will be generated:

```rust
impl ::std::convert::From<MyInts> for (i32, i32) {
    fn from(original: MyInts) -> (i32, i32) {
        (original.0, original.1)
    }
}
```



# Regular structs

For regular structs almost the same code is generated as for tuple structs
except in the way the field values are assigned to the new struct.
When deriving for a regular struct with a single field like this:

```rust
#[derive(Into)]
struct Point1D {
    x: i32,
}
```

Code like this will be generated:

```rust
impl ::std::convert::From<Point1D> for (i32) {
    fn from(original: Point1D) -> (i32) {
        (original.x)
    }
}
```

The behaviour is again a bit different when deriving for a struct with multiple
fields, because this also returns a tuple. For instance when deriving for a
tuple struct with two fields like this:

```rust
#[derive(Into)]
struct Point2D {
    x: i32,
    y: i32,
}

```

Code like this will be generated:

```rust
impl ::std::convert::From<Point2D> for (i32, i32) {
    fn from(original: Point2D) -> (i32, i32) {
        (original.x, original.y)
    }
}
```


# Enums

Deriving `Into` for enums is not supported as it would not always be successful.
This is what the currently unstable
[`TryInto`](https://doc.rust-lang.org/core/convert/trait.TryInto.html) should be
used for, which is currently not supported by this library.
