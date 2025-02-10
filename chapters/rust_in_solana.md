## Rust in Solana

### Arithmetic Overflow

* In the Carg.toml file,
    `overflow-checks = true`

```rust
[workspace]
members = [
    "programs/*"
]
resolver = "2"

[profile.release]
overflow-checks = true
lto = "fat"
codegen-units = 1
[profile.release.build-override]
opt-level = 3
incremental = false
codegen-units = 1
```

* adding overflow checks increases the compute cost of the transaction

* under some circumstances where compute cost is an issue, we need to set overflow-checks to false. To strategically check for overflows, you can use the Rust checked_* operators in Rust.

```rust
let x: u64 = y + z; // will silently overflow
let xSafe: u64 = y.checked_add(z).unwrap(); // will panic if overflow happens

// checked_sub, checked_mul, etc are also available
```

#### compute units

* By default, a transaction is limited to 200,000 compute units. If more than 200,000 compute units are consumed, the transaction reverts.

<br>

----

### IDL(Interface Definition Language)

* The IDL provides a standardized JSON file describing the Solana program's instructions and accounts.

* An IDL file in Solana plays a similar role as the ABI file in Solidity, specifying how to interact with the program’s/contract’s.

* Functions in Rust are snake_cased, but Anchor formats them in JavaScript land as camelCased.

<br>

----

### Function Visibility

* Public / External Functions: These are functions accessible both within and outside the program. In Solana, all functions declared are, by default, public. Everything in the #[program] block must be declared pub.

* Internal Functions: These are functions accessible within the program itself and programs that inherit it. Functions inside a nested pub mod block are not included in the built program, but still, they can be accessed within or outside the parent module.

* Private Functions: These are functions that are not publicly accessible and cannot be invoked from outside their module. Achieving private visibility in Rust/Solana involves defining a function within a specific module with the pub(in crate::<module>) keyword, which makes the function visible within just the module it was defined in.


* examples
```rust
#[program]
pub mod func_visibility {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("get_a_num: {}", get_a_num());
        some_private_function::private_function();

        Ok(())
    }

    pub fn add_two_numbers(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let result = calculate::add(x, y);
        
        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }

    pub mod some_internal_function {
        pub fn internal_function() {
            // Internal function logic...
        }
    }

    pub mod some_private_function {
        pub(in crate::func_visibility) fn private_function() {
            // Private function logic
        }
    }
    // The pub(in crate::func_visibility) keyword indicates
    // that private_function function is only visible within 
    // func_visibility module.
}

mod do_someting {
    // Import func_visibility module
    use crate::func_visibility;

    // we were able to access 
    // internal_function function from within its “parent” module
    // (func_visibility) and also from a separate module (do_something)
    // outside the func_visibility module.
    pub fn some_func_here() {
        // Call the internal_function from outside its parent module
        func_visibility::some_internal_function::internal_function();

        // Do something else...    
    }
}

mod calculate {
    pub fn add(x: u64, y: u64) -> u64 {
        x + y
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// a non pub function over here
fn get_a_num() -> u64 {
    2
}
```