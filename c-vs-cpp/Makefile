
BUILDIR=build

CFLAGS+=-g -Wall -O3 -march=native
CFLAGS+=$(shell pkg-config --cflags glib-2.0)
CLIBS+=$(shell pkg-config --libs glib-2.0)

CPPFLAGS+= -g -Wall -O3 -march=native

APPS=c-glib c-hash cpp-cread-stl cpp-ego-booster cpp-hash cpp-stl
BINS=$(addprefix $(BUILDIR)/,$(APPS))

all: $(BUILDIR) $(BINS) 

$(BUILDIR): 
	mkdir -p $@

$(BUILDIR)/%: %.cpp
	g++ $^ $(CPPFLAGS) $(CPPLIBS) -o $@ 

$(BUILDIR)/%: %.c
	gcc $^ $(CFLAGS) $(CLIBS) -o $@
	
clean: 
	rm -rf $(BUILDIR)

.PHONY: clean

#  valgrind --leak-check=full ./uniqwords-cpp < uniquewords.c
