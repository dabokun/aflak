# Contributing

Contributions are welcome, and they are greatly appreciated! Every
little bit helps, and credit will always be given.

## Bug reports

When [reporting a bug](https://github.com/aflak-vis/aflak/issues) please
include:

* Your operating system name and version.
* Your `aflak` version. You can get it by typing `aflak --version` in the
    terminal.
* Any details about your local setup that might be helpful in
    troubleshooting.
* Detailed steps to reproduce the bug.

## Feature requests and feedback

The best way to send feedback is to file an issue at
https://github.com/aflak-vis/aflak/issues.

If you are proposing a feature:

* Explain in detail how it would work.
* Keep the scope as narrow as possible, to make it easier to implement.
* Code contributions are welcome :)

## Development

To set up `aflak` for local development:

*1.* Fork [aflak](https://github.com/aflak-vis/aflak)
   (look for the "Fork" button).

*2.* Clone your fork locally::

```sh
git clone git@github.com:your_name_here/aflak.git
```

*3.* Create a branch for local development::

```sh
git checkout -b name-of-your-bugfix-or-feature
```

   Now you can make your changes locally.

*4.* When you're done making changes, run all the checks:

```sh
cd src
cargo test --all
cargo fmt
```

*5.* Commit your changes and push your branch to GitHub:

```sh
git add your-changes
git commit
git push origin name-of-your-bugfix-or-feature
```

*6.* Submit a pull request through the GitHub website.

### Pull Request Guidelines

If you need some code review or feedback while you're developing the code just
make the pull request.


# Software structure (for the developer)

aflak's entry point is in `aflak/src/src/main.rs`.

This repo has several crates each with a specific and defined objective.
Each of the crates is documented. Please refer to their respective doc using `cargo doc --open`.

- **aflak_cake** (*Computational mAKE*): Manage node graph and computational tasks (back-end).

Example for opening **aflak_cake**'s doc:

```sh
cd aflak/src/aflak_cake
# Open the doc
cargo doc --open
```

- **aflak_plot**: Visualization library built on *imgui* for *aflak*.
- **aflak_primitives**: Define transformation types and primitives for use in
astrophysics. All built-in nodes are defined in this crate.
Documentation about the list of node can be found
[in the wiki](https://github.com/aflak-vis/aflak/wiki/%E3%83%8E%E3%83%BC%E3%83%89%E4%B8%80%E8%A6%A7)
(in Japanese):
- **node_editor**: Node editor built on *aflak_cake* and *imgui*.
