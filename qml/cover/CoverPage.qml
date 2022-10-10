import QtQuick 2.0
import Sailfish.Silica 1.0

CoverBackground {
    Column{
        x: Theme.paddingLarge
        y: Theme.paddingLarge
        width: parent.width - 2 * x
        spacing: Theme.paddingLarge
        Label {
            id: label
            text: qsTr("PassFish")
            color: Theme.highlightColor
            fontSizeMode: Text.Fit
            font.pixelSize: Theme.fontSizeLarge
            bottomPadding: Theme.paddingLarge
            width: parent.width
            elide: Text.ElideNone
        }
        Column{
            width: parent.width
            spacing: Theme.paddingSmall
            Label {
                id: urlLabel
                color: Theme.highlightColor
                text: qsTr("Used Text:")
                font.pixelSize: Theme.fontSizeSmall
                truncationMode: TruncationMode.Fade
                width: parent.width
            }
            Label {
                id: generated_for_url
                text: passwordmaker.used_text
                font.pixelSize: Theme.fontSizeSmall
                truncationMode: TruncationMode.Fade
                width: parent.width
            }
        }
        Column{
            width: parent.width
            spacing: Theme.paddingSmall
            Label {
                id: readyLabel
                color: Theme.highlightColor
                text: passwordmaker.generated_password.length > 0
                      && passwordmaker.generator_state === 0
                      ? qsTr("Pass Ready")
                      : qsTr("Input Needed")
                font.pixelSize: Theme.fontSizeSmall
                truncationMode: TruncationMode.Fade
                width: parent.width
            }
        }
    }

    CoverActionList {
        id: coverAction
        enabled: passwordmaker.generated_password.length > 0
                 && passwordmaker.generator_state === 0
        CoverAction {
            iconSource: "image://theme/icon-s-clipboard"
            onTriggered: Clipboard.text = passwordmaker.generated_password;
        }

        CoverAction {
            iconSource: "image://theme/icon-cover-cancel"
            onTriggered: {
                passwordmaker.master_password = "";
                passwordmaker.url = "";
            }
        }
    }
}
