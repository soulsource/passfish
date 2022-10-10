<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE TS>
<TS version="2.1" language="en_GB" sourcelanguage="en_GB">
<context>
    <name>AboutPage</name>
    <message>
        <source>About PassFish</source>
        <translation>About PassFish</translation>
    </message>
    <message>
        <source>
&lt;p&gt;
    PassFish is a native re-implementation of &lt;a href=&quot;https://passwordmaker.org/&quot;&gt;PasswordMaker&lt;/a&gt; for Sailfish OS.
    It aims to be mostly compatible to the &lt;a href=&quot;https://sourceforge.net/projects/passwordmaker/files/Javascript%20Edition/&quot;&gt;Javascript Edition&lt;/a&gt;.
&lt;/p&gt;
&lt;p&gt;
    All credit for the development of the PasswordMaker Pro algorithm goes to the original authors of PasswordMaker Pro, Miquel Burns and Eric H. Jung.
&lt;/p&gt;
&lt;p&gt;
    As PassFish is not merely a port, but a full reimplementation from scratch, compatibility is not guaranteed. While the original source code was
    used as a guideline during implementation, the underlying technology is vastly different. Common use cases are tested by integration tests, but some
    edge cases might have been missed. In case you encounter such an issue, please report it on the
    &lt;a href=&quot;https://github.com/soulsource/passfish/issues&quot;&gt;github issue tracker&lt;/a&gt; of the project.
&lt;/p&gt;
&lt;p&gt;
    Speaking of integration tests: The hash algorithms were not re-implemented for this project in order to reduce the risk of introducing bugs. Instead the
    QCryptographicHash API is used where possible, and where not, the implementation from the RustCrypto Hashes repository is utilized (see list of dependencies below).
&lt;/p&gt;
&lt;p&gt;
    This program consists of two parts: The application itself (&quot;&lt;a href=&quot;https://github.com/soulsource/passfish&quot;&gt;PassFish&lt;/a&gt;&quot;), and a Rust crate that contains the
    implementation of the business logic (&quot;&lt;a href=&quot;https://github.com/soulsource/passwordmaker-rs&quot;&gt;&lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt;&lt;/a&gt;&quot;).
    This is important, as the two parts use different licenses. PassFish is published under the GPLv3 license, while &lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt; is published under LGPLv3. Please check the linked github pages for more details.
&lt;/p&gt;
&lt;p&gt;
    This program utilises several third party libraries. This list is kept up-to-date to the best of my knowledge. Only direct dependencies are listed,
    for transitive dependencies please see the linked websites. Similarly, the source code for those third-party dependencies that are published under an
    open source license can be found on the linked websites. To my knowledge the only non-open-source dependency is Silica. The listed licenses are just those
    used by &lt;i&gt;this&lt;/i&gt; project, most libraries are available under multiple licenses. Please see the libraries&apos; websites for details.&lt;br&gt;
    These libraries are linked dynamically:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://www.qt.io/&quot;&gt;Qt Quick&lt;/a&gt;: Used under &lt;a href=&quot;https://www.gnu.org/licenses/lgpl-3.0.en.html&quot;&gt;LGPLv3&lt;/a&gt;&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://sailfishos.org/develop/docs/silica/&quot;&gt;Sailfish Silica&lt;/a&gt;: Proprietary, &lt;a href=&quot;https://www.gnu.org/licenses/gpl-3.0&quot;&gt;GPL&lt;/a&gt; system library exception&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://github.com/sailfishos/libsailfishapp&quot;&gt;LibSailfishApp&lt;/a&gt;: Used under LGPL 2.1&lt;/li&gt;
    &lt;/ul&gt;
    These libraries and their dependencies are linked statically:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/libc&quot;&gt;libc Rust bindings&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/serde&quot;&gt;serde&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/toml&quot;&gt;toml-rs&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/dirs&quot;&gt;dirs&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/ripemd&quot;&gt;RustCrypto: RIPEMD&lt;/a&gt;: Used under MIT license&lt;/li&gt;
    &lt;/ul&gt;
    The &lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt; library has the following statically linked runtime dependencies:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/unicode-segmentation&quot;&gt;unicode-segmentation&lt;/a&gt;: Used under MIT license&lt;/li&gt;
    &lt;/ul&gt;
    While it is not a runtime dependency, the code generator for the Rust Qt bindings is worth noting:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://invent.kde.org/sdk/rust-qt-binding-generator&quot;&gt;Rust Qt Binding Generator&lt;/a&gt;&lt;/li&gt;
    &lt;/ul&gt;
    PassFish uses a modified version, which can be found in the &lt;a href=&quot;https://invent.kde.org/soulsource/rust-qt-binding-generator/-/tree/mockall_support&quot;&gt;mockall_support&lt;/a&gt; branch.
&lt;/p&gt;
&lt;p&gt;
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation,
    either version 3 of the License, or (at your option) any later version.
    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
    See the GNU General Public License for more details.
    You should have received a copy of the GNU General Public License along with this program. If not, see &lt;a href=&quot;http://www.gnu.org/licenses/&quot;&gt;http://www.gnu.org/licenses/&lt;/a&gt;.
&lt;/p&gt;
                </source>
        <translation>
&lt;p&gt;
    PassFish is a native re-implementation of &lt;a href=&quot;https://passwordmaker.org/&quot;&gt;PasswordMaker&lt;/a&gt; for Sailfish OS.
    It aims to be mostly compatible to the &lt;a href=&quot;https://sourceforge.net/projects/passwordmaker/files/Javascript%20Edition/&quot;&gt;Javascript Edition&lt;/a&gt;.
&lt;/p&gt;
&lt;p&gt;
    All credit for the development of the PasswordMaker Pro algorithm goes to the original authors of PasswordMaker Pro, Miquel Burns and Eric H. Jung.
&lt;/p&gt;
&lt;p&gt;
    As PassFish is not merely a port, but a full reimplementation from scratch, compatibility is not guaranteed. While the original source code was
    used as a guideline during implementation, the underlying technology is vastly different. Common use cases are tested by integration tests, but some
    edge cases might have been missed. In case you encounter such an issue, please report it on the
    &lt;a href=&quot;https://github.com/soulsource/passfish/issues&quot;&gt;github issue tracker&lt;/a&gt; of the project.
&lt;/p&gt;
&lt;p&gt;
    Speaking of integration tests: The hash algorithms were not re-implemented for this project in order to reduce the risk of introducing bugs. Instead the
    QCryptographicHash API is used where possible, and where not, the implementation from the RustCrypto Hashes repository is utilized (see list of dependencies below).
&lt;/p&gt;
&lt;p&gt;
    This program consists of two parts: The application itself (&quot;&lt;a href=&quot;https://github.com/soulsource/passfish&quot;&gt;PassFish&lt;/a&gt;&quot;), and a Rust crate that contains the
    implementation of the business logic (&quot;&lt;a href=&quot;https://github.com/soulsource/passwordmaker-rs&quot;&gt;&lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt;&lt;/a&gt;&quot;).
    This is important, as the two parts use different licenses. PassFish is published under the GPLv3 license, while &lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt; is published under LGPLv3. Please check the linked github pages for more details.
&lt;/p&gt;
&lt;p&gt;
    This program utilises several third party libraries. This list is kept up-to-date to the best of my knowledge. Only direct dependencies are listed,
    for transitive dependencies please see the linked websites. Similarly, the source code for those third-party dependencies that are published under an
    open source license can be found on the linked websites. To my knowledge the only non-open-source dependency is Silica. The listed licenses are just those
    used by &lt;i&gt;this&lt;/i&gt; project, most libraries are available under multiple licenses. Please see the libraries&apos; websites for details.&lt;br&gt;
    These libraries are linked dynamically:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://www.qt.io/&quot;&gt;Qt Quick&lt;/a&gt;: Used under &lt;a href=&quot;https://www.gnu.org/licenses/lgpl-3.0.en.html&quot;&gt;LGPLv3&lt;/a&gt;&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://sailfishos.org/develop/docs/silica/&quot;&gt;Sailfish Silica&lt;/a&gt;: Proprietary, &lt;a href=&quot;https://www.gnu.org/licenses/gpl-3.0&quot;&gt;GPL&lt;/a&gt; system library exception&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://github.com/sailfishos/libsailfishapp&quot;&gt;LibSailfishApp&lt;/a&gt;: Used under LGPL 2.1&lt;/li&gt;
    &lt;/ul&gt;
    These libraries and their dependencies are linked statically:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/libc&quot;&gt;libc Rust bindings&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/serde&quot;&gt;serde&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/toml&quot;&gt;toml-rs&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/dirs&quot;&gt;dirs&lt;/a&gt;: Used under MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/ripemd&quot;&gt;RustCrypto: RIPEMD&lt;/a&gt;: Used under MIT license&lt;/li&gt;
    &lt;/ul&gt;
    The &lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt; library has the following statically linked runtime dependencies:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/unicode-segmentation&quot;&gt;unicode-segmentation&lt;/a&gt;: Used under MIT license&lt;/li&gt;
    &lt;/ul&gt;
    While it is not a runtime dependency, the code generator for the Rust Qt bindings is worth noting:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://invent.kde.org/sdk/rust-qt-binding-generator&quot;&gt;Rust Qt Binding Generator&lt;/a&gt;&lt;/li&gt;
    &lt;/ul&gt;
    PassFish uses a modified version, which can be found in the &lt;a href=&quot;https://invent.kde.org/soulsource/rust-qt-binding-generator/-/tree/mockall_support&quot;&gt;mockall_support&lt;/a&gt; branch.
&lt;/p&gt;
&lt;p&gt;
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation,
    either version 3 of the License, or (at your option) any later version.
    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
    See the GNU General Public License for more details.
    You should have received a copy of the GNU General Public License along with this program. If not, see &lt;a href=&quot;http://www.gnu.org/licenses/&quot;&gt;http://www.gnu.org/licenses/&lt;/a&gt;.
&lt;/p&gt;
                </translation>
    </message>
</context>
<context>
    <name>CoverPage</name>
    <message>
        <source>PassFish</source>
        <translation>PassFish</translation>
    </message>
    <message>
        <source>Used Text:</source>
        <translation>Used Text:</translation>
    </message>
    <message>
        <source>Pass Ready</source>
        <translation>Pass Ready</translation>
    </message>
    <message>
        <source>Input Needed</source>
        <translation>Input Needed</translation>
    </message>
</context>
<context>
    <name>MainPage</name>
    <message>
        <source>App Settings</source>
        <translation>App Settings</translation>
    </message>
    <message>
        <source>About</source>
        <translation>About PassFish</translation>
    </message>
    <message>
        <source>PassFish</source>
        <translation>PassFish</translation>
    </message>
    <message>
        <source>Profile</source>
        <translation>Profile</translation>
    </message>
    <message>
        <source>URL</source>
        <translation>URL</translation>
    </message>
    <message>
        <source>Master Password</source>
        <translation>Master Password</translation>
    </message>
    <message>
        <source>Used Text</source>
        <translation>Used Text</translation>
    </message>
    <message>
        <source>Generating</source>
        <translation>Generating</translation>
    </message>
    <message>
        <source>Missing text to use</source>
        <translation>Missing text to use</translation>
    </message>
    <message>
        <source>Missing master password</source>
        <translation>Missing master password</translation>
    </message>
    <message>
        <source>Error in profile character list</source>
        <translation>Error in profile character list</translation>
    </message>
    <message>
        <source>Generated Password</source>
        <translation>Generated Password</translation>
    </message>
    <message>
        <source>Saving settings failed.</source>
        <translation>Saving settings failed.</translation>
    </message>
</context>
<context>
    <name>ProfileEditor</name>
    <message>
        <source>Edit Profile</source>
        <translation>Edit Profile</translation>
    </message>
    <message>
        <source>Required Field</source>
        <translation>Required Field</translation>
    </message>
    <message>
        <source>Profile Name</source>
        <translation>Profile Name</translation>
    </message>
    <message>
        <source>Profile name required.</source>
        <translation>Profile name required.</translation>
    </message>
    <message>
        <source>Use Protocol</source>
        <translation>Use Protocol</translation>
    </message>
    <message>
        <source>Include URL protocol (e.g. &quot;http://&quot;)</source>
        <translation>Include URL protocol (e.g. &quot;http://&quot;)</translation>
    </message>
    <message>
        <source>Use &quot;undefined&quot; if protocol is missing</source>
        <translation>Use &quot;undefined&quot; if protocol is missing</translation>
    </message>
    <message>
        <source>Enable to mimic weird behaviour of PasswordMaker Pro.</source>
        <translation>Enable to mimic weird behaviour of PasswordMaker Pro.</translation>
    </message>
    <message>
        <source>Use Userinfo</source>
        <translation>Use Userinfo</translation>
    </message>
    <message>
        <source>Include userinfo (e.g &quot;jane_doe:12345&quot;</source>
        <translation>Include userinfo (e.g. &quot;jane_doe:12345&quot;)</translation>
    </message>
    <message>
        <source>Use Subomain(s)</source>
        <translation>Use Subomain(s)</translation>
    </message>
    <message>
        <source>Include URL subdomain(s) (e.g. &quot;www.&quot;)</source>
        <translation>Include URL subdomain(s) (e.g. &quot;www.&quot;)</translation>
    </message>
    <message>
        <source>Use Domain</source>
        <translation>Use Domain</translation>
    </message>
    <message>
        <source>Include URL domain (e.g. &quot;example.com&quot;)</source>
        <translation>Include URL domain (e.g. &quot;example.com&quot;)</translation>
    </message>
    <message>
        <source>Use Port/Path</source>
        <translation>Use Port/Path</translation>
    </message>
    <message>
        <source>Include port and path (e.g &quot;:8080/file&quot;)</source>
        <translation>Include port and path (e.g &quot;:8080/file&quot;)</translation>
    </message>
    <message>
        <source>At least one URL part required.</source>
        <translation>At least one URL part required.</translation>
    </message>
    <message>
        <source>Password Length</source>
        <translation>Password Length</translation>
    </message>
    <message>
        <source>Hash Algorithm</source>
        <translation>Hash Algorithm</translation>
    </message>
    <message>
        <source>MD5 Version 0.6</source>
        <translation>MD5 Version 0.6</translation>
    </message>
    <message>
        <source>HMAC-MD5 Version 0.6</source>
        <translation>HMAC-MD5 Version 0.6</translation>
    </message>
    <message>
        <source>Use L33t</source>
        <translation>Use L33t</translation>
    </message>
    <message>
        <source>not at all</source>
        <translation>not at all</translation>
    </message>
    <message>
        <source>before generating</source>
        <translation>before generating</translation>
    </message>
    <message>
        <source>after generating</source>
        <translation>after generating</translation>
    </message>
    <message>
        <source>before and after generating</source>
        <translation>before and after generating</translation>
    </message>
    <message>
        <source>Leet Level</source>
        <translation>Leet Level</translation>
    </message>
    <message>
        <source>Default Characters</source>
        <translation>Default Characters</translation>
    </message>
    <message>
        <source>Alphanumeric</source>
        <translation>Alphanumeric</translation>
    </message>
    <message>
        <source>Letters only</source>
        <translation>Letters only</translation>
    </message>
    <message>
        <source>Numbers only</source>
        <translation>Numbers only</translation>
    </message>
    <message>
        <source>Special only</source>
        <translation>Special only</translation>
    </message>
    <message>
        <source>Hex</source>
        <translation>Hex</translation>
    </message>
    <message>
        <source>Custom</source>
        <translation>Custom</translation>
    </message>
    <message>
        <source>Characters Preset</source>
        <translation>Characters Preset</translation>
    </message>
    <message>
        <source>Need at least 2 characters.</source>
        <translation>Need at least 2 characters.</translation>
    </message>
    <message>
        <source>Characters</source>
        <translation>Characters</translation>
    </message>
    <message>
        <source>Username</source>
        <translation>Username</translation>
    </message>
    <message>
        <source>Modifier</source>
        <translation>Modifier</translation>
    </message>
    <message>
        <source>Prefix</source>
        <translation>Prefix</translation>
    </message>
    <message>
        <source>Suffix</source>
        <translation>Suffix</translation>
    </message>
</context>
<context>
    <name>ProfilesPage</name>
    <message>
        <source>Add Profile</source>
        <translation>Add Profile</translation>
    </message>
    <message>
        <source>Select/Edit Profiles</source>
        <translation>Select/Edit Profiles</translation>
    </message>
    <message>
        <source>Edit</source>
        <translation>Edit</translation>
    </message>
    <message>
        <source>Remove</source>
        <translation>Remove</translation>
    </message>
    <message>
        <source>Saving profiles failed.</source>
        <translation>Saving profiles failed.</translation>
    </message>
</context>
<context>
    <name>SettingsEditor</name>
    <message>
        <source>Edit Settings</source>
        <translation>Edit Settings</translation>
    </message>
    <message>
        <source>Hide Generated Password</source>
        <translation>Hide Generated Password</translation>
    </message>
    <message>
        <source>Auto-clear generated password</source>
        <translation>Auto-clear generated password</translation>
    </message>
    <message>
        <source>Auto-clear generated pass timeout</source>
        <translation>Auto-clear generated pass timeout</translation>
    </message>
    <message>
        <source>Auto-clear master password</source>
        <translation>Auto-clear master password</translation>
    </message>
    <message>
        <source>Auto-clear master pass timeout</source>
        <translation>Auto-clear master pass timeout</translation>
    </message>
    <message>
        <source>Profiles can be edited directly in the profiles selector.</source>
        <translation>Profiles can be edited directly in the profiles selector.</translation>
    </message>
</context>
</TS>
