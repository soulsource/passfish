import QtQuick 2.6
import Sailfish.Silica 1.0
import "../components"

Page {
    id: profilesSelector
    SilicaListView {
        id : profilesView
        anchors.fill: parent
        model : passwordmaker.profiles

        function store_profile_with_error_message() {
            var worked = profilesView.model.store();
            if(!worked) {
                storeFailureNotice.show();
            }
        }

        PullDownMenu {
            MenuItem {
                text: qsTr("Add profile")
                onClicked: {
                    profilesView.model.insertRows(profilesView.model.rowCount(),1);
                    profilesView.store_profile_with_error_message();
                }
            }
        }
        header: PageHeader {
            title: qsTr("Select/edit profiles")
        }
        delegate: ListItem {
            id: delegate
            width: parent.width
            ListView.onAdd: AddAnimation {
                target: delegate
            }
            ListView.onRemove: RemoveAnimation {
                target: delegate
            }
            Label {
                x: Theme.horizontalPageMargin
                text: name
                anchors.verticalCenter: parent.verticalCenter
                color: index === profilesView.model.current_profile ? Theme.highlightColor : Theme.primaryColor
            }
            menu: ContextMenu {
                MenuItem {
                    text: qsTr("Edit")
                    onClicked: {
                        var pg = pageStack.animatorPush(Qt.resolvedUrl("ProfileEditor.qml"),
                            {
                                profileName    : name,
                                useProtocol    : use_protocol,
                                useSubdomain   : use_subdomains,
                                useDomain      : use_domain,
                                usePortPath    : use_port_path,
                                passwordLength : password_length,
                                hashAlgorithm  : hash_algorithm,
                                useLeet        : use_leet,
                                leetLevel      : leet_level > 0 ? leet_level : 1,
                                characters     : characters,
                                username       : username,
                                modifier       : modifier,
                                prefix         : prefix,
                                suffix         : suffix,
                                useUserInfo    : use_user_info,
                                useDefaultFallbackForProtocol : use_undefined_as_protocol_fallback
                            });
                        pg.pageCompleted.connect(function(pg) {
                            pg.accepted.connect(function() {
                                name            = pg.profileName;
                                use_protocol    = pg.useProtocol;
                                use_subdomains  = pg.useSubdomain;
                                use_domain      = pg.useDomain;
                                use_port_path   = pg.usePortPath;
                                password_length = pg.passwordLength;
                                hash_algorithm  = pg.hashAlgorithm;
                                use_leet        = pg.useLeet;
                                leet_level      = pg.leetLevel;
                                characters      = pg.characters;
                                username        = pg.username;
                                modifier        = pg.modifier;
                                prefix          = pg.prefix;
                                suffix          = pg.suffix;
                                use_user_info   = pg.useUserInfo;
                                use_undefined_as_protocol_fallback = pg.useDefaultFallbackForProtocol;

                                if(index === profilesView.model.current_profile)
                                    passwordmaker.profile_changed();

                                profilesView.store_profile_with_error_message();
                            })
                        })
                    }
                }
                MenuItem {
                    text: qsTr("Remove")
                    enabled: profilesView.count > 1
                    onClicked: {
                        delegate.remorseDelete(function() {
                            var bWasCurrentProfile = index === profilesView.model.current_profile;

                            profilesView.model.removeRows(index,1);
                            profilesView.store_profile_with_error_message();

                            if(bWasCurrentProfile)
                                passwordmaker.profile_changed();
                        })
                    }
                }
            }
            onClicked: {
                if(profilesView.model.current_profile !== index)
                {
                    profilesView.model.current_profile = index;
                    passwordmaker.profile_changed();
                }
                pageContainer.navigateBack(PageStackAction.Animated);
            }
        }
        NoticeOptional {
            id: storeFailureNotice
            text: qsTr("Saving profiles failed.")
            useNotificationFallback: true
        }

        VerticalScrollDecorator {}
    }
}
