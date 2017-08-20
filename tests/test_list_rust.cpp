/* generated by rust_qt_binding_generator */
#include "test_list_rust.h"

namespace {
    template <typename T>
    struct option {
    private:
        T value;
        bool some;
    public:
        operator QVariant() const {
            if (some) {
                return QVariant(value);
            }
            return QVariant();
        }
    };
    struct qbytearray_t {
    private:
        const char* data;
        int len;
    public:
        qbytearray_t(const QByteArray& v):
            data(v.data()),
            len(v.size()) {
        }
        operator QByteArray() const {
            return QByteArray(data, len);
        }
    };
    struct qstring_t {
    private:
        const void* data;
        int len;
    public:
        qstring_t(const QString& v):
            data(static_cast<const void*>(v.utf16())),
            len(v.size()) {
        }
        operator QString() const {
            return QString::fromUtf8(static_cast<const char*>(data), len);
        }
    };
    struct qmodelindex_t {
        int row;
        quintptr id;
    };
}
typedef void (*qstring_set)(QString*, qstring_t*);
void set_qstring(QString* v, qstring_t* val) {
    *v = *val;
}
typedef void (*qbytearray_set)(QByteArray*, qbytearray_t*);
void set_qbytearray(QByteArray* v, qbytearray_t* val) {
    *v = *val;
}

extern "C" {
    Persons::Private* persons_new(Persons*,
        void (*)(const Persons*),
        void (*)(Persons*),
        void (*)(Persons*),
        void (*)(Persons*, int, int),
        void (*)(Persons*),
        void (*)(Persons*, int, int),
        void (*)(Persons*));
    void persons_free(Persons::Private*);
};
Persons::Persons(QObject *parent):
    QAbstractItemModel(parent),
    d(persons_new(this,
        [](const Persons* o) {
            emit o->newDataReady(QModelIndex());
        },
        [](Persons* o) {
            o->beginResetModel();
        },
        [](Persons* o) {
            o->endResetModel();
        },
        [](Persons* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Persons* o) {
            o->endInsertRows();
        },
        [](Persons* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Persons* o) {
            o->endRemoveRows();
        }
    )) {
    connect(this, &Persons::newDataReady, this, [this](const QModelIndex& i) {
        fetchMore(i);
    }, Qt::QueuedConnection);
}


Persons::~Persons() {
    persons_free(d);
}
extern "C" {
    void persons_data_user_name(const Persons::Private*, int, QString*, qstring_set);
    bool persons_set_data_user_name(Persons::Private*, int, qstring_t);
    void persons_sort(Persons::Private*, int column, Qt::SortOrder order = Qt::AscendingOrder);

    int persons_row_count(const Persons::Private*);
    bool persons_can_fetch_more(const Persons::Private*);
    void persons_fetch_more(Persons::Private*);
}
int Persons::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Persons::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Persons::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : persons_row_count(d);
}

QModelIndex Persons::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && column == 0) {
        return createIndex(row, 0, (quintptr)0);
    }
    return QModelIndex();
}

QModelIndex Persons::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Persons::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : persons_can_fetch_more(d);
}

void Persons::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        persons_fetch_more(d);
    }
}

void Persons::sort(int column, Qt::SortOrder order)
{
    persons_sort(d, column, order);
}
Qt::ItemFlags Persons::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    if (i.column() == 0) {
        flags |= Qt::ItemIsEditable;
    }
    return flags;
}
QVariant Persons::data(const QModelIndex &index, int role) const
{
    QVariant v;
    QString s;
    QByteArray b;
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::DisplayRole:
            persons_data_user_name(d, index.row(), &s, set_qstring);
            if (!s.isNull()) v.setValue<QString>(s);
            break;
        case Qt::EditRole:
            persons_data_user_name(d, index.row(), &s, set_qstring);
            if (!s.isNull()) v.setValue<QString>(s);
            break;
        }
        break;
    }
    return v;
}
QHash<int, QByteArray> Persons::roleNames() const {
    QHash<int, QByteArray> names;
    names.insert(Qt::DisplayRole, "userName");
    return names;
}
bool Persons::setData(const QModelIndex &index, const QVariant &value, int role)
{
    bool set = false;
    if (index.column() == 0) {
        if (role == Qt::EditRole) {
            set = persons_set_data_user_name(d, index.row(), value.value<QString>());
        }
    }
    if (set) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
    return set;
}
