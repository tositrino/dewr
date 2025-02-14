#
# main makefile, runs Makefiles in subdirectories
#
TOPTARGETS := all build clean check help doc format info lint release test 
SUBDIRS := $(wildcard module_??/.)

$(TOPTARGETS): $(SUBDIRS)
$(SUBDIRS):
	@echo "[$@ $(MAKECMDGOALS)]"
	$(MAKE) -C $@ $(MAKECMDGOALS)

.PHONY: $(TOPTARGETS) $(SUBDIRS)
