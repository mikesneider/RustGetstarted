# Get Started Rust
Trying Rust, to moving forward into Solana projects
adding - Guessing Game


### Understanding errors
'''
error[E0308]: mismatched types
  --> src/main.rs:17:27
   |
17 |     io::stdin().read_line(&mut algo);
   |                           ^^^^^^^^^ expected struct `String`, found `&str`
   |
   = note: expected mutable reference `&mut String`
              found mutable reference `&mut &str`

'''
This error is because if different String to $str (C++ basis)