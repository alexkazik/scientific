# Macro for Scientific (Arbitrary precision scientific number)

Allows the creation of `Scientific` number constants.

```
use scientific_macro::Scientific;
let n1 = Scientific!(1e100);
let n2 = Scientific!(1e80);
assert_eq!(&n1 + &n2, Scientific!(1.00000000000000000001e100));
// An f64 has only a precision of about 15.9 digits, this are already 21.
```
