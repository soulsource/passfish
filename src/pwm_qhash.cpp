#include <QCryptographicHash>

extern "C" {
    /**
     *  Forwards the call to QCryptographicHash::hash() and copies the output into the provided buffer.
     *  Returns zero on error (unsupported hash type or insufficient output_capacity). Number of written output bytes otherwise.
     */
    Q_DECL_EXPORT size_t pwm_qhash(size_t algorithm, const unsigned char * input, size_t input_length, unsigned char * output, size_t output_capacity)
    {
        if(algorithm > 6)
            return 0; //failed.
        const auto algo{static_cast<QCryptographicHash::Algorithm>(algorithm)};
        QCryptographicHash hasher{algo};
        hasher.addData(reinterpret_cast<const char *>(input), input_length);
        const auto hash{hasher.result()};
        const auto count_as_size{static_cast<size_t>(hash.length())};
        if(count_as_size > output_capacity)
            return 0;
        memcpy(output,hash.constData(),count_as_size);
        return count_as_size;
    }
}
