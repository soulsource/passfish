import QtQuick 2.6
import Sailfish.Silica 1.0
import "../components"
import PWM 1.0

Dialog {
    id: profileEditor

    property alias profileName: profileNameField.text
    property alias useProtocol: protocolField.checked
    property alias useSubdomain: subdomainField.checked
    property alias useDomain: domainField.checked
    property alias usePortPath: portPathField.checked
    property alias useUserInfo: userInfoField.checked
    property alias useDefaultFallbackForProtocol : useDefaultFallbackForProtocolField.checked
    property alias passwordLength: passwordLengthSlider.value
    property alias hashAlgorithm: hashAlgorithmComboBox.currentIndex
    property alias useLeet : useLeetComboBox.currentIndex
    property alias leetLevel : leetLevelSlider.value
    property alias characters : charactersField.text
    property alias username : usernameField.text
    property alias modifier : modifierField.text
    property alias prefix : prefixField.text
    property alias suffix : suffixField.text

    canAccept: {
        profileNameField.acceptableInput
                && charactersField.acceptableInput
                && urlPartsColumn.anySelected
    }

    onAcceptBlocked: {
        if(!urlPartsColumn.anySelected) {
            urlNotice.show();
        }
        else if(!profileNameField.acceptableInput) {
            nameNotice.show();
        }
        else if(!charactersField.acceptableInput) {
            charactersNotice.show();
        }
    }


    SilicaFlickable {
        anchors.fill: parent
        contentHeight: column.height

        VerticalScrollDecorator {}

        Column {
            id: column
            width: parent.width
            bottomPadding: Theme.paddingLarge
            DialogHeader {
                title: qsTr("Edit Profile")
            }
            TextField {
                id: profileNameField
                width: parent.width
                errorHighlight: !acceptableInput

                //description doesn't work on Sailfish 3. Use label instead if unavailable.
                readonly property bool descriptionAvailable : typeof(description) !== "undefined"
                label: !descriptionAvailable && errorHighlight ? qsTr("Required Field") : qsTr("Profile Name")
                hideLabelOnEmptyField: descriptionAvailable
                placeholderText: qsTr("Profile Name")

                Binding {
                    target: profileNameField
                    property: "description"
                    value: profileNameField.errorHighlight ? qsTr("Required Field") : ""
                    when: profileNameField.descriptionAvailable
                }

                validator: RegExpValidator{regExp: /.+/}
                //It doesn't make much sense to send the focus to the unrelated and rarely used fields waaaaay at the bottom and skip most relevant fields. Rather close the keyboard.
                EnterKey.iconSource: "image://theme/icon-m-enter-close"
                EnterKey.onClicked: focus = false
            }
            NoticeOptional {
                id: nameNotice
                text: qsTr("Profile name required.")
                useNotificationFallback: false
            }
            Column {
                id: urlPartsColumn
                width: parent.width
                topPadding: Theme.paddingLarge
                bottomPadding: Theme.paddingLarge

                property bool anySelected : (useProtocol || useSubdomain || useDomain || usePortPath || useUserInfo)

                TextSwitch {
                    id: protocolField
                    text: qsTr("Use Protocol")
                    description: qsTr("Include URL protocol (e.g. \"http://\")")
                    palette.highlightColor : down || urlPartsColumn.anySelected ? Theme.highlightColor : Theme.errorColor
                    highlighted: down || !urlPartsColumn.anySelected
                }
                TextSwitch {
                    id: useDefaultFallbackForProtocolField
                    visible: protocolField.checked
                    text: qsTr("Use \"undefined\" if protocol is missing")
                    description: qsTr("Enable to mimic weird behaviour of PasswordMaker Pro.")
                    palette.highlightColor : down ? Theme.highlightColor : Theme.errorColor
                    highlighted: down
                }
                TextSwitch {
                    id: userInfoField
                    text: qsTr("Use Userinfo")
                    description: qsTr("Include userinfo (e.g \"jane_doe:12345\"")
                    palette.highlightColor : down || urlPartsColumn.anySelected ? Theme.highlightColor : Theme.errorColor
                    highlighted: down || !urlPartsColumn.anySelected
                }
                TextSwitch {
                    id: subdomainField
                    text: qsTr("Use Subomain(s)")
                    description: qsTr("Include URL subdomain(s) (e.g. \"www.\")")
                    palette.highlightColor : down || urlPartsColumn.anySelected ? Theme.highlightColor : Theme.errorColor
                    highlighted: down || !urlPartsColumn.anySelected
                }
                TextSwitch {
                    id: domainField
                    text: qsTr("Use Domain")
                    description: qsTr("Include URL domain (e.g. \"example.com\")")
                    palette.highlightColor : down || urlPartsColumn.anySelected ? Theme.highlightColor : Theme.errorColor
                    highlighted: down || !urlPartsColumn.anySelected
                }
                TextSwitch {
                    id: portPathField
                    text: qsTr("Use Port/Path")
                    description: qsTr("Include port and path (e.g \":8080/file\")")
                    palette.highlightColor : down || urlPartsColumn.anySelected ? Theme.highlightColor : Theme.errorColor
                    highlighted: down || !urlPartsColumn.anySelected

                }
                NoticeOptional {
                    id: urlNotice
                    text: qsTr("At least one URL part required.")
                    useNotificationFallback: false
                }
            }
            Column {
                width: parent.width
                topPadding: Theme.paddingLarge
                bottomPadding: Theme.paddingLarge
                Slider {
                    id: passwordLengthSlider
                    minimumValue: 1
                    maximumValue: 50
                    stepSize: 1
                    width: parent.width
                    valueText : value
                    label: qsTr("Password Length")
                }
                ComboBox {
                    id: hashAlgorithmComboBox
                    label: qsTr("Hash Algorithm")
                    menu: ContextMenu {
                        MenuItem { text: "MD4" }
                        MenuItem { text: "HMAC-MD4" }
                        MenuItem { text: "MD5" }
                        MenuItem { text: qsTr("MD5 Version 0.6") }
                        MenuItem { text: "HMAC-MD5" }
                        MenuItem { text: qsTr("HMAC-MD5 Version 0.6") }
                        MenuItem { text: "SHA-1" }
                        MenuItem { text: "HMAC-SHA-1" }
                        MenuItem { text: "SHA-256" }
                        MenuItem { text: "HMAC-SHA-256" }
                        MenuItem { text: "RIPEMD-160" }
                        MenuItem { text: "HMAC-RIPEMD-160" }
                    }
                }
                ComboBox {
                    id: useLeetComboBox
                    label: qsTr("Use L33t")
                    menu: ContextMenu {
                        MenuItem { text: qsTr("not at all") }
                        MenuItem { text: qsTr("before generating") }
                        MenuItem { text: qsTr("after generating") }
                        MenuItem { text: qsTr("before and after generating") }
                    }
                }
                Slider {
                    id: leetLevelSlider
                    minimumValue: 1
                    maximumValue: 9
                    stepSize: 1
                    width: parent.width
                    valueText: value
                    visible: useLeetComboBox.currentIndex > 0
                    label: qsTr("Leet Level")
                }
            }
            Column {
                enabled: hashAlgorithmComboBox.currentIndex != 3 && hashAlgorithmComboBox.currentIndex != 5
                width: parent.width
                topPadding: Theme.paddingLarge
                bottomPadding: Theme.paddingLarge
                ListModel {
                    id: defaultCharacterValues
                    ListElement {
                        name: qsTr("Default Characters")
                        chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./"
                        userFacing: true
                    }
                    ListElement {
                        name: qsTr("Alphanumeric")
                        chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                        userFacing: true
                    }
                    ListElement {
                        name: qsTr("Letters only")
                        chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
                        userFacing: true
                    }
                    ListElement {
                        name: qsTr("Numbers only")
                        chars: "0123456789"
                        userFacing: true
                    }
                    ListElement {
                        name: qsTr("Special only")
                        chars: "`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./"
                        userFacing: true
                    }
                    ListElement {
                        name: qsTr("Hex")
                        chars: "0123456789abcdef"
                        userFacing: true
                    }
                    ListElement {
                        name: qsTr("Custom")
                        chars: ""
                        userFacing: false
                    }
                }
                ComboBox {
                    id: defaultCharactersMenu

                    function matchIndex(text) {
                        for(var i=0; i < defaultCharacterValues.count - 1;++i)
                            if(defaultCharacterValues.get(i).chars === text)
                                return i;
                        return defaultCharacterValues.count - 1;
                    }

                    Binding {
                        target: defaultCharactersMenu
                        property: "currentIndex"
                        value: defaultCharactersMenu.matchIndex(charactersField.text)
                    }

                    label: qsTr("Characters Preset")
                    menu: ContextMenu {
                        Repeater {
                            model: defaultCharacterValues
                            MenuItem {
                                text: name
                                visible: userFacing
                                onClicked: if(userFacing) charactersField.text = chars
                            }
                        }
                    }
                }
                TextField {
                    id: charactersField
                    width: parent.width
                    errorHighlight: !acceptableInput

                    //description doesn't work on Sailfish 3. Use label instead if unavailable.
                    readonly property bool descriptionAvailable : typeof(description) !== "undefined"
                    label: !descriptionAvailable && errorHighlight ? qsTr("Need at least 2 characters.") : qsTr("Characters")
                    hideLabelOnEmptyField: descriptionAvailable
                    placeholderText: qsTr("Characters")
                    validator: GraphemeCountValidator { minGraphemeCount: 2 }

                    //The text fields below are conceptually different. Close the keyboard.
                    EnterKey.iconSource: "image://theme/icon-m-enter-close"
                    EnterKey.onClicked: focus = false

                    Binding {
                        target: charactersField
                        property: "description"
                        value: charactersField.errorHighlight ? qsTr("Need at least 2 characters.") : ""
                        when: charactersField.descriptionAvailable
                    }
                    states: State{
                            name: "locked"
                            when: hashAlgorithmComboBox.currentIndex === 3 || hashAlgorithmComboBox.currentIndex === 5
                            PropertyChanges {
                                target: charactersField
                                text: "0123456789abcdef"

                            }
                        }

                }
                NoticeOptional {
                    id: charactersNotice
                    text: qsTr("Need at least 2 characters.")
                    useNotificationFallback: false
                }
            }
            Column {
                width: parent.width
                topPadding: Theme.paddingLarge
                bottomPadding: Theme.paddingLarge
                TextField {
                    id: usernameField
                    width: parent.width
                    label: qsTr("Username")
                    placeholderText: qsTr("Username")
                    EnterKey.iconSource: "image://theme/icon-m-enter-next"
                    EnterKey.onClicked: modifierField.focus = true
                }
                TextField {
                    id: modifierField
                    width: parent.width
                    label: qsTr("Modifier")
                    placeholderText: qsTr("Modifier")
                    EnterKey.iconSource: "image://theme/icon-m-enter-next"
                    EnterKey.onClicked: prefixField.focus = true
                }
                TextField {
                    id: prefixField
                    width: parent.width
                    label: qsTr("Prefix")
                    placeholderText: qsTr("Prefix")
                    EnterKey.iconSource: "image://theme/icon-m-enter-next"
                    EnterKey.onClicked: suffixField.focus = true
                }
                TextField {
                    id: suffixField
                    width: parent.width
                    label: qsTr("Suffix")
                    placeholderText: qsTr("Suffix")
                    //There are many non-keyboard-input switches on this page. Do not let the user confirm using keyboard.
                    EnterKey.iconSource: "image://theme/icon-m-enter-close"
                    EnterKey.onClicked: focus = false
                }
            }
        }
    }
}
