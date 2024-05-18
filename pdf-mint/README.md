# PDF Mint

## About

This tool renders PDFs from HTML documents using [Weasyprint](https://weasyprint.org/).

It uses templating and includes to allow for more complex PDF minting

It provides an API for minting pdfs so it can be used as a service.

It comes dockerised for ease of deployment.

## Usage

### API

```python
import mint from pdf_mint

template = "<html>...</html>"
data = { foo: "hello" }
resources = [...]
plugins = [...]

pdf = mint.create(template, plugins, data, resources)
```

## Notes

It generates [PDF/A-4](https://en.wikipedia.org/wiki/PDF/A) (latest archival quality pdf format) compliant files which can be verified using [Verapdf](https://verapdf.org/home/).
