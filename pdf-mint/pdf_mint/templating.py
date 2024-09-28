

def first(text, render):
    # return only first occurance of items
    result = render(text)
    return [ x.strip() for x in result.split(" || ") if x.strip() ][0]

def inject_x(text, render):
    # inject data into scope
    return render(text, {'x': 'data'})


def toc(text, render):
    # create table of contents
    result = render(text)
    lines = [ x.strip() for x in result.split(" || ") if x.strip() ]
    return "<ul>" + "".join([f"<li>{x}</li>" for x in lines]) + "</ul>"


lambdas = {
    'first': first,
    'inject_x': inject_x,
    'toc': toc
}