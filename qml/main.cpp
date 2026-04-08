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
    Q_INVOKABLE QVariantList getUserHistory(int coreIndex) {
        if (coreIndex >= 0 && coreIndex < m_userBuffers.size()) {
            return m_userBuffers[coreIndex];
        }
        return QVariantList(); // Return safe empty list if out of bounds
    }

public slots:
    void onStatsUpdated(const QList<qint64> &user, const QList<qint64> &system) {
        m_user.clear();
        m_system.clear();

        for (auto v : user) m_user.append(v);
        for (auto v : system) m_system.append(v);
        
        if (m_userBuffers.size() < m_user.size()) {
            m_userBuffers.resize(m_user.size());
        }

        for (int i = 0; i < m_user.size(); i++) {
            m_userBuffers[i].append(m_user[i]);
            if (m_userBuffers[i].size() > m_maxHistory) {
                m_userBuffers[i].removeFirst();
            }
        }
        emit statsChanged();
    }

signals:
    void statsChanged();

private:
    QVariantList m_user;
    QVariantList m_system;
    int m_maxHistory = 100;
    QList<QVariantList> m_userBuffers; // Internal storage: [Core0[h1, h2...], Core1[h1, h2...]]
};

int main(int argc, char *argv[]) {
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;

    SysInfoBridge bridge;
    engine.rootContext()->setContextProperty("sysInfo", &bridge);

    engine.load(QUrl::fromLocalFile("window.qml"));

    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}

#include "main.moc"
