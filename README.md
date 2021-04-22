# pdfxopen

pdfxopen is a package consists of two executable tools: pdfxopen and pdfxclose. It is a equivalent of [xpdfopen](https://www.ctan.org/pkg/xpdfopen) package in Tex Live for PDF-XChange Editor. It is mainly used for resolving the file-locking problem for PDF-XChange Editor (similar to the problem that **xpdfopen** aims to resolve for Adobe Acrobat Reader), so that manually terminating PDF viewer instance is no longer required in LaTeX compiling-previewing workflow.

Currently only work for Windows.

## Compile
```sh
cargo build --release
```

## Usage

### Basic Usage
```sh
$ pdfxopen C:/test.pdf
```

```sh
$ pdfxclose C:/test.pdf
```

### Example Usage with LaTeXmk

```makefile
[...]
$compiling_cmd = 'pdfxclose %D';
$success_cmd = 'pdfxopen %D';
```
