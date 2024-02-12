# Bionic Writer

[![Pypi Version](https://img.shields.io/pypi/v/bionic-writer.svg)](https://pypi.org/project/bionic-writer/)
[![Python Version](https://img.shields.io/pypi/pyversions/bionic-writer)](https://pypi.org/project/bionic-writer/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/u8slvn/bionic-writer/ci.yml)](https://github.com/u8slvn/bionic-writer/actions/workflows/ci.yml)
[![Codecov](https://img.shields.io/codecov/c/gh/u8slvn/bionic-writer)](https://app.codecov.io/gh/u8slvn/bionic-writer)
[![Project license](https://img.shields.io/pypi/l/bionic-writer)](https://pypi.org/project/bionic-writer/)

**Bio**nic **Wri**ter **i**s **a** **Pyt**hon **lib**rary **t**o **wr**ite **te**xt **wi**th **Bio**nic **Rea**ding **st**yle **f**or **a**ny **ki**nd **o**f **for**mat.

## About Bionic Reading method

From: [bionic-reading.com/](https://bionic-reading.com/br-method/)

> Bionic Reading® revises texts so that the most concise parts of words are highlighted. This guides the eye over the text and the brain remembers previously learned words more quickly. Save precious time. Gain advantages. Learn new things faster.

Does Bionic Reading really speed-up your reading speed? According to this [paper](https://blog.readwise.io/bionic-reading-results/), probably not. Whatsoever it's still a nice screen reading technique that seems to help many people to stay focused during their reading sessions.

## Usage

### Installation
```bash
pip install bionic-writer
```

### Quickstart examples

#### Write a Bionic Reading text in Markdown format:

```python
import bionic_writer

text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."

bionic_md = bionic_writer.write(text=text, affix="**", postfix="**")

print(bionic_md)
```
Output:

```text
**Lo**rem **ip**sum **do**lor **s**it **am**et, **conse**ctetur **adipi**scing **el**it.
```

#### Write a Bionic Reading text in HTML format:
```python
import bionic_writer

text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."

bionic_html = bionic_writer.write(text=text, affix="<b>", postfix="</b>")

print(bionic_html)
```

Output:

```text
<b>Lo</b>rem <b>ip</b>sum <b>do</b>lor <b>s</b>it <b>am</b>et, <b>conse</b>ctetur <b>adipi</b>scing <b>el</b>it.
```
