<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE TS>
<TS version="2.1" language="de_DE" sourcelanguage="en_GB">
<context>
    <name>AboutPage</name>
    <message>
        <source>About PassFish</source>
        <translation>Über PassFish</translation>
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
    PassFish ist eine native Neu-Implementierung von &lt;a href=&quot;https://passwordmaker.org/&quot;&gt;PasswordMaker Pro&lt;/a&gt; für Sailfish OS, 
    mit dem Ziel, zu der &lt;a href=&quot;https://sourceforge.net/projects/passwordmaker/files/Javascript%20Edition/&quot;&gt;Javascript Edition&lt;/a&gt; kompatibel zu sein.
&lt;/p&gt;
&lt;p&gt;
    Der Original-Algorithmus von PasswordMaker Pro stammt von Miquel Burns und Eric H. Jung.
&lt;/p&gt;
&lt;p&gt;
    Da es sich bei PassFish nicht um einen Port, sondern um eine kompette Neuimplementierung handelt, ist es nicht möglich, vollständige Kompatibilität zu garantieren.
    Auch wenn der Original-Quelltext während der Entwicklung als Hilfsmittel herangezogen wurde, gibt es doch grundlegende Unterschiede in der zugrunde liegenden Technologie.
    Für typische Inputs wurden zwar Integrations-Tests angelegt, es ist aber mehr als wahrscheinlich, dass der eine oder andere Spezialfall durch die Maschen gerutscht ist.
    Falls Sie ein derartiges Problem entdecken, melden Sie es doch bitte im &lt;a href=&quot;https://github.com/soulsource/passfish/issues&quot;&gt;Github Issue Tracker&lt;/a&gt; dieses Projekts.
&lt;/p&gt;
&lt;p&gt;
    Wenn wir schon bei Integrations-Tests sind: Um möglichst keine neuen Fehler zu verursachen, wurden die Hash-Algorithmen nicht erneut implementiert.
    Stattdessen wurde wo möglich die QCryptographicHash API verwendet. Wo dies nicht möglich war, wurde stattdessen auf das RustCrypto Hashes Projekt zurückgegriffen
    (mehr Details finden Sie in der Liste der Abhängigkeiten weiter unten).
&lt;/p&gt;
&lt;p&gt;
    Dieses Programm besteht aus zwei Teilen: Der Anwendung selbst (&quot;&lt;a href=&quot;https://github.com/soulsource/passfish&quot;&gt;PassFish&lt;/a&gt;&quot;), und einer Rust-Crate, welche den Nachbau des
    eigentlichen PasswordMaker Pro Algorithmus enthält (&quot;&lt;a href=&quot;https://github.com/soulsource/passwordmaker-rs&quot;&gt;&lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt;&lt;/a&gt;&quot;).
    Diese Trennung ist insofern wichtig, als dass die beiden Teile unter verschiedenen Lizenzen veröffentlicht wurden. PassFish selbst steht unter der GPLv3 Lizenz, während 
    &lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt; unter LGPLv3 weiterverwendet werden darf. Die verlinkten Github-Seiten der jeweiligen Repositories enthalten weitere Details.
&lt;/p&gt;
&lt;p&gt;
    Dieses Programm verwendet Software aus Drittquellen. Die Liste hier wird nach bestem Wissen und Gewissen aktuell gehalten. Hier sind nur die direkten Abhängigkeiten angegeben.
    Besuchen Sie bitte die Websites ebenjener, falls Sie Informationen über deren Abhängigkeiten benötigen. Selbiges gilt, falls Sie den Quelltext der als Open Source veröffentlichten Abhängigkeiten abrufen möchten.
    So weit ich das sagen kann, ist lediglich Silca selbst unter keiner offenen Lizenz verfügbar. Die hier genannten Lizenzen sind lediglich jene, die von &lt;i&gt;diesem&lt;/i&gt; Projekt verwendet werden. Die meisten der gelisteten
    Drittanbieter-Projekte sind auch unter anderen Lizenzen verfügbar.&lt;br&gt;
    Dies sind die dynamisch verknüpften Abhängigkeiten:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://www.qt.io/&quot;&gt;Qt Quick&lt;/a&gt;: Verwendet unter &lt;a href=&quot;https://www.gnu.org/licenses/lgpl-3.0.en.html&quot;&gt;LGPLv3&lt;/a&gt;&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://sailfishos.org/develop/docs/silica/&quot;&gt;Sailfish Silica&lt;/a&gt;: Proprietär, &lt;a href=&quot;https://www.gnu.org/licenses/gpl-3.0&quot;&gt;GPL&lt;/a&gt; Ausnahme für Systembibliotheken&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://github.com/sailfishos/libsailfishapp&quot;&gt;LibSailfishApp&lt;/a&gt;: Verwendet unter LGPL 2.1&lt;/li&gt;
    &lt;/ul&gt;
    Diese Bibliotheken (und deren Abhängigkeiten) sind statisch verknüpft:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/libc&quot;&gt;libc Rust bindings&lt;/a&gt;: Verwendet unter MIT Lizenz&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/serde&quot;&gt;serde&lt;/a&gt;: Verwendet unter MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/toml&quot;&gt;toml-rs&lt;/a&gt;: Verwendet unter MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/dirs&quot;&gt;dirs&lt;/a&gt;: Verwendet unter MIT license&lt;/li&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/ripemd&quot;&gt;RustCrypto: RIPEMD&lt;/a&gt;: Verwendet unter MIT license&lt;/li&gt;
    &lt;/ul&gt;
    Die &lt;nobr&gt;passwordmaker&amp;#8209;rs&lt;/nobr&gt; Bibliothek hat wiederum diese Abhängigkeiten:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://crates.io/crates/unicode-segmentation&quot;&gt;unicode-segmentation&lt;/a&gt;: Verwendet unter MIT license&lt;/li&gt;
    &lt;/ul&gt;
    Auch wenn es sich dabei nicht um eine Laufzeit-Abhängigkeit handelt, sollte der Code-Generator für die Qt Bindings für Rust nicht unerwähnt bleiben:
    &lt;ul&gt;
        &lt;li&gt;&lt;a href=&quot;https://invent.kde.org/sdk/rust-qt-binding-generator&quot;&gt;Rust Qt Binding Generator&lt;/a&gt;&lt;/li&gt;
    &lt;/ul&gt;
    PassFish verwendet eine modifizierte Version vor Rust Qt Binding Generator, welche im &lt;a href=&quot;https://invent.kde.org/soulsource/rust-qt-binding-generator/-/tree/mockall_support&quot;&gt;mockall_support&lt;/a&gt; Zweig verfügbar ist.
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
        <source>My Cover</source>
        <translation type="vanished">Mein Cover</translation>
    </message>
    <message>
        <source>PassFish</source>
        <translation>PassFish</translation>
    </message>
    <message>
        <source>Used Text:</source>
        <translatorcomment>Might be too long? Needs testing.</translatorcomment>
        <translation type="vanished">Verwendeter Text:</translation>
    </message>
    <message>
        <source>Pass Ready</source>
        <translation type="vanished">Passwort Bereit</translation>
    </message>
    <message>
        <source>Input Needed</source>
        <translation type="vanished">Input Benötigt</translation>
    </message>
    <message>
        <source>Used text:</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Status:</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Password ready</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Input needed</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>FirstPage</name>
    <message>
        <source>Show Page 2</source>
        <translation type="vanished">Zur Seite 2</translation>
    </message>
    <message>
        <source>UI Template</source>
        <translation type="vanished">UI-Vorlage</translation>
    </message>
    <message>
        <source>Hello Sailors</source>
        <translation type="vanished">Hallo Matrosen</translation>
    </message>
</context>
<context>
    <name>MainPage</name>
    <message>
        <source>App Settings</source>
        <translation type="vanished">Programmeinstellungen</translation>
    </message>
    <message>
        <source>About</source>
        <translation>Über PassFish</translation>
    </message>
    <message>
        <source>PassFish</source>
        <translation>PassFish</translation>
    </message>
    <message>
        <source>Profile</source>
        <translation>Profil</translation>
    </message>
    <message>
        <source>URL</source>
        <translation>URL</translation>
    </message>
    <message>
        <source>Master Password</source>
        <translation type="vanished">Hauptschlüssel</translation>
    </message>
    <message>
        <source>Used Text</source>
        <translation type="vanished">Verwendeter Text</translation>
    </message>
    <message>
        <source>Generating</source>
        <translation>Am Generieren</translation>
    </message>
    <message>
        <source>Missing text to use</source>
        <translation>Kein zu verwendender Text angegeben</translation>
    </message>
    <message>
        <source>Missing master password</source>
        <translation>Kein Hauptschlüssel angegeben</translation>
    </message>
    <message>
        <source>Error in profile character list</source>
        <translation>Fehler in der Profil-Zeichenliste</translation>
    </message>
    <message>
        <source>Generated Password</source>
        <translation type="vanished">Generiertes Passwort</translation>
    </message>
    <message>
        <source>Saving settings failed.</source>
        <translation>Fehler beim Speichern der Einstellungen.</translation>
    </message>
    <message>
        <source>App settings</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Master password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Used text</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Generated password</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>ProfileEditor</name>
    <message>
        <source>Edit Profile</source>
        <translation type="vanished">Profil bearbeiten</translation>
    </message>
    <message>
        <source>Required Field</source>
        <translation type="vanished">Benötigtes Feld</translation>
    </message>
    <message>
        <source>Profile Name</source>
        <translation type="vanished">Profilname</translation>
    </message>
    <message>
        <source>Profile name required.</source>
        <translation>Profilname wird benötigt.</translation>
    </message>
    <message>
        <source>Use Protocol</source>
        <translation type="vanished">Verwende Protokoll</translation>
    </message>
    <message>
        <source>Include URL protocol (e.g. &quot;http://&quot;)</source>
        <translation>Nutze Protokoll (z.B. &quot;http://&quot;)</translation>
    </message>
    <message>
        <source>Use &quot;undefined&quot; if protocol is missing</source>
        <translation>Verwende &quot;undefined&quot; bei fehlendem Protokoll</translation>
    </message>
    <message>
        <source>Enable to mimic weird behaviour of PasswordMaker Pro.</source>
        <translation type="vanished">Einschalten, um das Verhalten von PasswordMaker Pro zu imitieren.</translation>
    </message>
    <message>
        <source>Use Userinfo</source>
        <translation type="vanished">Verwende Nutzerdaten</translation>
    </message>
    <message>
        <source>Include userinfo (e.g &quot;jane_doe:12345&quot;</source>
        <translation type="vanished">Nutze Nutzerinfo (z.B. &quot;jane_doe:12345&quot;)</translation>
    </message>
    <message>
        <source>Use Subomain(s)</source>
        <translation type="vanished">Verwende Subdomain(s)</translation>
    </message>
    <message>
        <source>Include URL subdomain(s) (e.g. &quot;www.&quot;)</source>
        <translation>Nutze Subdomäne(n) (e.g. &quot;www&quot;)</translation>
    </message>
    <message>
        <source>Use Domain</source>
        <translation type="vanished">Verwende Domain</translation>
    </message>
    <message>
        <source>Include URL domain (e.g. &quot;example.com&quot;)</source>
        <translation>Nutze Domain (z.B. &quot;example.com&quot;)</translation>
    </message>
    <message>
        <source>Use Port/Path</source>
        <translation type="vanished">Verwende Port/Pfad</translation>
    </message>
    <message>
        <source>Include port and path (e.g &quot;:8080/file&quot;)</source>
        <translation>Nutze Port und Pfad (z.B. &quot;:8080/datei&quot;)</translation>
    </message>
    <message>
        <source>At least one URL part required.</source>
        <translation type="vanished">Mindesteins ein Teil der URL muss genutzt werden.</translation>
    </message>
    <message>
        <source>Password Length</source>
        <translation type="vanished">Passwortlänge</translation>
    </message>
    <message>
        <source>Hash Algorithm</source>
        <translation type="vanished">Hash-Algorithmus</translation>
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
        <translation type="vanished">Verwende L33t</translation>
    </message>
    <message>
        <source>not at all</source>
        <translation>Gar nicht</translation>
    </message>
    <message>
        <source>before generating</source>
        <translation>Vor der Generierung</translation>
    </message>
    <message>
        <source>after generating</source>
        <translation>Nach der Generierung</translation>
    </message>
    <message>
        <source>before and after generating</source>
        <translation>Vor und nach der Generierung</translation>
    </message>
    <message>
        <source>Leet Level</source>
        <translatorcomment>Leet genug für diese Welt?</translatorcomment>
        <translation type="vanished">Leet-Stärke</translation>
    </message>
    <message>
        <source>Default Characters</source>
        <translation type="vanished">Standard-Zeichenliste</translation>
    </message>
    <message>
        <source>Alphanumeric</source>
        <translation>Alphanumerisch</translation>
    </message>
    <message>
        <source>Letters only</source>
        <translation>Nur Buchstaben</translation>
    </message>
    <message>
        <source>Numbers only</source>
        <translation>Nur Ziffern</translation>
    </message>
    <message>
        <source>Special only</source>
        <translation>Nur Sonderzeichen</translation>
    </message>
    <message>
        <source>Hex</source>
        <translation>Hexadezimal</translation>
    </message>
    <message>
        <source>Custom</source>
        <translation>Angepasst</translation>
    </message>
    <message>
        <source>Characters Preset</source>
        <translation type="vanished">Zeichenlistenvorlage</translation>
    </message>
    <message>
        <source>Need at least 2 characters.</source>
        <translation>Es werden mindestens 2 Zeichen benötigt.</translation>
    </message>
    <message>
        <source>Characters</source>
        <translation>Zeichenliste</translation>
    </message>
    <message>
        <source>Username</source>
        <translation>Nutzername</translation>
    </message>
    <message>
        <source>Modifier</source>
        <translation>Modifikator</translation>
    </message>
    <message>
        <source>Prefix</source>
        <translation>Präfix</translation>
    </message>
    <message>
        <source>Suffix</source>
        <translation>Suffix</translation>
    </message>
    <message>
        <source>Edit profile</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Required field</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Profile name</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>URL parts to use</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Protocol</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Enable to mimic behaviour of JavaScript PasswordMaker Pro.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Userinfo</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Include userinfo (e.g &quot;jane_doe:12345&quot;)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Subomain(s)</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Domain</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Port and path</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>At least one URL part is required.</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Password generation settings</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Password length</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Hash algorithm</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Use l33t</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Leet level</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Default characters</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Characters preset</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>ProfilesPage</name>
    <message>
        <source>Add Profile</source>
        <translation type="vanished">Neues Profil</translation>
    </message>
    <message>
        <source>Select/Edit Profiles</source>
        <translation type="vanished">Profil Auswählen/Bearbeiten</translation>
    </message>
    <message>
        <source>Edit</source>
        <translation>Bearbeiten</translation>
    </message>
    <message>
        <source>Remove</source>
        <translation>Löschen</translation>
    </message>
    <message>
        <source>Saving profiles failed.</source>
        <translation>Fehler beim Speichern der Profile.</translation>
    </message>
    <message>
        <source>Add profile</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Select/edit profiles</source>
        <translation type="unfinished"></translation>
    </message>
</context>
<context>
    <name>SecondPage</name>
    <message>
        <source>Nested Page</source>
        <translation type="vanished">Unterseite</translation>
    </message>
    <message>
        <source>Item</source>
        <translation type="vanished">Element</translation>
    </message>
</context>
<context>
    <name>SettingsEditor</name>
    <message>
        <source>Edit Settings</source>
        <translation type="vanished">Einstellungen Anpassen</translation>
    </message>
    <message>
        <source>Hide Generated Password</source>
        <translation type="vanished">Generiertes Passwort verstecken</translation>
    </message>
    <message>
        <source>Auto-clear generated password</source>
        <translation>Generiertes Passwort automatisch löschen</translation>
    </message>
    <message>
        <source>Auto-clear generated pass timeout</source>
        <translation type="vanished">Zeit bis das generierte Passwort gelöscht wird</translation>
    </message>
    <message>
        <source>Auto-clear master password</source>
        <translation>Hauptschlüssel automatisch löschen</translation>
    </message>
    <message>
        <source>Auto-clear master pass timeout</source>
        <translation type="vanished">Zeit bis der Hauptschlüssel gelöscht wird</translation>
    </message>
    <message>
        <source>Profiles can be edited directly in the profiles selector.</source>
        <translation>Der Profileditor ist über die Profilauswahl erreichbar.</translation>
    </message>
    <message>
        <source>App settings</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Hide generated password</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Auto-clear generated password timeout</source>
        <translation type="unfinished"></translation>
    </message>
    <message>
        <source>Auto-clear master password timeout</source>
        <translation type="unfinished"></translation>
    </message>
</context>
</TS>
