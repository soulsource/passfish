#include "GraphemeCountValidator.h"
#include <QTextBoundaryFinder>

GraphemeCountValidator::GraphemeCountValidator(QObject *parent)
    : GraphemeCountValidator(0, parent)
{}

GraphemeCountValidator::GraphemeCountValidator(uint min_count, QObject *parent)
    : QValidator(parent)
    , minCount(min_count)
{}

uint GraphemeCountValidator::minGraphemeCount() const
{
    return minCount;
}

void GraphemeCountValidator::setMinGraphemeCount(uint c)
{
    if(minCount != c)
    {
        minCount = c;
        emit minGraphemeCountChanged(minCount);
        emit changed();
    }
}

QValidator::State GraphemeCountValidator::validate(QString & text, int&) const
{
    //One could write Rust FFI bindings and do this in Rust.
    //But that would be way more work for no gain.
    QTextBoundaryFinder boundsFinder{
        QTextBoundaryFinder::BoundaryType::Grapheme,
        text
    };
    int requiredBoundary = 0;
    for(uint i = 0; i < minCount; ++i)
        requiredBoundary = boundsFinder.toNextBoundary();
    return requiredBoundary >= 0
            ? QValidator::State::Acceptable
            : QValidator::State::Intermediate;
}
