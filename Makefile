.PHONY: all clean mona

all: forverif.pdf


forverif.pdf: forverif.tex
	pdflatex forverif.tex
	pdflatex forverif.tex

mona: 
	cd MONA && ./configure
	cd MONA && make
	ln -s MONA/Front/mona ./mona

clean:
	rm -f *.aux *.log *.pdf *.out
