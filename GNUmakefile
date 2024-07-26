# spell-checker:ignore (misc) testsuite runtest findstring (targets) busytest toybox distclean pkgs nextest ; (vars/env) BINDIR BUILDDIR CARGOFLAGS DESTDIR DOCSDIR INSTALLDIR INSTALLEES MULTICALL DATAROOTDIR TESTDIR manpages

# Config options
PROFILE         ?= debug
##gnostr default is MULTICALL
MULTICALL       ?= y
INSTALL         ?= install
ifneq (,$(filter install, $(MAKECMDGOALS)))
override PROFILE:=release
endif

PROFILE_CMD :=
ifeq ($(PROFILE),release)
	PROFILE_CMD = --release
endif

RM := rm -rf

# Binaries
CARGO  ?= cargo
CARGOFLAGS ?=

# Install directories
PREFIX ?= /usr/local
DESTDIR ?=
BINDIR ?= $(PREFIX)/bin
DATAROOTDIR ?= $(PREFIX)/share

INSTALLDIR_BIN=$(DESTDIR)$(BINDIR)

#prefix to apply to coreutils binary and all tool binaries
PROG_PREFIX =gnostr-
export PROG_PREFIX

# This won't support any directory with spaces in its name, but you can just
# make a symlink without spaces that points to the directory.
BASEDIR       ?= $(shell pwd)
BUILDDIR      := $(BASEDIR)/target/${PROFILE}
PKG_BUILDDIR  := $(BUILDDIR)/deps
DOCSDIR       := $(BASEDIR)/docs

BUSYBOX_ROOT := $(BASEDIR)/tmp
BUSYBOX_VER  := 1.35.0
BUSYBOX_SRC  := $(BUSYBOX_ROOT)/busybox-$(BUSYBOX_VER)

TOYBOX_ROOT := $(BASEDIR)/tmp
TOYBOX_VER  := 0.8.8
TOYBOX_SRC  := $(TOYBOX_ROOT)/toybox-$(TOYBOX_VER)

ifeq ($(SELINUX_ENABLED),)
	SELINUX_ENABLED := 0
	ifneq ($(OS),Windows_NT)
		ifeq ($(shell /sbin/selinuxenabled 2>/dev/null ; echo $$?),0)
			SELINUX_ENABLED := 1
		endif
	endif
endif

# Possible programs
# the function definition may have undescores
# while the resulting executable should have dashes
# so PROGS list has dashes
PROGS       := \
	base32 \
	base64 \
	basenc \
	basename \
	bech32 \
	blockheight \
	cat \
	cksum \
	cli \
	comm \
	cp \
	csplit \
	cut \
	date \
	dd \
	df \
	dir \
	dircolors \
	dirname \
	echo \
	encrypt-privkey \
	encrypt_privkey \
	env \
	expand \
	expr \
	factor \
	false \
	fmt \
	fold \
	get-relays \
	get_relays \
	git \
	hashsum \
	head \
	join \
	keypair \
	legit \
	link \
	ln \
	ls \
	mkdir \
	mktemp \
	more \
	mv \
	nl \
	nip11 \
	numfmt \
	nproc \
	od \
	paste \
	post \
	post-event \
	post_event \
	pr \
	printenv \
	printf \
	privkey-to-bech32 \
	privkey_to_bech32 \
	ptx \
	pubkey-to-bech32 \
	pubkey_to_bech32 \
	pwd \
	readlink \
	realpath \
	reflog \
	rm \
	rmdir \
	seq \
	shred \
	shuf \
	sleep \
	sort \
	split \
	sum \
	sync \
	tac \
	tail \
	template \
	tee \
	test \
	tr \
	true \
	truncate \
	tsort \
	tui \
	unexpand \
	uniq \
	vdir \
	wc \
	weeble \
	wobble \
	whoami \
	xq \
	yes

UNIX_PROGS := \
	arch \
	chgrp \
	chmod \
	chown \
	chroot \
	du \
	groups \
	hostid \
	hostname \
	id \
	install \
	kill \
	logname \
	mkfifo \
	mknod \
	nice \
	nohup \
	pathchk \
	pinky \
	sleep \
	stat \
	stdbuf \
	timeout \
	touch \
	tty \
	uname \
	unlink \
	uptime \
	users \
	who

SELINUX_PROGS := \
	chcon \
	runcon

ifneq ($(OS),Windows_NT)
	PROGS := $(PROGS) $(UNIX_PROGS)
endif

ifeq ($(SELINUX_ENABLED),1)
	PROGS := $(PROGS) $(SELINUX_PROGS)
endif

UTILS ?= $(PROGS)

# Programs with usable tests
TEST_PROGS  := \
	base32 \
	base64 \
	basename \
	bech32 \
	blockheight \
	cat \
	chcon \
	chgrp \
	chmod \
	chown \
	cksum \
	cli \
	comm \
	cp \
	csplit \
	cut \
	date \
	dircolors \
	dirname \
	echo \
	encrypt-privkey \
	env \
	expr \
	factor \
	false \
	fold \
	get-relays \
	git \
	hashsum \
	head \
	install \
	keypair \
	legit \
	link \
	ln \
	ls \
	mkdir \
	mktemp \
	mv \
	nl \
	nip11 \
	numfmt \
	od \
	paste \
	pathchk \
	pinky \
	pr \
	printf \
	privkey-to-bech32 \
	ptx \
	pubkey-to-bech32 \
	pwd \
	readlink \
	realpath \
	reflog \
	rm \
	rmdir \
	runcon \
	seq \
	sleep \
	sort \
	split \
	stat \
	stdbuf \
	sum \
	tac \
	tail \
	template \
	test \
	touch \
	tr \
	true \
	truncate \
	tsort \
	tui \
	uname \
	unexpand \
	uniq \
	unlink \
	wc \
	who

TESTS       := \
	$(sort $(filter $(UTILS),$(filter-out $(SKIP_UTILS),$(TEST_PROGS))))

TEST_NO_FAIL_FAST :=
TEST_SPEC_FEATURE :=
ifneq ($(SPEC),)
TEST_NO_FAIL_FAST :=--no-fail-fast
TEST_SPEC_FEATURE := test_unimplemented
else ifeq ($(SELINUX_ENABLED),1)
TEST_NO_FAIL_FAST :=
TEST_SPEC_FEATURE := feat_selinux
endif

define TEST_BUSYBOX
test_busybox_$(1):
	-(cd $(BUSYBOX_SRC)/testsuite && bindir=$(BUILDDIR) ./runtest $(RUNTEST_ARGS) $(1))
endef

# Output names
## we subst dashes for underscores in the PROGS list here
## REF: PROGS and build: build-pkgs build-coreutils
EXES        := \
	$(sort $(filter $(UTILS),$(filter-out $(SKIP_UTILS),$(subst -,_,$(PROGS)))))

INSTALLEES  := ${EXES}
ifeq (${MULTICALL}, y)
INSTALLEES  := ${INSTALLEES} coreutils
endif

all: build

do_install = $(INSTALL) ${1}
use_default := 1

build-pkgs:
	echo $(EXES)
ifneq (${MULTICALL}, y)
	${CARGO} build ${CARGOFLAGS} ${PROFILE_CMD} $(foreach pkg,$(EXES),-p uu_$(subst -,_,$(pkg)))
	##${CARGO} build ${CARGOFLAGS} ${PROFILE_CMD} $(foreach pkg,$(EXES),-p uu_$(subst _,-,$(pkg)))
	##${CARGO} build ${CARGOFLAGS} ${PROFILE_CMD} $(foreach pkg,$(EXES),-p uu_$(subst -,_,$(pkg)))
endif

build-coreutils:
	${CARGO} build ${CARGOFLAGS} --features "${EXES}" ${PROFILE_CMD} --no-default-features

build: build-coreutils build-pkgs

$(foreach test,$(filter-out $(SKIP_UTILS),$(PROGS)),$(eval $(call TEST_BUSYBOX,$(test))))

test:
	${CARGO} test ${CARGOFLAGS} --features "$(TESTS) $(TEST_SPEC_FEATURE)" --no-default-features $(TEST_NO_FAIL_FAST)

nextest:
	${CARGO} nextest run ${CARGOFLAGS} --features "$(TESTS) $(TEST_SPEC_FEATURE)" --no-default-features $(TEST_NO_FAIL_FAST)

test_toybox:
	-(cd $(TOYBOX_SRC)/ && make tests)

toybox-src:
	if [ ! -e "$(TOYBOX_SRC)" ] ; then \
		mkdir -p "$(TOYBOX_ROOT)" ; \
		wget "https://github.com/landley/toybox/archive/refs/tags/$(TOYBOX_VER).tar.gz" -P "$(TOYBOX_ROOT)" ; \
		tar -C "$(TOYBOX_ROOT)" -xf "$(TOYBOX_ROOT)/$(TOYBOX_VER).tar.gz" ; \
		sed -i -e "s|TESTDIR=\".*\"|TESTDIR=\"$(BUILDDIR)\"|g" $(TOYBOX_SRC)/scripts/test.sh; \
		sed -i -e "s/ || exit 1//g" $(TOYBOX_SRC)/scripts/test.sh; \
	fi ;

busybox-src:
	if [ ! -e "$(BUSYBOX_SRC)" ] ; then \
		mkdir -p "$(BUSYBOX_ROOT)" ; \
		wget "https://busybox.net/downloads/busybox-$(BUSYBOX_VER).tar.bz2" -P "$(BUSYBOX_ROOT)" ; \
		tar -C "$(BUSYBOX_ROOT)" -xf "$(BUSYBOX_ROOT)/busybox-$(BUSYBOX_VER).tar.bz2" ; \
	fi ;

# This is a busybox-specific config file their test suite wants to parse.
$(BUILDDIR)/.config: $(BASEDIR)/.busybox-config
	cp $< $@

# Test under the busybox test suite
$(BUILDDIR)/busybox: busybox-src build-coreutils $(BUILDDIR)/.config
	cp "$(BUILDDIR)/coreutils" "$(BUILDDIR)/busybox"
	chmod +x $@

prepare-busytest: $(BUILDDIR)/busybox
	# disable inapplicable tests
	-( cd "$(BUSYBOX_SRC)/testsuite" ; if [ -e "busybox.tests" ] ; then mv busybox.tests busybox.tests- ; fi ; )

ifeq ($(EXES),)
busytest:
else
busytest: $(BUILDDIR)/busybox $(addprefix test_busybox_,$(filter-out $(SKIP_UTILS),$(EXES)))
endif

clean:
	cargo clean
	cd $(DOCSDIR) && $(MAKE) clean

distclean: clean
	$(CARGO) clean $(CARGOFLAGS) && $(CARGO) update $(CARGOFLAGS)

manpages: build-coreutils
	mkdir -p $(BUILDDIR)/man/
	$(foreach prog, $(INSTALLEES), \
		$(BUILDDIR)/coreutils manpage $(prog) > $(BUILDDIR)/man/$(PROG_PREFIX)$(prog).1; \
	)
	$(foreach prog, $(INSTALLEES), \
		$(BUILDDIR)/gnostr-rs manpage $(prog) > $(BUILDDIR)/man/$(PROG_PREFIX)$(prog).1; \
	)

completions: build-coreutils
	mkdir -p $(BUILDDIR)/completions/zsh $(BUILDDIR)/completions/bash $(BUILDDIR)/completions/fish
	$(foreach prog, $(INSTALLEES), \
		$(BUILDDIR)/coreutils completion $(prog) zsh > $(BUILDDIR)/completions/zsh/_$(PROG_PREFIX)$(prog); \
		$(BUILDDIR)/coreutils completion $(prog) bash > $(BUILDDIR)/completions/bash/$(PROG_PREFIX)$(prog); \
		$(BUILDDIR)/coreutils completion $(prog) fish > $(BUILDDIR)/completions/fish/$(PROG_PREFIX)$(prog).fish; \
	)

install: build manpages completions
	mkdir -p $(INSTALLDIR_BIN)
	echo $(EXES)
	echo $(INSTALLEES)
ifeq (${MULTICALL}, y)
	$(INSTALL) $(BUILDDIR)/coreutils $(INSTALLDIR_BIN)/$(PROG_PREFIX)coreutils
	$(INSTALL) $(BUILDDIR)/gnostr-rs $(INSTALLDIR_BIN)/$(PROG_PREFIX)gnostr-rs
	$(INSTALL) $(BUILDDIR)/git-gnostr $(INSTALLDIR_BIN)/$(PROG_PREFIX)git-gnostr
	cd $(INSTALLDIR_BIN) && $(foreach prog, $(filter-out coreutils, $(INSTALLEES)), \
		ln -fs $(PROG_PREFIX)coreutils $(PROG_PREFIX)$(prog) &&) :
	#	ln -fs $(PROG_PREFIX)coreutils $(PROG_PREFIX)$(subst _,-,$(prog)) &&) :
	#	ln -fs $(PROG_PREFIX)coreutils $(PROG_PREFIX)$(subst -,_,$(prog)) &&) :
	$(if $(findstring test,$(INSTALLEES)), cd $(INSTALLDIR_BIN) && ln -fs $(PROG_PREFIX)coreutils $(PROG_PREFIX)[)
else
	$(foreach prog, $(INSTALLEES), \
		$(INSTALL) $(BUILDDIR)/$(prog) $(INSTALLDIR_BIN)/$(PROG_PREFIX)$(subst -,_,$(prog));)
	#	$(INSTALL) $(BUILDDIR)/$(prog) $(INSTALLDIR_BIN)/$(PROG_PREFIX)$(subst _,-,$(prog));)
	$(if $(findstring test,$(INSTALLEES)), $(INSTALL) $(BUILDDIR)/test $(INSTALLDIR_BIN)/$(PROG_PREFIX)[)
endif
	$(SUDO) mkdir -p $(DESTDIR)$(DATAROOTDIR)/man/man1
	$(foreach prog, $(INSTALLEES), \
		$(INSTALL) $(BUILDDIR)/man/$(PROG_PREFIX)$(prog).1 $(DESTDIR)$(DATAROOTDIR)/man/man1/; \
	)

	$(SUDO) mkdir -p $(DESTDIR)$(DATAROOTDIR)/zsh/site-functions
	$(SUDO) mkdir -p $(DESTDIR)$(DATAROOTDIR)/bash-completion/completions
	$(SUDO) mkdir -p $(DESTDIR)$(DATAROOTDIR)/fish/vendor_completions.d
	$(foreach prog, $(INSTALLEES), \
		$(INSTALL) $(BUILDDIR)/completions/zsh/_$(PROG_PREFIX)$(prog) $(DESTDIR)$(DATAROOTDIR)/zsh/site-functions/; \
		$(INSTALL) $(BUILDDIR)/completions/bash/$(PROG_PREFIX)$(prog) $(DESTDIR)$(DATAROOTDIR)/bash-completion/completions/; \
		$(INSTALL) $(BUILDDIR)/completions/fish/$(PROG_PREFIX)$(prog).fish $(DESTDIR)$(DATAROOTDIR)/fish/vendor_completions.d/; \
	)

uninstall:
ifeq (${MULTICALL}, y)
	rm -f $(addprefix $(INSTALLDIR_BIN)/,$(PROG_PREFIX)coreutils)
endif
	rm -f $(addprefix $(INSTALLDIR_BIN)/$(PROG_PREFIX),$(PROGS))
	rm -f $(INSTALLDIR_BIN)/$(PROG_PREFIX)[
	rm -f $(addprefix $(DESTDIR)$(DATAROOTDIR)/zsh/site-functions/_$(PROG_PREFIX),$(PROGS))
	rm -f $(addprefix $(DESTDIR)$(DATAROOTDIR)/bash-completion/completions/$(PROG_PREFIX),$(PROGS))
	rm -f $(addprefix $(DESTDIR)$(DATAROOTDIR)/fish/vendor_completions.d/$(PROG_PREFIX),$(addsuffix .fish,$(PROGS)))
	rm -f $(addprefix $(DESTDIR)$(DATAROOTDIR)/man/man1/$(PROG_PREFIX),$(addsuffix .1,$(PROGS)))

.PHONY: all build build-coreutils build-pkgs test distclean clean busytest install uninstall

-include cargo.mk
