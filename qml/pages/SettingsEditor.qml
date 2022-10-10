import QtQuick 2.6
import Sailfish.Silica 1.0

Dialog {
    id: settingsEditor

    property var clear_generated_password;
    property var clear_master_password;
    property alias hide_generated_password: hide_generated_passwordBox.checked;

    onAccepted: {
        clear_generated_password =
                clear_generated_passwordBox.checked
                ? clear_generated_password_time.sliderValue * 60
                : null;
        clear_master_password =
                clear_master_passwordBox.checked
                ? clear_master_password_time.sliderValue * 60
                : null;
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
                title: qsTr("Edit Settings")
            }
            TextSwitch {
                id: hide_generated_passwordBox
                text: qsTr("Hide Generated Password")
                palette.highlightColor : Theme.highlightColor
                highlighted: down
            }
            TextSwitch {
                id: clear_generated_passwordBox
                text: qsTr("Auto-clear generated password")
                palette.highlightColor : Theme.highlightColor
                highlighted: down
                checked: typeof clear_generated_password !== 'undefined'
            }
            Slider {
                id: clear_generated_password_time
                minimumValue: 1
                maximumValue: 15
                stepSize: 1
                width: parent.width
                valueText: value + " min"
                visible: clear_generated_passwordBox.checked
                label: qsTr("Auto-clear generated pass timeout")
                value: clear_generated_password/60 || 1
            }
            TextSwitch {
                id: clear_master_passwordBox
                text: qsTr("Auto-clear master password")
                palette.highlightColor : Theme.highlightColor
                highlighted: down
                checked: typeof clear_master_password !== 'undefined'
            }
            Slider {
                id: clear_master_password_time
                minimumValue: clear_generated_passwordBox.checked
                              ? clear_generated_password_time.sliderValue
                              : 1
                maximumValue: 30
                stepSize: 1
                width: parent.width
                valueText: sliderValue + " min"
                visible: clear_master_passwordBox.checked
                label: qsTr("Auto-clear master pass timeout")
                value: clear_master_password/60 || 5
            }
            Label {
                topPadding: Theme.paddingLarge
                width: parent.width - 2*Theme.horizontalPageMargin
                x: Theme.horizontalPageMargin
                wrapMode: Text.WordWrap
                color: Theme.highlightColor
                text: qsTr("Profiles can be edited directly in the profiles selector.")
            }
        }
    }
}
