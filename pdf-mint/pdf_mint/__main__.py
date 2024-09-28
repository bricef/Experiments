


from pathlib import Path


import typer

import mint

from weasyprint import HTML

app  = typer.Typer()

@app.command()
def generate (input: Path, output: Path = "./output.pdf", data = {}):
    """Converts a HTML document to a PDF file."""

    with open(input, "r") as f:
        html_content = f.read()

    
    pdf_content = mint.create(
        template=html_content,
        data={'custname':'John Doe'},
        base_url=input.parent
    )
    
    with open(output, "wb") as f:
        f.write(pdf_content)

@app.command()
def from_url(url: str, output: Path = "./output.pdf"):
    """Converts a URL to a PDF file."""
    HTML(url).write_pdf(output)


@app.command()
def version():
    """Prints the version of the PDF Mint application."""
    typer.echo("PDF Mint version 0.1.0")

if __name__ == "__main__":
    app()