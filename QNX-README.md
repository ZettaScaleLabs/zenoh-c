<img src="https://raw.githubusercontent.com/eclipse-zenoh/zenoh/master/zenoh-dragon.png" height="150">

# Eclipse Zenoh-C - QNX Support

**Support for QNX 7.1 is currently a work in progress with limited functionality supported.**

To compile Zenoh-C for QNX 7.1 a QNX development environment and a version of the Rust toolchain supporting the QNX targets are required. Internal development testing is carried out on the `x86_64-pc-nto-qnx710` target. The `aarch64-unknown-nto-qnx710` target is also supported.

## Supported Zenoh Features

The following Zenoh features are currently supported:

* auth_pubkey
* auth_usrpwd
* transport_multilink
* transport_compression
* transport_tcp
* transport_udp
* transport_ws

## How to build it

1. Clone the [source] with `git`:

   ```bash
   git clone https://github.com/eclipse-zenoh/zenoh-c.git
   ```

   [source]: https://github.com/eclipse-zenoh/zenoh-c

2. Checkout the `qnx-port-1.0.0` branch:

   ```bash
   cd zenoh-c
   git checkout qnx-port-1.0.0
   ```

3. Build Zenoh-C:

   Good CMake practice is to perform build outside of source directory, leaving source tree untouched. The examples below demonstrates this mode of building.

   ```bash
   mkdir -p build && cd build
   CC=qcc CFLAGS=-Vgcc_ntox86_64_cxx CXX=qcc AR=ntox86_64-ar cmake ../zenoh-c -DZENOHC_CARGO_FLAGS="--no-default-features;--features=zenoh/auth_pubkey,zenoh/auth_usrpwd,zenoh/transport_multilink,zenoh/transport_compression,zenoh/transport_tcp,zenoh/transport_udp,zenoh/transport_ws" -DZENOHC_CUSTOM_TARGET="x86_64-pc-nto-qnx710" -DCMAKE_INSTALL_PREFIX=<install location>
   cmake --build .
   ```

4. Build the examples (optional):

   To build the examples run the following command:

   ```bash
   cmake --build . --target examples
   ```

5. Install:

   To install zenoh-c library into system just build target `install`.

   ```bash
   cmake --build . --target install
   ```  

   By default only dynamic library is installed. Set `ZENOHC_INSTALL_STATIC_LIBRARY` variable to true to install static library also:

   ```bash
   CC=qcc CFLAGS=-Vgcc_ntox86_64_cxx CXX=qcc AR=ntox86_64-ar cmake ../zenoh-c -DZENOHC_CARGO_FLAGS="--no-default-features;--features=zenoh/auth_pubkey,zenoh/auth_usrpwd,zenoh/transport_multilink,zenoh/transport_compression,zenoh/transport_tcp,zenoh/transport_udp,zenoh/transport_ws" -DZENOHC_CUSTOM_TARGET="x86_64-pc-nto-qnx710" -DCMAKE_INSTALL_PREFIX=<install location> -DZENOHC_INSTALL_STATIC_LIBRARY=TRUE
   cmake --build . --target install
   ```

   The result of installation is the header files in `include` directory, the library files in `lib` directory and cmake package configuration files for package `zenohc` in `lib/cmake` directory. The library later can be loaded with CMake command `find_package(zenohc)`.
   Add dependency in CMakeLists.txt on target

   - `zenohc::shared` for linking dynamic library
   - `zenohc::static` for linking static library
   - `zenohc::lib` for linking static or dynamic library depending on boolean variable `ZENOHC_LIB_STATIC`

   For `Debug` configuration the library package `zenohc_debug` is installed side-by-side with release `zenohc` library. Suffix `d` is added to names of library files (libzenohc**d**.so).
