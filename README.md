# python-rustvocab
Vocabulary preprocessing functions written in Rust for Python.

## Building

On MacOS:

```
cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
mv target/release/libvocab.dynlib target/release/libvocab.d
cp target/release/libvocab.d <python code dir>
cd <python code dir>
python -c "import vocab"
```

## Usage

* vocab.count_words - count words from list of lists and return n most common words

```
import vocab
a = [["foo", "bar"], ["bar", "bas"], ["bar", "foo"]]
vocab.count_words(a)  # [('bar', 3), ('foo', 2)]
```


