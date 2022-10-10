#include <QtQuick>
#include <sailfishapp.h>
#include <stdio.h>
#include "Bindings.h"
#include "GraphemeCountValidator.h"

Q_DECL_EXPORT int main(int argc, char *argv[])
{
    QScopedPointer<QGuiApplication> app{SailfishApp::application(argc,argv)};
    QScopedPointer<QQuickView> view{SailfishApp::createView()};

    qmlRegisterType<GraphemeCountValidator>("PWM", 1, 0, "GraphemeCountValidator");

    PasswordMaker maker;
    view->rootContext()->setContextProperty("passwordmaker",&maker);
    QObject::connect(&maker,
                     &PasswordMaker::i_say_sexy_things_to_myself_while_im_dancingChanged,
                     &maker,
                     [&]() {maker.setI_say_sexy_things_to_myself_while_im_dancing(true);},
                     (Qt::ConnectionType)(Qt::AutoConnection | Qt::UniqueConnection)
    );

    view->setSource(SailfishApp::pathTo(QString("qml/PassFish.qml")));
    view->show();

    return app->exec();
}

