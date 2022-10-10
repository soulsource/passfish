# Harbour-PassFish

PassFish is a reimplementation of [PasswordMaker Pro](https://passwordmaker.org/) for Sailfish OS, that aims to be mostly compatible to
the [JavaScript edition](https://sourceforge.net/projects/passwordmaker/files/Javascript%20Edition/).

All credit for the development of the PasswordMaker Pro algorithm (and therefore for the high level flow of this library too) goes to the original authors of PasswordMaker Pro, [Miquel Burns](https://github.com/miquelfire) and [Eric H. Jung](https://github.com/ericjung). (I really hope I linked the correct profiles.)

The project consists of two parts, which are developed in tandem, but published under different licenses.
This part, the main application ("PassFish"), is licensed under the [GPL-3.0-or-later](https://spdx.org/licenses/GPL-3.0-or-later.html) (see LICENSE file).  The library containing the PasswordMaker logic
("[passwordmaker-rs](https://github.com/soulsource/passwordmaker-rs)") is licensed under LGPLv3 or later.

## Architecture
PassFish itself is mainly developed in Rust, with the GUI being defined in QML. The QML part also takes on the role of a Controller, as it is driving
the Rust logic.  The glue between those two parts is mostly auto-generated using [rust-qt-binding-generator](https://invent.kde.org/sdk/rust-qt-binding-generator),
but a few lines of glue code are also written by hand.

The Rust code can be roughly separated in three modules: The main part of the program, which is simply called passwordmaker, and which is responsible
for the interaction with the main page of the QML UI, the profiles, which are stored and read from disc, and the helperthread, which manages the actual
generation of the passwords.
Both, helperthread and passwordmaker modules interact with passwordmaker-rs.

## Building
To be able to build this project you will (obviously) need the Sailfish SDK.
Last time I checked the Virtualbox based build engine still had issues with cargo, so (unless those have been resolved in the meantime) you might want to install the Docker based build engine.
See https://forum.sailfishos.org/t/rust-howto-request/3187/10 for further information.

To install the Rust compiler in your Sailfish SDK build target, you can use the Sailfish SDK Qt Creator.  The relevant dialogue is under "Settings" -> "Sailfish OS" -> "Manage Build Targets".  For each build target, please install "cargo" and the rust standard library ("rust-std-static-<target>").

To update the Rust bindings (a build step in the project's CMakelists.txt) for Qt you'll also need to install [rust-qt-binding-generator](https://invent.kde.org/sdk/rust-qt-binding-generator) in the build engine VM.  This project uses a modified version, available in the [mockall_support](https://invent.kde.org/soulsource/rust-qt-binding-generator/-/tree/mockall_support) branch.  This branch is used to ease the creation of automated tests, and it includes some rather drastic changes to work around an undefined behaviour bug.

The cmake file will find the rust_qt_binding_generator binary if it's either in the PATH or in /home/mersdk/bin.
Since the build engine is using a rather old C library, cross compiling rust-qt-binding-generator on the host might not work.  When in doubt, you can just build it with the Sailfish build engine by running `sfdk engine exec sb2 -t SailfishOS-X.X.X.X-i486 cargo build`.  You can also [remote into the build engine](https://docs.sailfishos.org/Tools/Sailfish_SDK/FAQ/#build-engine), and just build it there (using a temporary `zypper install cargo` install, but don't forget to uninstall it again afterwards).

Last, but not least, I decided not to add any hacky auto-detect steps for the rust target triplet to CMakeLists.txt.  Instead you need to give the value to cmake on the cmake command line.  To do that with Qt Creator, open the Projects view and for each target and build configuration add the `-DRUST_TARGET_TRIPLET=some-target-triplet` setting to the "Initial CMake parameters" list.  I've used the same triplets that are being used by the official [rust.spec](https://github.com/sailfishos/rust/blob/5a164a7d8f91fc147458222f92db35b7567bac1c/rust.spec#L35-L37) file and those seem to work fine.  Then re-configure the target with the now changed initial parameters.

With that all set up an running, you should be able to build the project using the Qt Creator that comes with the Sailfish SDK.

## Notes for GUI development/porting
The model part of the application is (nearly) fully implemented in Rust, so adding a UI for another platform is as straightforward as adding new views (in either Qt Widgets and/or QML).

There is one **important** thing to take care of when adding another UI implementation: The rust-qt-binding-generator does (at the time of writing) not offer any way to add custom signals/slots to the auto-generated QObjects, and also access to the QObject pointer is private on the Rust side.  In other words: There is no way to set up connections from Rust.
Where I'm getting at is that if you create a new `PasswordMaker` object in either C++ or QML, you need to connect a signal:
The signal `i_say_sexy_things_to_myself_while_im_dancingChanged` needs to be connected to `set_i_say_sexy_things_to_myself_while_im_dancing`.  See PassFish.cpp for
details.
