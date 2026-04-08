#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QtDBus/QtDBus>
#include <QVariantList>

class SysInfoBridge : public QObject {
    Q_OBJECT
    Q_PROPERTY(QVariantList user READ user NOTIFY statsChanged)
    Q_PROPERTY(QVariantList system READ system NOTIFY statsChanged)

public:
    SysInfoBridge(QObject *parent = nullptr) : QObject(parent) {
        QDBusConnection::sessionBus().connect(
            "org.vivicado.Daemon",
            "/org/vivicado/SysInfo",
            "org.vivicado.SysInfo",
            "StatsUpdated",
            this,
            SLOT(onStatsUpdated(QList<qint64>, QList<qint64>))
        );
    }

    QVariantList user() const { return m_user; }
    QVariantList system() const { return m_system; }

public slots:
    void onStatsUpdated(const QList<qint64> &user, const QList<qint64> &system) {
        m_user.clear();
        m_system.clear();

        for (auto v : user) m_user.append(v);
        for (auto v : system) m_system.append(v);
        // 🔹 Debug output
        //qDebug() << "Stats updated:";
        //qDebug() << "User:" << user;
        //qDebug() << "System:" << system;
        emit statsChanged();
    }

signals:
    void statsChanged();

private:
    QVariantList m_user;
    QVariantList m_system;
};

int main(int argc, char *argv[]) {
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;

    SysInfoBridge bridge;
    engine.rootContext()->setContextProperty("sysInfo", &bridge);

    engine.load(QUrl::fromLocalFile("Chart.qml"));

    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}

#include "main.moc"
