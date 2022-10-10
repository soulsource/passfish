import QtQuick 2.6
import Sailfish.Silica 1.0
import Nemo.Notifications 1.0

//Just a wrapper around Notification, because I need to use it in a ternary in loader and don't want to pass in a string representation.
//See components/NoticeOptional.qml for details. If you know a simple and not too ugly solution to make it work without this file:
//I'm all ears.

Notification {
}
