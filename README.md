# README

So, this works, kind of.

The workflow I have found (not the same as the [docs](https://usehelix.com/getting_started#step-1-create-a-new-rails-project)), is...

1. Add the helix gem + `bundle install`
2. rails generate helix:crate some_crate
3. go to crate root + `cargo build`
4. `bundle install` again (to pick up crate as gem - not sure this is needed)
5. Run `rake build` (succeeds but ends with segmentation fault = not very reassuring)

Repeat 3 - 5 as edit the Ruby in the Rust file.

