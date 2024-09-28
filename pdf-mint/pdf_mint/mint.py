
import chevron


from templating import lambdas

def hydrate(template, data:dict):
    return chevron.render(template, data | lambdas)

from weasyprint import HTML

def convert(html_content = None, base_url = None):
    html = HTML(string=html_content, base_url=base_url)
    document = html.render()
    pdfstream = document.write_pdf()
    return pdfstream

def create(template, data, base_url=None):
    html = hydrate(template, data)
    pdf = convert(html, base_url=base_url)
    return pdf
