Ring can be built as a Rust library or as a C library.



Building the Rust Library
=========================

*ring*'s Rust crate is named ```ring```. You can build it
using ```cargo build --release``` and you can run the tests
with ```cargo test --release```. When you use ```cargo build```, you don't need
to follow the instructions below to build the C code separately, because
[build.rs](build.rs) does that automatically. Unlike a lot of Rust wrappers
around C libraries, the build script for ```ring``` is implemented using the
msbuild projects (on Windows) or the Makefile (on other platforms). Because
this is a little unusual, I would be particularly grateful if you could report
any problems building (or using) *ring*'s Rust crate.



Building the C Library on Linux and Similar Platforms
=====================================================

Here are the major differences between *ring* and BoringSSL & OpenSSL that
affect building:

* BoringSSL uses CMake and OpenSSL uses make(1). *ring* uses Visual Studio's
  native build system (msbuild) on Windows, and GNU Make otherwise.
* BoringSSL and OpenSSL both support building static or shared libraries.
  *ring* is only supported in static library form.
* *ring*'s static library is named ring-core.lib on Windows and libring-core.a
  on other platforms. BoringSSL and OpenSSL use different names.

There is no ./configure step.

GNU Make 3.81 or later is required. Perl 5.6.1 or later is also required
(unless you disable all the assembly language optimizations by building
with ```NO_ASM=1```). *ring* is designed with cross-compilation in mind so it
expects variables ```CC```, ```CXX```, and ```TARGET``` to be passed
to ```make```. For example, this will build a 32-bit x86 *ring* for Linux using
GCC 4.8:

    make -j6 CC=gcc-4.8 CXX=g++-4.8 TARGET=i586-pc-linux-gnu

You must include all four parts in ```TARGET```, except on Mac OS X you can
omit the ABI. For example, this will build a 64-bit x86-64 *ring* for Mac OS X:

    make -j6 CC=clang CXX=clang++ TARGET=x86_64-apple-darwin

All four parts of the target are required.

GCC 4.8 and later are supported, as are clang 3.4 and later. Other compilers
will also probably work without too much trouble. Note in particular that if
you are cross-compiling an x86 build on a 64-bit version of Linux, then you
need to have the proper gcc-multilibs and g++-multilibs packages or equivalent
installed.

The default build is a release build (```CMAKE_BUILD_TYPE=RELWITHDEBINFO```).
You can build a debug build by setting ```CMAKE_BUILD_TYPE``` to ```DEBUG```.
(Note that the variable is named to be consistent with CMake, but CMake is not
used.) For example, this will build *ring* in release mode with the default
version of clang on Mac OS X:

    make -j6 CC=clang-3.6 CXX=clang++-3.6 TARGET=x86_64-pc-linux-gnu CMAKE_BUILD_TYPE=DEBUG

Then compile your applications with ```-Iring/include``` (assuming you put *ring*
into the ```ring``` subdirectory of your project) and add ```$(RING_LDFLAGS)```
to LDFLAGS in your linking step. ```RING_LDFLAGS``` expands by default
to ```-pthread -Lbuild/$TARGET-$CC/lib -lring-core```. (It should also be
easy to build *ring* so that it doesn't depend on pthreads, but the build system
hasn't been enhanced to fully support that yet.)

Running the tests using ```make check``` requires Go (https://golang.org/) to
be in ```$PATH```. Example:

    make check -j6 CC=clang-3.6 CXX=clang++-3.6 TARGET=x86_64-pc-linux-gnu



Building the C Library on Windows
=================================

Note that currently the assembly language optimizations are NOT built on
Windows yet, only because the additions to the project files to support doing
so haven't been made yet.

The Windows build requires Visual Studio 2013 or later. Any edition will work.
Open ```ring.sln``` in Visual Studio and choose Build|Build Solution.
Alternatively, from a Visual Studio Native Tools Command Prompt:

    msbuild ring.sln

The built ```ring-core.lib``` will be put into a subdirectory of ```build/```
depending on which configuration and which platform you choose to build for.
For example, in a 32-bit release build, the result
is ```build\Win32-Release\lib\ring.lib```. In your application's project, add
*ring*'s ```include/``` subdirectory to the "Additional Include Directories,"
add the directory containing ```ring-core.lib``` to the
"Additional Library Directories", and add ```ring-core.lib``` to the
"Additional Dependencies."

On Windows, *ring* can be build in Debug or Release configurations for
platforms Win32 or x64. (It should be easy to add support for the ARM platform
too.) The solution builds correctly using either Visual Studio 2013 or Visual
Studio 2015 without any conversion steps being necessary. The project files are
mostly hand-written and rely heavily on the property sheets in ```mk\*.props```.
All Visual Studio features should work. However, Visual Studio's project property
editor often wrongly shows properties set in the property sheets as being
unset. In general, it is much better to edit the ```*.props``` files by hand
instead of using Visual Studio's GUI to edit the projects.

The tests currently require Go (https://golang.org/). For a 64-bit release
build, run this from within the *ring* subdirectory:

    go run util/all_tests.go --build-dir=build/x64-Release/test/ring