import QtQuick 2.6
import Sailfish.Silica 1.0
import "../components"

Page {
    id: mainPage

    readonly property var isBackground : Qt.application.state

    onIsBackgroundChanged: {
        if(isBackground === Qt.ApplicationInactive) {
            mainFlickable.restart_timers();
        }
    }

    SilicaFlickable {
        id: mainFlickable
        anchors.fill: parent
        contentHeight: column.height

        function store_settings_with_error_message() {
            var worked = passwordmaker.store_settings();
            if(!worked) {
                storeFailureNotice.show();
            }
        }
        function restart_timers() {
            if(autoClearMasterPasswordTimer.enabled && passwordmaker.master_password.length > 0) {
                autoClearMasterPasswordTimer.restart();
            }
            if(autoClearGeneratedPasswordTimer.enabled && (passwordmaker.used_text.length > 0 || passwordmaker.url.length > 0)) {
                autoClearGeneratedPasswordTimer.restart();
            }
        }

        PullDownMenu {
            MenuItem {
                text: qsTr("App settings")
                onClicked: {
                    var pg = pageStack.animatorPush(Qt.resolvedUrl("SettingsEditor.qml"),
                    {
                        clear_generated_password :  passwordmaker.settings.clear_generated_password_seconds,
                        clear_master_password : passwordmaker.settings.clear_master_password_seconds,
                        hide_generated_password : passwordmaker.settings.hide_generated_password
                    });
                    pg.pageCompleted.connect(function(pg) {
                        pg.accepted.connect(function() {
                            passwordmaker.settings.clear_generated_password_seconds
                                =pg.clear_generated_password;
                            passwordmaker.settings.clear_master_password_seconds
                                =pg.clear_master_password;
                            passwordmaker.settings.hide_generated_password
                                =pg.hide_generated_password;

                            mainFlickable.store_settings_with_error_message()
                        })
                    })
                }
            }
            MenuItem {
                text: qsTr("About")
                onClicked: pageStack.animatorPush(Qt.resolvedUrl("AboutPage.qml"))
            }
        }
        VerticalScrollDecorator {}
        Column {
            id: column

            width: mainPage.width
            spacing: Theme.paddingLarge
            PageHeader {
                title: qsTr("PassFish")
            }


            ValueButton {
                id: profileButton
                label: qsTr("Profile")
                value: passwordmaker.profiles.current_profile_name

                onClicked: pageStack.animatorPush(Qt.resolvedUrl("ProfilesPage.qml"))
            }
            TextField {
                id: url
                width: parent.width

                text: passwordmaker.url
                inputMethodHints: Qt.ImhUrlCharactersOnly | Qt.ImhNoPredictiveText | Qt.ImhNoAutoUppercase

                label: qsTr("URL")
                placeholderText: qsTr("URL")
                EnterKey.iconSource: "image://theme/icon-m-enter-next"
                EnterKey.onClicked: masterPassword.focus = true
                Binding {
                    target: passwordmaker
                    property: "url"
                    value: url.text
                }
            }
            PasswordField {
                id: masterPassword
                width: parent.width

                text: passwordmaker.master_password

                label: qsTr("Master password")
                placeholderText: qsTr("Master password")
                EnterKey.iconSource: "image://theme/icon-m-enter-close"
                EnterKey.onClicked: focus = false
                Binding {
                    target: passwordmaker
                    property: "master_password"
                    value: masterPassword.text
                }
            }
            TextField {
                id: usedText
                width: parent.width

                inputMethodHints: Qt.ImhNoPredictiveText | Qt.ImhNoAutoUppercase

                label: qsTr("Used text")
                hideLabelOnEmptyField: passwordmaker.used_text === ""
                onFocusChanged: if(focus) text = passwordmaker.used_text
                onPlaceholderTextChanged: if(!focus) text = ""
                placeholderText: passwordmaker.used_text === "" ? qsTr("Used text") : passwordmaker.used_text
                EnterKey.iconSource: "image://theme/icon-m-enter-close"
                EnterKey.onClicked: focus = false
                Binding {
                    target: passwordmaker
                    property: "used_text"
                    value: usedText.text
                    when: usedText.focus
                }
            }
            Separator {
                width: parent.width
                horizontalAlignment:Qt.AlignHCenter
                color: Theme.secondaryColor
            }
            Row{
                width: parent.width
                PasswordField {
                    id: generatedPassword
                    width: parent.width - copyToClipboard.width


                    function password_text_from_generator_state(g, t, e) {
                        switch(g){
                        case 0 :
                            return e === TextInput.Normal ? t : t.replace(/./g, passwordCharacter);
                        case 1 :
                            return qsTr("Generating");
                        case 2 :
                            return qsTr("Missing text to use");
                        case 3 :
                            return qsTr("Missing master password");
                        case 4 :
                            return qsTr("Error in profile character list");
                        default:
                            return "";
                        }
                    }

                    passwordEchoMode: passwordmaker.settings.hide_generated_password
                        ? TextInput.Password : TextInput.Normal
                    showEchoModeToggle: passwordmaker.settings.hide_generated_password
                    readOnly: true
                    focusOnClick: true
                    label: qsTr("Generated password")
                    hideLabelOnEmptyField: false
                    placeholderText:
                        password_text_from_generator_state(
                            passwordmaker.generator_state
                            , passwordmaker.generated_password
                            , echoMode)

                    onFocusChanged: if(focus) text = passwordmaker.generated_password; else text = "";
                    placeholderColor: color

                    /*
                    //Commented out for now. This is anyhow never visible, and causes log spam on Sailfish 3.
                    BusyIndicator {
                        id: busy
                        parent: null
                        size: BusyIndicatorSize.Small
                        running: passwordmaker.generator_state === 1
                    }
                    states: State {
                        when: passwordmaker.generator_state === 1
                        PropertyChanges {
                            target: generatedPassword
                            rightItem: busy
                        }
                    }
                    */
                }
                IconButton{
                    id: copyToClipboard
                    enabled: passwordmaker.generator_state === 0 && passwordmaker.generated_password.length > 0
                    icon.source: "image://theme/icon-m-clipboard"
                    onClicked: Clipboard.text = passwordmaker.generated_password;
                }
            }
            Timer {
                id: autoClearMasterPasswordTimer
                property bool enabled: typeof passwordmaker.settings.clear_master_password_seconds !== 'undefined'
                interval: passwordmaker.settings.clear_master_password_seconds*1000 || 10000000
                running: false
                onTriggered: if(enabled)
                                 passwordmaker.master_password = "";
            }
            Timer {
                id: autoClearGeneratedPasswordTimer
                property bool enabled: typeof passwordmaker.settings.clear_generated_password_seconds !== 'undefined'
                interval: passwordmaker.settings.clear_generated_password_seconds*1000 || 10000000
                running: false
                onTriggered: if(enabled){
                                 passwordmaker.used_text = "";
                                 passwordmaker.url = "";
                             }
            }
        }
        NoticeOptional {
            id: storeFailureNotice
            text: qsTr("Saving settings failed.")
            useNotificationFallback: true
        }

        Binding{
            target: autoClearMasterPasswordTimer
            property: "running"
            value: false
            when: !autoClearMasterPasswordTimer.enabled || isBackground === Qt.ApplicationActive
        }
        Binding{
            target: autoClearGeneratedPasswordTimer
            property: "running"
            value: false
            when: !autoClearGeneratedPasswordTimer.enabled || isBackground === Qt.ApplicationActive
        }
    }
    readonly property int current_profile_index : passwordmaker.profiles.current_profile
    onCurrent_profile_indexChanged: {mainFlickable.store_settings_with_error_message()}
}
