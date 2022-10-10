import QtQuick 2.6
import Sailfish.Silica 1.0

Page {
    id: aboutPage
    SilicaFlickable {
        anchors.fill: parent
        contentHeight: column.height
        contentWidth: parent.width
        Column {
            id: column
            width: aboutPage.width
            spacing: Theme.paddingLarge
            bottomPadding: Theme.paddingLarge

            PageHeader {
                title: qsTr("About PassFish")
            }
            Label {
                width: parent.width - 2*Theme.horizontalPageMargin
                x: Theme.horizontalPageMargin
                wrapMode: Text.WordWrap
                textFormat: Text.RichText
                onLinkActivated : Qt.openUrlExternally(link)
                color: Theme.highlightColor
                text: qsTr("
<p>
    PassFish is a native re-implementation of <a href=\"https://passwordmaker.org/\">PasswordMaker</a> for Sailfish OS.
    It aims to be mostly compatible to the <a href=\"https://sourceforge.net/projects/passwordmaker/files/Javascript%20Edition/\">Javascript Edition</a>.
</p>
<p>
    All credit for the development of the PasswordMaker Pro algorithm goes to the original authors of PasswordMaker Pro, Miquel Burns and Eric H. Jung.
</p>
<p>
    As PassFish is not merely a port, but a full reimplementation from scratch, compatibility is not guaranteed. While the original source code was
    used as a guideline during implementation, the underlying technology is vastly different. Common use cases are tested by integration tests, but some
    edge cases might have been missed. In case you encounter such an issue, please report it on the
    <a href=\"https://github.com/soulsource/passfish/issues\">github issue tracker</a> of the project.
</p>
<p>
    Speaking of integration tests: The hash algorithms were not re-implemented for this project in order to reduce the risk of introducing bugs. Instead the
    QCryptographicHash API is used where possible, and where not, the implementation from the RustCrypto Hashes repository is utilized (see list of dependencies below).
</p>
<p>
    This program consists of two parts: The application itself (\"<a href=\"https://github.com/soulsource/passfish\">PassFish</a>\"), and a Rust crate that contains the
    implementation of the business logic (\"<a href=\"https://github.com/soulsource/passwordmaker-rs\"><nobr>passwordmaker&#8209;rs</nobr></a>\").
    This is important, as the two parts use different licenses. PassFish is published under the GPLv3 license, while <nobr>passwordmaker&#8209;rs</nobr> is published under LGPLv3. Please check the linked github pages for more details.
</p>
<p>
    This program utilises several third party libraries. This list is kept up-to-date to the best of my knowledge. Only direct dependencies are listed,
    for transitive dependencies please see the linked websites. Similarly, the source code for those third-party dependencies that are published under an
    open source license can be found on the linked websites. To my knowledge the only non-open-source dependency is Silica. The listed licenses are just those
    used by <i>this</i> project, most libraries are available under multiple licenses. Please see the libraries' websites for details.<br>
    These libraries are linked dynamically:
    <ul>
        <li><a href=\"https://www.qt.io/\">Qt Quick</a>: Used under <a href=\"https://www.gnu.org/licenses/lgpl-3.0.en.html\">LGPLv3</a></li>
        <li><a href=\"https://sailfishos.org/develop/docs/silica/\">Sailfish Silica</a>: Proprietary, <a href=\"https://www.gnu.org/licenses/gpl-3.0\">GPL</a> system library exception</li>
        <li><a href=\"https://github.com/sailfishos/libsailfishapp\">LibSailfishApp</a>: Used under LGPL 2.1</li>
    </ul>
    These libraries and their dependencies are linked statically:
    <ul>
        <li><a href=\"https://crates.io/crates/libc\">libc Rust bindings</a>: Used under MIT license</li>
        <li><a href=\"https://crates.io/crates/serde\">serde</a>: Used under MIT license</li>
        <li><a href=\"https://crates.io/crates/toml\">toml-rs</a>: Used under MIT license</li>
        <li><a href=\"https://crates.io/crates/dirs\">dirs</a>: Used under MIT license</li>
        <li><a href=\"https://crates.io/crates/ripemd\">RustCrypto: RIPEMD</a>: Used under MIT license</li>
    </ul>
    The <nobr>passwordmaker&#8209;rs</nobr> library has the following statically linked runtime dependencies:
    <ul>
        <li><a href=\"https://crates.io/crates/unicode-segmentation\">unicode-segmentation</a>: Used under MIT license</li>
    </ul>
    While it is not a runtime dependency, the code generator for the Rust Qt bindings is worth noting:
    <ul>
        <li><a href=\"https://invent.kde.org/sdk/rust-qt-binding-generator\">Rust Qt Binding Generator</a></li>
    </ul>
    PassFish uses a modified version, which can be found in the <a href=\"https://invent.kde.org/soulsource/rust-qt-binding-generator/-/tree/mockall_support\">mockall_support</a> branch.
</p>
<p>
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation,
    either version 3 of the License, or (at your option) any later version.
    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
    See the GNU General Public License for more details.
    You should have received a copy of the GNU General Public License along with this program. If not, see <a href=\"http://www.gnu.org/licenses/\">http://www.gnu.org/licenses/</a>.
</p>
                ")
            }
        }

        VerticalScrollDecorator { flickable: aboutPage}
    }
}
