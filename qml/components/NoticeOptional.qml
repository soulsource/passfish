import QtQuick 2.6
import Sailfish.Silica 1.0

/// Helper to allow using Notice without breaking compilation on SailfishOS 3.4.
/// This helper conditionally loads either a Notice or a Notification
/// (or nothing if useNotificationFallback is disabled) based on the availability
/// of the Notice type.
Loader {
    property string text
    property string duration : "Notice.Short"
    property bool useNotificationFallback : true
    property bool transientNotificationFallback : duration === "Notice.Short"

    readonly property bool noticesAvailable : typeof(Notices) !== "undefined"

    function show() {
        asynchronous = false;
        if(item){
            if(item.show) {
                item.show();
                return;
            }
            else if(item.publish){
                item.publish();
                return;
            }
        }
        console.log("Notice could not be shown: " + text);
    }

    asynchronous: true
    source: noticesAvailable
            ? "../helpers/NoticeLoadable.qml"
            : (useNotificationFallback ? "../helpers/NotificationLoadable.qml" : "")

    Binding {
        target: item
        property: "text"
        value: text
        when: noticesAvailable && status == Loader.Ready
    }
    Binding {
        target: item
        property: "duration"
        value: duration
        when: noticesAvailable && status == Loader.Ready
    }
    //----------------------------------------------------------
    //fallback bindings
    Binding {
        target: item
        property: "summary"
        value: text
        when: !noticesAvailable && useNotificationFallback && status == Loader.Ready
    }
    Binding {
        target: item
        property: "previewSummary"
        value: text
        when: !noticesAvailable && useNotificationFallback && status == Loader.Ready
    }
    Binding {
        target: item
        property: "isTransient"
        value: transientNotificationFallback
        when: !noticesAvailable && useNotificationFallback && status == Loader.Ready
    }
}
