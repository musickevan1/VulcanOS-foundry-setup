// VulcanOS SDDM Theme
// Vulcan Forge - Minimal, welcoming, branded login screen

import QtQuick 2.0
import SddmComponents 2.0

Rectangle {
    id: root

    // Theme colors - Vulcan Forge palette
    readonly property color forgeBlack: "#1c1917"
    readonly property color charcoal: "#292524"
    readonly property color stoneGray: "#44403c"
    readonly property color mutedGray: "#78716c"
    readonly property color secondaryGray: "#a8a29e"
    readonly property color warmWhite: "#fafaf9"
    readonly property color forgeOrange: "#f97316"
    readonly property color amber: "#fbbf24"
    readonly property color successGreen: "#22c55e"
    readonly property color errorRed: "#ef4444"

    // Responsive dimensions
    width: Screen.width
    height: Screen.height
    color: forgeBlack

    // Background image
    Image {
        id: background
        anchors.fill: parent
        source: "background.png"
        fillMode: Image.PreserveAspectCrop
    }

    // Main content - centered
    Item {
        anchors.fill: parent

        Column {
            anchors.centerIn: parent
            spacing: 16
            width: 340

            // VulcanOS Logo
            Image {
                id: logo
                source: "logo.png"
                anchors.horizontalCenter: parent.horizontalCenter
                width: 120
                height: 120
                fillMode: Image.PreserveAspectFit
            }

            // Time display
            Text {
                id: timeLabel
                anchors.horizontalCenter: parent.horizontalCenter
                font.family: "Inter"
                font.pixelSize: 64
                font.bold: true
                color: warmWhite

                function updateTime() {
                    text = Qt.formatTime(new Date(), "HH:mm")
                }

                Timer {
                    interval: 1000
                    running: true
                    repeat: true
                    onTriggered: timeLabel.updateTime()
                }

                Component.onCompleted: updateTime()
            }

            // Date display
            Text {
                id: dateLabel
                anchors.horizontalCenter: parent.horizontalCenter
                font.family: "Inter"
                font.pixelSize: 18
                color: secondaryGray
                text: Qt.formatDate(new Date(), "dddd, MMMM d")
            }

            // Spacer
            Item { width: 1; height: 24 }

            // Login box
            Rectangle {
                anchors.horizontalCenter: parent.horizontalCenter
                width: 320
                height: loginColumn.height + 48
                color: Qt.rgba(41/255, 37/255, 36/255, 0.9)
                radius: 16
                border.color: stoneGray
                border.width: 1

                Column {
                    id: loginColumn
                    anchors.centerIn: parent
                    spacing: 14
                    width: 280

                    // Welcome text
                    Text {
                        anchors.horizontalCenter: parent.horizontalCenter
                        text: "Welcome to VulcanOS"
                        font.family: "Inter"
                        font.pixelSize: 14
                        color: mutedGray
                    }

                    // Username field
                    TextBox {
                        id: usernameField
                        width: parent.width
                        height: 44
                        text: userModel.lastUser
                        font.family: "Inter"
                        font.pixelSize: 14

                        // FIXED: color = background, textColor = text
                        color: forgeBlack
                        textColor: warmWhite
                        borderColor: focus ? forgeOrange : stoneGray
                        focusColor: forgeOrange

                        KeyNavigation.tab: passwordField
                        Keys.onReturnPressed: passwordField.focus = true
                    }

                    // Password field
                    PasswordBox {
                        id: passwordField
                        width: parent.width
                        height: 44
                        font.family: "Inter"
                        font.pixelSize: 14

                        // FIXED: color = background, textColor = text
                        color: forgeBlack
                        textColor: warmWhite
                        borderColor: focus ? forgeOrange : stoneGray
                        focusColor: forgeOrange

                        KeyNavigation.tab: loginButton
                        Keys.onReturnPressed: sddm.login(usernameField.text, text, sessionSelector.index)
                    }

                    // Error message
                    Text {
                        id: errorMessage
                        anchors.horizontalCenter: parent.horizontalCenter
                        font.family: "Inter"
                        font.pixelSize: 12
                        color: errorRed
                        text: ""
                        visible: text !== ""
                    }

                    // Login button
                    Button {
                        id: loginButton
                        width: parent.width
                        height: 44
                        text: "Login"
                        font.family: "Inter"
                        font.pixelSize: 14

                        // Button styling
                        color: forgeOrange
                        textColor: warmWhite
                        activeColor: "#ea580c"
                        pressedColor: "#c2410c"

                        onClicked: {
                            errorMessage.text = ""
                            sddm.login(usernameField.text, passwordField.text, sessionSelector.index)
                        }
                    }

                    // Session selector
                    ComboBox {
                        id: sessionSelector
                        width: parent.width
                        height: 40
                        model: sessionModel
                        index: sessionModel.lastIndex
                        font.family: "Inter"
                        font.pixelSize: 13

                        // FIXED: color = background, textColor = text
                        color: forgeBlack
                        textColor: secondaryGray
                        borderColor: stoneGray
                        focusColor: forgeOrange
                        hoverColor: charcoal
                        menuColor: charcoal
                        arrowColor: secondaryGray
                    }
                }
            }
        }

        // Power buttons - bottom right
        Row {
            anchors.bottom: parent.bottom
            anchors.right: parent.right
            anchors.margins: 24
            spacing: 16

            // Power off button
            Rectangle {
                width: 40
                height: 40
                radius: 8
                color: powerOffArea.containsMouse ? stoneGray : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: "⏻"
                    font.pixelSize: 20
                    color: powerOffArea.containsMouse ? warmWhite : mutedGray
                }

                MouseArea {
                    id: powerOffArea
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: sddm.powerOff()
                }
            }

            // Reboot button
            Rectangle {
                width: 40
                height: 40
                radius: 8
                color: rebootArea.containsMouse ? stoneGray : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: "↻"
                    font.pixelSize: 20
                    color: rebootArea.containsMouse ? warmWhite : mutedGray
                }

                MouseArea {
                    id: rebootArea
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: sddm.reboot()
                }
            }
        }

        // Caps Lock warning - bottom left
        Text {
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.margins: 24
            font.family: "Inter"
            font.pixelSize: 12
            color: amber
            text: keyboard.capsLock ? "⚠ Caps Lock is on" : ""
            visible: keyboard.capsLock
        }
    }

    // Handle login failure
    Connections {
        target: sddm
        onLoginFailed: {
            errorMessage.text = "Authentication failed"
            passwordField.text = ""
            passwordField.focus = true
        }
    }

    // Focus on start
    Component.onCompleted: {
        if (usernameField.text === "") {
            usernameField.focus = true
        } else {
            passwordField.focus = true
        }
    }
}
