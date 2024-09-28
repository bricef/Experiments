


from weasyprint import HTML, CSS
from weasyprint.text.fonts import FontConfiguration

import typer

def main (name: str):
    
    font_config = FontConfiguration()

    html = HTML(string='<h1>The title</h1>')

    css = CSS(string='''
        @font-face {
            font-family: Gentium;
            src: url(https://example.com/fonts/Gentium.otf);
        }
        h1 { font-family: Gentium }''', 
        font_config=font_config
    )

    document = html.render(stylesheets=[css], font_config=font_config)

    pdfstream = document.write_pdf()

if __name__ == "__main__":
    typer.run(main)