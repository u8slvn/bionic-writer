# Bionic Writer

**Bio**nic **Wri**ter **i**s **a** **Pyt**hon **lib**rary **t**o **wr**ite **te**xt **wi**th **Bio**nic **Rea**ding **st**yle **f**or **a**ny **ki**nd **o**f **for**mat.

## About Bionic Reading method

From: [bionic-reading.com/](https://bionic-reading.com/br-method/)

> Bionic Reading® revises texts so that the most concise parts of words are highlighted. This guides the eye over the text and the brain remembers previously learned words more quickly. Save precious time. Gain advantages. Learn new things faster.

Does Bionic Reading really speed-up your reading speed? According to this [paper](https://blog.readwise.io/bionic-reading-results/), probably not. Whatsoever it's still a nice screen reading technique that seems to help many people to stay focused during their reading session.

## Usage

### Install

```bash
pip install bionic-writer
```

### Quickstart examples

```python
import bionic_writer

text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."

# Write a stylized Bionic Reading text in Markdown format.
bionic_md = bionic_writer.write(text=text, affix="**", postfix="**")

# Write a stylized Bionic Reading text in HTML format.
bionic_html = bionic_writer.write(text=text, affix="<b>", postfix="</b>")

print("Markdown:\n", bionic_md, "\n")
print("HTML:\n", bionic_html)
```

Output:

```text
Markdown:
**Lo**rem **ip**sum **do**lor **s**it **am**et, **conse**ctetur **adipi**scing **el**it.

HTML:
<b>Lo</b>rem <b>ip</b>sum <b>do</b>lor <b>s</b>it <b>am</b>et, <b>conse</b>ctetur <b>adipi</b>scing <b>el</b>it.
```
