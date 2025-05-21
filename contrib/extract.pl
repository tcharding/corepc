#!/usr/bin/perl
# SPDX-License-Identifier: CC0-1.0
#
# Create the `types/src/vXYZ/mod.rs` rusdoc.

use strict;
use warnings;
use Getopt::Long qw(:config no_auto_abbrev);

# The script name.
my $SCRIPT = $0;

# The Bitcoin Core version we are working with.
my $CORE_VERSION = "0.17";
# The file holding output of `bitcoin-cli --help`.
my $RPC_HELP_FILE = "types/src/v17/rpc-api.txt";

# Command line options.
my $help = 0;
my $debug = 0;

sub help
{
	my ($exitcode) = @_;

	print << "EOM";

Usage: $SCRIPT [OPTIONS]

Options:

	-d, --debug			Display debugging output.
	-h, --help			Display this help and exit.

Generates the rustdocs for the `types/vX.rs` module. Before running script set CORE_VERSION and RPC_HELP_FILE.

EOM
	exit($exitcode);
}

GetOptions(
	'd|debug'		=> \$debug,
	'h|help'		=> \$help,
) or help(1);

help(0) if ($help);

main();

exit 0;

sub dprint
{
	printf(STDERR @_) if $debug;
}

sub main {

    # Open the file for reading
    open(my $fh, '<', $RPC_HELP_FILE) or die "Could not open file '$RPC_HELP_FILE': $!\n";

    # Loop over each line in the file
    while (my $line = <$fh>) {
        chomp($line);
        if ($line =~ /^== (.+) ==/) { # Section heading.
            my $section = $1; # Captures the placeholder text
            start_section($section);
        } elsif ($line =~ /^\s*$/) { # Blank line.
            end_section();
        } else {
            add_method($line);
        }
    }

    end_section();
    print_footer();

    # Close the file handle.
    close($fh);
}

sub print_header {
    print <<'EOM';
//! # JSON-RPC types for Bitcoin Core `v$CORE_VERSION`
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific. The
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
//!
//! ### Method name and implementation status
//!
//! Every JSON-RPC method supported by this version of Bitcoin Core is listed below along with its
//! current implementation status.
//!
EOM
}

sub print_footer {
    print <<'EOM'
//!
//! **Items marked omitted were omitted because:**
//!
//! - Method does not return anything.
//! - Method returns a simple type (e.g. bool or integer).
//! - Method is deprecated.
EOM
}

# Print the blurb for the start of a section.
sub start_section {
    my ($section) = @_; # Get the first argument

    print <<'EOM';
//! <details>
//! <summary> Methods from the $section section </summary>
//!
//! | JSON-RPC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
EOM
}

# Print the blurb for the end of a section.
sub end_section {
    print<<'EOM';
//!
//! </details>
//!
EOM
}

sub add_method {
    my ($line) = @_;
    if ($line =~ /^(\S+)/) {
        my ($method) = $1;
        printf "//! \| %-34s \|                 |\n", $method
    }
}
