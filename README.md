# Point ABSTraction

This is a point abstraction library, written in Rust, inspired by [PDAL](http://www.pdal.io/), specifically engineered to work with LiDAR data.

[![Build Status](https://travis-ci.org/gadomski/pabst.svg?branch=master)](https://travis-ci.org/gadomski/pabst)


## Another point abstraction library?

Yup!

**pabst** aims to be smaller in scope than PDAL.
Specifically, where PDAL tries to be a swiss-army knife and includes tools for manipulating and transforming data as well as converting between formats, **pabst** sticks to data format translation.
Thanks to Rust's dependency system, it is much easier to set up and use upstream projects, and so including **pabst** in your existing toolchain is easier than a similar operation would be in C++ land.

**pabst** is also more opinionated about the dimensions and formats that it supports.
It does not (as of this writing) have any support for formats-via-plugin or extra dimensions, the way that PDAL does.
This is because, again, Rust's dependency system makes it much easier to incorporate upstream projects ([las-rs](https://github.com/gadomski/las-rs) for one example) and use them to do the heavy lifting.
**pabst** will never do any actual format conversion itself, it will simply serve as a glue between other upstream libraries.


## License

This code is available under the MIT license, available in this source tree.


## Contributing

Issues and pull requests, you know the drill.
