#ifndef GRAPHEMECOUNTVALIDATOR_H
#define GRAPHEMECOUNTVALIDATOR_H

#include <QValidator>

class Q_GUI_EXPORT GraphemeCountValidator : public QValidator
{
    Q_OBJECT
    Q_PROPERTY(uint minGraphemeCount READ minGraphemeCount WRITE setMinGraphemeCount NOTIFY minGraphemeCountChanged)
public:
    explicit GraphemeCountValidator(QObject *parent = nullptr);
    explicit GraphemeCountValidator(uint min_count, QObject *parent = nullptr);
    virtual QValidator::State validate(QString &, int &) const override;

    uint minGraphemeCount() const;
    void setMinGraphemeCount(uint c);


Q_SIGNALS:
    void minGraphemeCountChanged(uint minGraphemeCount);

private:
    uint minCount;
};

#endif // GRAPHEMECOUNTVALIDATOR_H
