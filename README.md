# README

So, this works, kind of.

The workflow I have found (not the same as the [docs](https://usehelix.com/getting_started#step-1-create-a-new-rails-project)), is...

1. Add the helix gem + `bundle install`
2. rails generate helix:crate some_crate
3. go to crate root + `cargo build`
4. `bundle install` again (to pick up crate as gem - not sure this is needed, but maybe 1st time after crate created?)
5. Run `rake build` (succeeds but ends with segmentation fault = not very reassuring)

Repeat 5 whenever edit the Rust file.

## What about Rutie?

Clearly, Helix has some issues, and is now a deprecated project. So, what about Rutie?

Interesting article on why Deliveroo chose Rutie over Helix: [Moving from Ruby to Rust](https://deliveroo.engineering/2019/02/14/moving-from-ruby-to-rust.html#performance-improvements)

Let's try following this tutorial: https://vericred.com/using-rust-to-speed-up-your-ruby-apps-part-2-how-to-use-rust-with-ruby/
And simplify it by mixing in stuff from the [Rutie Docs - Using Rust in Ruby](https://github.com/danielpclark/rutie#using-rust-in-ruby)

Now we can simply edit and run our Rust by...
1. Run `$ cargo build --release` in the rooot of our `example_rust_lib`
2. Restart rails server

Lots of warnings ('Not FFI safe', etc), but no failures or seg faults! Nice!!
We're loading a whole of dependencies (serde stuff, etc.) in our `cargo.toml` (since this comes from the more complex Vericred approach), but these could be useful for more advanced stuff (passing in/out more complex data structures, multi-threading, etc.)