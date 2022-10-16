<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE TS>
<TS version="2.1">
<context>
    <name>AboutPage</name>
    <message>
        <location filename="../qml/pages/AboutPage.qml" line="17"/>
        <source>About PassFish</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/AboutPage.qml" line="26"/>
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
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>CoverPage</name>
    <message>
        <location filename="../qml/cover/CoverPage.qml" line="12"/>
        <source>PassFish</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/cover/CoverPage.qml" line="26"/>
        <source>Used text:</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/cover/CoverPage.qml" line="45"/>
        <source>Status:</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/cover/CoverPage.qml" line="54"/>
        <source>Password ready</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/cover/CoverPage.qml" line="55"/>
        <source>Input needed</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>MainPage</name>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="38"/>
        <source>App settings</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="61"/>
        <source>About</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="72"/>
        <source>PassFish</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="78"/>
        <source>Profile</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="90"/>
        <location filename="../qml/pages/MainPage.qml" line="91"/>
        <source>URL</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="106"/>
        <location filename="../qml/pages/MainPage.qml" line="107"/>
        <source>Master password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="122"/>
        <location filename="../qml/pages/MainPage.qml" line="126"/>
        <source>Used text</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="153"/>
        <source>Generating</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="155"/>
        <source>Missing text to use</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="157"/>
        <source>Missing master password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="159"/>
        <source>Error in profile character list</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="170"/>
        <source>Generated password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="229"/>
        <source>Saving settings failed.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/MainPage.qml" line="234"/>
        <source>Copied</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>ProfileEditor</name>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="56"/>
        <source>Edit profile</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="65"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="72"/>
        <source>Required field</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="65"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="67"/>
        <source>Profile name</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="83"/>
        <source>Profile name required.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="95"/>
        <source>URL parts to use</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="105"/>
        <source>Protocol</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="106"/>
        <source>Include URL protocol (e.g. &quot;http://&quot;)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="113"/>
        <source>Use &quot;undefined&quot; if protocol is missing</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="114"/>
        <source>Enable to mimic behaviour of JavaScript PasswordMaker Pro.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="120"/>
        <source>Userinfo</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="121"/>
        <source>Include userinfo (e.g &quot;jane_doe:12345&quot;)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="127"/>
        <source>Subdomain(s)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="128"/>
        <source>Include URL subdomain(s) (e.g. &quot;www.&quot;)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="134"/>
        <source>Domain</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="135"/>
        <source>Include URL domain (e.g. &quot;example.com&quot;)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="141"/>
        <source>Port and path</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="142"/>
        <source>Include port and path (e.g &quot;:8080/file&quot;)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="149"/>
        <source>At least one URL part is required.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="164"/>
        <source>Password generation settings</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="178"/>
        <source>Password length</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="182"/>
        <source>Hash algorithm</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="187"/>
        <source>MD5 Version 0.6</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="189"/>
        <source>HMAC-MD5 Version 0.6</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="200"/>
        <source>Use l33t</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="202"/>
        <source>not at all</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="203"/>
        <source>before generating</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="204"/>
        <source>after generating</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="205"/>
        <source>before and after generating</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="216"/>
        <source>Leet level</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="227"/>
        <source>Default characters</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="232"/>
        <source>Alphanumeric</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="237"/>
        <source>Letters only</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="242"/>
        <source>Numbers only</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="247"/>
        <source>Special only</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="252"/>
        <source>Hex</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="257"/>
        <source>Custom</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="278"/>
        <source>Characters preset</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="297"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="309"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="325"/>
        <source>Need at least 2 characters.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="297"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="299"/>
        <source>Characters</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="336"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="337"/>
        <source>Username</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="344"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="345"/>
        <source>Modifier</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="352"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="353"/>
        <source>Prefix</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfileEditor.qml" line="360"/>
        <location filename="../qml/pages/ProfileEditor.qml" line="361"/>
        <source>Suffix</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>ProfilesPage</name>
    <message>
        <location filename="../qml/pages/ProfilesPage.qml" line="21"/>
        <source>Add profile</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfilesPage.qml" line="29"/>
        <source>Select/edit profiles</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfilesPage.qml" line="48"/>
        <source>Edit</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfilesPage.qml" line="97"/>
        <source>Remove</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/ProfilesPage.qml" line="123"/>
        <source>Saving profiles failed.</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>SettingsEditor</name>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="34"/>
        <source>App settings</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="38"/>
        <source>Hide generated password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="44"/>
        <source>Auto-clear generated password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="57"/>
        <source>Auto-clear generated password timeout</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="62"/>
        <source>Auto-clear master password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="77"/>
        <source>Auto-clear master password timeout</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <location filename="../qml/pages/SettingsEditor.qml" line="86"/>
        <source>Profiles can be edited directly in the profiles selector.</source>
        <translation type="unfinished"></translation>
    </message>
</context>
</TS>
